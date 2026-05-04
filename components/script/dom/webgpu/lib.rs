/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub(crate) mod gpu;
pub(crate) mod gpuadapter;
pub(crate) mod gpuadapterinfo;
pub(crate) mod gpubindgroup;
pub(crate) mod gpubindgrouplayout;
pub(crate) mod gpubuffer;
pub(crate) mod gpubufferusage;
pub(crate) mod gpucanvascontext;
pub(crate) mod gpucolorwrite;
pub(crate) mod gpucommandbuffer;
pub(crate) mod gpucommandencoder;
pub(crate) mod gpucompilationinfo;
pub(crate) mod gpucompilationmessage;
pub(crate) mod gpucomputepassencoder;
pub(crate) mod gpucomputepipeline;
pub(crate) mod gpuconvert;
pub(crate) mod gpudevice;
pub(crate) mod gpudevicelostinfo;
pub(crate) mod gpuerror;
pub(crate) mod gpuinternalerror;
pub(crate) mod gpumapmode;
pub(crate) mod gpuoutofmemoryerror;
pub(crate) mod gpupipelineerror;
#[expect(dead_code)]
pub(crate) mod gpupipelinelayout;
pub(crate) mod gpuqueryset;
pub(crate) mod gpuqueue;
pub(crate) mod gpurenderbundle;
pub(crate) mod gpurenderbundleencoder;
pub(crate) mod gpurenderpassencoder;
pub(crate) mod gpurenderpipeline;
pub(crate) mod gpusampler;
pub(crate) mod gpushadermodule;
pub(crate) mod gpushaderstage;
pub(crate) mod gpusupportedfeatures;
pub(crate) mod gpusupportedlimits;
pub(crate) mod gputexture;
pub(crate) mod gputextureusage;
pub(crate) mod gputextureview;
pub(crate) mod gpuuncapturederrorevent;
pub(crate) mod gpuvalidationerror;
#[expect(dead_code)]
pub(crate) mod identityhub;
pub(crate) mod wgsllanguagefeatures;

pub struct DomTypeHolder;
impl script_bindings::DomTypes for DomTypeHolder {
    type GPU = crate::gpu::GPU;
    type GPUAdapter = crate::gpuadapter::GPUAdapter;
    type GPUAdapterInfo = crate::gpuadapterinfo::GPUAdapterInfo;
    type GPUBindGroup = crate::gpubindgroup::GPUBindGroup;
    type GPUBindGroupLayout = crate::gpubindgrouplayout::GPUBindGroupLayout;
    type GPUBuffer = crate::gpubuffer::GPUBuffer;
    type GPUBufferUsage = crate::gpubufferusage::GPUBufferUsage;
    type GPUCanvasContext = crate::gpucanvascontext::GPUCanvasContext;
    type GPUColorWrite = crate::gpucolorwrite::GPUColorWrite;
    type GPUCommandBuffer = crate::gpucommandbuffer::GPUCommandBuffer;
    type GPUCommandEncoder = crate::gpucommandencoder::GPUCommandEncoder;
    type GPUCompilationInfo = crate::gpucompilationinfo::GPUCompilationInfo;
    type GPUCompilationMessage = crate::gpucompilationmessage::GPUCompilationMessage;
    type GPUComputePassEncoder = crate::gpucomputepassencoder::GPUComputePassEncoder;
    type GPUComputePipeline = crate::gpucomputepipeline::GPUComputePipeline;
    type GPUDevice = crate::gpudevice::GPUDevice;
    type GPUDeviceLostInfo = crate::gpudevicelostinfo::GPUDeviceLostInfo;
    type GPUError = crate::gpuerror::GPUError;
    type GPUInternalError = crate::gpuinternalerror::GPUInternalError;
    type GPUMapMode = crate::gpumapmode::GPUMapMode;
    type GPUOutOfMemoryError = crate::gpuoutofmemoryerror::GPUOutOfMemoryError;
    type GPUPipelineError = crate::gpupipelineerror::GPUPipelineError;
    type GPUPipelineLayout = crate::gpupipelinelayout::GPUPipelineLayout;
    type GPUQuerySet = crate::gpuqueryset::GPUQuerySet;
    type GPUQueue = crate::gpuqueue::GPUQueue;
    type GPURenderBundle = crate::gpurenderbundle::GPURenderBundle;
    type GPURenderBundleEncoder = crate::gpurenderbundleencoder::GPURenderBundleEncoder;
    type GPURenderPassEncoder = crate::gpurenderpassencoder::GPURenderPassEncoder;
    type GPURenderPipeline = crate::gpurenderpipeline::GPURenderPipeline;
    type GPUSampler = crate::gpusampler::GPUSampler;
    type GPUShaderModule = crate::gpushadermodule::GPUShaderModule;
    type GPUShaderStage = crate::gpushaderstage::GPUShaderStage;
    type GPUSupportedFeatures = crate::gpusupportedfeatures::GPUSupportedFeatures;
    type GPUSupportedLimits = crate::gpusupportedlimits::GPUSupportedLimits;
    type GPUTexture = crate::gputexture::GPUTexture;
    type GPUTextureUsage = crate::gputextureusage::GPUTextureUsage;
    type GPUTextureView = crate::gputextureview::GPUTextureView;
    type GPUUncapturedErrorEvent = crate::gpuuncapturederrorevent::GPUUncapturedErrorEvent;
    type GPUValidationError = crate::gpuvalidationerror::GPUValidationError;
}
