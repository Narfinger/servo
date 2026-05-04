/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::{dom_struct, dom_struct2};
use js::rust::HandleObject;
use jstraceable_derive::JSTraceableInSub;
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::{DomTypes, codegen::GenericBindings::WebGPUBinding::GPUValidationErrorMethods, conversions::DerivedFrom, reflector::reflect_dom_object_with_proto, root::DomRoot, script_runtime::CanGc, str::DOMString};

use crate::gpuerror::GPUError;

#[dom_struct2]
pub(crate) struct GPUValidationError {
    gpu_error: GPUError,
}

impl GPUValidationError {
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

impl GPUValidationErrorMethods<script_bindings::DomTypeHolder> for GPUValidationError {
    /// <https://gpuweb.github.io/gpuweb/#dom-gpuvalidationerror-gpuvalidationerror>
    fn Constructor<D: DomTypes, G: DerivedFrom<D::GlobalScope>>(
        global: &G,
        proto: Option<HandleObject>,
        can_gc: CanGc,
        message: DOMString,
    ) -> DomRoot<Self> {
        Self::new_with_proto(global, proto, message, can_gc)
    }
}
