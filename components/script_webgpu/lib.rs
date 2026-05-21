#![allow(unused)]
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
#![cfg_attr(crown, feature(register_tool))]
// Register the linter `crown`, which is the Servo-specific linter for the script crate.
#![cfg_attr(crown, register_tool(crown))]

pub mod gpu;
pub mod gpuadapter;
pub mod gpuadapterinfo;
pub mod gpubindgroup;
pub mod gpubindgrouplayout;
pub mod gpubuffer;
pub mod gpubufferusage;
pub mod gpucanvascontext;
pub mod gpucolorwrite;
pub mod gpucommandbuffer;
pub mod gpucommandencoder;
pub mod gpucompilationinfo;
pub mod gpucompilationmessage;
pub mod gpucomputepassencoder;
pub mod gpucomputepipeline;
pub mod gpuconvert;
pub mod gpudevice;
pub mod gpudevicelostinfo;
pub mod gpuerror;
pub mod gpuinternalerror;
pub mod gpumapmode;
pub mod gpuoutofmemoryerror;
pub mod gpupipelineerror;
pub mod gpupipelinelayout;
pub mod gpuqueryset;
pub mod gpuqueue;
pub mod gpurenderbundle;
pub mod gpurenderbundleencoder;
pub mod gpurenderpassencoder;
pub mod gpurenderpipeline;
pub mod gpusampler;
pub mod gpushadermodule;
pub mod gpushaderstage;
pub mod gpusupportedfeatures;
pub mod gpusupportedlimits;
pub mod gputexture;
pub mod gputextureusage;
pub mod gputextureview;
pub mod gpuuncapturederrorevent;
pub mod gpuvalidationerror;
pub mod identityhub;
pub mod wgsllanguagefeatures;

use std::cell::UnsafeCell;
use std::ptr;

use dom_struct::dom_struct;
pub(crate) use js::gc::Traceable as JSTraceable;
use jstraceable_derive::JSTraceable;
use malloc_size_of::{MallocSizeOf, MallocSizeOfOps};
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::codegen::InheritTypes::GPUErrorTypeId;
use script_bindings::codegen::PrototypeList;
use script_bindings::conversions::{DerivedFrom, IDLInterface};
use script_bindings::inheritance::Castable;
pub(crate) use script_bindings::inheritance::HasParent;
pub(crate) use script_bindings::reflector::{DomObject, MutDomObject, Reflector};
use script_bindings::root::{Dom, DomRoot};
pub(crate) use script_bindings::trace::CustomTraceable;
use script_bindings::utils::DOMClass;
use script_bindings::weakref::WeakReferenceable;
use script_bindings::{DomTypes, script_runtime};

use crate::gpu::GPU;
use crate::gpuadapter::GPUAdapter;
use crate::gpubindgroup::GPUBindGroup;
use crate::gpubindgrouplayout::GPUBindGroupLayout;
use crate::gpubuffer::GPUBuffer;
use crate::gpucolorwrite::GPUColorWrite;
use crate::gpucommandbuffer::GPUCommandBuffer;
use crate::gpucommandencoder::GPUCommandEncoder;
use crate::gpucompilationinfo::GPUCompilationInfo;
use crate::gpucompilationmessage::GPUCompilationMessage;
use crate::gpucomputepassencoder::GPUComputePassEncoder;
use crate::gpucomputepipeline::GPUComputePipeline;
use crate::gpudevice::GPUDevice;
use crate::gpudevicelostinfo::GPUDeviceLostInfo;
use crate::gpuerror::GPUError;
use crate::gpuinternalerror::GPUInternalError;
use crate::gpuoutofmemoryerror::GPUOutOfMemoryError;
use crate::gpupipelineerror::GPUPipelineError;
use crate::gpupipelinelayout::GPUPipelineLayout;
use crate::gpuqueryset::GPUQuerySet;
use crate::gpuqueue::GPUQueue;
use crate::gpurenderbundle::GPURenderBundle;
use crate::gpurenderbundleencoder::GPURenderBundleEncoder;
use crate::gpurenderpassencoder::GPURenderPassEncoder;
use crate::gpurenderpipeline::GPURenderPipeline;
use crate::gpusampler::GPUSampler;
use crate::gpushadermodule::GPUShaderModule;
use crate::gpusupportedfeatures::GPUSupportedFeatures;
use crate::gpusupportedlimits::GPUSupportedLimits;
use crate::gputexture::GPUTexture;
use crate::gputextureusage::GPUTextureUsage;
use crate::gputextureview::GPUTextureView;
use crate::gpuuncapturederrorevent::GPUUncapturedErrorEvent;
use crate::gpuvalidationerror::GPUValidationError;
use crate::wgsllanguagefeatures::WGSLLanguageFeatures;

