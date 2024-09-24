mod highlight;
mod text;

use highlight::*;
use text::*;

use lopdf::{Dictionary, Object};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub enum AnnotationType {
    Text(TextAnnotation),
    Highlight(HighlightAnnotation),
}

impl Into<Object> for AnnotationType {
    fn into(self) -> Object {
        match self {
            Self::Highlight(t) => t.into(),
            Self::Text(t) => t.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Rect(f32, f32, f32, f32);

impl Into<Object> for Rect {
    fn into(self) -> Object {
        Object::Array(vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
        ])
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct QuadPoints(f32, f32, f32, f32, f32, f32, f32, f32);

impl Into<Object> for QuadPoints {
    fn into(self) -> Object {
        Object::Array(vec![
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
        ])
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Annotation {
    subtype: AnnotationType,
    rect: Rect,
    contents: Option<String>,
}

impl Annotation {
    pub fn text(rect: Rect) -> Self {
        let subtype = AnnotationType::Text(TextAnnotation::default());
        Self {
            subtype,
            rect,
            contents: None,
        }
    }

    pub fn contents(mut self, contents: String) -> Self {
        self.contents = Some(contents);
        self
    }
}

impl Into<Object> for Annotation {
    fn into(self) -> Object {
        let mut dict = Dictionary::new();
        dict.set("Type", "Annot");

        let inner_dict = match self.subtype.into() {
            Object::Dictionary(inner_dict) => inner_dict,
            o => panic!("Expected inner object to always convert to Object::Dictionary, but it's actually {o:?}")
        };
        dict.extend(&inner_dict);
        dict.set("Rect", self.rect);
        if let Some(contents) = self.contents {
            dict.set("Contents", contents);
        }
        Object::Dictionary(dict)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn text_text() {
        let annotation = Annotation::text(Rect(0.0, 0.0, 1.0, 1.0))
            .contents("The quick brown fox ate the lazy mouse".into());

        let object: Object = annotation.into();
        let debug = format!("{:?}", object);
        assert_eq!(
            "<</Type /Annot/Subtype /Text/Rect [0 0 1 1]/Contents /The quick brown fox ate the lazy mouse>>",
            debug
        );
    }
}
