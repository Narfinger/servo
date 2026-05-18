/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::borrow::Cow;
use std::cell::Cell;
use std::rc::Rc;

use dom_struct::dom_struct;
use js::jsapi::{HandleObject, Heap, JSObject};
use jstraceable_derive::JSTraceable;
use log::warn;
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::cell::DomRefCell;
use script_bindings::codegen::GenericBindings::WebGPUBinding::{
    GPUBindGroupDescriptor, GPUBindGroupLayoutDescriptor, GPUBufferDescriptor,
    GPUCommandEncoderDescriptor, GPUComputePipelineDescriptor, GPUDeviceLostReason,
    GPUDeviceMethods, GPUErrorFilter, GPUPipelineLayoutDescriptor,
    GPURenderBundleEncoderDescriptor, GPURenderPipelineDescriptor, GPUSamplerDescriptor,
    GPUShaderModuleDescriptor, GPUTextureDescriptor, GPUTextureFormat, GPUVertexStepMode,
};
use script_bindings::codegen::GenericUnionTypes::GPUPipelineLayoutOrGPUAutoLayoutMode;
use script_bindings::error::{Error, Fallible};
use script_bindings::realms::InRealm;
use script_bindings::reflector::reflect_dom_object;
use script_bindings::root::{Dom, DomRoot};
use script_bindings::str::USVString;
use script_bindings::trace::RootedTraceableBox;
use script_bindings::{DomTypes, cformat};
use webgpu_traits::{
    PopError, WebGPU, WebGPUComputePipeline, WebGPUComputePipelineResponse, WebGPUDevice,
    WebGPUPoppedErrorScopeResponse, WebGPUQueue, WebGPURenderPipeline,
    WebGPURenderPipelineResponse, WebGPURequest,
};
use wgpu_core::id::PipelineLayoutId;
use wgpu_core::pipeline as wgpu_pipe;
use wgpu_core::pipeline::RenderPipelineDescriptor;
use wgpu_types::{self, TextureFormat};

use super::gpudevicelostinfo::GPUDeviceLostInfo;
use super::gpuerror::AsWebGpu;
use super::gpupipelineerror::GPUPipelineError;
use super::gpusupportedlimits::GPUSupportedLimits;
use crate::Convert;
use crate::gpuadapter::GPUAdapter;
use crate::gpuadapterinfo::GPUAdapterInfo;
use crate::gpubindgroup::GPUBindGroup;
use crate::gpubindgrouplayout::GPUBindGroupLayout;
use crate::gpubuffer::GPUBuffer;
use crate::gpucommandencoder::GPUCommandEncoder;
use crate::gpucomputepipeline::GPUComputePipeline;
use crate::gpupipelinelayout::GPUPipelineLayout;
use crate::gpuqueue::GPUQueue;
use crate::gpurenderbundleencoder::GPURenderBundleEncoder;
use crate::gpurenderpipeline::GPURenderPipeline;
use crate::gpusampler::GPUSampler;
use crate::gpushadermodule::GPUShaderModule;
use crate::gpusupportedfeatures::GPUSupportedFeatures;
use crate::gputexture::GPUTexture;
use crate::script_runtime::CanGc;

#[derive(JSTraceable, MallocSizeOf)]
struct DroppableGPUDevice {
    #[no_trace]
    channel: WebGPU,
    #[no_trace]
    device: WebGPUDevice,
}

impl Drop for DroppableGPUDevice {
    fn drop(&mut self) {
        if let Err(e) = self
            .channel
            .0
            .send(WebGPURequest::DropDevice(self.device.0))
        {
            warn!("Failed to send DropDevice ({:?}) ({})", self.device.0, e);
        }
    }
}

#[dom_struct]
pub(crate) struct GPUDevice<D: DomTypes> {
    eventtarget: D::EventTarget,
    adapter: Dom<GPUAdapter<D>>,
    #[ignore_malloc_size_of = "mozjs"]
    extensions: Heap<*mut JSObject>,
    features: Dom<GPUSupportedFeatures<D>>,
    limits: Dom<GPUSupportedLimits<D>>,
    adapter_info: Dom<GPUAdapterInfo<D>>,
    label: DomRefCell<USVString>,
    default_queue: Dom<GPUQueue>,
    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-lost>
    #[conditional_malloc_size_of]
    lost_promise: DomRefCell<Rc<D::Promise>>,
    valid: Cell<bool>,
    droppable: DroppableGPUDevice,
}

