use serde::{Deserialize, Serialize};

use crate::Style;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Text {
    #[serde(rename = "mrkdwn")]
    Markdown(MarkdownText),

    #[serde(rename = "plain_text")]
    PlainText(PlainText),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MarkdownText {
    pub text: String,

    #[serde(default)]
    pub verbatim: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlainText {
    pub text: String,

    #[serde(default)]
    pub emoji: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfirmationDialog {
    pub title: Text,
    pub text: Text,
    pub confirm: Text,
    pub deny: Text,

    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub style: std::option::Option<Style>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Option {
    pub text: Text,
    pub value: String,

    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub description: std::option::Option<Text>,

    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub url: std::option::Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OptionGroup {
    pub label: Text,
    pub options: Vec<Option>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DispatchAction {
    #[serde(default)]
    pub trigger_actions_on: Vec<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct FilterAction {
    #[serde(default)]
    pub include: Vec<String>,

    #[serde(default)]
    pub exclude_external_shared_channels: bool,

    #[serde(default)]
    pub exclude_bot_users: bool,
}