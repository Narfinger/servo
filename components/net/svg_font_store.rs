use std::sync::{Arc, Mutex};

use fonts_traits::{
    FontDescriptor, FontIdentifier, FontTemplateRefMethods, SystemFontServiceProxy,
};
use malloc_size_of_derive::MallocSizeOf;
use resvg::usvg::fontdb::Source;
use resvg::usvg::{self};
use rustc_hash::FxHashMap;
use style::font_face::FamilyName;
use style::values::computed::font::{FontFamilyNameSyntax, SingleFontFamily};
use style::values::computed::{FontStretch, FontStyle, FontSynthesis, FontWeight};
use webrender_api::units::Au;

#[derive(MallocSizeOf)]
struct SvgFontStore {
    #[ignore_malloc_size_of = "TMP"]
    cache: Mutex<FxHashMap<usvg::Font, usvg::fontdb::ID>>,
    system_font_service_proxy: SystemFontServiceProxy,
}

impl SvgFontStore {
    #[expect(unsafe_code)]
    fn svg_select_font(
        &self,
        font: &usvg::Font,
        db: &mut Arc<usvg::fontdb::Database>,
    ) -> Option<usvg::fontdb::ID> {
        let mut cache = self.cache.lock().unwrap();
        if let Some(id) = cache.get(font) {
            Some(*id)
        } else {
            let stretch = FontStretch::from_percentage(1.0);
            let variant = style::computed_values::font_variant_caps::T::Normal;
            let pt_size = Au::from_f32_px(16.);

            let fontdescriptor = FontDescriptor {
                weight: FontWeight::from_float(font.weight() as f32),
                stretch,
                style: FontStyle::normal(),
                variant,
                pt_size,
                variation_settings: vec![],
                synthesis_weight: FontSynthesis::Auto,
                optical_sizing: style::computed_values::font_optical_sizing::T::Auto,
            };

            log::error!("DESCRIPTOR {:?}", fontdescriptor);

            let svg_family = match font.families().first().unwrap() {
                usvg::FontFamily::Serif => "Serif",
                usvg::FontFamily::SansSerif => "SansSerif",
                usvg::FontFamily::Cursive => "Cursive",
                usvg::FontFamily::Fantasy => "Fantasy",
                usvg::FontFamily::Monospace => "Monospace",
                usvg::FontFamily::Named(s) => s,
            };
            let font_family = SingleFontFamily::FamilyName(FamilyName {
                name: svg_family.into(),
                syntax: FontFamilyNameSyntax::Quoted,
            });

            let results = self
                .system_font_service_proxy
                .find_matching_font_templates(Some(&fontdescriptor), &font_family);
            log::error!("RESULTS {:?}", results);
            let font = results.first()?;

            if let FontIdentifier::Local(identifier) = font.identifier() {
                let shared_memory = identifier
                    .font_data_and_index()
                    .unwrap()
                    .data
                    .as_ipc_shared_memory();
                Arc::get_mut(db)
                    .unwrap()
                    .load_font_source(Source::Binary(shared_memory))
                    .first()
                    .cloned()
            } else {
                log::error!("NOONONO");
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

pub(crate) fn create_fn(
    system_font_service_proxy: SystemFontServiceProxy,
) -> usvg::FontResolver<'static> {
    let svg_font_store = SvgFontStore {
        system_font_service_proxy,
        cache: Mutex::new(FxHashMap::default()),
    };
    usvg::FontResolver {
        select_font: Box::new(move |font, db| svg_font_store.svg_select_font(font, db)),
        select_fallback: Box::new(move |c, ids, db| None),
        //svg_font_store.svg_select_fallback(c, c, ids, db)),
    }
}
