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
        pub(crate) mod import {
            pub(crate) mod module {
                pub(crate) use std::ptr;

                pub(crate) use script_bindings::codegen::PrototypeList;
                pub(crate) use script_bindings::conversions::IDLInterface;
                pub(crate) use script_bindings::utils::DOMClass;

                pub(crate) use crate::gpuadapterinfo::GPUAdapterInfo;
                pub(crate) use crate::gpubufferusage::GPUBufferUsage;
                pub(crate) use crate::gpucolorwrite::GPUColorWrite;
                pub(crate) use crate::gpucommandbuffer::GPUCommandBuffer;
                pub(crate) use crate::gpucompilationinfo::GPUCompilationInfo;
                pub(crate) use crate::gpudevicelostinfo::GPUDeviceLostInfo;
                pub(crate) use crate::gpuinternalerror::GPUInternalError;
                pub(crate) use crate::gpumapmode::GPUMapMode;
                pub(crate) use crate::gpuoutofmemoryerror::GPUOutOfMemoryError;
                pub(crate) use crate::gpupipelinelayout::GPUPipelineLayout;
                pub(crate) use crate::gpuqueryset::GPUQuerySet;
                pub(crate) use crate::gpurenderbundle::GPURenderBundle;
                pub(crate) use crate::gpusampler::GPUSampler;
                pub(crate) use crate::gpushaderstage::GPUShaderStage;
                pub(crate) use crate::gpusupportedfeatures::GPUSupportedFeatures;
                pub(crate) use crate::gpusupportedlimits::GPUSupportedLimits;
                pub(crate) use crate::gputextureusage::GPUTextureUsage;
                pub(crate) use crate::gpuvalidationerror::GPUValidationError;
                pub(crate) use crate::identityhub::IdentityHub;
                pub(crate) use crate::wgsllanguagefeatures::WGSLLanguageFeatures;
            }
        }
    }
}
/// Generated JS-Rust bindings.
#[allow(missing_docs, non_snake_case)]
pub(crate) mod codegen {
    pub mod IDLInterface {
        include!(concat!(
            env!("OUT_DIR"),
            "/GIDLInterfaceBindings/WebGPUBinding.rs"
        ));
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
use script_bindings::inheritance::Castable;
pub(crate) use script_bindings::inheritance::HasParent;
pub(crate) use script_bindings::reflector::{DomObject, MutDomObject, Reflector};
pub(crate) use script_bindings::trace::CustomTraceable;
use script_bindings::utils::DOMClass;

use crate::gpuerror::GPUError;
use crate::gpuinternalerror::GPUInternalError;
use crate::gpuoutofmemoryerror::GPUOutOfMemoryError;
use crate::gpuvalidationerror::GPUValidationError;
