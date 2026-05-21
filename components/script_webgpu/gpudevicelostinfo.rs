/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::marker::PhantomData;

use dom_struct::dom_struct;
use jstraceable_derive::JSTraceable;
use log::warn;
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::DomTypes;
use script_bindings::codegen::GenericBindings::WebGPUBinding::{
    GPUDeviceLostInfoMethods, GPUDeviceLostReason,
};
use script_bindings::reflector::{
    Reflector, reflect_dom_object, reflect_dom_object_test_with_wrap2,
};
use script_bindings::root::DomRoot;
use script_bindings::str::DOMString;

use crate::script_runtime::CanGc;

#[dom_struct]
pub struct GPUDeviceLostInfo {
    reflector_: Reflector,
    message: DOMString,
    reason: GPUDeviceLostReason,
}

impl GPUDeviceLostInfo {
    fn new_inherited(message: DOMString, reason: GPUDeviceLostReason) -> Self {
        Self {
            reflector_: Reflector::new(),
            message,
            reason,
        }
    }

    pub(crate) fn new<D>(
        global: &D::GlobalScope,
        message: DOMString,
        reason: GPUDeviceLostReason,
        can_gc: CanGc,
    ) -> DomRoot<Self>
    where
        D: DomTypes,
        Box<D::GPUDeviceLostInfo>: From<Box<GPUDeviceLostInfo>>,
        DomRoot<GPUDeviceLostInfo>: From<DomRoot<D::GPUDeviceLostInfo>>,
    {
        reflect_dom_object_test_with_wrap2::<D, _, _, _>(
            Box::new(GPUDeviceLostInfo::new_inherited(message, reason)),
            global,
            can_gc,
            script_bindings::codegen::GenericBindings::WebGPUBinding::GPUDeviceLostInfoWrap::<D>,
        )
    }
}

impl<D: DomTypes> GPUDeviceLostInfoMethods<D> for GPUDeviceLostInfo {
    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevicelostinfo-message>
    fn Message(&self) -> DOMString {
        self.message.clone()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevicelostinfo-reason>
    fn Reason(&self) -> GPUDeviceLostReason {
        self.reason
    }
}
