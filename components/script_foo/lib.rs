/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![cfg_attr(crown, feature(register_tool))]
// Register the linter `crown`, which is the Servo-specific linter for the script crate.
#![cfg_attr(crown, register_tool(crown))]

use js::conversions::ToJSValConvertible;
use js::gc::MutableHandleValue;
use js::jsapi::{HandleObject as RawHandleObject, JS_FreezeObject};
use js::rooted;
use script_bindings::script_runtime::{CanGc, JSContext as SafeJSContext};

pub mod canvas;

/// Returns a JSVal representing the frozen JavaScript array
pub fn to_frozen_array<T: ToJSValConvertible>(
    convertibles: &[T],
    cx: SafeJSContext,
    mut rval: MutableHandleValue,
    can_gc: CanGc,
) {
    script_bindings::conversions::SafeToJSValConvertible::safe_to_jsval(
        convertibles,
        cx,
        rval.reborrow(),
        can_gc,
    );

    rooted!(in(*cx) let obj = rval.to_object());
    unsafe { JS_FreezeObject(*cx, RawHandleObject::from(obj.handle())) };
}
