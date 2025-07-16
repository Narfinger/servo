/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;
use std::collections::hash_map::{Values, ValuesMut};
use std::rc::Rc;

use base::id::WebViewId;
use compositing_traits::rendering_context::RenderingContext;
use compositing_traits::{CompositorMsg, CompositorProxy};
use euclid::Size2D;
use gleam::gl::Gl;
use log::error;
use webrender::WebRenderOptions;
use webrender_api::units::DevicePixel;
use webrender_api::{DocumentId, FramePublishId, FrameReadyParams};

use crate::webview_renderer::UnknownWebView;

pub(crate) type RenderingGroupId = usize;

pub(crate) struct WebRenderInstance {
    pub(crate) rendering_context: Rc<dyn RenderingContext>,
    pub(crate) webrender: webrender::Renderer,
    pub(crate) webrender_gl: Rc<dyn Gl>,
    pub(crate) webrender_document: DocumentId,
}

#[derive(Clone)]
struct RenderNotifier {}

impl RenderNotifier {
    pub fn new() -> RenderNotifier {
        RenderNotifier {}
    }
}

impl webrender_api::RenderNotifier for RenderNotifier {
    fn clone(&self) -> Box<dyn webrender_api::RenderNotifier> {
        Box::new(RenderNotifier::new())
    }

    fn wake_up(&self, _composite_needed: bool) {}

    fn new_frame_ready(
        &self,
        document_id: DocumentId,
        _: FramePublishId,
        frame_ready_params: &FrameReadyParams,
    ) {
        error!("RenderNotifier not implemented");
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
}

impl<WebView> Default for WebViewManager<WebView> {
    fn default() -> Self {
        Self {
            webviews: Default::default(),
            painting_order: Default::default(),
            webview_groups: Default::default(),
            rendering_contexts: Default::default(),
            last_used_id: None,
        }
    }
}

impl<WebView> WebViewManager<WebView> {
    fn group_painting_order(&self, webview_id: WebViewId) -> &Vec<WebViewId> {
        let group_id = self.webview_groups.get(&webview_id).unwrap();
        &self.painting_order.get(group_id).unwrap()
    }

    fn group_painting_order_mut(&mut self, webview_id: WebViewId) -> &mut Vec<WebViewId> {
        let group_id = self.webview_groups.get(&webview_id).unwrap();
        self.painting_order.get_mut(group_id).unwrap()
    }

    pub(crate) fn render_instance(&self, group_id: RenderingGroupId) -> &WebRenderInstance {
        self.rendering_contexts.get(&group_id).unwrap()
    }

    pub(crate) fn render_instance_mut(
        &mut self,
        group_id: RenderingGroupId,
    ) -> &mut WebRenderInstance {
        self.rendering_contexts.get_mut(&group_id).unwrap()
    }

    pub(crate) fn add_webview_group(
        &mut self,
        rendering_context: Rc<dyn RenderingContext>,
        gl: Rc<dyn Gl>,
    ) -> RenderingGroupId {
        let new_group_id = self.last_used_id.unwrap_or_default() + 1;

        let notifier = Box::new(RenderNotifier::new());

        let (webrender, renderapi_sender) = webrender::create_webrender_instance(
            gl.clone(),
            notifier,
            WebRenderOptions::default(),
            None,
        )
        .expect("Could not");

        let webrender_document = renderapi_sender
            .create_api()
            .add_document(rendering_context.size2d().to_i32());
        let s = WebRenderInstance {
            webrender_document,
            rendering_context,
            webrender,
            webrender_gl: gl,
        };
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
        log::error!(
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
