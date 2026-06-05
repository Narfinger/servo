use std::rc::Rc;
use std::sync::Arc;

use script_bindings::error::Error;
use script_bindings::realms::InRealm;
use script_bindings::script_runtime::CanGc;
use servo_base::id::PipelineId;

use crate::identityhub::IdentityHub;

pub trait WGPUPromise {
    fn new_in_current_realm(_comp: InRealm, can_gc: CanGc) -> Rc<Self>;
    fn reject_error(&self, error: Error, can_gc: CanGc);
}

pub trait WGPUGobal {
    fn wgpu_id_hub(&self) -> Arc<IdentityHub>;
    fn pipeline_id(&self) -> PipelineId;
}
