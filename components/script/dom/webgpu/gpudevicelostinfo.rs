/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::{dom_struct, dom_struct2};
use jstraceable_derive::JSTraceableInSub;
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::reflector::{Reflector, reflect_dom_object};

#[dom_struct2]
pub(crate) struct GPUDeviceLostInfo {
    reflector_: Reflector,
    message: DOMString,
    reason: GPUDeviceLostReason,
}

impl GPUDeviceLostInfo {
    fn new_inherited(message: DOMString, reason: GPUDeviceLostReason) -> Self {
        Self {
            reflector_: Reflector::new(),
            message,
            reason,
        }
    }

    pub(crate) fn new(
        global: &GlobalScope,
        message: DOMString,
        reason: GPUDeviceLostReason,
        can_gc: CanGc,
    ) -> DomRoot<Self> {
        reflect_dom_object(
            Box::new(GPUDeviceLostInfo::new_inherited(message, reason)),
            global,
            can_gc,
        )
    }
}

impl GPUDeviceLostInfoMethods<crate::DomTypeHolder> for GPUDeviceLostInfo {
    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevicelostinfo-message>
    fn Message(&self) -> DOMString {
        self.message.clone()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpudevicelostinfo-reason>
    fn Reason(&self) -> GPUDeviceLostReason {
        self.reason
    }
}
