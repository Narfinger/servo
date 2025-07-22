/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::panic;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::hash_map::{Values, ValuesMut};
use std::rc::{Rc, Weak};
use std::thread::sleep;
use std::time::Duration;

use base::id::WebViewId;
use compositing_traits::rendering_context::{self, RenderingContext};
use compositing_traits::{CompositorMsg, CompositorProxy};
use euclid::Size2D;
use gleam::gl::Gl;
use log::{error, warn};
use servo_config::{opts, pref};
use webrender::{
    Compositor, RenderApi, RenderApiSender, ShaderPrecacheFlags, Transaction, UploadMethod,
    VertexUsageHint, WebRenderOptions,
};
use webrender_api::units::DevicePixel;
use webrender_api::{ColorF, DocumentId, FramePublishId, FrameReadyParams, RenderNotifier};

use crate::IOCompositor;
use crate::webview_renderer::UnknownWebView;

pub(crate) type RenderingGroupId = u64;

pub(crate) struct WebRenderInstance {
    pub(crate) rendering_context: Rc<dyn RenderingContext>,
    pub(crate) webrender: webrender::Renderer,
    pub(crate) webrender_gl: Rc<dyn Gl>,
    pub(crate) webrender_document: DocumentId,
    pub(crate) webrender_api: RenderApi,
    sender: RenderApiSender,
    notifier: MyRenderNotifier,
}

struct MyRenderNotifier {
    frame_ready_msg: RefCell<Vec<(DocumentId, bool)>>,
    sender: CompositorProxy,
}

impl MyRenderNotifier {
    pub fn new(sender: CompositorProxy) -> MyRenderNotifier {
        MyRenderNotifier {
            frame_ready_msg: RefCell::new(vec![]),
            sender,
        }
    }

    pub(crate) fn get(&self) -> Vec<(DocumentId, bool)> {
        //warn!("rendernotifier take");
        self.frame_ready_msg.take()
    }
}

impl webrender_api::RenderNotifier for MyRenderNotifier {
    fn clone(&self) -> Box<dyn webrender_api::RenderNotifier> {
        Box::new(MyRenderNotifier {
            frame_ready_msg: self.frame_ready_msg.clone(),
            sender: self.sender.clone(),
        })
    }

    fn wake_up(&self, _composite_needed: bool) {}

    fn new_frame_ready(
        &self,
        document_id: DocumentId,
        _: FramePublishId,
        frame_ready_params: &FrameReadyParams,
    ) {
        self.sender.send(CompositorMsg::NewWebRenderFrameReady(
            document_id,
            frame_ready_params.render,
        ));
        error!("RenderNotifier push from {document_id:?}");
    }
}

pub(crate) struct WebViewManager<WebView> {
    /// Our top-level browsing contexts. In the WebRender scene, their pipelines are the children of
    /// a single root pipeline that also applies any pinch zoom transformation.
    webviews: HashMap<WebViewId, WebView>,

    rendering_contexts: HashMap<RenderingGroupId, WebRenderInstance>,

    webview_groups: HashMap<WebViewId, RenderingGroupId>,

    /// The order to paint them in, topmost last.
    painting_order: HashMap<RenderingGroupId, Vec<WebViewId>>,

    last_used_id: Option<RenderingGroupId>,

    sender: CompositorProxy,
}

impl<WebView> WebViewManager<WebView> {
    pub(crate) fn new(sender: CompositorProxy) -> Self {
        Self {
            webviews: Default::default(),
            painting_order: Default::default(),
            webview_groups: Default::default(),
            rendering_contexts: Default::default(),
            last_used_id: None,
            sender,
        }
    }
}

impl<WebView> WebViewManager<WebView> {
    pub(crate) fn rendering_contexts(&self) -> impl Iterator<Item = &WebRenderInstance> {
        self.rendering_contexts.iter().map(|(_, v)| v)
    }

