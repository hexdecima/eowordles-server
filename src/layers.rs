use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub enum Layer {
    Any,
    Space,
    Surface,
    Underground,
    Caverns,
    Underworld,
    Unknown,
}

impl From<&str> for Layer {
    fn from(value: &str) -> Layer {
        use Layer::*;

        match value {
            "Any" => Any,
            "Space" => Space,
            "Surface" => Surface,
            "Underground" => Underground,
            "Caverns" => Caverns,
            "Underworld" => Underworld,
            _ => Unknown,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct LayerDiff {
    pub right: Vec<Layer>,
    pub wrong: Vec<Layer>,
}

