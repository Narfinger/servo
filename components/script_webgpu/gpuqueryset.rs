/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use jstraceable_derive::JSTraceable;
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::DomTypes;
use script_bindings::codegen::GenericBindings::WebGPUBinding::GPUQuerySetMethods;
use script_bindings::reflector::Reflector;
use script_bindings::str::USVString;

#[dom_struct]
pub(crate) struct GPUQuerySet {
    reflector_: Reflector,
}

// TODO: wgpu does not expose right fields right now
impl<D: DomTypes> GPUQuerySetMethods<D> for GPUQuerySet {
    /// <https://gpuweb.github.io/gpuweb/#dom-gpuqueryset-destroy>
    fn Destroy(&self) {
        todo!()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label>
    fn Label(&self) -> USVString {
        todo!()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpuobjectbase-label>
    fn SetLabel(&self, _value: USVString) {
        todo!()
    }
}
