/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::borrow::{Cow, ToOwned};
use std::cell::OnceCell;
use std::default::Default;
use std::ops::Deref;
use std::ptr::{self, NonNull};
use std::str::{EncodeUtf16, FromStr};
use std::sync::LazyLock;
use std::{fmt, slice, str};

use ascii::ToAsciiChar;
use html5ever::{LocalName, Namespace};
use js::conversions::{ToJSValConvertible, jsstr_to_string};
use js::gc::MutableHandleValue;
use js::jsapi::{Heap, JS_GetLatin1StringCharsAndLength, JSContext, JSString};
use js::rust::Trace;
use malloc_size_of::MallocSizeOfOps;
use regex::Regex;
use style::Atom;
use style::str::HTML_SPACE_CHARACTERS;
use tendril::encoding_rs;

fn char_to_latin1_u8(c: char) -> u8 {
    c.to_ascii_char().unwrap().into()
}

fn latin1_u8_to_char(c: u8) -> char {
    c.to_ascii_char().unwrap().into()
}

#[derive(Copy, Clone, Debug)]
pub enum EncodedBytes<'a> {
    Latin1Bytes(&'a [u8]),
    Utf8Bytes(&'a str),
}

impl<'a> EncodedBytes<'a> {
    pub fn split_commas(self) -> Box<dyn Iterator<Item = EncodedBytes<'a>> + 'a> {
        match self {
            EncodedBytes::Latin1Bytes(s) => Box::new(
                s.split(|byte| *byte == char_to_latin1_u8(','))
                    .map(EncodedBytes::Latin1Bytes),
            ),
            EncodedBytes::Utf8Bytes(s) => Box::new(s.split(',').map(EncodedBytes::Utf8Bytes)),
        }
    }

    pub fn char_indices(self) -> Box<dyn Iterator<Item = (usize, char)> + 'a> {
        match self {
            EncodedBytes::Latin1Bytes(items) => Box::new(
                items
                    .iter()
                    .enumerate()
                    .map(|(index, c)| (index, latin1_u8_to_char(*c))),
            ),
            EncodedBytes::Utf8Bytes(s) => Box::new(s.char_indices()),
        }
    }
}

impl<'a> PartialEq<str> for EncodedBytes<'a> {
    fn eq(&self, other: &str) -> bool {
        match self {
            EncodedBytes::Utf8Bytes(s) => *s == other,
            EncodedBytes::Latin1Bytes(s) => {
                let v = s.iter().map(|c| *c as char as u8).collect::<Vec<u8>>();
                v == *s
            },
        }
    }
}

impl<'a> PartialEq<&str> for EncodedBytes<'a> {
    fn eq(&self, other: &&str) -> bool {
        match self {
            EncodedBytes::Utf8Bytes(s) => s == other,
            EncodedBytes::Latin1Bytes(s) => {
                let v = s.iter().map(|c| *c as char as u8).collect::<Vec<u8>>();
                &String::from_utf8(v).unwrap() == other
            },
        }
    }
}

impl<'a> PartialEq<&str> for Box<EncodedBytes<'a>> {
    fn eq(&self, other: &&str) -> bool {
        match self.deref() {
            EncodedBytes::Utf8Bytes(s) => s == other,
            EncodedBytes::Latin1Bytes(s) => {
                let v = s.iter().map(|c| *c as char as u8).collect::<Vec<u8>>();
                &String::from_utf8(v).unwrap() == other
            },
        }
    }
}

