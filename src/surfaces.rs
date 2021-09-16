use crate::blocks;
use crate::composition::Text;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<blocks::Block>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_ts: Option<String>,

    #[serde(default = "default_mrkdwn")]
    pub mrkdwn: bool,
}

fn default_mrkdwn() -> bool {
    true
}

impl Default for Message {
    fn default() -> Self {
        Self {
            text: None,
            blocks: Vec::new(),
            thread_ts: None,
            mrkdwn: true,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Modal {
    pub title: Text,

    pub submit: Text,
    pub close: Text,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<blocks::Block>,
}
