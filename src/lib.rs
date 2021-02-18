use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub struct SlackMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    blocks: Vec<SlackBlock>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum SlackBlock {
    Section {
        text: SlackText,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        fields: Vec<SlackText>,
        #[serde(skip_serializing_if = "Option::is_none")]
        accessory: Option<SlackAccessory>,
    },
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "text")]
enum SlackText {
    #[serde(rename = "mrkdwn")]
    Markdown(String),

    #[serde(rename = "plain_text")]
    #[allow(dead_code)]
    PlainText(String),
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum SlackAccessory {
    Image { image_url: String, alt_text: String },
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
