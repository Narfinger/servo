/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::rc::Rc;

use dom_struct::dom_struct;
use js::jsapi::HandleObject;
use jstraceable_derive::JSTraceable;
use log::warn;
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::DomTypes;
use script_bindings::codegen::GenericBindings::WebGPUBinding::{
    GPUMethods, GPUPowerPreference, GPURequestAdapterOptions, GPUTextureFormat,
};
use script_bindings::error::Error;
use script_bindings::realms::InRealm;
use script_bindings::reflector::{Reflector, reflect_dom_object};
use script_bindings::root::DomRoot;
use script_bindings::str::DOMString;
use servo_constellation_traits::ScriptToConstellationMessage;
use webgpu_traits::WebGPUAdapterResponse;
use wgpu_types::PowerPreference;

use super::wgsllanguagefeatures::WGSLLanguageFeatures;
use crate::MutNullableDom;
use crate::gpuadapter::GPUAdapter;
use crate::script_runtime::CanGc;

#[dom_struct]
#[expect(clippy::upper_case_acronyms)]
pub(crate) struct GPU<D: DomTypes> {
    reflector_: Reflector,
    /// Same object for <https://www.w3.org/TR/webgpu/#dom-gpu-wgsllanguagefeatures>
    wgsl_language_features: MutNullableDom<WGSLLanguageFeatures<D>>,
}

impl<D: DomTypes> GPU<D> {
    pub(crate) fn new_inherited() -> GPU<D> {
        GPU {
            reflector_: Reflector::new(),
            wgsl_language_features: MutNullableDom::default(),
        }
    }

    pub(crate) fn new(global: &D::GlobalScope, can_gc: CanGc) -> DomRoot<GPU<D>> {
        reflect_dom_object(Box::new(GPU::new_inherited()), global, can_gc)
    }
}

impl<D: DomTypes> GPUMethods<D> for GPU<D>
where
    D: DomTypes,
    D::WGSLLanguageFeatures: From<WGSLLanguageFeatures<D>>,
{
    /// <https://gpuweb.github.io/gpuweb/#dom-gpu-requestadapter>
    fn RequestAdapter(
        &self,
        options: &GPURequestAdapterOptions,
        comp: InRealm,
        can_gc: CanGc,
    ) -> Rc<D::Promise> {
        let global = &self.global();
        let promise = D::Promise::new_in_current_realm(comp, can_gc);
        let task_source = global.task_manager().dom_manipulation_task_source();
        //let callback = callback_promise(&promise, self, task_source);

        let power_preference = match options.powerPreference {
            Some(GPUPowerPreference::Low_power) => PowerPreference::LowPower,
            Some(GPUPowerPreference::High_performance) => PowerPreference::HighPerformance,
            None => PowerPreference::default(),
        };
        let ids = global.wgpu_id_hub().create_adapter_id();

        let script_to_constellation_chan = global.script_to_constellation_chan();
        /*
        if script_to_constellation_chan
            .send(ScriptToConstellationMessage::RequestAdapter(
                callback,
                wgpu_core::instance::RequestAdapterOptions {
                    power_preference,
                    compatible_surface: None,
                    force_fallback_adapter: options.forceFallbackAdapter,
                },
                ids,
            ))
            .is_err()
        {
            promise.reject_error(Error::Operation(None), can_gc);
        }
         */
        promise
    }

    /// <https://gpuweb.github.io/gpuweb/#dom-gpu-getpreferredcanvasformat>
    fn GetPreferredCanvasFormat(&self) -> GPUTextureFormat {
        // From https://github.com/mozilla-firefox/firefox/blob/24d49101ce17b78c3ba1217d00297fe2891be6b3/dom/webgpu/Instance.h#L68
        if cfg!(target_os = "android") {
            GPUTextureFormat::Rgba8unorm
        } else {
            GPUTextureFormat::Bgra8unorm
        }
    }

    /// <https://www.w3.org/TR/webgpu/#dom-gpu-wgsllanguagefeatures>
    fn WgslLanguageFeatures(&self, can_gc: CanGc) -> DomRoot<D::WGSLLanguageFeatures> {
        self.wgsl_language_features
            .or_init(|| WGSLLanguageFeatures::new(&self.global(), None, can_gc))
            .into()
    }
}

/*
impl RoutedPromiseListener<WebGPUAdapterResponse> for GPU {
    fn handle_response(
        &self,
        cx: &mut js::context::JSContext,
        response: WebGPUAdapterResponse,
        promise: &Rc<Promise>,
    ) {
        match response {
            Some(Ok(adapter)) => {
                let adapter = GPUAdapter::new(
                    &self.global(),
                    adapter.channel,
                    DOMString::from(format!(
                        "{} ({:?})",
                        adapter.adapter_info.name, adapter.adapter_id.0
                    )),
                    HandleObject::null(),
                    adapter.features,
                    adapter.limits,
                    adapter.adapter_info,
                    adapter.adapter_id,
                    CanGc::from_cx(cx),
                );
                promise.resolve_native(&adapter, CanGc::from_cx(cx));
            },
            Some(Err(e)) => {
                warn!("Could not get GPUAdapter ({:?})", e);
                promise.resolve_native(&None::<GPUAdapter>, CanGc::from_cx(cx));
            },
            None => {
                warn!("Couldn't get a response, because WebGPU is disabled");
                promise.resolve_native(&None::<GPUAdapter>, CanGc::from_cx(cx));
            },
        }
    }
}
 */
