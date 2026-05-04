/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::{dom_struct, dom_struct2};
use js::rust::HandleObject;
use jstraceable_derive::JSTraceableInSub;
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::{codegen::GenericBindings::WebGPUBinding::GPUPipelineErrorReason, reflector::reflect_dom_object_with_proto, root::DomRoot, script_runtime::CanGc, str::DOMString};

/// <https://gpuweb.github.io/gpuweb/#gpupipelineerror>
#[dom_struct2]
pub(crate) struct GPUPipelineError {
    exception: DOMException,
    reason: GPUPipelineErrorReason,
}

impl GPUPipelineError {
    fn new_inherited(message: DOMString, reason: GPUPipelineErrorReason) -> Self {
        Self {
            exception: DOMException::new_inherited(message, "GPUPipelineError".into()),
            reason,
        }
    }

    pub(crate) fn new_with_proto(
        global: &GlobalScope,
        proto: Option<HandleObject>,
        message: DOMString,
        reason: GPUPipelineErrorReason,
        can_gc: CanGc,
    ) -> DomRoot<Self> {
        reflect_dom_object_with_proto(
            Box::new(Self::new_inherited(message, reason)),
            global,
            proto,
            can_gc,
        )
    }

    pub(crate) fn new(
        global: &GlobalScope,
        message: DOMString,
        reason: GPUPipelineErrorReason,
        can_gc: CanGc,
    ) -> DomRoot<Self> {
        Self::new_with_proto(global, None, message, reason, can_gc)
    }
}

impl GPUPipelineErrorMethods<crate::DomTypeHolder> for GPUPipelineError {
    /// <https://gpuweb.github.io/gpuweb/#dom-gpupipelineerror-constructor>
    fn Constructor(
        global: &GlobalScope,
        proto: Option<HandleObject>,
        can_gc: CanGc,
        message: DOMString,
        options: &GPUPipelineErrorInit,
    ) -> DomRoot<Self> {
        Self::new_with_proto(global, proto, message, options.reason, can_gc)
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpupipelineerror-reason>
    fn Reason(&self) -> GPUPipelineErrorReason {
        self.reason
    }
}