pub(crate) enum PipelineLayout {
    Implicit,
    Explicit(PipelineLayoutId),
}

impl PipelineLayout {
    pub(crate) fn explicit(&self) -> Option<PipelineLayoutId> {
        match self {
            PipelineLayout::Explicit(layout_id) => Some(*layout_id),
            PipelineLayout::Implicit => None,
        }
    }
}

impl<D: DomTypes> GPUDevice<D> {
    #[allow(clippy::too_many_arguments)]
    fn new_inherited(
        channel: WebGPU,
        adapter: &GPUAdapter<D>,
        features: &GPUSupportedFeatures<D>,
        limits: &GPUSupportedLimits<D>,
        adapter_info: &GPUAdapterInfo<D>,
        device: WebGPUDevice,
        queue: &GPUQueue<D>,
        label: String,
        lost_promise: Rc<D::Promise>,
    ) -> Self {
        Self {
            eventtarget: D::EventTarget::new_inherited(),
            adapter: Dom::from_ref(adapter),
            extensions: Heap::default(),
            features: Dom::from_ref(features),
            limits: Dom::from_ref(limits),
            adapter_info: Dom::from_ref(adapter_info),
            label: DomRefCell::new(USVString::from(label)),
            default_queue: Dom::from_ref(queue),
            lost_promise: DomRefCell::new(lost_promise),
            valid: Cell::new(true),
            droppable: DroppableGPUDevice { channel, device },
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        global: &D::GlobalScope,
        channel: WebGPU,
        adapter: &GPUAdapter<D>,
        extensions: HandleObject,
        features: wgpu_types::Features,
        limits: wgpu_types::Limits,
        device: WebGPUDevice,
        queue: WebGPUQueue,
        label: String,
        can_gc: CanGc,
    ) -> DomRoot<Self> {
        let queue = GPUQueue::new(global, channel.clone(), queue, can_gc);
        let limits = GPUSupportedLimits::new(global, limits, can_gc);
        let features = GPUSupportedFeatures::Constructor(global, None, features, can_gc).unwrap();
        let adapter_info = GPUAdapterInfo::clone_from(global, &adapter.Info(), can_gc);
        let lost_promise = Promise::new(global, can_gc);
        let device = reflect_dom_object(
            Box::new(GPUDevice::new_inherited(
                channel,
                adapter,
                &features,
                &limits,
                &adapter_info,
                device,
                &queue,
                label,
                lost_promise,
            )),
            global,
            can_gc,
        );
        queue.set_device(&device);
        device.extensions.set(*extensions);
        device
    }
}

impl GPUDevice {
    pub(crate) fn id(&self) -> WebGPUDevice {
        self.droppable.device
    }

    pub(crate) fn queue_id(&self) -> WebGPUQueue {
        self.default_queue.id()
    }

    pub(crate) fn channel(&self) -> WebGPU {
        self.droppable.channel.clone()
    }

    pub(crate) fn dispatch_error(&self, error: webgpu_traits::Error) {
        if let Err(e) = self.droppable.channel.0.send(WebGPURequest::DispatchError {
            device_id: self.id().0,
            error,
        }) {
            warn!("Failed to send WebGPURequest::DispatchError due to {e:?}");
        }
    }

    /// <https://gpuweb.github.io/gpuweb/#eventdef-gpudevice-uncapturederror>
    pub(crate) fn fire_uncaptured_error(&self, error: webgpu_traits::Error) {
        let this = Trusted::new(self);

        // Queue a global task, using the webgpu task source, to fire an event named
        // uncapturederror at a GPUDevice using GPUUncapturedErrorEvent.
        self.global().task_manager().webgpu_task_source().queue(
            /*
             *

            task!(fire_uncaptured_error: move || {
                let this = this.root();
                let error = GPUError::from_error(&this.global(), error, CanGc::deprecated_note());

                let event = GPUUncapturedErrorEvent::new(
                    &this.global(),
                    atom!("uncapturederror"),
                    &GPUUncapturedErrorEventInit {
                        error,
                        parent: EventInit::empty(),
                    },
                    CanGc::deprecated_note(),
                );

                event.upcast::<Event>().fire(this.upcast(), CanGc::deprecated_note());
            }),
             */
        );
    }

