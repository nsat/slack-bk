use serde::{Deserialize, Serialize};

use crate::composition::{self, Text};
use crate::util::Style;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Element {
    Button(Button),
    Checkboxes(CheckboxGroup),
    #[serde(rename = "datepicker")]
    DatePicker(DatePicker),
    Image(Image),
    MultiStaticSelect(MultiStaticSelect),
    MultiExternalSelect(MultiExternalSelect),
    MultiUsersSelect(MultiUsersSelect),
    MultiConversationsSelect(MultiConversationsSelect),
    MultiChannelsSelect(MultiChannelsSelect),
    Overflow(OverflowMenu),
    PlainTextInput(PlainTextInput),
    RadioButtons(RadioButtonGroup),
    StaticSelect(StaticSelect),
    ExternalSelect(ExternalSelect),
    UsersSelect(UsersSelect),
    ConversationsSelect(ConversationsSelect),
    ChannelsSelect(ChannelsSelect),
    #[serde(rename = "timepicker")]
    TimePicker(TimePicker),
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Button {
    pub text: Text,
    pub action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<composition::ConfirmationDialog>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CheckboxGroup {
    pub action_id: String,
    pub options: Vec<composition::Option>,
    pub initial_options: Option<Vec<composition::Option>>,
    pub confirm: Option<composition::ConfirmationDialog>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DatePicker {
    pub action_id: String,
    pub placeholder: Option<Text>,
    pub initial_date: Option<String>,
    pub confirm: Option<composition::ConfirmationDialog>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Image {
    pub image_url: String,
    pub alt_text: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MultiStaticSelect {
    pub placeholder: Text,
    pub action_id: String,
    pub options: Vec<composition::Option>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub option_groups: Vec<composition::OptionGroup>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub initial_options: Vec<composition::Option>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<composition::ConfirmationDialog>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_selected_items: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MultiExternalSelect {
    pub placeholder: Text,
    pub action_id: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_query_length: Option<usize>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub initial_options: Vec<composition::Option>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<composition::ConfirmationDialog>,

    pub max_selected_items: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MultiUsersSelect {
    pub placeholder: Text,
    pub action_id: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub initial_users: Vec<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<composition::ConfirmationDialog>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_selected_items: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MultiConversationsSelect {
    pub placeholder: Text,
    pub action_id: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub initial_conversations: Vec<String>,

    #[serde(default)]
    pub default_to_current_conversation: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<composition::ConfirmationDialog>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_selected_items: Option<usize>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<composition::FilterAction>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MultiChannelsSelect {
    pub placeholder: Text,
    pub action_id: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub initial_channels: Vec<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<composition::ConfirmationDialog>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_selected_items: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct OverflowMenu {
    pub action_id: String,
    pub options: Vec<composition::Option>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<composition::ConfirmationDialog>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PlainTextInput {
    pub action_id: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<Text>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_value: Option<String>,

    #[serde(default)]
    pub multiline: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dispatch_action_config: Option<composition::DispatchAction>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RadioButtonGroup {
    action_id: String,
    options: Vec<composition::Option>,
    initial_option: Option<composition::Option>,
    confirm: Option<composition::ConfirmationDialog>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct StaticSelect {
    pub placeholder: Text,
    pub action_id: String,
    pub options: Vec<composition::Option>,

    #[serde(default)]
    pub option_groups: Vec<composition::OptionGroup>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_option: Option<composition::Option>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<composition::ConfirmationDialog>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ExternalSelect {
    pub placeholder: Text,
    pub action_id: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_option: Option<composition::Option>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_query_length: Option<usize>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<composition::ConfirmationDialog>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UsersSelect {
    pub placeholder: Text,
    pub action_id: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_user: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<composition::ConfirmationDialog>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ConversationsSelect {
    pub placeholder: Text,
    pub action_id: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_conversation: Option<String>,

    #[serde(default)]
    pub default_to_current_conversation: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<composition::ConfirmationDialog>,

    #[serde(default)]
    pub response_url_enabled: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<composition::FilterAction>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ChannelsSelect {
    pub placeholder: Text,
    pub action_id: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_channel: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<composition::ConfirmationDialog>,

    #[serde(default)]
    pub response_url_enabled: bool,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TimePicker {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<Text>,

    pub action_id: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_time: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm: Option<composition::ConfirmationDialog>,
}
