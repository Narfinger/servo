/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::{dom_struct, dom_struct2};
use jstraceable_derive::JSTraceableInSub;
use log::warn;
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::codegen::GenericBindings::WebGPUBinding::GPUComputePassEncoderMethods;
use script_bindings::conversions::DerivedFrom;
use script_bindings::inheritance::HasParent;
use script_bindings::reflector::{Reflector, reflect_dom_object};
use script_bindings::root::{Dom, DomRoot};
use script_bindings::script_runtime::CanGc;
use script_bindings::str::USVString;
use script_bindings::{DomObject, DomRefCell, DomTypes};
use webgpu_traits::{WebGPU, WebGPUComputePass, WebGPURequest};

use crate::gpubindgroup::GPUBindGroup;
use crate::gpubuffer::GPUBuffer;
use crate::gpucommandencoder::GPUCommandEncoder;
use crate::gpucomputepipeline::GPUComputePipeline;
#[derive(JSTraceableInSub, MallocSizeOf)]
struct DroppableGPUComputePassEncoder {
    #[no_trace]
    channel: WebGPU,
    #[no_trace]
    compute_pass: WebGPUComputePass,
}

impl Drop for DroppableGPUComputePassEncoder {
    fn drop(&mut self) {
        if let Err(e) = self
            .channel
            .0
            .send(WebGPURequest::DropComputePass(self.compute_pass.0))
        {
            warn!("Failed to send WebGPURequest::DropComputePass with {e:?}");
        }
    }
}

#[dom_struct2]
pub(crate) struct GPUComputePassEncoder {
    reflector_: Reflector,
    label: DomRefCell<USVString>,
    command_encoder: Dom<GPUCommandEncoder>,
    droppable: DroppableGPUComputePassEncoder,
}

impl GPUComputePassEncoder {
    fn new_inherited(
        channel: WebGPU,
        parent: &GPUCommandEncoder,
        compute_pass: WebGPUComputePass,
        label: USVString,
    ) -> Self {
        Self {
            reflector_: Reflector::new(),
            label: DomRefCell::new(label),
            command_encoder: Dom::from_ref(parent),
            droppable: DroppableGPUComputePassEncoder {
                channel,
                compute_pass,
            },
        }
    }

    pub(crate) fn new<D: DomTypes, G: DerivedFrom<D::GlobalScope>>(
        global: &G,
        channel: WebGPU,
        parent: &GPUCommandEncoder,
        compute_pass: WebGPUComputePass,
        label: USVString,
        can_gc: CanGc,
    ) -> DomRoot<Self> {
        reflect_dom_object(
            Box::new(GPUComputePassEncoder::new_inherited(
                channel,
                parent,
                compute_pass,
                label,
            )),
            global,
            can_gc,
        )
    }
}

impl<D: DomTypes> GPUComputePassEncoderMethods<D> for GPUComputePassEncoder {
    /// <https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label>
    fn Label(&self) -> USVString {
        self.label.borrow().clone()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label>
    fn SetLabel(&self, value: USVString) {
        *self.label.borrow_mut() = value;
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpucomputepassencoder-dispatchworkgroups>
    fn DispatchWorkgroups(&self, x: u32, y: u32, z: u32) {
        if let Err(e) =
            self.droppable
                .channel
                .0
                .send(WebGPURequest::ComputePassDispatchWorkgroups {
                    compute_pass_id: self.droppable.compute_pass.0,
                    x,
                    y,
                    z,
                    device_id: self.command_encoder.device_id().0,
                })
        {
            warn!("Error sending WebGPURequest::ComputePassDispatchWorkgroups: {e:?}")
        }
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpucomputepassencoder-dispatchworkgroupsindirect>
    fn DispatchWorkgroupsIndirect(&self, buffer: &GPUBuffer, offset: u64) {
        if let Err(e) =
            self.droppable
                .channel
                .0
                .send(WebGPURequest::ComputePassDispatchWorkgroupsIndirect {
                    compute_pass_id: self.droppable.compute_pass.0,
                    buffer_id: buffer.id().0,
                    offset,
                    device_id: self.command_encoder.device_id().0,
                })
        {
            warn!("Error sending WebGPURequest::ComputePassDispatchWorkgroupsIndirect: {e:?}")
        }
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpurenderpassencoder-endpass>
    fn End(&self) {
        if let Err(e) = self
            .droppable
            .channel
            .0
            .send(WebGPURequest::EndComputePass {
                compute_pass_id: self.droppable.compute_pass.0,
                device_id: self.command_encoder.device_id().0,
                command_encoder_id: self.command_encoder.id().0,
            })
        {
            warn!("Failed to send WebGPURequest::EndComputePass: {e:?}");
        }
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpuprogrammablepassencoder-setbindgroup>
    fn SetBindGroup(&self, index: u32, bind_group: &GPUBindGroup, offsets: Vec<u32>) {
        if let Err(e) = self
            .droppable
            .channel
            .0
            .send(WebGPURequest::ComputePassSetBindGroup {
                compute_pass_id: self.droppable.compute_pass.0,
                index,
                bind_group_id: bind_group.id().0,
                offsets,
                device_id: self.command_encoder.device_id().0,
            })
        {
            warn!("Error sending WebGPURequest::ComputePassSetBindGroup: {e:?}")
        }
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpucomputepassencoder-setpipeline>
    fn SetPipeline(&self, pipeline: &GPUComputePipeline) {
        if let Err(e) = self
            .droppable
            .channel
            .0
            .send(WebGPURequest::ComputePassSetPipeline {
                compute_pass_id: self.droppable.compute_pass.0,
                pipeline_id: pipeline.id().0,
                device_id: self.command_encoder.device_id().0,
            })
        {
            warn!("Error sending WebGPURequest::ComputePassSetPipeline: {e:?}")
        }
    }
}
