/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use jstraceable_derive::JSTraceable;
use log::warn;
use malloc_size_of_derive::MallocSizeOf;
use webgpu_traits::{WebGPU, WebGPUDevice, WebGPURequest};
use wgpu_core::id::PipelineLayoutId;

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
