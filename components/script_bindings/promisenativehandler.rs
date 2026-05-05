/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::marker::PhantomData;

use dom_struct::dom_struct;
use js::realm::CurrentRealm;
use js::rust::{HandleValue, Trace};
use malloc_size_of::MallocSizeOf;

use crate::root::DomRoot;
use crate::script_runtime::CanGc;
use crate::trace::CustomTraceable;
use crate::{DomObject, DomTypes, Reflector, reflect_dom_object};

/// Types that implement the `Callback` trait follow the same rooting requirements
/// as types that use the `#[dom_struct]` attribute.
/// Prefer storing `Dom<T>` members inside them instead of `DomRoot<T>`
/// to minimize redundant work by the garbage collector.
pub trait Callback: crate::JSTraceable + MallocSizeOf {
    fn callback(&self, cx: &mut CurrentRealm, v: HandleValue);
}

#[dom_struct]
pub struct PromiseNativeHandler<D: DomTypes> {
    reflector: Reflector,
    resolve: Option<Box<dyn Callback>>,
    reject: Option<Box<dyn Callback>>,
    phantom: PhantomData<D>,
}

impl<D: DomTypes> PromiseNativeHandler<D> {
    pub fn new(
        global: &D::GlobalScope,
        resolve: Option<Box<dyn Callback>>,
        reject: Option<Box<dyn Callback>>,
        can_gc: CanGc,
    ) -> DomRoot<PromiseNativeHandler<D>> {
        todo!()
        /*
        reflect_dom_object(
            Box::new(PromiseNativeHandler {
                reflector: Reflector::new(),
                resolve,
                reject,
            }),
            global,
            can_gc,
        )
         */
    }

    pub fn resolved_callback(&self, cx: &mut CurrentRealm, v: HandleValue) {
        callback(&self.resolve, cx, v)
    }

    pub fn rejected_callback(&self, cx: &mut CurrentRealm, v: HandleValue) {
        callback(&self.reject, cx, v)
    }
}

fn callback(callback: &Option<Box<dyn Callback>>, cx: &mut CurrentRealm, v: HandleValue) {
    if let Some(ref callback) = *callback {
        callback.callback(cx, v)
    }
}
