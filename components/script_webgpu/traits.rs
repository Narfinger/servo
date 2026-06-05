use std::rc::Rc;
use std::sync::Arc;

use script_bindings::DomTypes;
use script_bindings::error::Error;
use script_bindings::realms::InRealm;
use script_bindings::script_runtime::{CanGc, JSContext};
use servo_base::id::PipelineId;
use webgpu_traits::{WebGPU, WebGPUDevice};
use wgpu_core::id::RenderPipelineId;

use crate::identityhub::IdentityHub;

pub trait WebGPUPromise {
    fn new_in_current_realm(_comp: InRealm, can_gc: CanGc) -> Rc<Self>;
    fn reject_error(&self, error: Error, can_gc: CanGc);
}

pub trait WebGPUGlobalTrait<D: DomTypes> {
    fn global(&self) -> D::GlobalScope;
    fn wgpu_id_hub(&self) -> Arc<IdentityHub>;
    fn pipeline_id(&self) -> PipelineId;
    fn get_cx() -> JSContext;
    fn script_to_constellation_chan(&self) -> ();
}

pub trait GPUDeviceTrait {
    fn channel(&self) -> WebGPU;
    fn id(&self) -> WebGPUDevice;
    fn dispatch_error(&self, error: webgpu_traits::Error);
}

pub trait WebGPURenderPipelineTrait {
    fn id(&self) -> RenderPipelineId;
}
