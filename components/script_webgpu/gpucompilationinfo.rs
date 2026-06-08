/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use js::rust::MutableHandleValue;
use jstraceable_derive::JSTraceable;
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::DomTypes;
use script_bindings::codegen::GenericBindings::WebGPUBinding::{
    GPUCompilationInfoMethods, GPUCompilationInfoWrap,
};
use script_bindings::reflector::{Reflector, reflect_dom_object_with_wrap_and_proto};
use script_bindings::root::DomRoot;
use script_bindings::script_runtime::{CanGc, JSContext};
use script_foo::to_frozen_array;
use webgpu_traits::ShaderCompilationInfo;

use crate::gpucompilationmessage::GPUCompilationMessage;

#[dom_struct]
pub struct GPUCompilationInfo {
    reflector_: Reflector,
    // currently we only get one message from wgpu
    msg: Vec<DomRoot<GPUCompilationMessage>>,
}

impl GPUCompilationInfo {
    pub(crate) fn new_inherited(msg: Vec<DomRoot<GPUCompilationMessage>>) -> Self {
        Self {
            reflector_: Reflector::new(),
            msg,
        }
    }

    pub(crate) fn new<D>(
        global: &D::GlobalScope,
        msg: Vec<DomRoot<GPUCompilationMessage>>,
        can_gc: CanGc,
    ) -> DomRoot<Self>
    where
        D: DomTypes<GPUCompilationInfo = GPUCompilationInfo>,
    {
        reflect_dom_object_with_wrap_and_proto::<D, _, _, _>(
            Box::new(Self::new_inherited(msg)),
            global,
            None,
            can_gc,
            GPUCompilationInfoWrap::<D>,
        )
    }

    pub(crate) fn from<D>(
        global: &D::GlobalScope,
        error: Option<ShaderCompilationInfo>,
        can_gc: CanGc,
    ) -> DomRoot<Self>
    where
        D: DomTypes<
                GPUCompilationInfo = GPUCompilationInfo,
                GPUCompilationMessage = GPUCompilationMessage,
            >,
    {
        Self::new::<D>(
            global,
            if let Some(error) = error {
                vec![GPUCompilationMessage::from::<D>(global, error, can_gc)]
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
