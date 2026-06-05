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
    GPUComputePipelineDescriptor, GPUComputePipelineMethods, GPUComputePipelineWrap,
};
use script_bindings::error::Fallible;
use script_bindings::reflector::{Reflector, reflect_dom_object_with_wrap};
use script_bindings::root::{Dom, DomRoot};
use script_bindings::script_runtime::CanGc;
use script_bindings::str::USVString;
use servo_base::generic_channel::GenericCallback;
use webgpu_traits::{
    WebGPU, WebGPUBindGroupLayout, WebGPUComputePipeline, WebGPUComputePipelineResponse,
    WebGPURequest,
};
use wgpu_core::pipeline::ComputePipelineDescriptor;

use crate::gpubindgrouplayout::GPUBindGroupLayout;
use crate::gpuconvert::WebGPUConvert;
use crate::gpupipelinelayout::GPUPipelineLayout;
use crate::traits::WebGPUGlobalTrait;

#[derive(JSTraceable, MallocSizeOf)]
struct DroppableGPUComputePipeline {
    #[no_trace]
    channel: WebGPU,
    #[no_trace]
    compute_pipeline: WebGPUComputePipeline,
}

impl Drop for DroppableGPUComputePipeline {
    fn drop(&mut self) {
        if let Err(e) = self
            .channel
            .0
            .send(WebGPURequest::DropComputePipeline(self.compute_pipeline.0))
        {
            warn!(
                "Failed to send WebGPURequest::DropComputePipeline({:?}) ({})",
                self.compute_pipeline.0, e
            );
        };
    }
}

#[dom_struct]
pub(crate) struct GPUComputePipeline {
    reflector_: Reflector,
    label: DomRefCell<USVString>,
    device: Dom<GPUDevice>,
    droppable: DroppableGPUComputePipeline,
}

impl GPUComputePipeline {
    fn new_inherited<D>(
        compute_pipeline: WebGPUComputePipeline,
        label: USVString,
        device: &GPUDevice,
    ) -> Self
    where
        D: DomTypes<GPUComputePipeline = GPUComputePipeline>,
    {
        Self {
            reflector_: Reflector::new(),
            label: DomRefCell::new(label),
            device: Dom::from_ref(device),
            droppable: DroppableGPUComputePipeline {
                channel: device.channel(),
                compute_pipeline,
            },
        }
    }

    pub(crate) fn new<D>(
        global: &D::GlobalScope,
        compute_pipeline: WebGPUComputePipeline,
        label: USVString,
        device: &GPUDevice,
        can_gc: CanGc,
    ) -> DomRoot<Self>
    where
        D: DomTypes<GPUComputePipeline = GPUComputePipeline>,
    {
        reflect_dom_object_with_wrap::<D, _, _, _>(
            Box::new(GPUComputePipeline::new_inherited::<D>(
                compute_pipeline,
                label,
                device,
            )),
            global,
            can_gc,
            GPUComputePipelineWrap::<D>,
        )
    }
}

impl GPUComputePipeline {
    pub(crate) fn id(&self) -> &WebGPUComputePipeline {
        &self.droppable.compute_pipeline
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-createcomputepipeline>
    pub(crate) fn create<D>(
        device: &GPUDevice,
        descriptor: &GPUComputePipelineDescriptor<D>,
        async_sender: Option<GenericCallback<WebGPUComputePipelineResponse>>,
    ) -> WebGPUComputePipeline
    where
        D: DomTypes<GPUPipelineLayout = GPUPipelineLayout, GPUShaderModule = GPUShaderModule>,
        GPUDevice: WebGPUGlobalTrait<D>,
    {
        let compute_pipeline_id = device.wgpu_id_hub().create_compute_pipeline_id();

        let pipeline_layout = device.get_pipeline_layout_data(&descriptor.parent.layout);

        let desc = ComputePipelineDescriptor {
            label: (&descriptor.parent.parent).convert(),
            layout: pipeline_layout.explicit(),
            stage: (&descriptor.compute).convert(),
            cache: None,
        };

        device
            .channel()
            .0
            .send(WebGPURequest::CreateComputePipeline {
                device_id: device.id().0,
                compute_pipeline_id,
                descriptor: desc,
                async_sender,
            })
            .expect("Failed to create WebGPU ComputePipeline");

        WebGPUComputePipeline(compute_pipeline_id)
    }
}

impl<D> GPUComputePipelineMethods<D> for GPUComputePipeline
where
    D: DomTypes<GPUBindGroupLayout = GPUBindGroupLayout>,
    GPUComputePipeline: WebGPUGlobalTrait<D>,
{
    /// <https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label>
    fn Label(&self) -> USVString {
        self.label.borrow().clone()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label>
    fn SetLabel(&self, value: USVString) {
        *self.label.borrow_mut() = value;
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpupipelinebase-getbindgrouplayout>
    fn GetBindGroupLayout(&self, index: u32) -> Fallible<DomRoot<GPUBindGroupLayout>> {
        let id = self.wgpu_id_hub().create_bind_group_layout_id();

        if let Err(e) = self
            .droppable
            .channel
            .0
            .send(WebGPURequest::ComputeGetBindGroupLayout {
                device_id: self.device.id().0,
                pipeline_id: self.id().0,
                index,
                id,
            })
        {
            warn!("Failed to send WebGPURequest::ComputeGetBindGroupLayout {e:?}");
        }

        Ok(GPUBindGroupLayout::new::<D>(
            &self.global(),
            self.droppable.channel.clone(),
            WebGPUBindGroupLayout(id),
            USVString::default(),
            CanGc::deprecated_note(),
        ))
    }
}
