use serde_derive::{Serialize, Deserialize};

// TODO: Custom serialize/deserialize to encode literals

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum RbxValue {
    #[serde(rename_all = "PascalCase")]
    String {
        value: String,
    },
    #[serde(rename_all = "PascalCase")]
    Number {
        value: f64,
    },
    #[serde(rename_all = "PascalCase")]
    Bool {
        value: bool,
    },
    #[serde(rename_all = "PascalCase")]
    Vector3 {
        value: [f64; 3],
    },
    #[serde(rename_all = "PascalCase")]
    Color3 {
        value: [u8; 3],
    },
}