    pub(crate) fn clear_background(&self, webview_group_id: RenderingGroupId) {
        error!("CLEAR CLEAR CLEAR");
        let rtc = self.rendering_contexts.get(&webview_group_id).unwrap();
        error!("DOCUMENTID {:?}", rtc.webrender_document);
        let gl = &rtc.webrender_gl;
        {
            debug_assert_eq!(
                (
                    gl.get_error(),
                    gl.check_frame_buffer_status(gleam::gl::FRAMEBUFFER)
                ),
                (gleam::gl::NO_ERROR, gleam::gl::FRAMEBUFFER_COMPLETE)
            );
        }

        // Always clear the entire RenderingContext, regardless of how many WebViews there are
        // or where they are positioned. This is so WebView actually clears even before the
        // first WebView is ready.
        let color = servo_config::pref!(shell_background_color_rgba);
        if webview_group_id == 1 {
            gl.clear_color(0.2, 0.3, 1.0, 0.5);
        } else {
            gl.clear_color(0.8, 0.3, 0.2, 0.5);
        }

        //color[0] as f32,
        //color[1] as f32,
        //color[2] as f32,
        //color[3] as f32,
        //);
        gl.clear(gleam::gl::COLOR_BUFFER_BIT);
    }

    pub(crate) fn send_transaction(&mut self, webview_id: WebViewId, transaction: Transaction) {
        let gid = self.group_id(webview_id).unwrap();
        self.send_transaction_to_group(gid, transaction);
    }

    pub(crate) fn send_transaction_to_group(
        &mut self,
        gid: RenderingGroupId,
        transaction: Transaction,
    ) {
        //warn!("sending some transaction to {gid}");
        let rect = self.rendering_contexts.get_mut(&gid).unwrap();
        rect.webrender_api
            .send_transaction(rect.webrender_document, transaction);
    }

    pub(crate) fn send_transaction_all(&mut self, transaction_creator: impl Fn() -> Transaction) {
        for i in self.rendering_contexts.values_mut() {
            let document_id = i.webrender_document;
            let t = transaction_creator();
            i.webrender_api.send_transaction(document_id, t);
        }
    }

    pub(crate) fn flush_scene_builder(&self) {
        for i in self.rendering_contexts.values() {
            i.webrender_api.flush_scene_builder();
        }
    }

    pub(crate) fn deinit(&mut self) {
        panic!("DEINIT");
        for (_group_id, webrender_instance) in self.rendering_contexts.drain() {
            webrender_instance
                .rendering_context
                .make_current()
                .expect("Foo");
            webrender_instance.webrender.deinit();
        }
        self.last_used_id = None;
        self.painting_order.clear();
        self.webviews.clear();
        self.webview_groups.clear();
    }

    fn group_painting_order_mut(&mut self, webview_id: WebViewId) -> &mut Vec<WebViewId> {
        let group_id = self.webview_groups.get(&webview_id).unwrap();
        self.painting_order.get_mut(group_id).unwrap()
    }

    pub(crate) fn render_instance(&self, group_id: RenderingGroupId) -> &WebRenderInstance {
        self.rendering_contexts.get(&group_id).unwrap()
    }

    pub(crate) fn document_id(&self, webview_id: &WebViewId) -> DocumentId {
        self.webview_groups
            .get(webview_id)
            .and_then(|rgid| self.rendering_contexts.get(rgid))
            .map(|rg| rg.webrender_document)
            .expect("Could not find")
    }

    pub(crate) fn render_instance_mut(
        &mut self,
        group_id: RenderingGroupId,
    ) -> &mut WebRenderInstance {
        self.rendering_contexts.get_mut(&group_id).unwrap()
    }

    fn webrender_options(&self, id: u64) -> WebRenderOptions {
        let clear_color = if id == 1 {
            ColorF::new(0.1, 0.3, 0.7, 1.0)
        } else {
            ColorF::new(0.8, 0.3, 0.1, 1.0)
        };
        webrender::WebRenderOptions {
            // We force the use of optimized shaders here because rendering is broken
            // on Android emulators with unoptimized shaders. This is due to a known
            // issue in the emulator's OpenGL emulation layer.
            // See: https://github.com/servo/servo/issues/31726
            use_optimized_shaders: false,
            //resource_override_path: opts.shaders_dir.clone(),
            precache_flags: if pref!(gfx_precache_shaders) {
                ShaderPrecacheFlags::FULL_COMPILE
            } else {
                ShaderPrecacheFlags::empty()
            },
            enable_aa: pref!(gfx_text_antialiasing_enabled),
            enable_subpixel_aa: pref!(gfx_subpixel_text_antialiasing_enabled),
            allow_texture_swizzling: pref!(gfx_texture_swizzling_enabled),
            clear_color,
            upload_method: UploadMethod::PixelBuffer(VertexUsageHint::Stream),
            panic_on_gl_error: true,
            size_of_op: Some(servo_allocator::usable_size),
            renderer_id: Some(id),
            ..Default::default()
        }
    }

