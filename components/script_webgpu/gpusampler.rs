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
    GPUSamplerDescriptor, GPUSamplerMethods, GPUSamplerWrap,
};
use script_bindings::reflector::{Reflector, reflect_dom_object_with_wrap};
use script_bindings::root::DomRoot;
use script_bindings::script_runtime::CanGc;
use script_bindings::str::USVString;
use webgpu_traits::{WebGPU, WebGPUDevice, WebGPURequest, WebGPUSampler};
use wgpu_core::resource::SamplerDescriptor;

use crate::gpuconvert::WebGPUConvert;
use crate::traits::{GPUDeviceTrait, WebGPUGlobalTrait};

#[derive(JSTraceable, MallocSizeOf)]
struct DroppableGPUSampler {
    #[no_trace]
    channel: WebGPU,
    #[no_trace]
    sampler: WebGPUSampler,
}

impl Drop for DroppableGPUSampler {
    fn drop(&mut self) {
        if let Err(e) = self
            .channel
            .0
            .send(WebGPURequest::DropSampler(self.sampler.0))
        {
            warn!("Failed to send DropSampler ({:?}) ({})", self.sampler.0, e);
        }
    }
}

#[dom_struct]
pub(crate) struct GPUSampler {
    reflector_: Reflector,
    label: DomRefCell<USVString>,
    #[no_trace]
    device: WebGPUDevice,
    compare_enable: bool,
    dropppable: DroppableGPUSampler,
}

impl GPUSampler {
    fn new_inherited(
        channel: WebGPU,
        device: WebGPUDevice,
        compare_enable: bool,
        sampler: WebGPUSampler,
        label: USVString,
    ) -> Self {
        Self {
            reflector_: Reflector::new(),
            label: DomRefCell::new(label),
            device,
            compare_enable,
            dropppable: DroppableGPUSampler { channel, sampler },
        }
    }

    pub(crate) fn new<D>(
        global: &D::GlobalScope,
        channel: WebGPU,
        device: WebGPUDevice,
        compare_enable: bool,
        sampler: WebGPUSampler,
        label: USVString,
        can_gc: CanGc,
    ) -> DomRoot<Self>
    where
        D: DomTypes<GPUSampler = GPUSampler>,
    {
        reflect_dom_object_with_wrap::<D, _, _, _>(
            Box::new(GPUSampler::new_inherited(
                channel,
                device,
                compare_enable,
                sampler,
                label,
            )),
            global,
            can_gc,
            GPUSamplerWrap::<D>,
        )
    }
}

impl GPUSampler {
    pub(crate) fn id(&self) -> WebGPUSampler {
        self.dropppable.sampler
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-createsampler>
    pub(crate) fn create<D>(
        device: &D::GPUDevice,
        descriptor: &GPUSamplerDescriptor,
        can_gc: CanGc,
    ) -> DomRoot<GPUSampler>
    where
        D: DomTypes<GPUSampler = GPUSampler>,
        D::GPUDevice: GPUDeviceTrait + WebGPUGlobalTrait<D>,
    {
        let sampler_id = device.wgpu_id_hub().create_sampler_id();
        let compare_enable = descriptor.compare.is_some();
        let desc = SamplerDescriptor {
            label: (&descriptor.parent).convert(),
            address_modes: [
                descriptor.addressModeU.convert(),
                descriptor.addressModeV.convert(),
                descriptor.addressModeW.convert(),
            ],
            mag_filter: descriptor.magFilter.convert(),
            min_filter: descriptor.minFilter.convert(),
            mipmap_filter: descriptor.mipmapFilter.convert(),
            lod_min_clamp: *descriptor.lodMinClamp,
            lod_max_clamp: *descriptor.lodMaxClamp,
            compare: descriptor.compare.map(WebGPUConvert::convert),
            anisotropy_clamp: 1,
            border_color: None,
        };

        device
            .channel()
            .0
            .send(WebGPURequest::CreateSampler {
                device_id: device.id().0,
                sampler_id,
                descriptor: desc,
            })
            .expect("Failed to create WebGPU sampler");

        let sampler = WebGPUSampler(sampler_id);

        GPUSampler::new::<D>(
            &device.global(),
            device.channel(),
            device.id(),
            compare_enable,
            sampler,
            descriptor.parent.label.clone(),
            can_gc,
        )
    }
}

impl<D: DomTypes> GPUSamplerMethods<D> for GPUSampler {
    /// <https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label>
    fn Label(&self) -> USVString {
        self.label.borrow().clone()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label>
    fn SetLabel(&self, value: USVString) {
        *self.label.borrow_mut() = value;
    }
}
