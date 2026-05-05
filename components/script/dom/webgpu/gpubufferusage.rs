/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::{dom_struct, dom_struct2};
use jstraceable_derive::JSTraceableInSub;
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::DomObject;
use script_bindings::inheritance::HasParent;
use script_bindings::reflector::Reflector;
#[dom_struct2]
pub(crate) struct GPUBufferUsage {
    reflector_: Reflector,
}
