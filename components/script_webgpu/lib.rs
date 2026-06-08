/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub mod datablock;
pub mod gpuadapterinfo;
pub mod gpubufferusage;
pub mod gpucolorwrite;
pub mod gpucommandbuffer;
pub mod gpucompilationinfo;
pub mod gpucompilationmessage;
pub mod gpuconvert;
pub mod gpudevice;
pub mod gpudevicelostinfo;
pub mod gpuerror;
pub mod gpuinternalerror;
pub mod gpumapmode;
pub mod gpuoutofmemoryerror;
#[expect(dead_code)]
pub mod gpupipelinelayout;
pub mod gpuqueryset;
pub mod gpurenderbundle;
pub mod gpusampler;
pub mod gpushaderstage;
pub mod gpusupportedfeatures;
pub mod gpusupportedlimits;
pub mod gputextureusage;
pub mod gpuvalidationerror;
#[expect(dead_code)]
pub mod identityhub;
pub mod traits;
pub mod wgsllanguagefeatures;

pub(crate) mod dom {
    pub(crate) mod types {}
    pub(crate) mod bindings {
        pub(crate) use script_bindings::*;
    }
}

/// Generated JS-Rust bindings.
#[allow(missing_docs, non_snake_case)]
pub(crate) mod codegen {
    pub mod IDLInterface {

        //include!(concat!(env!("OUT_DIR"), "/GPUIDLInterfaceBindings.rs"));
    }
    pub(crate) mod ConcreteInheritTypes {
        pub(crate) use crate::gpuerror::GPUError;
        pub(crate) use crate::gpuinternalerror::GPUInternalError;
        pub(crate) use crate::gpuoutofmemoryerror::GPUOutOfMemoryError;
        pub(crate) use crate::gpuvalidationerror::GPUValidationError;
        include!(concat!(env!("OUT_DIR"), "/GPUConcreteInheritTypes.rs"));
    }
}

use std::ptr;

pub(crate) use js::gc::Traceable as JSTraceable;
use script_bindings::codegen::PrototypeList;
use script_bindings::conversions::IDLInterface;
pub(crate) use script_bindings::inheritance::HasParent;
pub(crate) use script_bindings::reflector::{DomObject, MutDomObject, Reflector};
pub(crate) use script_bindings::trace::CustomTraceable;
use script_bindings::utils::DOMClass;

use crate::gpuerror::GPUError;
use crate::gpuinternalerror::GPUInternalError;
use crate::gpuoutofmemoryerror::GPUOutOfMemoryError;
use crate::gpuvalidationerror::GPUValidationError;

impl IDLInterface for GPUOutOfMemoryError {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUOutOfMemoryError_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 350;
    const PROTO_LAST: u16 = 350;
}
impl IDLInterface for GPUValidationError {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUValidationError_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 351;
    const PROTO_LAST: u16 = 351;
}
impl IDLInterface for GPUInternalError {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        ptr::eq(class, unsafe {
            &crate::dom::bindings::codegen::GenericBindings::WebGPUBinding::GPUInternalError_Binding::Class.get().dom_class
        })
    }
    const PROTO_FIRST: u16 = 349;
    const PROTO_LAST: u16 = 349;
}

impl IDLInterface for GPUError {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        class.interface_chain[0] == PrototypeList::ID::GPUError
    }
    const PROTO_FIRST: u16 = 348;
    const PROTO_LAST: u16 = 351;
}