    pub(crate) fn add_webview_group(
        &mut self,
        rendering_context: Rc<dyn RenderingContext>,
    ) -> RenderingGroupId {
        error!(
            "Adding webview group! map {:?} id {:?}",
            self.webview_groups.keys(),
            self.last_used_id
        );
        let new_group_id = {
            *self.last_used_id.get_or_insert(0) += 1;
            self.last_used_id.unwrap()
        };

        error!("WebGroupId {:?} {:?}", new_group_id, self.last_used_id);
        let gl = rendering_context.gleam_gl_api();
        error!("Running on {}", gl.get_string(gleam::gl::RENDERER));
        error!("OpenGL Version {}", gl.get_string(gleam::gl::VERSION));

        debug_assert_eq!(
            (
                gl.get_error(),
                gl.check_frame_buffer_status(gleam::gl::FRAMEBUFFER)
            ),
            (gleam::gl::NO_ERROR, gleam::gl::FRAMEBUFFER_COMPLETE)
        );
        let notifier = MyRenderNotifier::new(self.sender.clone());

        let (webrender, sender) = webrender::create_webrender_instance(
            gl.clone(),
            notifier.clone(),
            self.webrender_options(new_group_id),
            None,
        )
        .expect("Could not");

        let api = sender.create_api();
        let webrender_document = api.add_document(rendering_context.size2d().to_i32());
        let s = WebRenderInstance {
            sender,
            webrender_api: api,
            webrender_document,
            rendering_context,
            webrender,
            webrender_gl: gl,
            notifier,
        };

        // This would otherwise drop the previous webrender instance which will error
        // in mysterious ways
        assert!(!self.rendering_contexts.contains_key(&new_group_id));

        self.rendering_contexts.insert(new_group_id, s);
        self.painting_order.insert(new_group_id, vec![]);
        new_group_id
    }

    pub(crate) fn groups(&self) -> Vec<RenderingGroupId> {
        self.painting_order.keys().cloned().collect()
    }

    pub(crate) fn rendering_context_size(&self) -> Size2D<u32, DevicePixel> {
        self.rendering_contexts
            .values()
            .next()
            .expect("No Context")
            .rendering_context
            .size2d()
    }

    pub(crate) fn present_all(&self) {
        for webrender in self.rendering_contexts() {
            webrender.rendering_context.present();
        }
    }

    pub(crate) fn group_id(&self, webview_id: WebViewId) -> Option<RenderingGroupId> {
        self.webview_groups.get(&webview_id).cloned()
    }

    pub(crate) fn remove(&mut self, webview_id: WebViewId) -> Result<WebView, UnknownWebView> {
        let painting_order = self.group_painting_order_mut(webview_id);
        painting_order.retain(|b| *b != webview_id);
        self.webviews
            .remove(&webview_id)
            .ok_or(UnknownWebView(webview_id))
    }

    pub(crate) fn get(&self, webview_id: WebViewId) -> Option<&WebView> {
        self.webviews.get(&webview_id)
    }

    pub(crate) fn get_mut(&mut self, webview_id: WebViewId) -> Option<&mut WebView> {
        self.webviews.get_mut(&webview_id)
    }

    /// Returns true iff the painting order actually changed.
    pub(crate) fn show(&mut self, webview_id: WebViewId) -> Result<bool, UnknownWebView> {
        if !self.webviews.contains_key(&webview_id) {
            return Err(UnknownWebView(webview_id));
        }
        let painting_order = self.group_painting_order_mut(webview_id);
        if !painting_order.contains(&webview_id) {
            painting_order.push(webview_id);
            return Ok(true);
        }
        Ok(false)
    }

    /// Returns true iff the painting order actually changed.
    pub(crate) fn hide(&mut self, webview_id: WebViewId) -> Result<bool, UnknownWebView> {
        if !self.webviews.contains_key(&webview_id) {
            return Err(UnknownWebView(webview_id));
        }
        let painting_order = self.group_painting_order_mut(webview_id);
        if painting_order.contains(&webview_id) {
            painting_order.retain(|b| *b != webview_id);
            return Ok(true);
        }
        Ok(false)
    }

