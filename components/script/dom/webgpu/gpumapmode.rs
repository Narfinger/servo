/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::{dom_struct, dom_struct2};
use script_bindings::reflector::Reflector;
use jstraceable_derive::JSTraceableInSub;
#[dom_struct2]
pub(crate) struct GPUMapMode {
    reflector_: Reflector,
}
