/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::cell::Ref;
use std::rc::Rc;
use std::sync::Arc;

use euclid::default::Size2D;
use js::context::NoGC;
use pixels::Snapshot;
use script_bindings::DomTypes;
use script_bindings::callback::{CallbackContainer, RootedCallback};
use script_bindings::error::{Error, Fallible};
/*
use script_webgpu::traits::{
    EventTargetTrait, HtmlCanvasElementTrait, HtmlImageElementTrait, ImageBitmapTrait,
    ImageDataTrait, OffscreenCanvasTrait, OriginIsCleanTrait, WebGPUGlobalTrait,
    WebGPUHTMLVideoTrait, WebGPUPromiseCallbackTrait,
};
 */
use script_bindings::reflector::{DomGlobalGeneric, DomObject};
use serde::Serialize;
use serde::de::DeserializeOwned;
use servo_base::generic_channel::GenericCallback;
use servo_url::MutableOrigin;

use crate::dom::GlobalScope;
use crate::dom::bindings::reflector::DomGlobal;
use crate::dom::promise::RootedPromise;
use crate::dom::types::{
    EventTarget, HTMLCanvasElement, HTMLImageElement, HTMLVideoElement, ImageBitmap, ImageData,
    OffscreenCanvas,
};
use crate::routed_promise::{RoutedPromiseListener, callback_promise};
use crate::tasks::task::TaskOnce;

pub(crate) mod gpu {
    use script_bindings::reflector::DomObject;
    use stylo_malloc_size_of::MallocSizeOf;

