use zathura_plugin::{
    plugin_entry, DocumentInfo, DocumentRef, PageInfo, PageRef, PluginError, ZathuraPlugin,
};

use std::fs;

use pango::{Alignment, ContextExt, FontDescription, LayoutExt, WrapMode};
use pangocairo::functions::{create_context, create_layout, show_layout, update_layout};
use pulldown_cmark::{Event, Parser, Tag};

struct DocumentData {
    file: String,
}

struct RustCmarkPlugin {}

fn draw_markdown(file: &str, width: i32, cairo: &mut cairo::Context) -> f64 {
    let mut font = FontDescription::new();
    font.set_family("Arial");
    font.set_absolute_size((32 * pango::SCALE) as f64);

    let parser = Parser::new(&file);
    let context = create_context(cairo).expect("Failed to create drawing context.");
    let layout = create_layout(cairo).expect("Failed to create layout context.");

    layout.set_width(width * pango::SCALE);
    layout.set_alignment(Alignment::Left);
    layout.set_wrap(WrapMode::Word);

    let set_font = |size| {
        let old_size = font.get_size();
        font.set_absolute_size((size * pango::SCALE) as f64);
        layout.set_font_description(Some(&font));
        old_size
    };

    let mut current_height = 0.;
    // let mut current_size = 
    let mut size_queue: Vec<f64> = vec![];

    for event in parser.into_iter() {
        match event {
            Event::Start(tag) => {
                match tag {
                    _ => {
                        // todo!();
                    }
                }
            }
            Event::Text(text) => {
                layout.set_text(&text);
                update_layout(cairo, &layout);
                show_layout(cairo, &layout);

                let (width, height) = layout.get_pixel_size();

                current_height += height as f64;
                cairo.move_to(0., current_height);

                // cairo.stroke_extents

                // ex.width

                // cairo.text_path(&text);
                // cairo.fill();
            }
            Event::End(tag) => {
                match tag {
                    Tag::Heading => {
                        size_
                    }
                    _ => {
                        // todo!();
                    }
                }
            }
            _ => {
                // todo!();
            }
        }
    }

    current_height
}

impl ZathuraPlugin for RustCmarkPlugin {
    type DocumentData = DocumentData;
    type PageData = ();

    fn document_open(doc: DocumentRef<'_>) -> Result<DocumentInfo<Self>, PluginError> {
        // let parser = Parser::new(text);

        let path = doc.path_utf8().expect("Failed to convert to path!");
        let file = fs::read_to_string(path).expect("Failed to read file!");

        Ok(DocumentInfo {
            page_count: 1,
            plugin_data: DocumentData { file },
        })
    }

    fn page_init(
        page: PageRef<'_>,
        doc_data: &mut Self::DocumentData,
    ) -> Result<PageInfo<Self>, PluginError> {
        let image = cairo::ImageSurface::create(cairo::Format::Rgb24, 1, 1)
            .expect("Failed to create image for dry-run");
        let mut cairo = cairo::Context::new(&image);
        let width = 800.;

        let height = draw_markdown(&doc_data.file, width as i32, &mut cairo);

        Ok(PageInfo {
            width,
            height,
            plugin_data: (),
        })
    }

    fn page_render(
        page: PageRef<'_>,
        doc_data: &mut Self::DocumentData,
        page_data: &mut Self::PageData,
        cairo: &mut cairo::Context,
        printing: bool,
    ) -> Result<(), PluginError> {
        let width = page.width();

        draw_markdown(&doc_data.file, width as i32, cairo);

        Ok(())
    }
}

plugin_entry!("RustCmarkPlugin", RustCmarkPlugin, ["text/markdown"]);
