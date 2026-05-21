/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::borrow::Cow;
use std::marker::PhantomData;

use dom_struct::dom_struct;
use jstraceable_derive::JSTraceable;
use log::warn;
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::DomTypes;
use script_bindings::cell::DomRefCell;
use script_bindings::codegen::GenericBindings::WebGPUBinding::{
    GPUBindGroupLayoutDescriptor, GPUBindGroupLayoutMethods,
};
use script_bindings::error::Fallible;
use script_bindings::reflector::{
    Reflector, reflect_dom_object, reflect_dom_object_test_with_wrap2,
};
use script_bindings::root::DomRoot;
use script_bindings::str::USVString;
use webgpu_traits::{WebGPU, WebGPUBindGroupLayout, WebGPURequest};
use wgpu_core::binding_model::BindGroupLayoutDescriptor;

use crate::Convert;
use crate::gpuconvert::convert_bind_group_layout_entry;
use crate::gpudevice::GPUDevice;
use crate::script_runtime::CanGc;

#[derive(JSTraceable, MallocSizeOf)]
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

#[dom_struct]
pub struct GPUBindGroupLayout {
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

    pub(crate) fn new<D>(
        global: &D::GlobalScope,
        channel: WebGPU,
        bind_group_layout: WebGPUBindGroupLayout,
        label: USVString,
        can_gc: CanGc,
    ) -> DomRoot<Self>
    where
        D: DomTypes,
        Box<D::GPUBindGroupLayout>: From<Box<GPUBindGroupLayout>>,
        DomRoot<GPUBindGroupLayout>: From<DomRoot<D::GPUBindGroupLayout>>,
    {
        reflect_dom_object_test_with_wrap2::<D, _, _, _>(
            Box::new(GPUBindGroupLayout::new_inherited(
                channel,
                bind_group_layout,
                label,
            )),
            global,
            can_gc,
            script_bindings::codegen::GenericBindings::WebGPUBinding::GPUBindGroupLayoutWrap::<D>,
        )
    }
}

impl GPUBindGroupLayout {
    pub(crate) fn id(&self) -> WebGPUBindGroupLayout {
        self.droppable.bind_group_layout
    }

    /// <https://gpuweb.github.io/gpuweb/#GPUDevice-createBindGroupLayout>
    pub(crate) fn create<D: DomTypes>(
        device: &GPUDevice,
        descriptor: &GPUBindGroupLayoutDescriptor,
        can_gc: CanGc,
    ) -> Fallible<DomRoot<GPUBindGroupLayout>> {
        let entries = descriptor
            .entries
            .iter()
            .map(|bgle| convert_bind_group_layout_entry::<D>(bgle, device))
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

        todo!()
        /*
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
         */
    }
}

impl<D: DomTypes> GPUBindGroupLayoutMethods<D> for GPUBindGroupLayout {
    /// <https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label>
    fn Label(&self) -> USVString {
        self.label.borrow().clone()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label>
    fn SetLabel(&self, value: USVString) {
        *self.label.borrow_mut() = value;
    }
}