// A holder that provides interior mutability for GC-managed values such as
/// `Dom<T>`, with nullability represented by an enclosing Option wrapper.
/// Essentially a `Cell<Option<Dom<T>>>`, but safer.
///
/// This should only be used as a field in other DOM objects; see warning
/// on `Dom<T>`.
#[cfg_attr(crown, crown::unrooted_must_root_lint::must_root)]
#[derive(JSTraceable)]
pub(crate) struct MutNullableDom<T: DomObject> {
    ptr: UnsafeCell<Option<Dom<T>>>,
}

impl<T: DomObject> MutNullableDom<T> {
    /// Create a new `MutNullableDom`.
    pub(crate) fn new(initial: Option<&T>) -> MutNullableDom<T> {
        //assert_in_script();
        MutNullableDom {
            ptr: UnsafeCell::new(initial.map(Dom::from_ref)),
        }
    }

    /// Retrieve a copy of the current inner value. If it is `None`, it is
    /// initialized with the result of `cb` first.
    pub(crate) fn or_init<F>(&self, cb: F) -> DomRoot<T>
    where
        F: FnOnce() -> DomRoot<T>,
    {
        //assert_in_script();
        match self.get() {
            Some(inner) => inner,
            None => {
                let inner = cb();
                self.set(Some(&inner));
                inner
            },
        }
    }

    /// Get a rooted value out of this object
    pub(crate) fn get(&self) -> Option<DomRoot<T>> {
        //assert_in_script();
        unsafe { ptr::read(self.ptr.get()).map(|o| DomRoot::from_ref(&*o)) }
    }

    /// Set this `MutNullableDom` to the given value.
    pub(crate) fn set(&self, val: Option<&T>) {
        //assert_in_script();
        unsafe {
            *self.ptr.get() = val.map(|p| Dom::from_ref(p));
        }
    }

    /// Gets the current value out of this object and sets it to `None`.
    pub(crate) fn take(&self) -> Option<DomRoot<T>> {
        let value = self.get();
        self.set(None);
        value
    }

    /// Sets the current value of this [`MutNullableDom`] to `None`.
    pub(crate) fn clear(&self) {
        self.set(None)
    }

    /// Runs the given callback on the object if it's not null.
    pub(crate) fn if_is_some<F, R>(&self, cb: F) -> Option<&R>
    where
        F: FnOnce(&T) -> &R,
    {
        unsafe {
            if let Some(ref value) = *self.ptr.get() {
                Some(cb(value))
            } else {
                None
            }
        }
    }
}

impl<T: DomObject> PartialEq for MutNullableDom<T> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { *self.ptr.get() == *other.ptr.get() }
    }
}

impl<T: DomObject> PartialEq<Option<&T>> for MutNullableDom<T> {
    fn eq(&self, other: &Option<&T>) -> bool {
        unsafe { *self.ptr.get() == other.map(Dom::from_ref) }
    }
}

