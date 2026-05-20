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
use script_bindings::codegen::GenericBindings::WebGPUBinding::GPUValidationErrorMethods;
use script_bindings::reflector::{
    reflect_dom_object_test_with_wrap2_with_proto, reflect_dom_object_with_proto,
};
use script_bindings::root::{Dom, DomRoot, Root};
use script_bindings::str::DOMString;

use crate::gpuerror::GPUError;
use crate::script_runtime::CanGc;

#[dom_struct]
pub(crate) struct GPUValidationError<D: DomTypes> {
    gpu_error: GPUError<D>,
    phantom: PhantomData<D>,
}

impl<D: DomTypes> GPUValidationError<D>
where
    D: DomTypes,
    Box<D::GPUValidationError>: From<Box<GPUValidationError<D>>>,
    DomRoot<GPUValidationError<D>>: From<DomRoot<D::GPUValidationError>>,
{
    fn new_inherited(message: DOMString) -> Self {
        todo!()
        /*
        Self {
            gpu_error: GPUError::new_inherited(message),
            phantom: PhantomData,
        }
         */
    }

    pub(crate) fn new_with_proto(
        global: &D::GlobalScope,
        proto: Option<HandleObject>,
        message: DOMString,
        can_gc: CanGc,
    ) -> DomRoot<Self> {
        reflect_dom_object_test_with_wrap2_with_proto::<D, _, _, _>(
            Box::new(Self::new_inherited(message)),
            global,
            proto,
            can_gc,
            script_bindings::codegen::GenericBindings::WebGPUBinding::GPUValidationErrorWrap::<D>,
        )
    }
}

impl<D> GPUValidationErrorMethods<D> for GPUValidationError<D>
where
    D: DomTypes,
    D::GPUValidationError: From<GPUValidationError<D>>,
    Root<Dom<<D as DomTypes>::GPUValidationError>>: From<Root<Dom<GPUValidationError<D>>>>,
    D: DomTypes,
    Box<D::GPUValidationError>: From<Box<GPUValidationError<D>>>,
    DomRoot<GPUValidationError<D>>: From<DomRoot<D::GPUValidationError>>,
{
    /// <https://gpuweb.github.io/gpuweb/#dom-gpuvalidationerror-gpuvalidationerror>
    fn Constructor(
        global: &D::GlobalScope,
        proto: Option<HandleObject>,
        can_gc: CanGc,
        message: DOMString,
    ) -> DomRoot<D::GPUValidationError> {
        Self::new_with_proto(global, proto, message, can_gc).into()
    }
}
