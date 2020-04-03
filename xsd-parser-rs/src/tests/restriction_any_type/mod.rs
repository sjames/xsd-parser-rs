use super::utils;

#[test]
fn deserialization_works() {
    mod expected {
        use crate::generator::validator::Validate;
        use std::io::{Read, Write};
        use yaserde::{YaDeserialize, YaSerialize};

        include!("expected.rs");
    }

    let ser = include_str!("example.xml");

    let de: expected::AppSequenceType = yaserde::de::from_str(&ser).unwrap();

    assert_eq!(
        de,
        expected::AppSequenceType {
            instance_id: 7,
            sequence_id: Some("http://www.company.org/cum/sonoras".to_string()),
            message_number: 7,
        }
    );
}

#[test]
fn generator_does_not_panic() {
    println!("{}", utils::generate(include_str!("input.xsd")))
}

#[test]
fn generator_output_has_correct_ast() {
    utils::ast_test(include_str!("input.xsd"), include_str!("expected.rs"));
}
