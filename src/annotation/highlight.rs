use super::QuadPoints;

use lopdf::{Dictionary, Object};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct HighlightAnnotation {
    quad_points: QuadPoints,
}

impl HighlightAnnotation {
    pub fn new(quad_points: QuadPoints) -> Self {
        Self { quad_points }
    }
}

impl Into<Object> for HighlightAnnotation {
    fn into(self) -> Object {
        let mut dict = Dictionary::new();
        dict.set("Subtype", "Highlight");
        dict.set("QuadPoints", self.quad_points);
        Object::Dictionary(dict)
    }
}