#[cfg_attr(crown, allow(crown::unrooted_must_root))]
////// A DOMString.
///
/// This type corresponds to the [`DOMString`] type in WebIDL.
///
/// [`DOMString`]: https://webidl.spec.whatwg.org/#idl-DOMString
///
/// Conceptually, a DOMString has the same value space as a JavaScript String,
/// i.e., an array of 16-bit *code units* representing UTF-16, potentially with
/// unpaired surrogates present (also sometimes called WTF-16).
///
/// However, Rust `String`s are guaranteed to be valid UTF-8, and as such have
/// a *smaller value space* than WTF-16 (i.e., some JavaScript String values
/// can not be represented as a Rust `String`). This introduces the question of
/// what to do with values being passed from JavaScript to Rust that contain
/// unpaired surrogates.
///
/// The hypothesis is that it does not matter much how exactly those values are
/// transformed, because  passing unpaired surrogates into the DOM is very rare.
/// Instead Servo withh replace the unpaired surrogate by a U+FFFD replacement
/// character.
///
/// Currently, the lack of crash reports about this issue provides some
/// evidence to support the hypothesis. This evidence will hopefully be used to
/// convince other browser vendors that it would be safe to replace unpaired
/// surrogates at the boundary between JavaScript and native code. (This would
/// unify the `DOMString` and `USVString` types, both in the WebIDL standard
/// and in Servo.)
///
/// This string class will keep either the Reference to the mozjs object alive
/// or will have an internal rust string.
/// We currently default to doing most of the string operation on the rust side.
/// As this conversion was anyway needed, it does not much extra cost.
/// You should assume that all the functions incur the conversion cost.
///
pub struct DOMString {
    rust_string: OnceCell<String>,
    js_context: Option<*mut JSContext>,
    js_string: Option<std::boxed::Box<Heap<*mut JSString>>>,
}

impl std::fmt::Debug for DOMString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LazyDOMString")
            .field("rust_string", &self.rust_string)
            .finish()
    }
}

impl Clone for DOMString {
    fn clone(&self) -> Self {
        self.make_me_string();
        Self {
            rust_string: self.rust_string.clone(),
            js_context: None,
            js_string: None,
        }
    }
}

unsafe impl Trace for DOMString {
    unsafe fn trace(&self, tracer: *mut js::jsapi::JSTracer) {
        // We can safely delete the jsstring if we already converted to a rust string.
        if self.rust_string.get().is_none() {
            if let Some(ref s) = self.js_string {
                unsafe { s.trace(tracer) }
            }
        }
    }
}

