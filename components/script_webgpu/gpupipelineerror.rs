/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use js::rust::HandleObject;
use jstraceable_derive::JSTraceable;
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::DomTypes;
use script_bindings::codegen::GenericBindings::WebGPUBinding::{
    GPUPipelineErrorInit, GPUPipelineErrorMethods, GPUPipelineErrorReason, GPUPipelineErrorWrap,
};
use script_bindings::reflector::{
    reflect_dom_object_with_proto, reflect_dom_object_with_wrap_and_proto,
};
use script_bindings::root::DomRoot;
use script_bindings::script_runtime::CanGc;
use script_bindings::str::DOMString;

/// <https://gpuweb.github.io/gpuweb/#gpupipelineerror>
#[dom_struct]
pub(crate) struct GPUPipelineError {
    //exception: DOMException,
    reason: GPUPipelineErrorReason,
}

impl GPUPipelineError {
    fn new_inherited(message: DOMString, reason: GPUPipelineErrorReason) -> Self {
        Self {
            //      exception: DOMException::new_inherited(message, "GPUPipelineError".into()),
            reason,
        }
    }

    pub(crate) fn new_with_proto<D>(
        global: &D::GlobalScope,
        proto: Option<HandleObject>,
        message: DOMString,
        reason: GPUPipelineErrorReason,
        can_gc: CanGc,
    ) -> DomRoot<Self>
    where
        D: DomTypes<GPUPipelineError = GPUPipelineError>,
    {
        reflect_dom_object_with_wrap_and_proto::<D, _, _, _>(
            Box::new(Self::new_inherited(message, reason)),
            global,
            proto,
            can_gc,
            GPUPipelineErrorWrap::<D>,
        )
    }

    pub(crate) fn new<D>(
        global: &D::GlobalScope,
        message: DOMString,
        reason: GPUPipelineErrorReason,
        can_gc: CanGc,
    ) -> DomRoot<Self>
    where
        D: DomTypes<GPUPipelineError = GPUPipelineError>,
    {
        Self::new_with_proto::<D>(global, None, message, reason, can_gc)
    }
}

impl<D> GPUPipelineErrorMethods<D> for GPUPipelineError
where
    D: DomTypes<GPUPipelineError = GPUPipelineError>,
{
    /// <https://gpuweb.github.io/gpuweb/#dom-gpupipelineerror-constructor>
    fn Constructor(
        global: &D::GlobalScope,
        proto: Option<HandleObject>,
        can_gc: CanGc,
        message: DOMString,
        options: &GPUPipelineErrorInit,
    ) -> DomRoot<Self> {
        Self::new_with_proto::<D>(global, proto, message, options.reason, can_gc)
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpupipelineerror-reason>
    fn Reason(&self) -> GPUPipelineErrorReason {
        self.reason
    }
}
