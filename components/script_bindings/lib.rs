/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![cfg_attr(crown, feature(register_tool))]
// Register the linter `crown`, which is the Servo-specific linter for the script crate.
#![cfg_attr(crown, register_tool(crown))]

#[macro_use]
extern crate js;
#[macro_use]
extern crate jstraceable_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate malloc_size_of_derive;

pub mod assert;
pub mod callback;
pub mod cell;
mod constant;
mod constructor;
pub mod conversions;
pub mod domstring;
pub mod error;
mod finalize;
mod guard;
mod import;
pub mod inheritance;
pub mod interface;
pub mod interfaces;
pub mod iterable;
pub mod like;
mod lock;
mod mem;
mod namespace;
pub mod num;
pub mod principals;
pub mod proxyhandler;
pub mod realms;
pub mod record;
pub mod reflector;
pub mod root;
pub mod script_runtime;
pub mod settings_stack;
pub mod str;
pub mod structuredclone;
pub mod trace;
pub mod utils;
pub mod weakref;

#[allow(non_snake_case, unsafe_op_in_unsafe_fn)]
pub mod codegen {
    pub mod Globals {
        include!(concat!(env!("OUT_DIR"), "/Globals.rs"));
    }
    #[allow(unused_imports, clippy::enum_variant_names)]
    pub mod InheritTypes {
        include!(concat!(env!("OUT_DIR"), "/InheritTypes.rs"));
    }
    #[allow(clippy::upper_case_acronyms)]
    pub mod PrototypeList {
        include!(concat!(env!("OUT_DIR"), "/PrototypeList.rs"));
    }
    pub(crate) mod DomTypes {
        include!(concat!(env!("OUT_DIR"), "/DomTypes.rs"));
    }
    #[allow(
        clippy::extra_unused_type_parameters,
        clippy::missing_safety_doc,
        clippy::result_unit_err
    )]
    pub mod GenericBindings {
        include!(concat!(env!("OUT_DIR"), "/Bindings/mod.rs"));
    }
    #[allow(
        non_camel_case_types,
        unused_imports,
        unused_variables,
        clippy::large_enum_variant,
        clippy::upper_case_acronyms,
        clippy::enum_variant_names
    )]
    pub mod GenericUnionTypes {
        include!(concat!(env!("OUT_DIR"), "/GenericUnionTypes.rs"));
    }
    pub mod RegisterBindings {
        include!(concat!(env!("OUT_DIR"), "/RegisterBindings.rs"));
    }
}

use euclid::default::Size2D;
// These trait exports are public, because they are used in the DOM bindings.
// Since they are used in derive macros,
// it is useful that they are accessible at the root of the crate.
pub(crate) use js::gc::Traceable as JSTraceable;
use pixels::Snapshot;

pub use crate::codegen::DomTypes::DomTypes;
pub(crate) use crate::reflector::{DomObject, MutDomObject, Reflector};
use crate::root::{Dom, DomRoot};
pub(crate) use crate::trace::CustomTraceable;

/// Non rooted variant of [`crate::dom::bindings::codegen::UnionTypes::HTMLCanvasElementOrOffscreenCanvas`]
#[cfg_attr(crown, crown::unrooted_must_root_lint::must_root)]
#[derive(Clone, JSTraceable, MallocSizeOf)]
pub enum HTMLCanvasElementOrOffscreenCanvas<D: DomTypes> {
    HTMLCanvasElement(Dom<D::HTMLCanvasElement>),
    OffscreenCanvas(Dom<D::OffscreenCanvas>),
}

pub trait CanvasContext {
    type ID;

    fn context_id(&self) -> Self::ID;

    //fn canvas<D: DomTypes>(&self) -> Option<HTMLCanvasElementOrOffscreenCanvas<D>>;

    fn resize(&self);

    // Resets the backing bitmap (to transparent or opaque black) without the
    // context state reset.
    // Used by OffscreenCanvas.transferToImageBitmap.
    fn reset_bitmap(&self);

    /// Returns none if area of canvas is zero.
    ///
    /// In case of other errors it returns cleared snapshot
    fn get_image_data(&self) -> Option<Snapshot>;

    fn origin_is_clean(&self) -> bool {
        true
    }

    /*
    fn size(&self) -> Size2D<u32> {
        self.canvas()
            .map(|canvas| canvas.size())
            .unwrap_or_default()
    }
     */

    fn mark_as_dirty(&self);

    fn onscreen<D: DomTypes>(&self) -> bool {
        todo!()

        /*
        let Some(canvas) = self.canvas() else {
            return false;
        };

        match canvas {
            HTMLCanvasElementOrOffscreenCanvas::HTMLCanvasElement(canvas) => {
                canvas.upcast::<D::Node>().is_connected()
            },
            // FIXME(34628): Offscreen canvases should be considered offscreen if a placeholder is set.
            // <https://www.w3.org/TR/webgpu/#abstract-opdef-updating-the-rendering-of-a-webgpu-canvas>
            HTMLCanvasElementOrOffscreenCanvas::OffscreenCanvas(_) => false,
        }
         */
    }
}
