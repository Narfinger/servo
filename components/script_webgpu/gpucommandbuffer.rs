/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use jstraceable_derive::JSTraceable;
use log::warn;
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::DomTypes;
use script_bindings::cell::DomRefCell;
use script_bindings::codegen::GenericBindings::WebGPUBinding::{
    GPUCommandBufferMethods, GPUCommandBufferWrap,
};
use script_bindings::reflector::{Reflector, reflect_dom_object_with_wrap};
use script_bindings::root::DomRoot;
use script_bindings::script_runtime::CanGc;
use script_bindings::str::USVString;
use webgpu_traits::{WebGPU, WebGPUCommandBuffer, WebGPURequest};

#[derive(JSTraceable, MallocSizeOf)]
struct DroppableGPUCommandBuffer {
    #[no_trace]
    channel: WebGPU,
    #[no_trace]
    command_buffer: WebGPUCommandBuffer,
}

impl Drop for DroppableGPUCommandBuffer {
    fn drop(&mut self) {
        if let Err(e) = self
            .channel
            .0
            .send(WebGPURequest::DropCommandBuffer(self.command_buffer.0))
        {
            warn!(
                "Failed to send DropCommandBuffer({:?}) ({})",
                self.command_buffer.0, e
            );
        }
    }
}

#[dom_struct]
pub(crate) struct GPUCommandBuffer {
    reflector_: Reflector,
    label: DomRefCell<USVString>,
    droppable: DroppableGPUCommandBuffer,
}

impl GPUCommandBuffer {
    fn new_inherited(
        channel: WebGPU,
        command_buffer: WebGPUCommandBuffer,
        label: USVString,
    ) -> Self {
        Self {
            reflector_: Reflector::new(),
            label: DomRefCell::new(label),
            droppable: DroppableGPUCommandBuffer {
                channel,
                command_buffer,
            },
        }
    }

    pub(crate) fn new<D>(
        global: &D::GlobalScope,
        channel: WebGPU,
        command_buffer: WebGPUCommandBuffer,
        label: USVString,
        can_gc: CanGc,
    ) -> DomRoot<Self>
    where
        D: DomTypes<GPUCommandBuffer = GPUCommandBuffer>,
    {
        reflect_dom_object_with_wrap::<D, _, _, _>(
            Box::new(GPUCommandBuffer::new_inherited(
                channel,
                command_buffer,
                label,
            )),
            global,
            can_gc,
            GPUCommandBufferWrap::<D>,
        )
    }
}

impl GPUCommandBuffer {
    pub(crate) fn id(&self) -> WebGPUCommandBuffer {
        self.droppable.command_buffer
    }
}

impl<D: DomTypes> GPUCommandBufferMethods<D> for GPUCommandBuffer {
    /// <https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label>
    fn Label(&self) -> USVString {
        self.label.borrow().clone()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label>
    fn SetLabel(&self, value: USVString) {
        *self.label.borrow_mut() = value;
    }
}
