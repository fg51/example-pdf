use lopdf::content::{Content, Operation};
use lopdf::dictionary;
use lopdf::ObjectId;
use lopdf::{Dictionary, Document, Object};

pub const PDF_VERSION: &'static str = "1.5";
pub const BEGIN_TEXT: &'static str = "BT";
pub const END_TEXT: &'static str = "ET";
pub const TEXT_FONT: &'static str = "Tf";
pub const TEXT_POSITION: &'static str = "Td";
pub const TEXT_ONE: &'static str = "Tj";
pub const TEXT_ANY: &'static str = "TJ";

pub fn document() -> Document {
    Document::with_version(PDF_VERSION)
}

pub fn font<'a>(base: &'a str) -> Dictionary {
    dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => base,
    }
}

pub fn resources(font_id: ObjectId) -> Dictionary {
    dictionary! {
        "Font" => dictionary! {
            "F1" => font_id,
        },
    }
}

pub fn page(pages_id: ObjectId, content_id: ObjectId) -> Dictionary {
    dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id,
    }
}

pub fn pages(page_id: ObjectId, resources_id: ObjectId) -> Dictionary {
    dictionary! {
        "Type" => "Pages",
        "Kids" => vec![page_id.into()],
        "Count" => 1,
        "Resources" => resources_id,
        "MediaBox" => vec![0.into(), 0.into(),595.into(),842.into()],
    }
}

pub fn catalog(pages_id: ObjectId) -> Dictionary {
    dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    }
}

pub struct ContentBuilder<'a> {
    font_resource: Option<&'a str>,
    font_size: Option<u16>,
    position: Option<(u32, u32)>,
    texts: Vec<&'a str>,
}

impl<'a> ContentBuilder<'a> {
    pub fn new() -> Self {
        Self {
            font_resource: None,
            font_size: None,
            position: None,
            texts: vec![],
        }
    }

    pub fn font(self, resource: &'a str, size: u16) -> Self {
        Self {
            font_resource: Some(resource),
            font_size: Some(size),
            ..self
        }
    }

    pub fn position(self, position: (u32, u32)) -> Self {
        Self {
            position: Some(position),
            ..self
        }
    }

    pub fn text(mut self, text: &'a str) -> Self {
        self.texts.push(text);
        self
    }

    pub fn build(self) -> Content {
        let position = self.position.unwrap();
        let text = self.texts.join("");
        Content {
            operations: vec![
                Operation::new(BEGIN_TEXT, vec![]),
                Operation::new(
                    TEXT_FONT,
                    vec![
                        self.font_resource.unwrap().into(),
                        self.font_size.unwrap().into(),
                    ],
                ),
                Operation::new(
                    TEXT_POSITION,
                    vec![position.0.into(), position.1.into()],
                ),
                Operation::new(TEXT_ONE, vec![Object::string_literal(text)]),
                Operation::new(END_TEXT, vec![]),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
