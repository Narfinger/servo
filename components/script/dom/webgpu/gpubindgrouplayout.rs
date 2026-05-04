/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::borrow::Cow;

use dom_struct::{dom_struct, dom_struct2};
use jstraceable_derive::{JSTraceable, JSTraceableInSub};
use log::warn;
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::codegen::GenericBindings::WebGPUBinding::{
    GPUBindGroupLayoutDescriptor, GPUBindGroupLayoutMethods,
};
use script_bindings::conversions::DerivedFrom;
use script_bindings::error::Fallible;
use script_bindings::reflector::{Reflector, reflect_dom_object};
use script_bindings::root::DomRoot;
use script_bindings::script_runtime::CanGc;
use script_bindings::str::USVString;
use script_bindings::{DomRefCell, DomTypes};
use webgpu_traits::{WebGPU, WebGPUBindGroupLayout, WebGPURequest};
use wgpu_core::binding_model::BindGroupLayoutDescriptor;

use crate::gpuconvert::convert_bind_group_layout_entry;
use crate::gpudevice::GPUDevice;

#[derive(JSTraceableInSub, MallocSizeOf)]
struct DroppableGPUBindGroupLayout {
    #[no_trace]
    channel: WebGPU,
    #[no_trace]
    bind_group_layout: WebGPUBindGroupLayout,
}

impl Drop for DroppableGPUBindGroupLayout {
    fn drop(&mut self) {
        if let Err(e) = self
            .channel
            .0
            .send(WebGPURequest::DropBindGroupLayout(self.bind_group_layout.0))
        {
            warn!(
                "Failed to send WebGPURequest::DropBindGroupLayout({:?}) ({})",
                self.bind_group_layout.0, e
            );
        };
    }
}

#[dom_struct2]
pub(crate) struct GPUBindGroupLayout {
    reflector_: Reflector,
    label: DomRefCell<USVString>,
    droppable: DroppableGPUBindGroupLayout,
}

impl GPUBindGroupLayout {
    fn new_inherited(
        channel: WebGPU,
        bind_group_layout: WebGPUBindGroupLayout,
        label: USVString,
    ) -> Self {
        Self {
            reflector_: Reflector::new(),
            label: DomRefCell::new(label),
            droppable: DroppableGPUBindGroupLayout {
                channel,
                bind_group_layout,
            },
        }
    }

    pub(crate) fn new<D: DomTypes, G: DerivedFrom<D::GlobalScope>>(
        global: &G,
        channel: WebGPU,
        bind_group_layout: WebGPUBindGroupLayout,
        label: USVString,
        can_gc: CanGc,
    ) -> DomRoot<Self> {
        reflect_dom_object(
            Box::new(GPUBindGroupLayout::new_inherited(
                channel,
                bind_group_layout,
                label,
            )),
            global,
            can_gc,
        )
    }
}

impl GPUBindGroupLayout {
    pub(crate) fn id(&self) -> WebGPUBindGroupLayout {
        self.droppable.bind_group_layout
    }

    /// <https://gpuweb.github.io/gpuweb/#GPUDevice-createBindGroupLayout>
    pub(crate) fn create(
        device: &GPUDevice,
        descriptor: &GPUBindGroupLayoutDescriptor,
        can_gc: CanGc,
    ) -> Fallible<DomRoot<GPUBindGroupLayout>> {
        let entries = descriptor
            .entries
            .iter()
            .map(|bgle| convert_bind_group_layout_entry(bgle, device))
            .collect::<Fallible<Result<Vec<_>, _>>>()?;

        let desc = match entries {
            Ok(entries) => Some(BindGroupLayoutDescriptor {
                label: (&descriptor.parent).convert(),
                entries: Cow::Owned(entries),
            }),
            Err(error) => {
                device.dispatch_error(error);
                None
            },
        };

        let bind_group_layout_id = device.global().wgpu_id_hub().create_bind_group_layout_id();
        device
            .channel()
            .0
            .send(WebGPURequest::CreateBindGroupLayout {
                device_id: device.id().0,
                bind_group_layout_id,
                descriptor: desc,
            })
            .expect("Failed to create WebGPU BindGroupLayout");

        let bgl = WebGPUBindGroupLayout(bind_group_layout_id);

        Ok(GPUBindGroupLayout::new(
            &device.global(),
            device.channel(),
            bgl,
            descriptor.parent.label.clone(),
            can_gc,
        ))
    }
}

impl GPUBindGroupLayoutMethods<script_bindings::DomTypeHolder> for GPUBindGroupLayout {
    /// <https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label>
    fn Label(&self) -> USVString {
        self.label.borrow().clone()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label>
    fn SetLabel(&self, value: USVString) {
        *self.label.borrow_mut() = value;
    }
}
