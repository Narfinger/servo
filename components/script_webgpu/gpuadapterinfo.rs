/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::marker::PhantomData;

use dom_struct::dom_struct;
use jstraceable_derive::JSTraceable;
use log::warn;
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::DomTypes;
use script_bindings::codegen::GenericBindings::WebGPUBinding::GPUAdapterInfoMethods;
use script_bindings::reflector::{
    DomObject, Reflector, reflect_dom_object, reflect_dom_object_test_with_wrap,
};
use script_bindings::root::DomRoot;
use script_bindings::str::DOMString;

use crate::gpuadapter::GPUAdapter;
use crate::script_runtime::CanGc;

#[dom_struct]
pub struct GPUAdapterInfo {
    reflector_: Reflector,
    vendor: DOMString,
    architecture: DOMString,
    device: DOMString,
    description: DOMString,
    subgroup_min_size: u32,
    subgroup_max_size: u32,
    is_fallback_adapter: bool,
}

impl GPUAdapterInfo {
    fn new_inherited(
        vendor: DOMString,
        architecture: DOMString,
        device: DOMString,
        description: DOMString,
        subgroup_min_size: u32,
        subgroup_max_size: u32,
        is_fallback_adapter: bool,
    ) -> Self {
        Self {
            reflector_: Reflector::new(),
            vendor,
            architecture,
            device,
            description,
            subgroup_min_size,
            subgroup_max_size,
            is_fallback_adapter,
        }
    }

    #[expect(clippy::too_many_arguments)]
    pub(crate) fn new<D>(
        global: &D::GlobalScope,
        vendor: DOMString,
        architecture: DOMString,
        device: DOMString,
        description: DOMString,
        subgroup_min_size: u32,
        subgroup_max_size: u32,
        is_fallback_adapter: bool,
        can_gc: CanGc,
    ) -> DomRoot<Self>
    where
        D: DomTypes<GPUAdapterInfo = GPUAdapterInfo>,
    {
        reflect_dom_object_test_with_wrap::<D, _, _, _>(
            Box::new(Self::new_inherited(
                vendor,
                architecture,
                device,
                description,
                subgroup_min_size,
                subgroup_max_size,
                is_fallback_adapter,
            )),
            global,
            can_gc,
            script_bindings::codegen::GenericBindings::WebGPUBinding::GPUAdapterInfoWrap::<D>,
        )
    }

    pub(crate) fn clone_from<D>(
        global: &D::GlobalScope,
        info: &GPUAdapterInfo,
        can_gc: CanGc,
    ) -> DomRoot<Self>
    where
        D: DomTypes<GPUAdapterInfo = GPUAdapterInfo>,
    {
        Self::new::<D>(
            global,
            info.vendor.clone(),
            info.architecture.clone(),
            info.device.clone(),
            info.description.clone(),
            info.subgroup_min_size,
            info.subgroup_max_size,
            info.is_fallback_adapter,
            can_gc,
        )
    }
}

impl<D: DomTypes> GPUAdapterInfoMethods<D> for GPUAdapterInfo {
    /// <https://gpuweb.github.io/gpuweb/#dom-gpuadapterinfo-vendor>
    fn Vendor(&self) -> DOMString {
        self.vendor.clone()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpuadapterinfo-architecture>
    fn Architecture(&self) -> DOMString {
        self.architecture.clone()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpuadapterinfo-device>
    fn Device(&self) -> DOMString {
        self.device.clone()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpuadapterinfo-description>
    fn Description(&self) -> DOMString {
        self.description.clone()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpuadapterinfo-subgroupminsize>
    fn SubgroupMinSize(&self) -> u32 {
        self.subgroup_min_size
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpuadapterinfo-subgroupmaxsize>
    fn SubgroupMaxSize(&self) -> u32 {
        self.subgroup_max_size
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpuadapterinfo-isfallbackadapter>
    fn IsFallbackAdapter(&self) -> bool {
        self.is_fallback_adapter
    }
}

impl From<GPUAdapter> for GPUAdapterInfo {
    fn from(value: GPUAdapter) -> Self {
        todo!()
    }
}
