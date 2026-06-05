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
    GPUCommandBufferDescriptor, GPUCommandEncoderDescriptor, GPUCommandEncoderMethods,
    GPUCommandEncoderWrap, GPUComputePassDescriptor, GPUExtent3D, GPURenderPassDescriptor,
    GPUSize64, GPUTexelCopyBufferInfo, GPUTexelCopyTextureInfo, GPUVertexBufferLayout,
};
use script_bindings::codegen::GenericUnionTypes::RangeEnforcedUnsignedLongSequenceOrGPUExtent3DDict;
use script_bindings::error::Fallible;
use script_bindings::reflector::{Reflector, reflect_dom_object_with_wrap};
use script_bindings::root::{Dom, DomRoot};
use script_bindings::script_runtime::CanGc;
use script_bindings::str::USVString;
use webgpu_traits::{
    WebGPU, WebGPUCommandBuffer, WebGPUCommandEncoder, WebGPUComputePass, WebGPUDevice,
    WebGPURenderPass, WebGPURequest,
};
use wgpu_core::command as wgpu_com;

use crate::gpubuffer::GPUBuffer;
use crate::gpucommandbuffer::GPUCommandBuffer;
use crate::gpucomputepassencoder::GPUComputePassEncoder;
use crate::gpuconvert::{WebGPUConvert, WebGPUTryConvert, convert_load_op};
use crate::gpudevice::GPUDevice;
use crate::gpurenderpassencoder::GPURenderPassEncoder;
use crate::gputexture::GPUTexture;
use crate::gputextureview::GPUTextureView;
use crate::traits::WebGPUGlobalTrait;

#[derive(JSTraceable, MallocSizeOf)]
struct DroppableGPUCommandEncoder {
    #[no_trace]
    channel: WebGPU,
    #[no_trace]
    encoder: WebGPUCommandEncoder,
}

#[dom_struct]
pub(crate) struct GPUCommandEncoder {
    reflector_: Reflector,
    droppable: DroppableGPUCommandEncoder,
    label: DomRefCell<USVString>,
    device: Dom<GPUDevice>,
}

impl Drop for DroppableGPUCommandEncoder {
    fn drop(&mut self) {
        if let Err(e) = self
            .channel
            .0
            .send(WebGPURequest::DropCommandEncoder(self.encoder.0))
        {
            warn!("Failed to send WebGPURequest::DropCommandEncoder with {e:?}");
        }
    }
}

impl GPUCommandEncoder {
    pub(crate) fn new_inherited<D>(
        channel: WebGPU,
        device: &GPUDevice,
        encoder: WebGPUCommandEncoder,
        label: USVString,
    ) -> Self
    where
        D: DomTypes<GPUCommandEncoder = GPUCommandEncoder>,
    {
        Self {
            droppable: DroppableGPUCommandEncoder { channel, encoder },
            reflector_: Reflector::new(),
            label: DomRefCell::new(label),
            device: Dom::from_ref(device),
        }
    }

    pub(crate) fn new<D>(
        global: &D::GlobalScope,
        channel: WebGPU,
        device: &GPUDevice,
        encoder: WebGPUCommandEncoder,
        label: USVString,
        can_gc: CanGc,
    ) -> DomRoot<Self>
    where
        D: DomTypes<GPUCommandEncoder = GPUCommandEncoder>,
    {
        reflect_dom_object_with_wrap::<D, _, _, _>(
            Box::new(GPUCommandEncoder::new_inherited::<D>(
                channel, device, encoder, label,
            )),
            global,
            can_gc,
            GPUCommandEncoderWrap::<D>,
        )
    }
}

impl GPUCommandEncoder {
    pub(crate) fn id(&self) -> WebGPUCommandEncoder {
        self.droppable.encoder
    }

