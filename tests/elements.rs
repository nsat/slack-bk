use serde_json::json;
use slack_bk::elements::Element;

#[test]
fn button() {
    serde_json::from_value::<Element>(json! {
        {
            "type": "button",
            "text": {
              "type": "plain_text",
              "text": "Click Me"
            },
            "value": "click_me_123",
            "action_id": "button"
        }
    })
    .unwrap();
    serde_json::from_value::<Element>(json! {
        {
            "type": "button",
            "text": {
              "type": "plain_text",
              "text": "Save"
            },
            "style": "primary",
            "value": "click_me_123",
            "action_id": "button"
          }
    })
    .unwrap();
    serde_json::from_value::<Element>(json! {
        {
            "type": "button",
            "action_id": "button",
            "text": {
              "type": "plain_text",
              "text": "Link Button"
            },
            "url": "https://api.slack.com/block-kit"
          }
    })
    .unwrap();
}

#[test]
fn checkboxes() {
    serde_json::from_value::<Element>(json! {
        {
            "type": "checkboxes",
            "action_id": "this_is_an_action_id",
            "initial_options": [{
                "value": "A1",
                "text": {
                    "type": "plain_text",
                    "text": "Checkbox 1"
                }
            }],
            "options": [
                {
                    "value": "A1",
                    "text": {
                        "type": "plain_text",
                        "text": "Checkbox 1"
                    }
                },
                {
                    "value": "A2",
                    "text": {
                        "type": "plain_text",
                        "text": "Checkbox 2"
                    }
                }
            ]
        }
    })
    .unwrap();
}

#[test]
fn date_picker() {
    serde_json::from_value::<Element>(json! {
        {
            "type": "datepicker",
            "action_id": "datepicker123",
            "initial_date": "1990-04-28",
            "placeholder": {
              "type": "plain_text",
              "text": "Select a date"
            }
        }
    })
    .unwrap();
}

#[test]
fn image() {
    serde_json::from_value::<Element>(json! {
        {
            "type": "image",
            "image_url": "http://placekitten.com/700/500",
            "alt_text": "Multiple cute kittens"
        }
    })
    .unwrap();
}

#[test]
fn multi_static_select() {
    serde_json::from_value::<Element>(json! {
        {
            "action_id": "text1234",
            "type": "multi_static_select",
            "placeholder": {
              "type": "plain_text",
              "text": "Select items"
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
fn multi_external_select() {
    serde_json::from_value::<Element>(json! {
        {
            "action_id": "text1234",
            "type": "multi_external_select",
            "placeholder": {
              "type": "plain_text",
              "text": "Select items"
            },
            "min_query_length": 3
          }
    })
    .unwrap();
}

#[test]
fn multi_users_select() {
    serde_json::from_value::<Element>(json! {
        {
            "action_id": "text1234",
            "type": "multi_users_select",
            "placeholder": {
              "type": "plain_text",
              "text": "Select users"
            }
        }
    })
    .unwrap();
}

#[test]
fn multi_conversations_select() {
    serde_json::from_value::<Element>(json! {
        {
            "action_id": "text1234",
            "type": "multi_conversations_select",
            "placeholder": {
              "type": "plain_text",
              "text": "Select conversations"
            }
        }
    })
    .unwrap();
}

#[test]
fn multi_channels_select() {
    serde_json::from_value::<Element>(json! {
        {
            "action_id": "text1234",
            "type": "multi_channels_select",
            "placeholder": {
              "type": "plain_text",
              "text": "Select channels"
            }
        }
    })
    .unwrap();
}

#[test]
fn overflow() {
    serde_json::from_value::<Element>(json! {
        {
            "type": "overflow",
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
              },
              {
                "text": {
                  "type": "plain_text",
                  "text": "*this is plain_text text*"
                },
                "value": "value-3"
              },
              {
                "text": {
                  "type": "plain_text",
                  "text": "*this is plain_text text*"
                },
                "value": "value-4"
              }
            ],
            "action_id": "overflow"
        }
    })
    .unwrap();
}

#[test]
fn plain_text_input() {
    serde_json::from_value::<Element>(json! {
        {
            "type": "plain_text_input",
            "action_id": "plain_input",
            "placeholder": {
              "type": "plain_text",
              "text": "Enter some plain text"
            }
        }
    })
    .unwrap();
}

#[test]
fn radio_buttons() {
    serde_json::from_value::<Element>(json! {
        {
            "type": "radio_buttons",
            "action_id": "this_is_an_action_id",
            "initial_option": {
              "value": "A1",
              "text": {
                "type": "plain_text",
                "text": "Radio 1"
              }
            },
            "options": [
              {
                "value": "A1",
                "text": {
                  "type": "plain_text",
                  "text": "Radio 1"
                }
              },
              {
                "value": "A2",
                "text": {
                  "type": "plain_text",
                  "text": "Radio 2"
                }
              }
            ]
        }
    })
    .unwrap();
}

#[test]
fn static_select() {
    serde_json::from_value::<Element>(json! {
        {
            "action_id": "text1234",
            "type": "static_select",
            "placeholder": {
              "type": "plain_text",
              "text": "Select an item"
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
fn external_select() {
    serde_json::from_value::<Element>(json! {
        {
            "action_id": "text1234",
            "type": "external_select",
            "placeholder": {
              "type": "plain_text",
              "text": "Select an item"
            },
            "min_query_length": 3
        }
    })
    .unwrap();
}

#[test]
fn users_select() {
    serde_json::from_value::<Element>(json! {
        {
            "action_id": "text1234",
            "type": "users_select",
            "placeholder": {
              "type": "plain_text",
              "text": "Select an item"
            }
        }
    })
    .unwrap();
}

#[test]
fn conversations_select() {
    serde_json::from_value::<Element>(json! {
        {
            "action_id": "text1234",
            "type": "conversations_select",
            "placeholder": {
              "type": "plain_text",
              "text": "Select an item"
            }
        }
    })
    .unwrap();
}

#[test]
fn channels_select() {
    serde_json::from_value::<Element>(json! {
        {
            "action_id": "text1234",
            "type": "channels_select",
            "placeholder": {
              "type": "plain_text",
              "text": "Select an item"
            }
        }
    })
    .unwrap();
}

#[test]
fn time_picker() {
    serde_json::from_value::<Element>(json! {
        {
            "type": "timepicker",
            "action_id": "timepicker123",
            "initial_time": "11:40",
            "placeholder": {
              "type": "plain_text",
              "text": "Select a time"
            }
        }
    })
    .unwrap();
}
