use slack_bk::{
    blocks::{Actions, Block, Section},
    composition::Text,
    elements::{Button, Element},
    surfaces::Message,
    util::Style,
};

fn main() {
    let message = Message {
        blocks: vec![
            Block::Section(Section {
                text: Some(Text::markdown("You have a new request:\n*<fakeLink.toEmployeeProfile.com|Fred Enriquez - New device request>*")),
                ..Default::default()
            }),
            Block::Section(Section {
                fields: vec![
                    Text::markdown("*Type:*\nComputer (laptop)"),
                    Text::markdown("*When:*\nSubmitted Aut 10"),
                    Text::markdown("*Last Update:*\nMar 10, 2015 (3 years, 5 months)"),
                    Text::markdown("*Reason:*\nAll vowel keys aren't working."),
                    Text::markdown("*Specs:*\n\"Cheetah Pro 15\" - Fast, really fast\""),
                ],
                ..Default::default()
            }),
            Block::Actions(Actions {
                elements: vec![
                    Element::Button(Button {
                        text: Text::plain("Approve"),
                        style: Some(Style::Primary),
                        value: Some("click_me_123".into()),
                        ..Default::default()
                    }),
                    Element::Button(Button {
                        text: Text::plain("Deny"),
                        style: Some(Style::Danger),
                        value: Some("click_me_123".into()),
                        ..Default::default()
                    })
                ],
                ..Default::default()
            })
        ],
        ..Default::default()
    };

    serde_json::to_writer_pretty(std::io::stdout(), &message).unwrap();
}
