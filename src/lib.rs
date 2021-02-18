use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SlackMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub blocks: Vec<SlackBlock>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SlackBlock {
    Section {
        text: SlackText,

        #[serde(skip_serializing_if = "Vec::is_empty")]
        fields: Vec<SlackText>,

        #[serde(skip_serializing_if = "Option::is_none")]
        accessory: Option<SlackAccessory>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "text")]
pub enum SlackText {
    #[serde(rename = "mrkdwn")]
    Markdown(String),

    #[serde(rename = "plain_text")]
    #[allow(dead_code)]
    PlainText(String),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SlackAccessory {
    Image { image_url: String, alt_text: String },
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
