use std::rc::Rc;
use std::sync::Arc;

use script_bindings::DomTypes;
use script_bindings::error::{Error, Fallible};
use script_bindings::realms::InRealm;
use script_bindings::script_runtime::{CanGc, JSContext};
use servo_base::id::PipelineId;
use webgpu_traits::{WebGPU, WebGPUBindGroupLayout, WebGPUBuffer, WebGPUDevice, WebGPUTextureView};
use wgpu_core::id::RenderPipelineId;
use wgpu_types::TextureFormat;

use crate::identityhub::IdentityHub;

pub trait WebGPUPromise {
    fn new_in_current_realm(_comp: InRealm, can_gc: CanGc) -> Rc<Self>;
    fn reject_error(&self, error: Error, can_gc: CanGc);
}

pub trait WebGPUGlobalTrait<D: DomTypes> {
    fn get_global(&self) -> &D::GlobalScope;
    fn wgpu_id_hub(&self) -> Arc<IdentityHub>;
    fn pipeline_id(&self) -> PipelineId;
    fn get_cx() -> JSContext;
    fn script_to_constellation_chan(&self) -> ();
}

pub trait GPUDeviceTrait {
    fn channel(&self) -> WebGPU;
    fn id(&self) -> WebGPUDevice;
    fn dispatch_error(&self, error: webgpu_traits::Error);
    //fn validate_texture_format_required_features(&self, &crate::dom::bindings::codegen::Bindings::WebGPUBinding::GPUTextureFormat)->Fallible<TextureFormat>;
}

pub trait WebGPURenderPipelineTrait {
    fn id(&self) -> RenderPipelineId;
}

pub trait GPUTextureViewTrait {
    fn id(&self) -> WebGPUTextureView;
}

pub trait GPUBufferTrait {
    fn id(&self) -> WebGPUBuffer;
}

pub trait GPUTextureTrait {
    fn get_default_view(&self) -> WebGPUTextureView;
}

pub trait GPUBindGroupLayoutTrait {
    fn id(&self) -> WebGPUBindGroupLayout;
}