impl DOMString {
    /// Creates a new `DOMString`.
    pub fn new() -> DOMString {
        DOMString {
            rust_string: OnceCell::from(String::new()),
            js_context: None,
            js_string: None,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.make_me_string().as_bytes()
    }

    /// This method will do some work if necessary but not an allocation.
    /// It returns the bytes either in Utf8 or Latin1 encoded, depending on the
    /// raw mozjs string.
    #[allow(unused)]
    fn bytes<'a>(&'a self) -> EncodedBytes<'a> {
        self.debug_js();
        match self.rust_string.get() {
            Some(s) => EncodedBytes::Utf8Bytes(s.as_str()),
            None => {
                let mut length = 0;
                unsafe {
                    let chars = JS_GetLatin1StringCharsAndLength(
                        self.js_context.unwrap(),
                        ptr::null(),
                        self.js_string.as_ref().unwrap().get(),
                        &mut length,
                    );
                    assert!(!chars.is_null());

                    EncodedBytes::Latin1Bytes(slice::from_raw_parts(chars, length))
                }
            },
        }
    }

    pub fn chars(&self) -> impl Iterator<Item = char> {
        self.make_me_string().chars()
    }

    pub fn clear(&mut self) {
        if let Some(val) = self.rust_string.get_mut() {
            val.clear();
        } else {
            self.rust_string
                .set(String::new())
                .expect("Error in clearing");
        }
    }

    /// Debug the current  state of the string
    #[allow(unused)]
    fn debug_js(&self) {
        if self.js_string.is_some() && self.rust_string.get().is_none() {
            unsafe {
                println!(
                    "jsstring {:?}",
                    jsstr_to_string(
                        self.js_context.unwrap(),
                        ptr::NonNull::new(self.js_string.as_ref().unwrap().get()).unwrap()
                    )
                );
            }
        } else {
            println!("only rust string {:?}", self.rust_string.get().unwrap());
        }
    }

    pub fn encode_utf16(&self) -> EncodeUtf16<'_> {
        self.make_me_string().encode_utf16()
    }

    /// Take the jsstring. If it only has Latin1 characters, we store the ptr in a Heap::boxed
    /// Otherwise we convert the string to a rust string.
    pub fn from_js_string(cx: *mut JSContext, value: js::gc::HandleValue) -> DOMString {
        let string_ptr = unsafe { js::rust::ToString(cx, value) };
        if !string_ptr.is_null() {
            let latin1 = unsafe { js::jsapi::JS_DeprecatedStringHasLatin1Chars(string_ptr) };
            if latin1 {
                let h = Heap::boxed(string_ptr);
                DOMString {
                    rust_string: OnceCell::new(),
                    js_context: Some(cx),
                    js_string: Some(h),
                }
            } else {
                // We need to convert the string anyway as it is not just latin1
                DOMString::from_string(unsafe {
                    jsstr_to_string(cx, ptr::NonNull::new(string_ptr).unwrap())
                })
            }
        } else {
            DOMString::from_string(String::new())
        }
    }

    /// Creates a new `DOMString` from a `String`.
    pub fn from_string(s: String) -> DOMString {
        DOMString {
            rust_string: OnceCell::from(s),
            js_context: None,
            js_string: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.make_me_string().is_empty()
    }

    pub fn len(&self) -> usize {
        self.make_me_string().len()
    }

    pub fn make_ascii_lowercase(&mut self) {
        self.make_me_string();
        self.rust_string.get_mut().unwrap().make_ascii_lowercase();
    }

    /// Convert the mozjs string to a rust string if necessary and safe the result.
    /// Returns the &str
    fn make_me_string(&self) -> &str {
        self.rust_string.get_or_init(|| unsafe {
            jsstr_to_string(
                self.js_context.unwrap(),
                NonNull::new(self.js_string.as_ref().unwrap().get()).unwrap(),
            )
        })
    }

    /// This method is here for compatibilities sake.
    pub fn str(&self) -> &str {
        self.make_me_string()
    }

    pub fn push_str(&mut self, s: &str) {
        self.make_me_string();
        self.rust_string.get_mut().unwrap().push_str(s)
    }

    pub fn strip_leading_and_trailing_ascii_whitespace(&mut self) {
        if self.is_empty() {
            return;
        }

        self.make_me_string();
        let s = self.rust_string.get_mut().unwrap();

        let trailing_whitespace_len = s
            .trim_end_matches(|ref c| char::is_ascii_whitespace(c))
            .len();
        s.truncate(trailing_whitespace_len);
        if s.is_empty() {
            return;
        }

        let first_non_whitespace = s.find(|ref c| !char::is_ascii_whitespace(c)).unwrap();
        s.replace_range(0..first_non_whitespace, "");
    }

    /// This is a dom spec
    pub fn is_valid_floating_point_number_string(&self) -> bool {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"^-?(?:\d+\.\d+|\d+|\.\d+)(?:(e|E)(\+|\-)?\d+)?$").unwrap()
        });

        RE.is_match(self.make_me_string()) && self.parse_floating_point_number().is_some()
    }

    pub fn parse<T: FromStr + std::fmt::Debug>(&self) -> Result<T, <T as FromStr>::Err> {
        self.make_me_string().parse::<T>()
    }

    /// This is a domspec
    /// <https://html.spec.whatwg.org/multipage/#rules-for-parsing-floating-point-number-values>
    pub fn parse_floating_point_number(&self) -> Option<f64> {
        todo!("NYI")
        //self.to_domstring().parse_floating_point_number()
    }

    /// This is a dom spec
    pub fn set_best_representation_of_the_floating_point_number(&mut self) {
        //self.to_domstring()
        //    .set_best_representation_of_the_floating_point_number();
    }

    pub fn to_lowercase(&self) -> String {
        self.make_me_string().to_lowercase()
    }

    pub fn to_uppercase(&self) -> String {
        self.make_me_string().to_uppercase()
    }

    pub fn strip_newlines(&mut self) {
        self.make_me_string();
        self.rust_string
            .get_mut()
            .unwrap()
            .retain(|c| c != '\r' && c != '\n');
    }

    pub fn replace(self, needle: &str, replace_char: &str) -> DOMString {
        self.make_me_string();
        let new_string = self.rust_string.get().unwrap().to_owned();
        DOMString::from_string(new_string.replace(needle, replace_char))
    }

    pub fn split(&self, c: char) -> impl Iterator<Item = &str> {
        self.make_me_string().split(c)
    }

    pub fn find(&self, c: char) -> Option<usize> {
        self.make_me_string().find(c)
    }

    /// Pattern is not yet stable in rust, hence, we need different methods for str and char
    pub fn starts_with(&self, c: char) -> bool {
        self.make_me_string().starts_with(c)
    }

    pub fn starts_with_str(&self, needle: &str) -> bool {
        self.make_me_string().starts_with(needle)
    }

    pub fn contains(&self, needle: &str) -> bool {
        self.make_me_string().contains(needle)
    }

    pub fn to_ascii_lowercase(&self) -> String {
        self.make_me_string().to_ascii_lowercase()
    }

    pub fn contains_html_space_characters(&self) -> bool {
        self.make_me_string().contains(HTML_SPACE_CHARACTERS)
    }
}

