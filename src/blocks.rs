use serde::{Deserialize, Serialize};

use crate::composition::Text;
use crate::elements;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Block {
    Actions(Actions),
    Context(Context),
    Divider(Divider),
    File(File),
    Header(Header),
    Image(Image),
    Input(Input),
    Section(Section),
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Actions {
    pub elements: Vec<elements::Element>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ContextElement {
    Image(elements::Element),
    Text(Text),
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Context {
    pub elements: Vec<ContextElement>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Divider {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FileSource {
    Remote,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub external_id: String,

    pub source: FileSource,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Header {
    pub text: Text,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Image {
    pub image_url: String,
    pub alt_text: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Text>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Input {
    pub label: Text,
    pub element: elements::Element,

    #[serde(default)]
    pub dispatch_action: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hint: Option<Text>,

    #[serde(default)]
    pub optional: bool,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Section {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<Text>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<Text>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accessory: Option<elements::Element>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}
