use crate::hsl::*;
use random::Random;
use std::any::{type_name, Any, TypeId};
use svg::node::element::Element;

pub trait Layer: Any {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element>;

    fn exclusions(&self) -> Vec<TypeId> {
        vec![]
    }

    fn layer_type(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn layer_name(&self) -> String {
        type_name::<Self>().split("::").last().unwrap().to_string()
    }
}
