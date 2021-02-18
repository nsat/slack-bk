use slack_bk::{
    blocks::{Block, Section},
    composition::Text,
    elements::{Element, Image},
    surfaces::Message,
};
use std::collections::BTreeMap;

fn main() {
    let meta: BTreeMap<&str, &str> = vec![
        ("Time", "2021-02-18T16:53:23Z"),
        ("Dry Run", "false"),
        ("Run ID", "00000000-0000-0000-0000-000000000000"),
        ("Environment", "production"),
    ]
    .into_iter()
    .collect();

    let schedule = "payload";

    let message = Message {
        text: Some(format!("Successfully ran the {} scheduler", schedule)),
        blocks: vec![
            Block::Section(Section {
                text: Some(Text::markdown(format!(":tada: *Successfully ran the {} scheduler*", schedule))),
                fields: meta.iter().map(|(k,v)| Text::markdown(format!("*{}*\n{}", k, v))).collect(),
                accessory: Some(
                    Element::Image(Image{
                        image_url: "https://3bu5rt3kig1aitd5o24s5qym-wpengine.netdna-ssl.com/wp-content/uploads/2018/01/tick.png".into(),
                        alt_text: "Success".into()
                    })
                ),
                block_id: None
            })
        ],
        thread_ts: None,
        mrkdwn: true,
    };

    serde_json::to_writer_pretty(std::io::stdout(), &message).unwrap();
}