impl<T: DomObject> Default for MutNullableDom<T> {
    fn default() -> MutNullableDom<T> {
        //assert_in_script();
        MutNullableDom {
            ptr: UnsafeCell::new(None),
        }
    }
}

impl<T: DomObject> MallocSizeOf for MutNullableDom<T> {
    fn size_of(&self, _ops: &mut MallocSizeOfOps) -> usize {
        // See comment on MallocSizeOf for Dom<T>.
        0
    }
}

#[derive(JSTraceable, MallocSizeOf)]
pub(crate) struct DataBlock(());

impl DataBlock {
    fn new_zeroed(size: usize) -> DataBlock {
        DataBlock(())
    }
}

pub(crate) trait Convert<T> {
    fn convert(self) -> T;
}

pub(crate) trait TryConvert<T> {
    type Error;

    fn try_convert(self) -> Result<T, Self::Error>;
}

pub(crate) struct GPUColor(());

#[derive(Default, MallocSizeOf)]
pub(crate) struct PromiseStub(());

unsafe impl JSTraceable for PromiseStub {
    unsafe fn trace(&self, trc: *mut js::jsapi::JSTracer) {}
}

impl WeakReferenceable for GPUDevice {}

impl GPUError {
    #[allow(dead_code)]
    pub(crate) fn type_id(&self) -> &'static GPUErrorTypeId {
        unsafe {
            &script_bindings::conversions::get_dom_class(self.reflector().get_jsobject().get())
                .unwrap()
                .type_id
                .gpuerror
        }
    }
}

/*
impl Castable for GPUDevice {}
impl DerivedFrom<EventTarget> for GPUDevice {}

impl Castable for GPUError {}
impl DerivedFrom<GPUError> for GPUError {}

impl Castable for GPUInternalError {}
impl DerivedFrom<GPUError> for GPUInternalError {}

impl Castable for GPUOutOfMemoryError {}
impl DerivedFrom<GPUError> for GPUOutOfMemoryError {}

impl Castable for GPUPipelineError {}
impl DerivedFrom<DOMException> for GPUPipelineError {}

impl Castable for GPUUncapturedErrorEvent {}
impl DerivedFrom<Event> for GPUUncapturedErrorEvent {}

impl Castable for GPUValidationError {}
impl DerivedFrom<GPUError> for GPUValidationError {}
 */

/////////////////////////////////////// IDL INTERFACE
impl IDLInterface for GPU {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPU_Binding::Class
                .get()
                .dom_class
        })
    }
    const PROTO_FIRST: u16 = 331;
    const PROTO_LAST: u16 = 331;
}

impl IDLInterface for GPUAdapter {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUAdapter_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 332;
    const PROTO_LAST: u16 = 332;
}

impl IDLInterface for GPUBindGroup {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUBindGroup_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 334;
    const PROTO_LAST: u16 = 334;
}

impl IDLInterface for GPUBindGroupLayout {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUBindGroupLayout_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 335;
    const PROTO_LAST: u16 = 335;
}

impl IDLInterface for GPUBuffer {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUBuffer_Binding::Class
                .get()
                .dom_class
        })
    }
    const PROTO_FIRST: u16 = 336;
    const PROTO_LAST: u16 = 336;
}

impl IDLInterface for GPUColorWrite {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUColorWrite_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 338;
    const PROTO_LAST: u16 = 338;
}

impl IDLInterface for GPUCommandBuffer {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUCommandBuffer_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 339;
    const PROTO_LAST: u16 = 339;
}

impl IDLInterface for GPUCommandEncoder {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUCommandEncoder_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 340;
    const PROTO_LAST: u16 = 340;
}

impl IDLInterface for GPUCompilationInfo {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUCompilationInfo_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 341;
    const PROTO_LAST: u16 = 341;
}

impl IDLInterface for GPUCompilationMessage {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUCompilationMessage_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 342;
    const PROTO_LAST: u16 = 342;
}

