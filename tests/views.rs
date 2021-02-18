use serde::de::DeserializeOwned;
use slack_bk::{Message, Modal};
use std::fs::File;

fn from_file<T: DeserializeOwned>(file: &str) -> T {
    let json_file = File::open(file).expect("file not found");

    serde_json::from_reader(json_file).expect("error while reading json")
}

#[test]
fn message_template_1() {
    let _msg = from_file::<Message>("tests/data/message1.json");
}

#[test]
fn message_template_2() {
    let _msg = from_file::<Message>("tests/data/message2.json");
}

#[test]
fn message_template_3() {
    let _msg = from_file::<Message>("tests/data/message3.json");
}

#[test]
fn message_template_4() {
    let _msg = from_file::<Message>("tests/data/message4.json");
}

#[test]
fn message_template_5() {
    let _msg = from_file::<Message>("tests/data/message5.json");
}

#[test]
fn message_template_6() {
    let _msg = from_file::<Message>("tests/data/message6.json");
}

#[test]
fn modal_1() {
    let _modal = from_file::<Modal>("tests/data/modal1.json");
}
#[test]
fn modal_2() {
    let _modal = from_file::<Modal>("tests/data/modal1.json");
}
#[test]
fn modal_3() {
    let _modal = from_file::<Modal>("tests/data/modal1.json");
}
#[test]
fn modal_4() {
    let _modal = from_file::<Modal>("tests/data/modal1.json");
}
#[test]
fn modal_5() {
    let _modal = from_file::<Modal>("tests/data/modal1.json");
}
