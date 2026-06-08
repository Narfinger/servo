/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::borrow::Cow;

use dom_struct::dom_struct;
use jstraceable_derive::JSTraceable;
use log::warn;
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::DomTypes;
use script_bindings::cell::DomRefCell;
use script_bindings::codegen::GenericBindings::WebGPUBinding::{
    GPUPipelineLayoutDescriptor, GPUPipelineLayoutMethods, GPUPipelineLayoutWrap,
};
use script_bindings::reflector::{Reflector, reflect_dom_object_with_wrap};
use script_bindings::root::DomRoot;
use script_bindings::script_runtime::CanGc;
use script_bindings::str::USVString;
use webgpu_traits::{WebGPU, WebGPUBindGroupLayout, WebGPUPipelineLayout, WebGPURequest};
use wgpu_core::binding_model::PipelineLayoutDescriptor;

use crate::gpubindgrouplayout::GPUBindGroupLayout;
use crate::gpuconvert::WebGPUConvert;
use crate::traits::{GPUDeviceTrait, WebGPUGlobalTrait};

#[derive(JSTraceable, MallocSizeOf)]
struct DroppableGPUPipelineLayout {
    #[no_trace]
    channel: WebGPU,
    #[no_trace]
    pipeline_layout: WebGPUPipelineLayout,
}

impl Drop for DroppableGPUPipelineLayout {
    fn drop(&mut self) {
        if let Err(e) = self
            .channel
            .0
            .send(WebGPURequest::DropPipelineLayout(self.pipeline_layout.0))
        {
            warn!(
                "Failed to send DropPipelineLayout ({:?}) ({})",
                self.pipeline_layout.0, e
            );
        }
    }
}

#[dom_struct]
pub(crate) struct GPUPipelineLayout {
    reflector_: Reflector,
    label: DomRefCell<USVString>,
    #[no_trace]
    bind_group_layouts: Vec<WebGPUBindGroupLayout>,
    droppable: DroppableGPUPipelineLayout,
}

impl GPUPipelineLayout {
    fn new_inherited(
        channel: WebGPU,
        pipeline_layout: WebGPUPipelineLayout,
        label: USVString,
        bgls: Vec<WebGPUBindGroupLayout>,
    ) -> Self {
        Self {
            reflector_: Reflector::new(),
            label: DomRefCell::new(label),
            bind_group_layouts: bgls,
            droppable: DroppableGPUPipelineLayout {
                channel,
                pipeline_layout,
            },
        }
    }

    pub(crate) fn new<D>(
        global: &D::GlobalScope,
        channel: WebGPU,
        pipeline_layout: WebGPUPipelineLayout,
        label: USVString,
        bgls: Vec<WebGPUBindGroupLayout>,
        can_gc: CanGc,
    ) -> DomRoot<Self>
    where
        D: DomTypes<GPUPipelineLayout = GPUPipelineLayout>,
    {
        reflect_dom_object_with_wrap::<D, _, _>(
            Box::new(GPUPipelineLayout::new_inherited(
                channel,
                pipeline_layout,
                label,
                bgls,
            )),
            global,
            can_gc,
            GPUPipelineLayoutWrap::<D>,
        )
    }
}

impl GPUPipelineLayout {
    pub(crate) fn id(&self) -> WebGPUPipelineLayout {
        self.droppable.pipeline_layout
    }

    pub(crate) fn bind_group_layouts(&self) -> Vec<WebGPUBindGroupLayout> {
        self.bind_group_layouts.clone()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-createpipelinelayout>
    pub(crate) fn create<D>(
        device: &D::GPUDevice,
        descriptor: &GPUPipelineLayoutDescriptor<D>,
        can_gc: CanGc,
    ) -> DomRoot<GPUPipelineLayout>
    where
        D: DomTypes<GPUBindGroupLayout = GPUBindGroupLayout, GPUPipelineLayout = GPUPipelineLayout>,
        D::GPUDevice: GPUDeviceTrait + WebGPUGlobalTrait<D>,
    {
        let bgls = descriptor
            .bindGroupLayouts
            .iter()
            .map(|each| each.id())
            .collect::<Vec<_>>();

        let desc = PipelineLayoutDescriptor {
            label: (&descriptor.parent).convert(),
            // TODO(sagudev): this needs webidl sync
            bind_group_layouts: Cow::Owned(bgls.iter().map(|l| Some(l.0)).collect::<Vec<_>>()),
            immediate_size: 0,
        };

        let pipeline_layout_id = device.wgpu_id_hub().create_pipeline_layout_id();
        device
            .channel()
            .0
            .send(WebGPURequest::CreatePipelineLayout {
                device_id: device.id().0,
                pipeline_layout_id,
                descriptor: desc,
            })
            .expect("Failed to create WebGPU PipelineLayout");

        let pipeline_layout = WebGPUPipelineLayout(pipeline_layout_id);
        GPUPipelineLayout::new::<D>(
            &device.global(),
            device.channel(),
            pipeline_layout,
            descriptor.parent.label.clone(),
            bgls,
            can_gc,
        )
    }
}

impl<D: DomTypes> GPUPipelineLayoutMethods<D> for GPUPipelineLayout {
    /// <https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label>
    fn Label(&self) -> USVString {
        self.label.borrow().clone()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label>
    fn SetLabel(&self, value: USVString) {
        *self.label.borrow_mut() = value;
    }
}
