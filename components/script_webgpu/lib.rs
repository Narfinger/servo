/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![cfg_attr(crown, feature(register_tool))]
// Register the linter `crown`, which is the Servo-specific linter for the script crate.
#![cfg_attr(crown, register_tool(crown))]

mod gpu;
mod gpuadapter;
mod gpuadapterinfo;
mod gpubindgroup;
mod gpubindgrouplayout;
mod gpubuffer;
mod gpubufferusage;
mod gpucanvascontext;
mod gpucolorwrite;
mod gpucommandbuffer;
mod gpucommandencoder;
mod gpucompilationinfo;
mod gpucompilationmessage;
mod gpucomputepassencoder;
mod gpucomputepipeline;
mod gpuconvert;
mod gpudevice;
mod gpudevicelostinfo;
mod gpuerror;
mod gpuinternalerror;
mod gpumapmode;
mod gpuoutofmemoryerror;
mod gpupipelineerror;
mod gpupipelinelayout;
mod gpuqueryset;
mod gpuqueue;
mod gpurenderbundle;
mod gpurenderbundleencoder;
mod gpurenderpassencoder;
mod gpurenderpipeline;
mod gpusampler;
mod gpushadermodule;
mod gpushaderstage;
mod gpusupportedfeatures;
mod gpusupportedlimits;
mod gputexture;
mod gputextureusage;
mod gputextureview;
mod gpuuncapturederrorevent;
mod gpuvalidationerror;
mod identityhub;
mod wgsllanguagefeatures;

use std::cell::UnsafeCell;
use std::ptr;

use dom_struct::dom_struct;
pub(crate) use js::gc::Traceable as JSTraceable;
use jstraceable_derive::JSTraceable;
use malloc_size_of::{MallocSizeOf, MallocSizeOfOps};
use malloc_size_of_derive::MallocSizeOf;
pub(crate) use script_bindings::inheritance::HasParent;
pub(crate) use script_bindings::reflector::{DomObject, MutDomObject, Reflector};
use script_bindings::root::{Dom, DomRoot};
pub(crate) use script_bindings::trace::CustomTraceable;
use script_bindings::{DomTypes, script_runtime};

// A holder that provides interior mutability for GC-managed values such as
/// `Dom<T>`, with nullability represented by an enclosing Option wrapper.
/// Essentially a `Cell<Option<Dom<T>>>`, but safer.
///
/// This should only be used as a field in other DOM objects; see warning
/// on `Dom<T>`.
#[cfg_attr(crown, crown::unrooted_must_root_lint::must_root)]
#[derive(JSTraceable)]
pub(crate) struct MutNullableDom<T: DomObject> {
    ptr: UnsafeCell<Option<Dom<T>>>,
}

impl<T: DomObject> MutNullableDom<T> {
    /// Create a new `MutNullableDom`.
    pub(crate) fn new(initial: Option<&T>) -> MutNullableDom<T> {
        //assert_in_script();
        MutNullableDom {
            ptr: UnsafeCell::new(initial.map(Dom::from_ref)),
        }
    }

    /// Retrieve a copy of the current inner value. If it is `None`, it is
    /// initialized with the result of `cb` first.
    pub(crate) fn or_init<F>(&self, cb: F) -> DomRoot<T>
    where
        F: FnOnce() -> DomRoot<T>,
    {
        //assert_in_script();
        match self.get() {
            Some(inner) => inner,
            None => {
                let inner = cb();
                self.set(Some(&inner));
                inner
            },
        }
    }

    /// Get a rooted value out of this object
    pub(crate) fn get(&self) -> Option<DomRoot<T>> {
        //assert_in_script();
        unsafe { ptr::read(self.ptr.get()).map(|o| DomRoot::from_ref(&*o)) }
    }

    /// Set this `MutNullableDom` to the given value.
    pub(crate) fn set(&self, val: Option<&T>) {
        //assert_in_script();
        unsafe {
            *self.ptr.get() = val.map(|p| Dom::from_ref(p));
        }
    }

    /// Gets the current value out of this object and sets it to `None`.
    pub(crate) fn take(&self) -> Option<DomRoot<T>> {
        let value = self.get();
        self.set(None);
        value
    }

    /// Sets the current value of this [`MutNullableDom`] to `None`.
    pub(crate) fn clear(&self) {
        self.set(None)
    }

    /// Runs the given callback on the object if it's not null.
    pub(crate) fn if_is_some<F, R>(&self, cb: F) -> Option<&R>
    where
        F: FnOnce(&T) -> &R,
    {
        unsafe {
            if let Some(ref value) = *self.ptr.get() {
                Some(cb(value))
            } else {
                None
            }
        }
    }
}

impl<T: DomObject> PartialEq for MutNullableDom<T> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { *self.ptr.get() == *other.ptr.get() }
    }
}

impl<T: DomObject> PartialEq<Option<&T>> for MutNullableDom<T> {
    fn eq(&self, other: &Option<&T>) -> bool {
        unsafe { *self.ptr.get() == other.map(Dom::from_ref) }
    }
}

impl<T: DomObject> Default for MutNullableDom<T> {
    fn default() -> MutNullableDom<T> {
        //assert_in_script();
        MutNullableDom {
            ptr: UnsafeCell::new(None),
        }
    }
}

impl<T: DomObject> MallocSizeOf for MutNullableDom<T> {
    fn size_of(&self, _ops: &mut MallocSizeOfOps) -> usize {
        // See comment on MallocSizeOf for Dom<T>.
        0
    }
}

#[derive(JSTraceable, MallocSizeOf)]
pub(crate) struct DataBlock(());

impl DataBlock {
    fn new_zeroed(size: usize) -> DataBlock {
        DataBlock(())
    }
}

pub(crate) trait Convert<T> {
    fn convert(self) -> T;
}

pub(crate) trait TryConvert<T> {
    type Error;

    fn try_convert(self) -> Result<T, Self::Error>;
}

pub(crate) struct GPUColor(());

#[derive(Default, MallocSizeOf)]
pub(crate) struct PromiseStub(());

unsafe impl JSTraceable for PromiseStub {
    unsafe fn trace(&self, trc: *mut js::jsapi::JSTracer) {}
}
