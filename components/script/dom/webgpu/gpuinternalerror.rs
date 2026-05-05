/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::{dom_struct, dom_struct2};
use js::rust::HandleObject;
use jstraceable_derive::JSTraceableInSub;
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::codegen::GenericBindings::WebGPUBinding::GPUInternalErrorMethods;
use script_bindings::conversions::DerivedFrom;
use script_bindings::inheritance::HasParent;
use script_bindings::reflector::reflect_dom_object_with_proto;
use script_bindings::root::DomRoot;
use script_bindings::script_runtime::CanGc;
use script_bindings::str::DOMString;
use script_bindings::{DomObject, DomTypes};

use crate::gpuerror::GPUError;
#[dom_struct2]
pub(crate) struct GPUInternalError {
    gpu_error: GPUError,
}

impl GPUInternalError {
    fn new_inherited(message: DOMString) -> Self {
        Self {
            gpu_error: GPUError::new_inherited(message),
        }
    }

    pub(crate) fn new_with_proto<D: DomTypes, G: DerivedFrom<D::GlobalScope>>(
        global: &G,
        proto: Option<HandleObject>,
        message: DOMString,
        can_gc: CanGc,
    ) -> DomRoot<Self> {
        reflect_dom_object_with_proto(
            Box::new(Self::new_inherited(message)),
            global,
            proto,
            can_gc,
        )
    }
}

impl<D: DomTypes> GPUInternalErrorMethods<D> for GPUInternalError {
    /// <https://gpuweb.github.io/gpuweb/#dom-GPUInternalError-GPUInternalError>
    fn Constructor<D: DomTypes, G: DerivedFrom<D::GlobalScope>>(
        global: &G,
        proto: Option<HandleObject>,
        can_gc: CanGc,
        message: DOMString,
    ) -> DomRoot<Self> {
        Self::new_with_proto(global, proto, message, can_gc)
    }
}