    /// <https://gpuweb.github.io/gpuweb/#abstract-opdef-validate-texture-format-required-features>
    ///
    /// Validates that the device suppports required features,
    /// and if so returns an ok containing wgpu's `TextureFormat`
    pub(crate) fn validate_texture_format_required_features(
        &self,
        format: &GPUTextureFormat,
    ) -> Fallible<TextureFormat> {
        let texture_format: TextureFormat = (*format).convert();
        if self
            .features
            .wgpu_features()
            .contains(texture_format.required_features())
        {
            Ok(texture_format)
        } else {
            Err(Error::Type(cformat!(
                "{texture_format:?} is not supported by this GPUDevice"
            )))
        }
    }

    pub(crate) fn is_lost(&self) -> bool {
        self.lost_promise.borrow().is_fulfilled()
    }

    pub(crate) fn get_pipeline_layout_data(
        &self,
        layout: &GPUPipelineLayoutOrGPUAutoLayoutMode<D>,
    ) -> PipelineLayout {
        if let GPUPipelineLayoutOrGPUAutoLayoutMode::GPUPipelineLayout(layout) = layout {
            PipelineLayout::Explicit(layout.id().0)
        } else {
            PipelineLayout::Implicit
        }
    }

    pub(crate) fn parse_render_pipeline<'a>(
        &self,
        descriptor: &GPURenderPipelineDescriptor<D>,
    ) -> Fallible<RenderPipelineDescriptor<'a>> {
        let pipeline_layout = self.get_pipeline_layout_data(&descriptor.parent.layout);
        let desc = wgpu_pipe::RenderPipelineDescriptor {
            label: (&descriptor.parent.parent).convert(),
            layout: pipeline_layout.explicit(),
            cache: None,
            vertex: wgpu_pipe::VertexState {
                stage: (&descriptor.vertex.parent).convert(),
                buffers: Cow::Owned(
                    descriptor
                        .vertex
                        .buffers
                        .iter()
                        .map(|buffer| wgpu_pipe::VertexBufferLayout {
                            array_stride: buffer.arrayStride,
                            step_mode: match buffer.stepMode {
                                GPUVertexStepMode::Vertex => wgpu_types::VertexStepMode::Vertex,
                                GPUVertexStepMode::Instance => wgpu_types::VertexStepMode::Instance,
                            },
                            attributes: Cow::Owned(
                                buffer
                                    .attributes
                                    .iter()
                                    .map(|att| wgpu_types::VertexAttribute {
                                        format: att.format.convert(),
                                        offset: att.offset,
                                        shader_location: att.shaderLocation,
                                    })
                                    .collect::<Vec<_>>(),
                            ),
                        })
                        .collect::<Vec<_>>(),
                ),
            },
            fragment: descriptor
                .fragment
                .as_ref()
                .map(|stage| -> Fallible<wgpu_pipe::FragmentState> {
                    Ok(wgpu_pipe::FragmentState {
                        stage: (&stage.parent).convert(),
                        targets: Cow::Owned(
                            stage
                                .targets
                                .iter()
                                .map(|state| {
                                    self.validate_texture_format_required_features(&state.format)
                                        .map(|format| {
                                            Some(wgpu_types::ColorTargetState {
                                                format,
                                                write_mask:
                                                    wgpu_types::ColorWrites::from_bits_retain(
                                                        state.writeMask,
                                                    ),
                                                blend: state.blend.as_ref().map(|blend| {
                                                    wgpu_types::BlendState {
                                                        color: (&blend.color).convert(),
                                                        alpha: (&blend.alpha).convert(),
                                                    }
                                                }),
                                            })
                                        })
                                })
                                .collect::<Result<Vec<_>, _>>()?,
                        ),
                    })
                })
                .transpose()?,
            primitive: (&descriptor.primitive).convert(),
            depth_stencil: descriptor
                .depthStencil
                .as_ref()
                .map(|dss_desc| {
                    self.validate_texture_format_required_features(&dss_desc.format)
                        .map(|format| wgpu_types::DepthStencilState {
                            format,
                            // TODO(sagudev): these need webidl sync
                            depth_write_enabled: Some(dss_desc.depthWriteEnabled),
                            depth_compare: Some(dss_desc.depthCompare.convert()),
                            stencil: wgpu_types::StencilState {
                                front: wgpu_types::StencilFaceState {
                                    compare: dss_desc.stencilFront.compare.convert(),

                                    fail_op: dss_desc.stencilFront.failOp.convert(),
                                    depth_fail_op: dss_desc.stencilFront.depthFailOp.convert(),
                                    pass_op: dss_desc.stencilFront.passOp.convert(),
                                },
                                back: wgpu_types::StencilFaceState {
                                    compare: dss_desc.stencilBack.compare.convert(),
                                    fail_op: dss_desc.stencilBack.failOp.convert(),
                                    depth_fail_op: dss_desc.stencilBack.depthFailOp.convert(),
                                    pass_op: dss_desc.stencilBack.passOp.convert(),
                                },
                                read_mask: dss_desc.stencilReadMask,
                                write_mask: dss_desc.stencilWriteMask,
                            },
                            bias: wgpu_types::DepthBiasState {
                                constant: dss_desc.depthBias,
                                slope_scale: *dss_desc.depthBiasSlopeScale,
                                clamp: *dss_desc.depthBiasClamp,
                            },
                        })
                })
                .transpose()?,
            multisample: wgpu_types::MultisampleState {
                count: descriptor.multisample.count,
                mask: descriptor.multisample.mask as u64,
                alpha_to_coverage_enabled: descriptor.multisample.alphaToCoverageEnabled,
            },
            multiview_mask: None,
        };
        Ok(desc)
    }

    /// <https://gpuweb.github.io/gpuweb/#lose-the-device>
    pub(crate) fn lose(&self, reason: GPUDeviceLostReason, msg: String) {
        let this = Trusted::new(self);

        // Queue a global task, using the webgpu task source, to resolve device.lost
        // promise with a new GPUDeviceLostInfo with reason and message.
        /*
        self.global().task_manager().webgpu_task_source().queue(
             *
            task!(resolve_device_lost: move || {
                let this = this.root();

                let lost_promise = &(*this.lost_promise.borrow());
                let lost = GPUDeviceLostInfo::new(&this.global(), msg.into(), reason, CanGc::deprecated_note());
                lost_promise.resolve_native(&*lost, CanGc::deprecated_note());
            }),
        );
             */
    }
}