    pub(crate) fn device_id(&self) -> WebGPUDevice {
        self.device.id()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-createcommandencoder>
    pub(crate) fn create<D>(
        device: &GPUDevice,
        descriptor: &GPUCommandEncoderDescriptor,
        can_gc: CanGc,
    ) -> DomRoot<GPUCommandEncoder>
    where
        D: DomTypes<GPUDevice = GPUDevice, GPUCommandEncoder = GPUCommandEncoder>,
        GPUDevice: WebGPUGlobalTrait<D>,
    {
        let command_encoder_id = device.wgpu_id_hub().create_command_encoder_id();
        device
            .channel()
            .0
            .send(WebGPURequest::CreateCommandEncoder {
                device_id: device.id().0,
                command_encoder_id,
                desc: wgpu_types::CommandEncoderDescriptor {
                    label: (&descriptor.parent).convert(),
                },
            })
            .expect("Failed to create WebGPU command encoder");

        let encoder = WebGPUCommandEncoder(command_encoder_id);

        GPUCommandEncoder::new::<D>(
            &device.global(),
            device.channel(),
            device,
            encoder,
            descriptor.parent.label.clone(),
            can_gc,
        )
    }
}

impl<D> GPUCommandEncoderMethods<D> for GPUCommandEncoder
where
    D: DomTypes<
            GPUComputePassEncoder = GPUComputePassEncoder,
            GPURenderPassEncoder = GPURenderPassEncoder,
            GPUCommandBuffer = GPUCommandBuffer,
            GPUBuffer = GPUBuffer,
            GPUCommandEncoder = GPUCommandEncoder,
            GPUTextureView = GPUTextureView,
            GPUTexture = GPUTexture,
        >,
    GPUCommandEncoder: WebGPUGlobalTrait<D>,
{
    /// <https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label>
    fn Label(&self) -> USVString {
        self.label.borrow().clone()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label>
    fn SetLabel(&self, value: USVString) {
        *self.label.borrow_mut() = value;
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpucommandencoder-begincomputepass>
    fn BeginComputePass(
        &self,
        descriptor: &GPUComputePassDescriptor,
    ) -> DomRoot<GPUComputePassEncoder> {
        let compute_pass_id = self.wgpu_id_hub().create_compute_pass_id();

        if let Err(e) = self
            .droppable
            .channel
            .0
            .send(WebGPURequest::BeginComputePass {
                command_encoder_id: self.id().0,
                compute_pass_id,
                label: (&descriptor.parent).convert(),
                device_id: self.device.id().0,
            })
        {
            warn!("Failed to send WebGPURequest::BeginComputePass {e:?}");
        }

        GPUComputePassEncoder::new::<D>(
            &self.global(),
            self.droppable.channel.clone(),
            self,
            WebGPUComputePass(compute_pass_id),
            descriptor.parent.label.clone(),
            CanGc::deprecated_note(),
        )
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpucommandencoder-beginrenderpass>
    fn BeginRenderPass(
        &self,
        descriptor: &GPURenderPassDescriptor<D>,
    ) -> Fallible<DomRoot<GPURenderPassEncoder>> {
        let depth_stencil_attachment = descriptor.depthStencilAttachment.as_ref().map(|ds| {
            wgpu_com::RenderPassDepthStencilAttachment {
                depth: wgpu_com::PassChannel {
                    load_op: ds
                        .depthLoadOp
                        .as_ref()
                        .map(|l| convert_load_op(l, ds.depthClearValue.map(|v| *v))),
                    store_op: ds.depthStoreOp.as_ref().map(WebGPUConvert::convert),
                    read_only: ds.depthReadOnly,
                },
                stencil: wgpu_com::PassChannel {
                    load_op: ds
                        .stencilLoadOp
                        .as_ref()
                        .map(|l| convert_load_op(l, Some(ds.stencilClearValue))),
                    store_op: ds.stencilStoreOp.as_ref().map(WebGPUConvert::convert),
                    read_only: ds.stencilReadOnly,
                },
                view: ds.view.convert().0,
            }
        });

        let color_attachments = descriptor
            .colorAttachments
            .iter()
            .map(|color| -> Fallible<_> {
                Ok(Some(wgpu_com::RenderPassColorAttachment {
                    resolve_target: color.resolveTarget.as_ref().map(|t| t.convert().0),
                    load_op: convert_load_op(
                        &color.loadOp,
                        color
                            .clearValue
                            .as_ref()
                            .map(|color| (color).try_convert())
                            .transpose()?
                            .unwrap_or_default(),
                    ),
                    store_op: color.storeOp.convert(),
                    view: color.view.convert().0,
                    depth_slice: None,
                }))
            })
            .collect::<Fallible<Vec<_>>>()?;
        let render_pass_id = self.wgpu_id_hub().create_render_pass_id();

        if let Err(e) = self
            .droppable
            .channel
            .0
            .send(WebGPURequest::BeginRenderPass {
                command_encoder_id: self.id().0,
                render_pass_id,
                label: (&descriptor.parent).convert(),
                depth_stencil_attachment,
                color_attachments,
                device_id: self.device.id().0,
            })
        {
            warn!("Failed to send WebGPURequest::BeginRenderPass {e:?}");
        }

        Ok(GPURenderPassEncoder::new::<D>(
            &self.global(),
            self.droppable.channel.clone(),
            WebGPURenderPass(render_pass_id),
            self,
            descriptor.parent.label.clone(),
            CanGc::deprecated_note(),
        ))
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpucommandencoder-copybuffertobuffer>
    fn CopyBufferToBuffer(
        &self,
        source: &GPUBuffer,
        source_offset: GPUSize64,
        destination: &GPUBuffer,
        destination_offset: GPUSize64,
        size: GPUSize64,
    ) {
        self.droppable
            .channel
            .0
            .send(WebGPURequest::CopyBufferToBuffer {
                command_encoder_id: self.droppable.encoder.0,
                source_id: source.id().0,
                source_offset,
                destination_id: destination.id().0,
                destination_offset,
                size,
                device_id: self.device.id().0,
            })
            .expect("Failed to send CopyBufferToBuffer");
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpucommandencoder-copybuffertotexture>
    fn CopyBufferToTexture(
        &self,
        source: &GPUTexelCopyBufferInfo<D>,
        destination: &GPUTexelCopyTextureInfo<D>,
        copy_size: GPUExtent3D,
    ) -> Fallible<()> {
        self.droppable
            .channel
            .0
            .send(WebGPURequest::CopyBufferToTexture {
                command_encoder_id: self.droppable.encoder.0,
                source: source.convert(),
                destination: destination.try_convert()?,
                copy_size: (&copy_size).try_convert()?,
                device_id: self.device.id().0,
            })
            .expect("Failed to send CopyBufferToTexture");

        Ok(())
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpucommandencoder-copybuffertotexture>
    fn CopyTextureToBuffer(
        &self,
        source: &GPUTexelCopyTextureInfo<D>,
        destination: &GPUTexelCopyBufferInfo<D>,
        copy_size: GPUExtent3D,
    ) -> Fallible<()> {
        self.droppable
            .channel
            .0
            .send(WebGPURequest::CopyTextureToBuffer {
                command_encoder_id: self.droppable.encoder.0,
                source: source.try_convert()?,
                destination: destination.convert(),
                copy_size: (&copy_size).try_convert()?,
                device_id: self.device.id().0,
            })
            .expect("Failed to send CopyTextureToBuffer");

        Ok(())
    }

    /// <https://gpuweb.github.io/gpuweb/#GPUCommandEncoder-copyTextureToTexture>
    fn CopyTextureToTexture(
        &self,
        source: &GPUTexelCopyTextureInfo<D>,
        destination: &GPUTexelCopyTextureInfo<D>,
        copy_size: GPUExtent3D,
    ) -> Fallible<()> {
        self.droppable
            .channel
            .0
            .send(WebGPURequest::CopyTextureToTexture {
                command_encoder_id: self.droppable.encoder.0,
                source: source.try_convert()?,
                destination: destination.try_convert()?,
                copy_size: (&copy_size).try_convert()?,
                device_id: self.device.id().0,
            })
            .expect("Failed to send CopyTextureToTexture");

        Ok(())
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpucommandencoder-finish>
    fn Finish(&self, descriptor: &GPUCommandBufferDescriptor) -> DomRoot<GPUCommandBuffer> {
        let command_buffer_id = self.wgpu_id_hub().create_command_buffer_id();
        self.droppable
            .channel
            .0
            .send(WebGPURequest::CommandEncoderFinish {
                command_encoder_id: self.droppable.encoder.0,
                device_id: self.device.id().0,
                desc: wgpu_types::CommandBufferDescriptor {
                    label: (&descriptor.parent).convert(),
                },
                command_buffer_id,
            })
            .expect("Failed to send Finish");

        let buffer = WebGPUCommandBuffer(command_buffer_id);
        GPUCommandBuffer::new::<D>(
            &self.global(),
            self.droppable.channel.clone(),
            buffer,
            descriptor.parent.label.clone(),
            CanGc::deprecated_note(),
        )
    }
}