    /// Returns true iff the painting order actually changed.
    pub(crate) fn hide_all(&mut self, group_id: RenderingGroupId) -> bool {
        let v = self.painting_order.get_mut(&group_id);
        let painting_order = v.unwrap();
        if !painting_order.is_empty() {
            painting_order.clear();
            return true;
        }
        false
    }

    /// Returns true iff the painting order actually changed.
    pub(crate) fn raise_to_top(&mut self, webview_id: WebViewId) -> Result<bool, UnknownWebView> {
        if !self.webviews.contains_key(&webview_id) {
            return Err(UnknownWebView(webview_id));
        }
        let painting_order = self.group_painting_order_mut(webview_id);
        if painting_order.last() != Some(&webview_id) {
            self.hide(webview_id)?;
            self.show(webview_id)?;
            return Ok(true);
        }
        Ok(false)
    }

    pub(crate) fn painting_order(
        &self,
        group_id: RenderingGroupId,
    ) -> impl Iterator<Item = (&WebViewId, &WebView)> {
        error!(
            "groups: {:?} || wvs: {:?} || groupid {:?} || painting {:?}",
            self.webview_groups,
            self.webviews.keys(),
            group_id,
            self.painting_order
        );
        self.painting_order
            .get(&group_id)
            .expect("Could not find group")
            .iter()
            .flat_map(move |webview_id| self.get(*webview_id).map(|b| (webview_id, b)))
    }

    //pub(crate) fn entry(&mut self, webview_id: WebViewId) -> Entry<'_, WebViewId, WebView> {
    //    self.webviews.entry(webview_id)
    //}

    pub(crate) fn add_webview(
        &mut self,
        group_id: RenderingGroupId,
        webview_id: WebViewId,
        webview: WebView,
    ) {
        self.webviews.entry(webview_id).or_insert(webview);
        self.webview_groups.entry(webview_id).or_insert(group_id);
    }

    pub(crate) fn iter(&self) -> Values<'_, WebViewId, WebView> {
        self.webviews.values()
    }

    pub(crate) fn my_iter_mut(
        &mut self,
    ) -> impl Iterator<Item = (&WebViewId, &mut WebView, &WebRenderInstance)> {
        self.webviews.iter_mut().map(|(id, wv)| {
            (
                id,
                wv,
                self.webview_groups
                    .get(id)
                    .and_then(|gid| self.rendering_contexts.get(gid))
                    .expect("Could not get gid"),
            )
        })
    }

    pub(crate) fn iter_mut(&mut self) -> ValuesMut<'_, WebViewId, WebView> {
        self.webviews.values_mut()
    }
}

#[cfg(test)]
mod test {
    use base::id::{BrowsingContextId, Index, PipelineNamespace, PipelineNamespaceId, WebViewId};

    use crate::webview_manager::WebViewManager;
    use crate::webview_renderer::UnknownWebView;

    fn top_level_id(namespace_id: u32, index: u32) -> WebViewId {
        WebViewId(BrowsingContextId {
            namespace_id: PipelineNamespaceId(namespace_id),
            index: Index::new(index).unwrap(),
        })
    }

    fn webviews_sorted<WebView: Clone>(
        webviews: &WebViewManager<WebView>,
    ) -> Vec<(WebViewId, WebView)> {
        let mut keys = webviews.webviews.keys().collect::<Vec<_>>();
        keys.sort();
        keys.iter()
            .map(|&id| (*id, webviews.webviews.get(id).cloned().unwrap()))
            .collect()
    }

