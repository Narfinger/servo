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
use script_bindings::codegen::GenericBindings::WebGPUBinding::{
    GPUPipelineErrorInit, GPUPipelineErrorMethods, GPUPipelineErrorReason,
};
use script_bindings::reflector::{
    Reflector, reflect_dom_object_test_with_wrap2_with_proto, reflect_dom_object_with_proto,
};
use script_bindings::root::DomRoot;
use script_bindings::str::DOMString;

use crate::script_runtime::CanGc;

/// <https://gpuweb.github.io/gpuweb/#gpupipelineerror>
#[dom_struct]
pub(crate) struct GPUPipelineError {
    reflector: Reflector,
    //exception: D::DOMException,
    reason: GPUPipelineErrorReason,
}

impl GPUPipelineError {
    fn new_inherited(message: DOMString, reason: GPUPipelineErrorReason) -> Self {
        todo!()
        /*
        Self {
            reflector: Reflector::new(),
            exception: D::DOMException::new_inherited(message, "GPUPipelineError".into()),
            reason,
        }
         */
    }

    pub(crate) fn new_with_proto<D>(
        global: &D::GlobalScope,
        proto: Option<HandleObject>,
        message: DOMString,
        reason: GPUPipelineErrorReason,
        can_gc: CanGc,
    ) -> DomRoot<Self>
    where
        D: DomTypes,
        Box<D::GPUPipelineError>: From<Box<GPUPipelineError>>,
        DomRoot<GPUPipelineError>: From<DomRoot<D::GPUPipelineError>>,
        D::GPUPipelineError: From<GPUPipelineError>,
        DomRoot<D::GPUPipelineError>: From<DomRoot<GPUPipelineError>>,
    {
        reflect_dom_object_test_with_wrap2_with_proto::<D, _, _, _>(
            Box::new(Self::new_inherited(message, reason)),
            global,
            proto,
            can_gc,
            script_bindings::codegen::GenericBindings::WebGPUBinding::GPUPipelineErrorWrap::<D>,
        )
    }

    pub(crate) fn new<D>(
        global: &D::GlobalScope,
        message: DOMString,
        reason: GPUPipelineErrorReason,
        can_gc: CanGc,
    ) -> DomRoot<Self>
    where
        D: DomTypes,
        Box<D::GPUPipelineError>: From<Box<GPUPipelineError>>,
        DomRoot<GPUPipelineError>: From<DomRoot<D::GPUPipelineError>>,
        D::GPUPipelineError: From<GPUPipelineError>,
        DomRoot<D::GPUPipelineError>: From<DomRoot<GPUPipelineError>>,
    {
        Self::new_with_proto::<D>(global, None, message, reason, can_gc)
    }
}

impl<D> GPUPipelineErrorMethods<D> for GPUPipelineError
where
    D: DomTypes,
    Box<D::GPUPipelineError>: From<Box<GPUPipelineError>>,
    DomRoot<GPUPipelineError>: From<DomRoot<D::GPUPipelineError>>,
    D::GPUPipelineError: From<GPUPipelineError>,
    DomRoot<D::GPUPipelineError>: From<DomRoot<GPUPipelineError>>,
{
    /// <https://gpuweb.github.io/gpuweb/#dom-gpupipelineerror-constructor>
    fn Constructor(
        global: &D::GlobalScope,
        proto: Option<HandleObject>,
        can_gc: CanGc,
        message: DOMString,
        options: &GPUPipelineErrorInit,
    ) -> DomRoot<D::GPUPipelineError> {
        Self::new_with_proto::<D>(global, proto, message, options.reason, can_gc).into()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpupipelineerror-reason>
    fn Reason(&self) -> GPUPipelineErrorReason {
        self.reason
    }
}