impl IDLInterface for GPUComputePassEncoder {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUComputePassEncoder_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 343;
    const PROTO_LAST: u16 = 343;
}

impl IDLInterface for GPUComputePipeline {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUComputePipeline_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 344;
    const PROTO_LAST: u16 = 344;
}

impl IDLInterface for GPUDevice {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUDevice_Binding::Class
                .get()
                .dom_class
        })
    }
    const PROTO_FIRST: u16 = 175;
    const PROTO_LAST: u16 = 175;
}

impl IDLInterface for GPUDeviceLostInfo {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUDeviceLostInfo_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 345;
    const PROTO_LAST: u16 = 345;
}

impl IDLInterface for GPUError {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        class.interface_chain[0] == PrototypeList::ID::GPUError
    }
    const PROTO_FIRST: u16 = 346;
    const PROTO_LAST: u16 = 349;
}

impl IDLInterface for GPUInternalError {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUInternalError_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 347;
    const PROTO_LAST: u16 = 347;
}

impl IDLInterface for GPUOutOfMemoryError {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUOutOfMemoryError_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 348;
    const PROTO_LAST: u16 = 348;
}

impl IDLInterface for GPUPipelineError {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUPipelineError_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 55;
    const PROTO_LAST: u16 = 55;
}

impl IDLInterface for GPUPipelineLayout {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUPipelineLayout_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 350;
    const PROTO_LAST: u16 = 350;
}

impl IDLInterface for GPUQuerySet {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUQuerySet_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 351;
    const PROTO_LAST: u16 = 351;
}

impl IDLInterface for GPUQueue {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUQueue_Binding::Class
                .get()
                .dom_class
        })
    }
    const PROTO_FIRST: u16 = 352;
    const PROTO_LAST: u16 = 352;
}

impl IDLInterface for GPURenderBundle {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPURenderBundle_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 353;
    const PROTO_LAST: u16 = 353;
}

impl IDLInterface for GPURenderBundleEncoder {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPURenderBundleEncoder_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 354;
    const PROTO_LAST: u16 = 354;
}
impl IDLInterface for GPURenderPassEncoder {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPURenderPassEncoder_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 355;
    const PROTO_LAST: u16 = 355;
}

impl IDLInterface for GPURenderPipeline {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPURenderPipeline_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 356;
    const PROTO_LAST: u16 = 356;
}

impl IDLInterface for GPUSampler {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUSampler_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 357;
    const PROTO_LAST: u16 = 357;
}

impl IDLInterface for GPUShaderModule {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUShaderModule_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 358;
    const PROTO_LAST: u16 = 358;
}

impl IDLInterface for GPUSupportedFeatures {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUSupportedFeatures_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 360;
    const PROTO_LAST: u16 = 360;
}

impl IDLInterface for GPUSupportedLimits {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUSupportedLimits_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 362;
    const PROTO_LAST: u16 = 362;
}

impl IDLInterface for GPUTexture {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUTexture_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 363;
    const PROTO_LAST: u16 = 363;
}

impl IDLInterface for GPUTextureUsage {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUTextureUsage_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 364;
    const PROTO_LAST: u16 = 364;
}

impl IDLInterface for GPUTextureView {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUTextureView_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 365;
    const PROTO_LAST: u16 = 365;
}

impl IDLInterface for GPUUncapturedErrorEvent {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUUncapturedErrorEvent_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 105;
    const PROTO_LAST: u16 = 105;
}

impl IDLInterface for GPUValidationError {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUValidationError_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 349;
    const PROTO_LAST: u16 = 349;
}

impl IDLInterface for WGSLLanguageFeatures {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::WGSLLanguageFeatures_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 510;
    const PROTO_LAST: u16 = 510;
}

pub(crate) mod dom {
    pub(crate) mod bindings {
        pub(crate) mod codegen {
            pub mod GenericBindings {
                pub(crate) use script_bindings::codegen::GenericBindings::*;
            }
        }
    }
}