impl Ord for DOMString {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.make_me_string().cmp(other.make_me_string())
    }
}

impl PartialOrd for DOMString {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.make_me_string().partial_cmp(other.make_me_string())
    }
}

impl Extend<char> for DOMString {
    fn extend<T: IntoIterator<Item = char>>(&mut self, iter: T) {
        self.make_me_string();
        self.rust_string.get_mut().unwrap().extend(iter)
    }
}

impl ToJSValConvertible for DOMString {
    unsafe fn to_jsval(&self, cx: *mut JSContext, rval: MutableHandleValue) {
        self.make_me_string();
        unsafe {
            self.rust_string.get().unwrap().to_jsval(cx, rval);
        }
    }
}

impl std::hash::Hash for DOMString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.make_me_string();
        self.rust_string.get().hash(state);
    }
}

impl From<&str> for DOMString {
    fn from(contents: &str) -> DOMString {
        DOMString::from_string(String::from(contents))
    }
}

impl From<DOMString> for String {
    fn from(val: DOMString) -> Self {
        val.make_me_string();
        val.rust_string.get().unwrap().to_owned()
    }
}

impl From<DOMString> for Vec<u8> {
    fn from(mut value: DOMString) -> Self {
        value.make_me_string();
        value.rust_string.take().unwrap().as_bytes().to_vec()
    }
}

impl From<Cow<'_, str>> for DOMString {
    fn from(value: Cow<'_, str>) -> Self {
        DOMString::from_string(value.into_owned())
    }
}

impl malloc_size_of::MallocSizeOf for DOMString {
    fn size_of(&self, ops: &mut MallocSizeOfOps) -> usize {
        if let Some(s) = self.rust_string.get() {
            s.size_of(ops)
        } else {
            0
        }
    }
}

impl std::fmt::Display for DOMString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.make_me_string();
        fmt::Display::fmt(self.rust_string.get().unwrap(), f)
    }
}

impl Default for DOMString {
    fn default() -> Self {
        DOMString::new()
    }
}

impl std::cmp::PartialEq<&str> for DOMString {
    fn eq(&self, other: &&str) -> bool {
        self.make_me_string();
        self.rust_string.get().unwrap() == *other
    }
}

impl std::cmp::PartialEq<str> for DOMString {
    fn eq(&self, other: &str) -> bool {
        self.make_me_string();
        self.rust_string.get().unwrap() == other
    }
}

impl std::cmp::PartialEq<DOMString> for str {
    fn eq(&self, other: &DOMString) -> bool {
        other.make_me_string() == self
    }
}

impl std::cmp::PartialEq for DOMString {
    fn eq(&self, other: &Self) -> bool {
        self.make_me_string();
        other.make_me_string();
        self.rust_string.get() == other.rust_string.get()
    }
}

impl std::cmp::Eq for DOMString {}

impl From<std::string::String> for DOMString {
    fn from(value: String) -> Self {
        DOMString::from_string(value)
    }
}

impl From<DOMString> for LocalName {
    fn from(mut contents: DOMString) -> LocalName {
        contents.make_me_string();
        LocalName::from(contents.rust_string.take().unwrap())
    }
}

impl From<DOMString> for Namespace {
    fn from(mut contents: DOMString) -> Namespace {
        contents.make_me_string();
        Namespace::from(contents.rust_string.take().unwrap())
    }
}

impl From<DOMString> for Atom {
    fn from(mut contents: DOMString) -> Atom {
        contents.make_me_string();
        Atom::from(contents.rust_string.take().unwrap())
    }
}

impl From<EncodedBytes<'_>> for DOMString {
    fn from(value: EncodedBytes<'_>) -> Self {
        match value {
            EncodedBytes::Utf8Bytes(s) => DOMString::from_string(s.to_string()),
            EncodedBytes::Latin1Bytes(s) => {
                DOMString::from_string(encoding_rs::mem::decode_latin1(s).into_owned())
            },
        }
    }
}
