/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use js::rust::HandleObject;
use jstraceable_derive::JSTraceable;
use log::warn;
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::DomTypes;
use script_bindings::codegen::GenericBindings::WebGPUBinding::{
    GPUUncapturedErrorEventInit, GPUUncapturedErrorEventMethods,
};
use script_bindings::reflector::{
    reflect_dom_object_test_with_wrap2_with_proto, reflect_dom_object_with_proto,
};
use script_bindings::root::{Dom, DomRoot};
use script_bindings::str::DOMString;
use stylo_atoms::Atom;

use crate::gpuerror::GPUError;
use crate::script_runtime::CanGc;

#[dom_struct]
pub struct GPUUncapturedErrorEvent {
    //event: D::Event,
    #[ignore_malloc_size_of = "Because it is non-owning"]
    gpu_error: Dom<GPUError>,
}

impl GPUUncapturedErrorEvent {
    fn new_inherited<D: DomTypes>(init: &GPUUncapturedErrorEventInit<D>) -> Self {
        todo!()
        /*
        Self {
            gpu_error: Dom::from_ref(&init.error),
            event: D::Event::new_inherited(),
        }
         */
    }

    pub fn new<D>(
        global: &D::GlobalScope,
        event_type: Atom,
        init: &GPUUncapturedErrorEventInit<D>,
        can_gc: CanGc,
    ) -> DomRoot<Self>
    where
        D: DomTypes,
        Box<D::GPUUncapturedErrorEvent>: From<Box<GPUUncapturedErrorEvent>>,
        DomRoot<GPUUncapturedErrorEvent>: From<DomRoot<D::GPUUncapturedErrorEvent>>,
    {
        Self::new_with_proto(global, None, event_type, init, can_gc)
    }

    fn new_with_proto<D>(
        global: &D::GlobalScope,
        proto: Option<HandleObject>,
        event_type: Atom,
        init: &GPUUncapturedErrorEventInit<D>,
        can_gc: CanGc,
    ) -> DomRoot<Self>
    where
        D: DomTypes,
        Box<D::GPUUncapturedErrorEvent>: From<Box<GPUUncapturedErrorEvent>>,
        DomRoot<GPUUncapturedErrorEvent>: From<DomRoot<D::GPUUncapturedErrorEvent>>,
    {
        let event = reflect_dom_object_test_with_wrap2_with_proto::<D, _, _, _>(
            Box::new(GPUUncapturedErrorEvent::new_inherited(init)),
            global,
            proto,
            can_gc,
            script_bindings::codegen::GenericBindings::WebGPUBinding::GPUUncapturedErrorEventWrap::<
                D,
            >,
        );
        /*
        event
            .event
            .init_event(event_type, init.parent.bubbles, init.parent.cancelable);
             */
        event
    }
}

impl<D> GPUUncapturedErrorEventMethods<D> for GPUUncapturedErrorEvent
where
    D: DomTypes,
    D::GPUUncapturedErrorEvent: From<GPUUncapturedErrorEvent>,
    D::GPUError: From<GPUError>,
    Box<D::GPUUncapturedErrorEvent>: From<Box<GPUUncapturedErrorEvent>>,
    DomRoot<GPUUncapturedErrorEvent>: From<DomRoot<D::GPUUncapturedErrorEvent>>,
{
    /// <https://gpuweb.github.io/gpuweb/#dom-gpuuncapturederrorevent-gpuuncapturederrorevent>
    fn Constructor(
        global: &D::GlobalScope,
        proto: Option<HandleObject>,
        can_gc: CanGc,
        event_type: DOMString,
        init: &GPUUncapturedErrorEventInit<D>,
    ) -> DomRoot<D::GPUUncapturedErrorEvent> {
        todo!()
        //GPUUncapturedErrorEvent::new_with_proto(global, proto, event_type.into(), init, can_gc)
        //    .into()
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpuuncapturederrorevent-error>
    fn Error(&self) -> DomRoot<D::GPUError> {
        todo!()
        //DomRoot::from_ref(&self.gpu_error).into()
    }

    /// <https://dom.spec.whatwg.org/#dom-event-istrusted>
    fn IsTrusted(&self) -> bool {
        todo!()
        //self.event.IsTrusted()
    }
}
