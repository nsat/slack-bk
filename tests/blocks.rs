use serde_json::json;
use slack_bk::blocks::Block;

#[test]
fn actions() {
    serde_json::from_value::<Block>(json! {
        {
            "type": "actions",
            "block_id": "actions1",
            "elements": [
              {
                "type": "static_select",
                "placeholder":{
                    "type": "plain_text",
                    "text": "Which witch is the witchiest witch?"
                },
                "action_id": "select_2",
                "options": [
                  {
                    "text": {
                        "type": "plain_text",
                        "text": "Matilda"
                    },
                    "value": "matilda"
                  },
                  {
                    "text": {
                        "type": "plain_text",
                        "text": "Glinda"
                    },
                    "value": "glinda"
                  },
                  {
                    "text": {
                        "type": "plain_text",
                        "text": "Granny Weatherwax"
                    },
                    "value": "grannyWeatherwax"
                  },
                  {
                    "text": {
                        "type": "plain_text",
                        "text": "Hermione"
                    },
                    "value": "hermione"
                  }
                ]
              },
              {
                "type": "button",
                "text": {
                    "type": "plain_text",
                    "text": "Cancel"
                },
                "value": "cancel",
                "action_id": "button_1"
              }
            ]
          }
    })
    .unwrap();
}

#[test]
fn context() {
    serde_json::from_value::<Block>(json! {
        {
            "type": "context",
            "elements": [
              {
                "type": "image",
                "image_url": "https://image.freepik.com/free-photo/red-drawing-pin_1156-445.jpg",
                "alt_text": "images"
              },
              {
                "type": "mrkdwn",
                "text": "Location: **Dogpatch**"
              }
            ]
        }
    })
    .unwrap();
}

#[test]
fn divider() {
    serde_json::from_value::<Block>(json! {
        {
            "type": "divider"
        }
    })
    .unwrap();
}

#[test]
fn fileblock() {
    serde_json::from_value::<Block>(json! {
        {
            "type": "file",
            "external_id": "ABCD1",
            "source": "remote",
        }
    })
    .unwrap();
}

#[test]
fn header() {
    serde_json::from_value::<Block>(json! {
        {
            "type": "header",
            "text": {
              "type": "plain_text",
              "text": "Budget Performance"
            }
        }
    })
    .unwrap();
}

#[test]
fn image() {
    serde_json::from_value::<Block>(json! {
        {
            "type": "image",
            "title": {
              "type": "plain_text",
              "text": "Please enjoy this photo of a kitten"
            },
            "block_id": "image4",
            "image_url": "http://placekitten.com/500/500",
            "alt_text": "An incredibly cute kitten."
        }
    })
    .unwrap();
}

#[test]
fn input() {
    serde_json::from_value::<Block>(json! {
        {
            "type": "input",
            "element": {
              "type": "plain_text_input",
              "action_id": "foo"
            },
            "label": {
              "type": "plain_text",
              "text": "Label",
              "emoji": true
            }
        }
    })
    .unwrap();
}

#[test]
fn section_1() {
    serde_json::from_value::<Block>(json! {
        {
            "type": "section",
            "text": {
              "type": "mrkdwn",
              "text": "A message *with some bold text* and _some italicized text_."
            }
        }
    })
    .unwrap();
}

#[test]
fn section_2() {
    serde_json::from_value::<Block>(json! {
        {
            "type": "section",
            "text": {
              "text": "A message *with some bold text* and _some italicized text_.",
              "type": "mrkdwn"
            },
            "fields": [
              {
                "type": "mrkdwn",
                "text": "High"
              },
              {
                "type": "plain_text",
                "emoji": true,
                "text": "String"
              }
            ]
          }
    })
    .unwrap();
}

#[test]
fn section_3() {
    serde_json::from_value::<Block>(json! {
        {
            "type": "section",
            "text": {
              "text": "*Sally* has requested you set the deadline for the Nano launch project",
              "type": "mrkdwn"
            },
            "accessory": {
              "type": "datepicker",
              "action_id": "datepicker123",
              "initial_date": "1990-04-28",
              "placeholder": {
                "type": "plain_text",
                "text": "Select a date"
              }
            }
        }
    })
    .unwrap();
}