    #[expect(clippy::upper_case_acronyms)]
    pub(crate) type GPU = Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPU = script_webgpu::gpu::GPU<crate::DomTypeHolder>;
}
pub(crate) mod gpuadapter {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUAdapter =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUAdapter = script_webgpu::gpuadapter::GPUAdapter<crate::DomTypeHolder>;
}
pub(crate) mod gpuadapterinfo {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUAdapterInfo =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUAdapterInfo =
    //    script_webgpu::gpuadapterinfo::GPUAdapterInfo<crate::DomTypeHolder>;
}
pub(crate) mod gpubindgroup {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUBindGroup =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUBindGroup = script_webgpu::gpubindgroup::GPUBindGroup<crate::DomTypeHolder>;
}
pub(crate) mod gpubindgrouplayout {
    pub(crate) type GPUBindGroupLayout =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUBindGroupLayout =
    //    script_webgpu::gpubindgrouplayout::GPUBindGroupLayout<crate::DomTypeHolder>;
}
pub(crate) mod gpubuffer {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUBuffer =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUBuffer = script_webgpu::gpubuffer::GPUBuffer<crate::DomTypeHolder>;
}
pub(crate) mod gpubufferusage {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUBufferUsage =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUBufferUsage =
    //    script_webgpu::gpubufferusage::GPUBufferUsage<crate::DomTypeHolder>;
}
//pub(crate) mod gpucanvascontext;
pub(crate) mod gpucolorwrite {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUColorWrite =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUColorWrite =
    //    script_webgpu::gpucolorwrite::GPUColorWrite<crate::DomTypeHolder>;
}
pub(crate) mod gpucommandbuffer {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUCommandBuffer =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUCommandBuffer =
    //    script_webgpu::gpucommandbuffer::GPUCommandBuffer<crate::DomTypeHolder>;
}
pub(crate) mod gpucommandencoder {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUCommandEncoder =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUCommandEncoder =
    //    script_webgpu::gpucommandencoder::GPUCommandEncoder<crate::DomTypeHolder>;
}
pub(crate) mod gpucompilationinfo {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUCompilationInfo =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUCompilationInfo =
    //    script_webgpu::gpucompilationinfo::GPUCompilationInfo<crate::DomTypeHolder>;
}
pub(crate) mod gpucompilationmessage {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUCompilationMessage =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUCompilationMessage =
    //    script_webgpu::gpucompilationmessage::GPUCompilationMessage<crate::DomTypeHolder>;
}
pub(crate) mod gpucomputepassencoder {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUComputePassEncoder =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUComputePassEncoder =
    //   script_webgpu::gpucomputepassencoder::GPUComputePassEncoder<crate::DomTypeHolder>;
}
pub(crate) mod gpucomputepipeline {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUComputePipeline =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUComputePipeline =
    //   script_webgpu::gpucomputepipeline::GPUComputePipeline<crate::DomTypeHolder>;
}
pub(crate) mod gpudevice {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUDevice =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUDevice = script_webgpu::gpudevice::GPUDevice<crate::DomTypeHolder>;
}
pub(crate) mod gpudevicelostinfo {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUDeviceLostInfo =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUDeviceLostInfo =
    //    script_webgpu::gpudevicelostinfo::GPUDeviceLostInfo<crate::DomTypeHolder>;
}
pub(crate) mod gpuerror {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUError =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUError = script_webgpu::gpuerror::GPUError<crate::DomTypeHolder>;
}
pub(crate) mod gpuexternaltexture {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUExternalTexture =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    pub(crate) type PlanarTexture =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUExternalTexture =
    //    script_webgpu::gpuexternaltexture::GPUExternalTexture<crate::DomTypeHolder>;
    //pub(crate) type PlanarTexture =
    //   script_webgpu::gpuexternaltexture::PlanarTexture<crate::DomTypeHolder>;
}
pub(crate) mod gpuinternalerror {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUInternalError =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUInternalError =
    //   script_webgpu::gpuinternalerror::GPUInternalError<crate::DomTypeHolder>;
}
pub(crate) mod gpumapmode {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUMapMode =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUMapMode = script_webgpu::gpumapmode::GPUMapMode<crate::DomTypeHolder>;
}
pub(crate) mod gpuoutofmemoryerror {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUOutOfmemoryError =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUOutOfMemoryError =
    //   script_webgpu::gpuoutofmemoryerror::GPUOutOfMemoryError<crate::DomTypeHolder>;
}
pub(crate) mod gpupipelineerror {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUPipelineError =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUPipelineError =
    //   script_webgpu::gpupipelineerror::GPUPipelineError<crate::DomTypeHolder>;
}
pub(crate) mod gpupipelinelayout {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUPipelineLayout =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUPipelineLayout =
    //   script_webgpu::gpupipelinelayout::GPUPipelineLayout<crate::DomTypeHolder>;
}
pub(crate) mod gpuqueryset {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUQuerySet =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUQuerySet = script_webgpu::gpuqueryset::GPUQuerySet<crate::DomTypeHolder>;
}
pub(crate) mod gpuqueue {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUQueue =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUQueue = script_webgpu::gpuqueue::GPUQueue<crate::DomTypeHolder>;
}
pub(crate) mod gpurenderbundle {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPURenderBundle =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPURenderBundle =
    //   script_webgpu::gpurenderbundle::GPURenderBundle<crate::DomTypeHolder>;
}
pub(crate) mod gpurenderbundleencoder {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPURenderBundleEncoder =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPURenderBundleEncoder =
    //   script_webgpu::gpurenderbundleencoder::GPURenderBundleEncoder<crate::DomTypeHolder>;
}
pub(crate) mod gpurenderpassencoder {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPURenderPassEncoder =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPURenderPassEncoder =
    //    script_webgpu::gpurenderpassencoder::GPURenderPassEncoder<crate::DomTypeHolder>;
}
pub(crate) mod gpurenderpipeline {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPURenderPipeline =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPURenderPipeline =
    //   script_webgpu::gpurenderpipeline::GPURenderPipeline<crate::DomTypeHolder>;
}
pub(crate) mod gpusampler {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUSampler =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUSampler = script_webgpu::gpusampler::GPUSampler<crate::DomTypeHolder>;
}
pub(crate) mod gpushadermodule {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUShaderModule =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUShaderModule =
    //   script_webgpu::gpushadermodule::GPUShaderModule<crate::DomTypeHolder>;
}
pub(crate) mod gpushaderstage {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUShaderStage =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUShaderStage =
    //   script_webgpu::gpushaderstage::GPUShaderStage<crate::DomTypeHolder>;
}
pub(crate) mod gpusupportedfeatures {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUSupportedFeatures =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUSupportedFeatures =
    //   script_webgpu::gpusupportedfeatures::GPUSupportedFeatures<crate::DomTypeHolder>;
}
pub(crate) mod gpusupportedlimits {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUSupportedLimits =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUSupportedLimits =
    //   script_webgpu::gpusupportedlimits::GPUSupportedLimits<crate::DomTypeHolder>;
}
pub(crate) mod gputexture {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUTexture =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUTexture = script_webgpu::gputexture::GPUTexture<crate::DomTypeHolder>;
}
pub(crate) mod gputextureusage {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUTextureUsage =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUTextureUsage =
    //   script_webgpu::gputextureusage::GPUTextureUsage<crate::DomTypeHolder>;
}
pub(crate) mod gputextureview {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUTextureView =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUTextureView =
    //   script_webgpu::gputextureview::GPUTextureView<crate::DomTypeHolder>;
}
pub(crate) mod gpuuncapturederrorevent {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUUncapturedErrorEvent =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUUncapturedErrorEvent =
    //   script_webgpu::gpuuncapturederrorevent::GPUUncapturedErrorEvent<crate::DomTypeHolder>;
}
pub(crate) mod gpuvalidationerror {
    use script_bindings::reflector::DomObject;

    pub(crate) type GPUValidationError =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type GPUValidationError =
    //   script_webgpu::gpuvalidationerror::GPUValidationError<crate::DomTypeHolder>;
}
pub(crate) mod identityhub {
    use script_bindings::reflector::DomObject;

    pub(crate) type IdentityHub =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type IdentityHub = script_webgpu::identityhub::IdentityHub;
}
pub(crate) mod wgsllanguagefeatures {
    use script_bindings::reflector::DomObject;

    pub(crate) type WGSLLanguageFeatures =
        Box<dyn DomObject<ReflectorType = ()> + js::rust::Trace + MallocSizeOf>;
    //pub(crate) type WGSLLanguageFeatures =
    //   script_webgpu::wgsllanguagefeatures::WGSLLanguageFeatures<crate::DomTypeHolder>;
}

