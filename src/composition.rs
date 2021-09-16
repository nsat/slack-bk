use serde::{Deserialize, Serialize};

use crate::util::Style;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Text {
    #[serde(rename = "mrkdwn")]
    Markdown(MarkdownText),

    #[serde(rename = "plain_text")]
    PlainText(PlainText),
}

impl Text {
    pub fn plain<S: Into<String>>(text: S) -> Self {
        Self::PlainText(PlainText {
            text: text.into(),
            emoji: true,
        })
    }

    pub fn markdown<S: Into<String>>(text: S) -> Self {
        Self::Markdown(MarkdownText {
            text: text.into(),
            verbatim: false,
        })
    }
}

impl Default for Text {
    fn default() -> Self {
        Text::Markdown(Default::default())
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MarkdownText {
    pub text: String,

    #[serde(default)]
    pub verbatim: bool,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PlainText {
    pub text: String,

    #[serde(default)]
    pub emoji: bool,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ConfirmationDialog {
    pub title: Text,
    pub text: Text,
    pub confirm: Text,
    pub deny: Text,

    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub style: std::option::Option<Style>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Option {
    pub text: Text,
    pub value: String,

    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub description: std::option::Option<Text>,

    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub url: std::option::Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct OptionGroup {
    pub label: Text,
    pub options: Vec<Option>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DispatchAction {
    #[serde(default)]
    pub trigger_actions_on: Vec<String>,
}
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct FilterAction {
    #[serde(default)]
    pub include: Vec<String>,

    #[serde(default)]
    pub exclude_external_shared_channels: bool,

    #[serde(default)]
    pub exclude_bot_users: bool,
}
