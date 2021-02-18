use serde_json::json;
use slack_bk::composition::*;

#[test]
fn text() {
    serde_json::from_value::<Text>(json! {
        {
            "type": "mrkdwn",
            "text": "A message *with some bold text* and _some italicized text_."
        }
    })
    .unwrap();
}
#[test]
fn option() {
    serde_json::from_value::<Option>(json! {
        {
            "text": {
                "type": "plain_text",
                "text": "Maru"
            },
            "value": "maru"
        }
    })
    .unwrap();
}
#[test]
fn dispatch_action() {
    serde_json::from_value::<DispatchAction>(json! {
        {
            "trigger_actions_on": ["on_character_entered"]
        }
    })
    .unwrap();
}
#[test]
fn option_groups() {
    serde_json::from_value::<OptionGroup>(json! {
        {
            "label": {
              "type": "plain_text",
              "text": "Group 1"
            },
            "options": [
              {
                "text": {
                    "type": "plain_text",
                    "text": "*this is plain_text text*"
                },
                "value": "value-0"
              },
              {
                "text": {
                    "type": "plain_text",
                    "text": "*this is plain_text text*"
                },
                "value": "value-1"
              },
              {
                "text": {
                    "type": "plain_text",
                    "text": "*this is plain_text text*"
                },
                "value": "value-2"
              }
            ]
        }
    })
    .unwrap();
}
#[test]
fn filter() {
    serde_json::from_value::<FilterAction>(json! {
        {
            "include": [
              "public",
              "mpim"
            ],
            "exclude_bot_users" : true
        }
    })
    .unwrap();
}

#[test]
fn confirm() {
    serde_json::from_value::<ConfirmationDialog>(json! {
        {
            "title": {
                "type": "plain_text",
                "text": "Are you sure?"
            },
            "text": {
                "type": "mrkdwn",
                "text": "Wouldn't you prefer a good game of _chess_?"
            },
            "confirm": {
                "type": "plain_text",
                "text": "Do it"
            },
            "deny": {
                "type": "plain_text",
                "text": "Stop, I've changed my mind!"
            }
        }
    })
    .unwrap();
}