/*
impl<S, T> WebGPUPromiseCallbackTrait<crate::DomTypeHolder, S, T> for RootedPromise
where
    S: DomObject
        + DomGlobalGeneric<crate::DomTypeHolder>
        + RoutedPromiseListener<crate::DomTypeHolder, T>,
    T: Serialize + 'static + Send + DeserializeOwned,
{
    fn callback_promise_dom_manipulation_task_source(&self, d: &S) -> GenericCallback<T> {
        let task_manager = <S as DomGlobal>::global(d).task_manager();
        callback_promise(self, d, task_manager.dom_manipulation_task_source())
    }
}

impl WebGPUGlobalTrait<crate::DomTypeHolder> for GlobalScope {
    fn global_wgpu_id_hub(&self) -> Arc<script_webgpu::identityhub::IdentityHub> {
        self.wgpu_id_hub()
    }

    fn queue_webgpu_task_source(&self, task: impl TaskOnce + 'static) {
        self.task_manager().webgpu_task_source().queue(task);
    }

    fn add_webgpu_device(
        &self,
        device: &script_webgpu::gpudevice::GPUDevice<crate::DomTypeHolder>,
    ) {
        self.add_gpu_device(device)
    }
}

impl WebGPUHTMLVideoTrait<crate::DomTypeHolder> for HTMLVideoElement {
    fn planar_video_for_webgpu(
        &self,
        device: &script_webgpu::gpudevice::GPUDevice<crate::DomTypeHolder>,
    ) -> Fallible<(
        Size2D<u32>,
        Option<Rc<script_webgpu::gpuexternaltexture::PlanarTexture<crate::DomTypeHolder>>>,
    )> {
        HTMLVideoElement::planar_video_for_webgpu(self, device)
    }

    fn is_usable(&self) -> bool {
        HTMLVideoElement::is_usable(self)
    }

    fn get_current_frame_data(&self) -> Option<pixels::Snapshot> {
        HTMLVideoElement::get_current_frame_data(self)
    }
}

impl EventTargetTrait<crate::DomTypeHolder> for EventTarget {
    fn new_inherited() -> <crate::DomTypeHolder as DomTypes>::EventTarget {
        EventTarget::new_inherited()
    }

    fn get_event_handler_common<T: CallbackContainer<crate::DomTypeHolder>>(
        &self,
        cx: &mut js::context::JSContext,
        ty: &str,
    ) -> Option<RootedCallback<T>> {
        EventTarget::get_event_handler_common(self, cx, ty)
    }

    fn set_event_handler_common<T: CallbackContainer<crate::DomTypeHolder>>(
        &self,
        cx: &mut js::context::JSContext,
        ty: &str,
        listener: Option<RootedCallback<T>>,
    ) {
        EventTarget::set_event_handler_common(self, cx, ty, listener);
    }
}

impl OriginIsCleanTrait for HTMLVideoElement {
    fn origin_is_clean(&self) -> bool {
        HTMLVideoElement::origin_is_clean(self)
    }
}

impl OriginIsCleanTrait for ImageBitmap {
    fn origin_is_clean(&self) -> bool {
        ImageBitmap::origin_is_clean(self)
    }
}

impl ImageBitmapTrait for ImageBitmap {
    fn bitmap_data(&self) -> Ref<'_, Option<Snapshot>> {
        ImageBitmap::bitmap_data(self)
    }
}

impl ImageDataTrait for ImageData {
    fn is_detached(&self, cx: &mut js::context::JSContext) -> bool {
        ImageData::is_detached(self, cx)
    }
    fn get_snapshot(&self, no_gc: &NoGC) -> Snapshot {
        ImageData::get_snapshot(self, no_gc)
    }
}

impl HtmlImageElementTrait for HTMLImageElement {
    fn is_usable(&self) -> Result<bool, Error> {
        HTMLImageElement::is_usable(self)
    }
    fn get_raster_image_data(&self) -> Option<Snapshot> {
        HTMLImageElement::get_raster_image_data(self)
    }
    fn same_origin(&self, origin: &MutableOrigin) -> bool {
        HTMLImageElement::same_origin(self, origin)
    }
}

impl OriginIsCleanTrait for OffscreenCanvas {
    fn origin_is_clean(&self) -> bool {
        OffscreenCanvas::origin_is_clean(self)
    }
}

impl OffscreenCanvasTrait for OffscreenCanvas {
    fn get_image_data(&self) -> Option<Snapshot> {
        OffscreenCanvas::get_image_data(self)
    }
}

impl OriginIsCleanTrait for HTMLCanvasElement {
    fn origin_is_clean(&self) -> bool {
        HTMLCanvasElement::origin_is_clean(self)
    }
}

impl HtmlCanvasElementTrait for HTMLCanvasElement {
    fn is_valid(&self) -> bool {
        HTMLCanvasElement::is_valid(self)
    }
    fn get_image_data(&self) -> Option<Snapshot> {
        HTMLCanvasElement::get_image_data(self)
    }
}
 */
