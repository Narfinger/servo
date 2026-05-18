/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use js::rust::MutableHandleValue;
use jstraceable_derive::JSTraceable;
use log::warn;
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::DomTypes;
use script_bindings::codegen::GenericBindings::WebGPUBinding::GPUCompilationInfoMethods;
use script_bindings::reflector::{Reflector, reflect_dom_object_with_proto};
use script_bindings::root::DomRoot;
use webgpu_traits::ShaderCompilationInfo;

use crate::gpucompilationmessage::GPUCompilationMessage;
use crate::script_runtime::{CanGc, JSContext};

#[dom_struct]
pub(crate) struct GPUCompilationInfo {
    reflector_: Reflector,
    // currently we only get one message from wgpu
    msg: Vec<DomRoot<GPUCompilationMessage>>,
}

impl<D: DomTypes> GPUCompilationInfo {
    pub(crate) fn new_inherited(msg: Vec<DomRoot<GPUCompilationMessage>>) -> Self {
        Self {
            reflector_: Reflector::new(),
            msg,
        }
    }

    pub(crate) fn new(
        global: &D::GlobalScope,
        msg: Vec<DomRoot<GPUCompilationMessage>>,
        can_gc: CanGc,
    ) -> DomRoot<Self> {
        reflect_dom_object_with_proto(Box::new(Self::new_inherited(msg)), global, None, can_gc)
    }

    pub(crate) fn from(
        global: &D::GlobalScope,
        error: Option<ShaderCompilationInfo>,
        can_gc: CanGc,
    ) -> DomRoot<Self> {
        Self::new(
            global,
            if let Some(error) = error {
                vec![GPUCompilationMessage::from(global, error, can_gc)]
            } else {
                Vec::new()
            },
            can_gc,
        )
    }
}

impl<D: DomTypes> GPUCompilationInfoMethods<D> for GPUCompilationInfo {
    /// <https://gpuweb.github.io/gpuweb/#dom-gpucompilationinfo-messages>
    fn Messages(&self, cx: JSContext, can_gc: CanGc, retval: MutableHandleValue) {
        to_frozen_array(self.msg.as_slice(), cx, retval, can_gc)
    }
}
