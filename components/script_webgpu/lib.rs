/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![cfg_attr(crown, feature(register_tool))]
#![deny(unsafe_code)]
// Register the linter `crown`, which is the Servo-specific linter for the script crate.
#![cfg_attr(crown, register_tool(crown))]

mod gpu;
mod gpuadapter;
mod gpuadapterinfo;
mod gpubindgroup;
mod gpubindgrouplayout;
mod gpubuffer;
mod gpubufferusage;
mod gpucanvascontext;
mod gpucolorwrite;
mod gpucommandbuffer;
mod gpucommandencoder;
mod gpucompilationinfo;
mod gpucompilationmessage;
mod gpucomputepassencoder;
mod gpucomputepipeline;
mod gpuconvert;
mod gpudevice;
mod gpudevicelostinfo;
mod gpuerror;
mod gpuinternalerror;
mod gpumapmode;
mod gpuoutofmemoryerror;
mod gpupipelineerror;
mod gpupipelinelayout;
mod gpuqueryset;
mod gpuqueue;
mod gpurenderbundle;
mod gpurenderbundleencoder;
mod gpurenderpassencoder;
mod gpurenderpipeline;
mod gpusampler;
mod gpushadermodule;
mod gpushaderstage;
mod gpusupportedfeatures;
mod gpusupportedlimits;
mod gputexture;
mod gputextureusage;
mod gputextureview;
mod gpuuncapturederrorevent;
mod gpuvalidationerror;
mod identityhub;
mod wgsllanguagefeatures;

pub(crate) use js::gc::Traceable as JSTraceable;
pub(crate) use script_bindings::DomTypes;
pub(crate) use script_bindings::inheritance::HasParent;
pub(crate) use script_bindings::reflector::{AssociatedMemory, DomObject, MutDomObject, Reflector};
use script_bindings::script_runtime;
pub(crate) use script_bindings::trace::CustomTraceable;
