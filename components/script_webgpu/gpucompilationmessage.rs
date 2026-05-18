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
    GPUCompilationMessageMethods, GPUCompilationMessageType,
};
use script_bindings::reflector::{Reflector, reflect_dom_object};
use script_bindings::root::DomRoot;
use script_bindings::str::DOMString;
use webgpu_traits::ShaderCompilationInfo;

use crate::script_runtime::CanGc;
#[dom_struct]
pub(crate) struct GPUCompilationMessage<D: DomTypes> {
    reflector_: Reflector,
    message: DOMString,
    mtype: GPUCompilationMessageType,
    line_num: u64,
    line_pos: u64,
    offset: u64,
    length: u64,
    phantom: PhantomData<D>,
}

impl<D: DomTypes> GPUCompilationMessage<D> {
    fn new_inherited(
        message: DOMString,
        mtype: GPUCompilationMessageType,
        line_num: u64,
        line_pos: u64,
        offset: u64,
        length: u64,
    ) -> Self {
        Self {
            reflector_: Reflector::new(),
            message,
            mtype,
            line_num,
            line_pos,
            offset,
            length,
            phantom: PhantomData,
        }
    }

    #[expect(clippy::too_many_arguments)]
    pub(crate) fn new(
        global: &D::GlobalScope,
        message: DOMString,
        mtype: GPUCompilationMessageType,
        line_num: u64,
        line_pos: u64,
        offset: u64,
        length: u64,
        can_gc: CanGc,
    ) -> DomRoot<Self> {
        reflect_dom_object(
            Box::new(Self::new_inherited(
                message, mtype, line_num, line_pos, offset, length,
            )),
            global,
            can_gc,
        )
    }

    pub(crate) fn from(
        global: &D::GlobalScope,
        info: ShaderCompilationInfo,
        can_gc: CanGc,
    ) -> DomRoot<Self> {
        GPUCompilationMessage::new(
            global,
            info.message.into(),
            GPUCompilationMessageType::Error,
            info.line_number,
            info.line_pos,
            info.offset,
            info.length,
            can_gc,
        )
    }
}

impl<D: DomTypes> GPUCompilationMessageMethods<D> for GPUCompilationMessage<D> {
    /// <https://gpuweb.github.io/gpuweb/#dom-gpucompilationmessage-message>
    fn Message(&self) -> DOMString {
        self.message.to_owned()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpucompilationmessage-type>
    fn Type(&self) -> GPUCompilationMessageType {
        self.mtype
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpucompilationmessage-linenum>
    fn LineNum(&self) -> u64 {
        self.line_num
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpucompilationmessage-linepos>
    fn LinePos(&self) -> u64 {
        self.line_pos
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpucompilationmessage-offset>
    fn Offset(&self) -> u64 {
        self.offset
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpucompilationmessage-length>
    fn Length(&self) -> u64 {
        self.length
    }
}
