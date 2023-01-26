extern crate serde;
extern crate toml;

use serde::de::Deserialize;
use serde_json::{json, Value};

macro_rules! bad {
    ($toml:expr, $msg:expr) => {
        match toml::from_str::<Value>($toml) {
            Ok(s) => panic!("parsed to: {:#?}", s),
            Err(e) => assert_eq!(e.to_string(), $msg),
        }
    };
}

#[test]
fn newlines_after_tables() {
    let s = "
        [a] foo = 1
        [[b]] foo = 1
    ";
    bad!(
        s,
        "expected newline, found an identifier at line 2 column 13"
    );

    let mut d = toml::de::Deserializer::new(s);
    d.set_require_newline_after_table(false);
    let value = Value::deserialize(&mut d).unwrap();
    assert_eq!(value["a"]["foo"], json!(1));
    assert_eq!(value["b"][0]["foo"], json!(1));
}

#[test]
fn allow_duplicate_after_longer() {
    let s = "
        [dependencies.openssl-sys]
        version = 1

        [dependencies]
        libc = 1

        [dependencies]
        bitflags = 1
    ";
    bad!(
        s,
        "redefinition of table `dependencies` for key `dependencies` at line 8 column 9"
    );

    let mut d = toml::de::Deserializer::new(s);
    d.set_allow_duplicate_after_longer_table(true);
    let value = Value::deserialize(&mut d).unwrap();
    assert_eq!(value["dependencies"]["openssl-sys"]["version"], json!(1));
    assert_eq!(value["dependencies"]["libc"], json!(1));
    assert_eq!(value["dependencies"]["bitflags"], json!(1));
}