/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::marker::PhantomData;

use dom_struct::dom_struct;
use js::gc::HandleObject;
use js::realm::CurrentRealm;
use js::rust::{HandleValue, Trace};
use malloc_size_of::MallocSizeOf;

use crate::reflector::{DomGlobalGeneric, DomObjectWrap, reflect_dom_object};
use crate::root::{Dom, DomRoot, Root};
use crate::script_runtime::{CanGc, JSContext};
use crate::trace::CustomTraceable;
use crate::{DomObject, DomTypes, MutDomObject, Reflector};

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

/*
 *
impl<D: DomTypes> DomObjectWrap<D> for PromiseNativeHandler<D>
where
    Self = D::PromiseNativeHandler,
{
    const WRAP: unsafe fn(
        &mut js::context::JSContext,
        &D::GlobalScope,
        Option<HandleObject>,
        Box<Self>,
    ) -> Root<Dom<Self>> = crate::codegen::GenericBindings::PromiseNativeHandlerBinding::PromiseNativeHandler_Binding::Wrap::<D>;
}
 */

/*
* impl DomObjectWrap<crate::DomTypeHolder> for PromiseNativeHandler {
    const WRAP: unsafe fn(
        &mut JSContext,
        &GlobalScope,
        Option<HandleObject>,
        Box<Self>,
    ) -> Root<Dom<Self>> = crate::dom::bindings::codegen::GenericBindings::PromiseNativeHandlerBinding::PromiseNativeHandler_Binding::Wrap::<crate::DomTypeHolder>;
}
*/

trait PrototypeID {
    const id: u16;
}

/// PromiseNativeHandler id=437

/*
 *
#[cfg_attr(crown, allow(crown::unrooted_must_root))]
pub unsafe fn Wrap<D: DomTypes, T: DomObject + PrototypeID + MutDomObject>(
    cx: &mut JSContext,
    scope: &D::GlobalScope,
    given_proto: Option<HandleObject>,
    object: Box<T>,
) -> DomRoot<T> {
    let raw = Root::new(crate::root::MaybeUnreflectedDom::from_box(object));

    let scope = scope.reflector().get_jsobject();
    assert!(!scope.get().is_null());
    assert!(((*js::rust::get_object_class(scope.get())).flags & js::JSCLASS_IS_GLOBAL) != 0);
    let _ac = js::jsapi::JSAutoRealm::new(cx.raw_cx(), scope.get());

    rooted!(&in(cx) let mut canonical_proto = std::ptr::null_mut::<js::jsapi::JSObject>());
    GetProtoObject::<D>(cx, scope, canonical_proto.handle_mut());
    assert!(!canonical_proto.is_null());

    rooted!(&in(cx) let mut proto = std::ptr::null_mut::<js::jsapi::JSObject>());
    if let Some(given) = given_proto {
        proto.set(*given);
        if js::rust::get_context_realm(cx.raw_cx()) != js::rust::get_object_realm(*given) {
            assert!(JS_WrapObject(cx.raw_cx(), proto.handle_mut()));
        }
    } else {
        proto.set(*canonical_proto);
    }
    rooted!(&in(cx) let obj = JS_NewObjectWithGivenProto(
        cx.raw_cx(),
        &Class.get().base,
        proto.handle(),
    ));
    assert!(!obj.is_null());
    js::jsapi::JS_SetReservedSlot(
        obj.get(),
        crate::conversions::DOM_OBJECT_SLOT,
        &js::jsval::PrivateValue(raw.as_ptr() as *const libc::c_void),
    );

    let root = raw.reflect_with(obj.get());
    root.reflector().set_proto_id(T::id);

    DomRoot::from_ref(&*root)
}

 */
impl<D: DomTypes> PromiseNativeHandler<D>
where
    Self: Sized,
{
    pub fn new(
        global: &D::GlobalScope,
        resolve: Option<Box<dyn Callback>>,
        reject: Option<Box<dyn Callback>>,
        can_gc: CanGc,
    ) -> DomRoot<PromiseNativeHandler<D>> {
        todo!()
        /*
         *
        reflect_dom_object(
            Box::new(PromiseNativeHandler {
                reflector: Reflector::new(),
                resolve,
                reject,
                phantom: PhantomData,
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
