/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::marker::PhantomData;

use dom_struct::dom_struct;
use js::rust::HandleObject;
use jstraceable_derive::JSTraceable;
use log::warn;
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::DomTypes;
use script_bindings::codegen::GenericBindings::WebGPUBinding::{GPUErrorFilter, GPUErrorMethods};
use script_bindings::reflector::{
    Reflector, reflect_dom_object_test_with_wrap2_with_proto, reflect_dom_object_with_proto,
};
use script_bindings::root::DomRoot;
use script_bindings::str::DOMString;
use webgpu_traits::{Error, ErrorFilter};

use crate::Convert;
use crate::gpuinternalerror::GPUInternalError;
use crate::gpuoutofmemoryerror::GPUOutOfMemoryError;
use crate::gpuvalidationerror::GPUValidationError;
use crate::script_runtime::CanGc;

#[dom_struct]
pub struct GPUError {
    reflector_: Reflector,
    message: DOMString,
}

impl GPUError {
    pub(crate) fn new_inherited(message: DOMString) -> Self {
        Self {
            reflector_: Reflector::new(),
            message,
        }
    }

    #[expect(dead_code)]
    pub(crate) fn new<D>(
        global: &D::GlobalScope,
        message: DOMString,
        can_gc: CanGc,
    ) -> DomRoot<Self>
    where
        D: DomTypes,
        Box<D::GPUError>: From<Box<GPUError>>,
        DomRoot<GPUError>: From<DomRoot<D::GPUError>>,
    {
        Self::new_with_proto::<D>(global, None, message, can_gc)
    }

    pub(crate) fn new_with_proto<D>(
        global: &D::GlobalScope,
        proto: Option<HandleObject>,
        message: DOMString,
        can_gc: CanGc,
    ) -> DomRoot<Self>
    where
        D: DomTypes,
        Box<D::GPUError>: From<Box<GPUError>>,
        DomRoot<GPUError>: From<DomRoot<D::GPUError>>,
    {
        reflect_dom_object_test_with_wrap2_with_proto::<D, _, _, _>(
            Box::new(GPUError::new_inherited(message)),
            global,
            proto,
            can_gc,
            script_bindings::codegen::GenericBindings::WebGPUBinding::GPUErrorWrap::<D>,
        )
    }

    pub(crate) fn from_error<D>(
        global: &D::GlobalScope,
        error: Error,
        can_gc: CanGc,
    ) -> DomRoot<Self>
    where
        D: DomTypes,
        Box<D::GPUError>: From<Box<GPUError>>,
        DomRoot<GPUError>: From<DomRoot<D::GPUError>>,
    {
        todo!()
        /*
        match error {
            Error::Validation(msg) => DomRoot::upcast(GPUValidationError::new_with_proto(
                global,
                None,
                msg.into(),
                can_gc,
            )),
            Error::OutOfMemory(msg) => DomRoot::upcast(GPUOutOfMemoryError::new_with_proto(
                global,
                None,
                msg.into(),
                can_gc,
            )),
            Error::Internal(msg) => DomRoot::upcast(GPUInternalError::new_with_proto(
                global,
                None,
                msg.into(),
                can_gc,
            )),
        }
         */
    }
}

impl<D: DomTypes> GPUErrorMethods<D> for GPUError {
    /// <https://gpuweb.github.io/gpuweb/#dom-gpuerror-message>
    fn Message(&self) -> DOMString {
        self.message.clone()
    }
}

impl Convert<GPUErrorFilter> for ErrorFilter {
    fn convert(self) -> GPUErrorFilter {
        match self {
            ErrorFilter::Validation => GPUErrorFilter::Validation,
            ErrorFilter::OutOfMemory => GPUErrorFilter::Out_of_memory,
            ErrorFilter::Internal => GPUErrorFilter::Internal,
        }
    }
}

pub(crate) trait AsWebGpu {
    fn as_webgpu(&self) -> ErrorFilter;
}

impl AsWebGpu for GPUErrorFilter {
    fn as_webgpu(&self) -> ErrorFilter {
        match self {
            GPUErrorFilter::Validation => ErrorFilter::Validation,
            GPUErrorFilter::Out_of_memory => ErrorFilter::OutOfMemory,
            GPUErrorFilter::Internal => ErrorFilter::Internal,
        }
    }
}
