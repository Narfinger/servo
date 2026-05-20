/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::marker::PhantomData;

use dom_struct::dom_struct;
use jstraceable_derive::JSTraceable;
use log::warn;
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::DomTypes;
use script_bindings::cell::DomRefCell;
use script_bindings::codegen::GenericBindings::WebGPUBinding::GPUTextureViewMethods;
use script_bindings::reflector::{
    Reflector, reflect_dom_object, reflect_dom_object_test_with_wrap2,
};
use script_bindings::root::{Dom, DomRoot};
use script_bindings::str::USVString;
use webgpu_traits::{WebGPU, WebGPURequest, WebGPUTextureView};

use crate::gputexture::GPUTexture;
use crate::script_runtime::CanGc;

#[derive(JSTraceable, MallocSizeOf)]
struct DroppableGPUTextureView {
    #[ignore_malloc_size_of = "defined in webgpu"]
    #[no_trace]
    channel: WebGPU,
    #[no_trace]
    texture_view: WebGPUTextureView,
}

impl Drop for DroppableGPUTextureView {
    fn drop(&mut self) {
        if let Err(e) = self
            .channel
            .0
            .send(WebGPURequest::DropTextureView(self.texture_view.0))
        {
            warn!(
                "Failed to send DropTextureView ({:?}) ({})",
                self.texture_view.0, e
            );
        }
    }
}

#[dom_struct]
pub(crate) struct GPUTextureView<D: DomTypes> {
    reflector_: Reflector,
    label: DomRefCell<USVString>,
    texture: Dom<GPUTexture<D>>,
    droppable: DroppableGPUTextureView,
    phantom: PhantomData<D>,
}

impl<D: DomTypes> GPUTextureView<D>
where
    D: DomTypes,
    Box<D::GPUTextureView>: From<Box<GPUTextureView<D>>>,
    DomRoot<GPUTextureView<D>>: From<DomRoot<D::GPUTextureView>>,
{
    fn new_inherited(
        channel: WebGPU,
        texture_view: WebGPUTextureView,
        texture: &GPUTexture<D>,
        label: USVString,
    ) -> GPUTextureView<D> {
        Self {
            reflector_: Reflector::new(),
            texture: Dom::from_ref(texture),
            label: DomRefCell::new(label),
            droppable: DroppableGPUTextureView {
                channel,
                texture_view,
            },
            phantom: PhantomData,
        }
    }

    pub(crate) fn new(
        global: &D::GlobalScope,
        channel: WebGPU,
        texture_view: WebGPUTextureView,
        texture: &GPUTexture<D>,
        label: USVString,
        can_gc: CanGc,
    ) -> DomRoot<GPUTextureView<D>> {
        reflect_dom_object_test_with_wrap2::<D, _, _, _>(
            Box::new(GPUTextureView::new_inherited(
                channel,
                texture_view,
                texture,
                label,
            )),
            global,
            can_gc,
            script_bindings::codegen::GenericBindings::WebGPUBinding::GPUTextureViewWrap::<D>,
        )
    }
}

impl<D: DomTypes> GPUTextureView<D> {
    pub(crate) fn id(&self) -> WebGPUTextureView {
        self.droppable.texture_view
    }
}

impl<D: DomTypes> GPUTextureViewMethods<D> for GPUTextureView<D> {
    /// <https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label>
    fn Label(&self) -> USVString {
        self.label.borrow().clone()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label>
    fn SetLabel(&self, value: USVString) {
        *self.label.borrow_mut() = value;
    }
}
