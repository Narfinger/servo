/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub mod datablock;
pub mod gpu;
pub mod gpuadapter;
pub mod gpuadapterinfo;
pub mod gpubindgroup;
pub mod gpubindgrouplayout;
pub mod gpubufferusage;
pub mod gpucanvascontext;
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

pub(crate) use js::gc::Traceable as JSTraceable;
pub(crate) use script_bindings::inheritance::HasParent;
pub(crate) use script_bindings::reflector::{DomObject, MutDomObject, Reflector};
pub(crate) use script_bindings::trace::CustomTraceable;
