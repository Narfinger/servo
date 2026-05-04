/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::{dom_struct, dom_struct2};
use js::rust::HandleObject;
use jstraceable_derive::JSTraceableInSub;
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::DomTypes;
use script_bindings::codegen::GenericBindings::WebGPUBinding::{
    GPUUncapturedErrorEventInit, GPUUncapturedErrorEventMethods,
};
use script_bindings::conversions::DerivedFrom;
use script_bindings::reflector::reflect_dom_object_with_proto;
use script_bindings::root::{Dom, DomRoot};
use script_bindings::script_runtime::CanGc;
use script_bindings::str::DOMString;
use stylo_atoms::Atom;

use crate::gpuerror::GPUError;

#[dom_struct2]
pub(crate) struct GPUUncapturedErrorEvent {
    event: Event,
    #[ignore_malloc_size_of = "Because it is non-owning"]
    gpu_error: Dom<GPUError>,
}

impl GPUUncapturedErrorEvent {
    fn new_inherited(init: &GPUUncapturedErrorEventInit) -> Self {
        Self {
            gpu_error: Dom::from_ref(&init.error),
            event: Event::new_inherited(),
        }
    }

    pub(crate) fn new<D: DomTypes, G: DerivedFrom<D::GlobalScope>>(
        global: &G,
        event_type: Atom,
        init: &GPUUncapturedErrorEventInit,
        can_gc: CanGc,
    ) -> DomRoot<Self> {
        Self::new_with_proto(global, None, event_type, init, can_gc)
    }

    fn new_with_proto<D: DomTypes, G: DerivedFrom<D::GlobalScope>>(
        global: &G,
        proto: Option<HandleObject>,
        event_type: Atom,
        init: &GPUUncapturedErrorEventInit,
        can_gc: CanGc,
    ) -> DomRoot<Self> {
        let event = reflect_dom_object_with_proto(
            Box::new(GPUUncapturedErrorEvent::new_inherited(init)),
            global,
            proto,
            can_gc,
        );
        event
            .event
            .init_event(event_type, init.parent.bubbles, init.parent.cancelable);
        event
    }
}

impl GPUUncapturedErrorEventMethods<crate::DomTypeHolder> for GPUUncapturedErrorEvent {
    /// <https://gpuweb.github.io/gpuweb/#dom-gpuuncapturederrorevent-gpuuncapturederrorevent>
    fn Constructor<D: DomTypes, G: DerivedFrom<D::GlobalScope>>(
        global: &G,
        proto: Option<HandleObject>,
        can_gc: CanGc,
        event_type: DOMString,
        init: &GPUUncapturedErrorEventInit,
    ) -> DomRoot<Self> {
        GPUUncapturedErrorEvent::new_with_proto(global, proto, event_type.into(), init, can_gc)
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpuuncapturederrorevent-error>
    fn Error(&self) -> DomRoot<GPUError> {
        DomRoot::from_ref(&self.gpu_error)
    }

    /// <https://dom.spec.whatwg.org/#dom-event-istrusted>
    fn IsTrusted(&self) -> bool {
        self.event.IsTrusted()
    }
}
