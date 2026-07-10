use std::{convert::identity, sync::{Arc, RwLock}};

use fonts_traits::{FontDescriptor, FontIdentifier, FontTemplateDescriptor, SystemFontServiceMessage, SystemFontServiceProxy};
use malloc_size_of_derive::MallocSizeOf;
use resvg::usvg;
use rustc_hash::FxHashMap;
use webrender_api::FontTemplate;

#[derive(MallocSizeOf)]
struct SvgFontStore {
    #[ignore_malloc_size_of]
    cache: RwLock<FxHashMap<usvg::Font, usvg::fontdb::ID>>,
    system_font_service_proxy: SystemFontServiceProxy,
}

impl SvgFontStore {
    #[expect(unsafe_code)]
    fn svg_select_font(
        &self,
        font: &usvg::Font,
        db: &mut Arc<usvg::fontdb::Database>,
    ) -> Option<usvg::fontdb::ID> {
        if let Some(id) = self.cache.read().unwrap().get(font) {
            Some(id)
        } else {


        let description = FontTemplateDescriptor::new(font.weight(), font.stretch(), font.style());

        let matching_template = self.system_font_service_proxy.find_matching_font_templates(descriptor, family_descriptor).first()?;
        let f = matching_template.borrow();
        let identifier = f.identifier();
        if let FontIdentifier::Local(local_font_identifier) = identifier {
            let data_and_index = unsafe { local_font_identifier.font_data_and_index()?};
            let indices = db.load_font_source(data_and_index.as_ipc_shared_memory());
            let index = indices.first().cloned()?;
            self.cache.write().unwrap().insert(font.clone(), index);
            Some(index)
        } else {
            None
        }
        }
    }

    fn svg_select_fallback(
        &self,
        c: char,
        ids: &[usvg::fontdb::ID],
        db: &mut Arc<usvg::fontdb::Database>,
    ) -> Option<usvg::fontdb::ID> {
        None
    }
}


pub(crate) fn create_fn(system_font_service_proxy: SystemFontServiceProxy) -> usvg::FontResolver {
    let svg_font_store = SvgFontStore {
        system_font_service_proxy,
        cache: FxHashMap::default(),
    };
    usvg::FontResolver {
        usvg::FontResolver {
            select_font: Box::new(|font, db| svg_font_store.svg_select_font(proxy, font, db)),
            select_fallback: Box::new(|c, ids, db| svg_font_store.svg_select_fallback(proxy, c, ids, db)),
        },
    }
}
