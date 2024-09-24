use lopdf::{Dictionary, Object};
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub enum TextAnnotationMarkedState {
    Marked,
    #[default]
    Unmarked,
}

#[derive(Clone, Debug, Default, Serialize)]
pub enum TextAnnotationReviewState {
    Accepted,
    Rejected,
    Cancelled,
    Completed,
    #[default]
    None,
}

#[derive(Clone, Debug, Serialize)]
pub enum TextAnnotationStateModel {
    Marked(TextAnnotationMarkedState),
    Review(TextAnnotationReviewState),
}

impl Into<Object> for TextAnnotationStateModel {
    fn into(self) -> Object {
        let mut dict = Dictionary::new();
        match self {
            Self::Marked(marked) => {
                dict.set("StateModel", "Marked");
                let state = match marked {
                    TextAnnotationMarkedState::Marked => "Marked",
                    TextAnnotationMarkedState::Unmarked => "Unmarked",
                };
                dict.set("State", state);
            }
            Self::Review(review) => {
                dict.set("StateModel", "Review");
                let state = match review {
                    TextAnnotationReviewState::Accepted => "Accepted",
                    TextAnnotationReviewState::Rejected => "Rejected",
                    TextAnnotationReviewState::Cancelled => "Cancelled",
                    TextAnnotationReviewState::Completed => "Completed",
                    TextAnnotationReviewState::None => "None",
                };
                dict.set("State", state);
            }
        }
        Object::Dictionary(dict)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct TextAnnotation {
    open: Option<bool>,
    name: Option<String>,
    state: Option<TextAnnotationStateModel>,
}

impl TextAnnotation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn state(mut self, state: TextAnnotationStateModel) -> Self {
        self.state = Some(state);
        self
    }
}

impl Into<Object> for TextAnnotation {
    fn into(self) -> Object {
        let mut dict = Dictionary::new();
        dict.set("Subtype", "Text");
        if let Some(open) = self.open {
            dict.set("Open", open);
        }
        if let Some(name) = self.name {
            dict.set("Name", name);
        }
        if let Some(state) = self.state {
            let Object::Dictionary(inner_dict) = state.into() else {
                unreachable!()
            };
            dict.extend(&inner_dict);
        }
        Object::Dictionary(dict)
    }
}