impl<D> GPUDeviceMethods<D> for GPUDevice<D>
where
    D: DomTypes,
    D::GPUSupportedFeatures: From<GPUSupportedFeatures<D>>,
    D::GPUSupportedLimits: From<GPUSupportedLimits<D>>,
    D::GPUAdapterInfo: From<GPUAdapter<D>>,
{
    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-features>
    fn Features(&self) -> DomRoot<D::GPUSupportedFeatures> {
        DomRoot::from_ref(&self.features.into())
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-limits>
    fn Limits(&self) -> DomRoot<D::GPUSupportedLimits> {
        DomRoot::from_ref(&self.limits.into())
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-adapterinfo>
    fn AdapterInfo(&self) -> DomRoot<D::GPUAdapterInfo> {
        DomRoot::from_ref(&self.adapter_info.into())
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-queue>
    fn GetQueue(&self) -> DomRoot<D::GPUQueue> {
        DomRoot::from_ref(&self.default_queue.into())
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label>
    fn Label(&self) -> USVString {
        self.label.borrow().clone()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label>
    fn SetLabel(&self, value: USVString) {
        *self.label.borrow_mut() = value;
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-lost>
    fn Lost(&self) -> Rc<D::Promise> {
        self.lost_promise.borrow().clone()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-createbuffer>
    fn CreateBuffer(&self, descriptor: &GPUBufferDescriptor) -> Fallible<DomRoot<D::GPUBuffer>> {
        GPUBuffer::create(self, descriptor.into(), CanGc::deprecated_note()).into()
    }

    /// <https://gpuweb.github.io/gpuweb/#GPUDevice-createBindGroupLayout>
    fn CreateBindGroupLayout(
        &self,
        descriptor: &GPUBindGroupLayoutDescriptor,
    ) -> Fallible<DomRoot<D::GPUBindGroupLayout>> {
        GPUBindGroupLayout::create(self, descriptor.into(), CanGc::deprecated_note()).into()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-createpipelinelayout>
    fn CreatePipelineLayout(
        &self,
        descriptor: &GPUPipelineLayoutDescriptor<D>,
    ) -> DomRoot<D::GPUPipelineLayout> {
        GPUPipelineLayout::create(self, descriptor.into(), CanGc::deprecated_note()).into()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-createbindgroup>
    fn CreateBindGroup(&self, descriptor: &GPUBindGroupDescriptor<D>) -> DomRoot<D::GPUBindGroup> {
        GPUBindGroup::create(self, descriptor.into(), CanGc::deprecated_note()).into()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-createshadermodule>
    fn CreateShaderModule(
        &self,
        descriptor: RootedTraceableBox<GPUShaderModuleDescriptor>,
        comp: InRealm,
        can_gc: CanGc,
    ) -> DomRoot<D::GPUShaderModule> {
        GPUShaderModule::create(self, descriptor.into(), comp, can_gc).into()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-createcomputepipeline>
    fn CreateComputePipeline(
        &self,
        descriptor: &GPUComputePipelineDescriptor<D>,
    ) -> DomRoot<D::GPUComputePipeline> {
        let compute_pipeline = GPUComputePipeline::create(self, descriptor, None);
        GPUComputePipeline::new(
            &self.global(),
            compute_pipeline,
            descriptor.parent.parent.label.clone(),
            self,
            CanGc::deprecated_note(),
        )
        .into()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-createcomputepipelineasync>
    fn CreateComputePipelineAsync(
        &self,
        descriptor: &GPUComputePipelineDescriptor<D>,
        comp: InRealm,
        can_gc: CanGc,
    ) -> Rc<D::Promise> {
        let promise = D::Promise::new_in_current_realm(comp, can_gc);
        /*
         *
        let callback = callback_promise(
            &promise,
            self,
            self.global().task_manager().dom_manipulation_task_source(),
        );
        GPUComputePipeline::create(self, descriptor, Some(callback));
         */
        promise
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-createcommandencoder>
    fn CreateCommandEncoder(
        &self,
        descriptor: &GPUCommandEncoderDescriptor,
    ) -> DomRoot<D::GPUCommandEncoder> {
        GPUCommandEncoder::create(self, descriptor, CanGc::deprecated_note()).into()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-createtexture>
    fn CreateTexture(&self, descriptor: &GPUTextureDescriptor) -> Fallible<DomRoot<D::GPUTexture>> {
        GPUTexture::create(self, descriptor, CanGc::deprecated_note()).into()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-createsampler>
    fn CreateSampler(&self, descriptor: &GPUSamplerDescriptor) -> DomRoot<D::GPUSampler> {
        GPUSampler::create(self, descriptor, CanGc::deprecated_note()).into()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-createrenderpipeline>
    fn CreateRenderPipeline(
        &self,
        descriptor: &GPURenderPipelineDescriptor<D>,
    ) -> Fallible<DomRoot<D::GPURenderPipeline>> {
        let desc = self.parse_render_pipeline(descriptor)?;
        let render_pipeline = GPURenderPipeline::create(self, desc, None)?;
        Ok(GPURenderPipeline::new(
            &self.global(),
            render_pipeline,
            descriptor.parent.parent.label.clone(),
            self,
            CanGc::deprecated_note(),
        ))
        .into()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-createrenderpipelineasync>
    fn CreateRenderPipelineAsync(
        &self,
        descriptor: &GPURenderPipelineDescriptor<D>,
        comp: InRealm,
        can_gc: CanGc,
    ) -> Fallible<Rc<D::Promise>> {
        let desc = self.parse_render_pipeline(descriptor)?;
        let promise = D::Promise::new_in_current_realm(comp, can_gc);
        /*
        let callback = callback_promise(
            &promise,
            self,
            self.global().task_manager().dom_manipulation_task_source(),
        );
        GPURenderPipeline::create(self, desc, Some(callback))?;
         */
        Ok(promise)
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-createrenderbundleencoder>
    fn CreateRenderBundleEncoder(
        &self,
        descriptor: &GPURenderBundleEncoderDescriptor,
    ) -> Fallible<DomRoot<D::GPURenderBundleEncoder>> {
        GPURenderBundleEncoder::create(self, descriptor, CanGc::deprecated_note()).into()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-pusherrorscope>
    fn PushErrorScope(&self, filter: GPUErrorFilter) {
        if self
            .droppable
            .channel
            .0
            .send(WebGPURequest::PushErrorScope {
                device_id: self.id().0,
                filter: filter.as_webgpu(),
            })
            .is_err()
        {
            warn!("Failed sending WebGPURequest::PushErrorScope");
        }
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-poperrorscope>
    fn PopErrorScope(&self, comp: InRealm, can_gc: CanGc) -> Rc<D::Promise> {
        let promise = D::Promise::new_in_current_realm(comp, can_gc);
        /*
        let callback = callback_promise(
            &promise,
            self,
            self.global().task_manager().dom_manipulation_task_source(),
        );
        if self
            .droppable
            .channel
            .0
            .send(WebGPURequest::PopErrorScope {
                device_id: self.id().0,
                callback,
            })
            .is_err()
        {
            warn!("Error when sending WebGPURequest::PopErrorScope");
        }
         */
        promise
    }

    // https://gpuweb.github.io/gpuweb/#dom-gpudevice-onuncapturederror
    event_handler!(uncapturederror, GetOnuncapturederror, SetOnuncapturederror);

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevice-destroy>
    fn Destroy(&self) {
        if self.valid.get() {
            self.valid.set(false);

            if let Err(e) = self
                .droppable
                .channel
                .0
                .send(WebGPURequest::DestroyDevice(self.id().0))
            {
                warn!("Failed to send DestroyDevice ({:?}) ({})", self.id().0, e);
            }
        }
    }
}

/*
impl RoutedPromiseListener<WebGPUPoppedErrorScopeResponse> for GPUDevice {
    fn handle_response(
        &self,
        cx: &mut js::context::JSContext,
        response: WebGPUPoppedErrorScopeResponse,
        promise: &Rc<Promise>,
    ) {
        match response {
            Ok(None) | Err(PopError::Lost) => {
                promise.resolve_native(&None::<Option<GPUError>>, CanGc::from_cx(cx))
            },
            Err(PopError::Empty) => {
                promise.reject_error(Error::Operation(None), CanGc::from_cx(cx))
            },
            Ok(Some(error)) => {
                let error = GPUError::from_error(&self.global(), error, CanGc::from_cx(cx));
                promise.resolve_native(&error, CanGc::from_cx(cx));
            },
        }
    }
}

impl RoutedPromiseListener<WebGPUComputePipelineResponse> for GPUDevice {
    fn handle_response(
        &self,
        cx: &mut js::context::JSContext,
        response: WebGPUComputePipelineResponse,
        promise: &Rc<Promise>,
    ) {
        match response {
            Ok(pipeline) => promise.resolve_native(
                &GPUComputePipeline::new(
                    &self.global(),
                    WebGPUComputePipeline(pipeline.id),
                    pipeline.label.into(),
                    self,
                    CanGc::from_cx(cx),
                ),
                CanGc::from_cx(cx),
            ),
            Err(webgpu_traits::Error::Validation(msg)) => promise.reject_native(
                &GPUPipelineError::new(
                    &self.global(),
                    msg.into(),
                    GPUPipelineErrorReason::Validation,
                    CanGc::from_cx(cx),
                ),
                CanGc::from_cx(cx),
            ),
            Err(webgpu_traits::Error::OutOfMemory(msg) | webgpu_traits::Error::Internal(msg)) => {
                promise.reject_native(
                    &GPUPipelineError::new(
                        &self.global(),
                        msg.into(),
                        GPUPipelineErrorReason::Internal,
                        CanGc::from_cx(cx),
                    ),
                    CanGc::from_cx(cx),
                )
            },
        }
    }
}

impl RoutedPromiseListener<WebGPURenderPipelineResponse> for GPUDevice {
    fn handle_response(
        &self,
        cx: &mut js::context::JSContext,
        response: WebGPURenderPipelineResponse,
        promise: &Rc<Promise>,
    ) {
        match response {
            Ok(pipeline) => promise.resolve_native(
                &GPURenderPipeline::new(
                    &self.global(),
                    WebGPURenderPipeline(pipeline.id),
                    pipeline.label.into(),
                    self,
                    CanGc::from_cx(cx),
                ),
                CanGc::from_cx(cx),
            ),
            Err(webgpu_traits::Error::Validation(msg)) => promise.reject_native(
                &GPUPipelineError::new(
                    &self.global(),
                    msg.into(),
                    GPUPipelineErrorReason::Validation,
                    CanGc::from_cx(cx),
                ),
                CanGc::from_cx(cx),
            ),
            Err(webgpu_traits::Error::OutOfMemory(msg) | webgpu_traits::Error::Internal(msg)) => {
                promise.reject_native(
                    &GPUPipelineError::new(
                        &self.global(),
                        msg.into(),
                        GPUPipelineErrorReason::Internal,
                        CanGc::from_cx(cx),
                    ),
                    CanGc::from_cx(cx),
                )
            },
        }
    }
}
 */