    #[test]
    fn test() {
        PipelineNamespace::install(PipelineNamespaceId(0));
        let mut webviews = WebViewManager::default();

        // entry() adds the webview to the map, but not the painting order.
        webviews.entry(WebViewId::new()).or_insert('a');
        webviews.entry(WebViewId::new()).or_insert('b');
        webviews.entry(WebViewId::new()).or_insert('c');
        assert!(webviews.get(top_level_id(0, 1)).is_some());
        assert!(webviews.get(top_level_id(0, 2)).is_some());
        assert!(webviews.get(top_level_id(0, 3)).is_some());
        assert_eq!(
            webviews_sorted(&webviews),
            vec![
                (top_level_id(0, 1), 'a'),
                (top_level_id(0, 2), 'b'),
                (top_level_id(0, 3), 'c'),
            ]
        );
        assert!(webviews.painting_order.is_empty());

        // add() returns WebViewAlreadyExists if the webview id already exists.
        webviews.entry(top_level_id(0, 3)).or_insert('d');
        assert!(webviews.get(top_level_id(0, 3)).is_some());

        // Other methods return UnknownWebView or None if the webview id doesn’t exist.
        assert_eq!(
            webviews.remove(top_level_id(1, 1)),
            Err(UnknownWebView(top_level_id(1, 1)))
        );
        assert_eq!(webviews.get(top_level_id(1, 1)), None);
        assert_eq!(webviews.get_mut(top_level_id(1, 1)), None);
        assert_eq!(
            webviews.show(top_level_id(1, 1)),
            Err(UnknownWebView(top_level_id(1, 1)))
        );
        assert_eq!(
            webviews.hide(top_level_id(1, 1)),
            Err(UnknownWebView(top_level_id(1, 1)))
        );
        assert_eq!(
            webviews.raise_to_top(top_level_id(1, 1)),
            Err(UnknownWebView(top_level_id(1, 1)))
        );

        // For webviews not yet visible, both show() and raise_to_top() add the given webview on top.
        assert_eq!(webviews.show(top_level_id(0, 2)), Ok(true));
        assert_eq!(webviews.show(top_level_id(0, 2)), Ok(false));
        assert_eq!(webviews.painting_order, vec![top_level_id(0, 2)]);
        assert_eq!(webviews.raise_to_top(top_level_id(0, 1)), Ok(true));
        assert_eq!(webviews.raise_to_top(top_level_id(0, 1)), Ok(false));
        assert_eq!(
            webviews.painting_order,
            vec![top_level_id(0, 2), top_level_id(0, 1)]
        );
        assert_eq!(webviews.show(top_level_id(0, 3)), Ok(true));
        assert_eq!(webviews.show(top_level_id(0, 3)), Ok(false));
        assert_eq!(
            webviews.painting_order,
            vec![top_level_id(0, 2), top_level_id(0, 1), top_level_id(0, 3)]
        );

        // For webviews already visible, show() does nothing, while raise_to_top() makes it on top.
        assert_eq!(webviews.show(top_level_id(0, 1)), Ok(false));
        assert_eq!(
            webviews.painting_order,
            vec![top_level_id(0, 2), top_level_id(0, 1), top_level_id(0, 3)]
        );
        assert_eq!(webviews.raise_to_top(top_level_id(0, 1)), Ok(true));
        assert_eq!(webviews.raise_to_top(top_level_id(0, 1)), Ok(false));
        assert_eq!(
            webviews.painting_order,
            vec![top_level_id(0, 2), top_level_id(0, 3), top_level_id(0, 1)]
        );

        // hide() removes the webview from the painting order, but not the map.
        assert_eq!(webviews.hide(top_level_id(0, 3)), Ok(true));
        assert_eq!(webviews.hide(top_level_id(0, 3)), Ok(false));
        assert_eq!(
            webviews.painting_order,
            vec![top_level_id(0, 2), top_level_id(0, 1)]
        );
        assert_eq!(
            webviews_sorted(&webviews),
            vec![
                (top_level_id(0, 1), 'a'),
                (top_level_id(0, 2), 'b'),
                (top_level_id(0, 3), 'c'),
            ]
        );

        // painting_order() returns only the visible webviews, in painting order.
        let mut painting_order = webviews.painting_order();
        assert_eq!(painting_order.next(), Some((&top_level_id(0, 2), &'b')));
        assert_eq!(painting_order.next(), Some((&top_level_id(0, 1), &'a')));
        assert_eq!(painting_order.next(), None);
        drop(painting_order);

        // remove() removes the given webview from both the map and the painting order.
        assert!(webviews.remove(top_level_id(0, 1)).is_ok());
        assert!(webviews.remove(top_level_id(0, 2)).is_ok());
        assert!(webviews.remove(top_level_id(0, 3)).is_ok());
        assert!(webviews_sorted(&webviews).is_empty());
        assert!(webviews.painting_order.is_empty());
    }
}
