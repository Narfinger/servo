/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use js::rust::HandleObject;
use jstraceable_derive::JSTraceable;
use log::warn;
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::DomTypes;
use script_bindings::codegen::GenericBindings::WebGPUBinding::GPUOutOfMemoryErrorMethods;
use script_bindings::reflector::{
    reflect_dom_object_test_with_wrap2_with_proto, reflect_dom_object_with_proto,
};
use script_bindings::root::DomRoot;
use script_bindings::str::DOMString;

use crate::gpuerror::GPUError;
use crate::gpuinternalerror::GPUInternalError;
use crate::script_runtime::CanGc;

#[dom_struct]
pub struct GPUOutOfMemoryError {
    gpu_error: GPUError,
}

impl GPUOutOfMemoryError {
    fn new_inherited(message: DOMString) -> Self {
        Self {
            gpu_error: GPUError::new_inherited(message),
        }
    }

    pub(crate) fn new_with_proto<D>(
        global: &D::GlobalScope,
        proto: Option<HandleObject>,
        message: DOMString,
        can_gc: CanGc,
    ) -> DomRoot<Self>
    where
        D: DomTypes,
        Box<D::GPUInternalError>: From<Box<GPUInternalError>>,
        DomRoot<GPUInternalError>: From<DomRoot<D::GPUInternalError>>,
        Box<D::GPUError>: From<Box<GPUError>>,
        DomRoot<GPUError>: From<DomRoot<D::GPUError>>,
        Box<D::GPUOutOfMemoryError>: From<Box<GPUOutOfMemoryError>>,
        DomRoot<GPUOutOfMemoryError>: From<DomRoot<D::GPUOutOfMemoryError>>,
    {
        reflect_dom_object_test_with_wrap2_with_proto::<D, _, _, _>(
            Box::new(Self::new_inherited(message)),
            global,
            proto,
            can_gc,
            script_bindings::codegen::GenericBindings::WebGPUBinding::GPUOutOfMemoryErrorWrap::<D>,
        )
    }
}

impl<D> GPUOutOfMemoryErrorMethods<D> for GPUOutOfMemoryError
where
    D: DomTypes,
    Box<D::GPUInternalError>: From<Box<GPUInternalError>>,
    DomRoot<GPUInternalError>: From<DomRoot<D::GPUInternalError>>,
    Box<D::GPUError>: From<Box<GPUError>>,
    DomRoot<GPUError>: From<DomRoot<D::GPUError>>,
    D: DomTypes,
    Box<D::GPUInternalError>: From<Box<GPUInternalError>>,
    DomRoot<GPUInternalError>: From<DomRoot<D::GPUInternalError>>,
    Box<D::GPUError>: From<Box<GPUError>>,
    DomRoot<GPUError>: From<DomRoot<D::GPUError>>,
    Box<D::GPUOutOfMemoryError>: From<Box<GPUOutOfMemoryError>>,
    DomRoot<GPUOutOfMemoryError>: From<DomRoot<D::GPUOutOfMemoryError>>,
{
    /// <https://gpuweb.github.io/gpuweb/#dom-GPUOutOfMemoryError-GPUOutOfMemoryError>
    fn Constructor(
        global: &D::GlobalScope,
        proto: Option<HandleObject>,
        can_gc: CanGc,
        message: DOMString,
    ) -> DomRoot<D::GPUOutOfMemoryError> {
        todo!() //Self::new_with_proto(global, proto, message, can_gc)
    }
}
