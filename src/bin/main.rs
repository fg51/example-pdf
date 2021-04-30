use lopdf::dictionary;
use lopdf::{Object, Stream};

use example_pdf as lib;

use lib::{catalog, document, font, page, pages, resources, ContentBuilder};

pub fn main() {
    let font_family = "Courier";

    let mut doc = document();

    let pages_id = doc.new_object_id();
    let font_id = doc.add_object(font(font_family));
    let resources_id = doc.add_object(resources(font_id));

    let content = ContentBuilder::new()
        .font("F1", 48)
        .position((100, 600))
        .text("good bye")
        .build();
    let content_id =
        doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap())); // Result

    let page_id = doc.add_object(page(pages_id, content_id));

    let pages = pages(page_id, resources_id);
    doc.objects.insert(pages_id, Object::Dictionary(pages));
    let catalog_id = doc.add_object(catalog(pages_id));
    doc.trailer.set("Root", catalog_id);
    doc.compress();
    doc.save("example.pdf").unwrap(); // Result
}
