use serde::{Deserialize, Serialize};

pub type EleType = u16;

pub const ELE_DIR: EleType = 0;
pub const ELE_RUST_CODE: EleType = 1;
pub const ELE_RUST_TOML: EleType = 2;
pub const ELE_RUST_MOD: EleType = 3;
pub const ELE_RUST_MAKEFILE: EleType = 4;


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Element {
    pub path: String,
    pub element_type: EleType,
    pub template_name: String,
}

impl Element {
    pub fn new(path: String, element_type: EleType, template_name: String) -> Self {
        Element {
            path,
            element_type,
            template_name,
        }
    }
}