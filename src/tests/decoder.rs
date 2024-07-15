use alloc::vec::Vec;
use serde_json::{json, Value};

use crate::decoder::{
    decode_trait_schema, dobs_decode, dobs_parse_parameters,
    types::{Pattern, SVGTraitType, TraitSchema},
};

impl TraitSchema {
    pub fn new(
        name: &str,
        type_: SVGTraitType,
        dob0_trait: &str,
        pattern: Pattern,
        args: Value,
    ) -> Self {
        Self {
            name: name.to_owned(),
            type_,
            dob0_trait: dob0_trait.to_owned(),
            pattern,
            args,
        }
    }

    pub fn encode(&self) -> Vec<Value> {
        vec![
            Value::String(self.name.clone()),
            Value::String(match self.type_ {
                SVGTraitType::Attributes => "attributes".to_owned(),
                SVGTraitType::Elements => "elements".to_owned(),
            }),
            Value::String(self.dob0_trait.clone()),
            Value::String(match self.pattern {
                Pattern::Options => "options".to_owned(),
                Pattern::Range => "range".to_owned(),
                Pattern::Raw => "raw".to_owned(),
            }),
            self.args.clone(),
        ]
    }
}

#[test]
fn test_basic_decode() {
    // generated from `test_generate_basic_example` case
    let images_base = "[[\"IMAGE.0\",\"attributes\",\"\",\"raw\",\"width='200' height='200' xmlns='http://www.w3.org/2000/svg'\"],[\"IMAGE.0\",\"attributes\",\"Name\",\"options\",[[\"Alice\",\"fill='#0000FF'\"],[\"Bob\",\"fill='#00FF00'\"],[\"Ethan\",\"fill='#FF0000'\"],[[\"*\"],\"fill='#FFFFFF'\"]]],[\"IMAGE.0\",\"elements\",\"Age\",\"range\",[[[0,50],\"<image href='btcfs://b2f4560f17679d3e3fca66209ac425c660d28a252ef72444c3325c6eb0364393i0' />\"],[[51,100],\"<image href='btcfs://eb3910b3e32a5ed9460bd0d75168c01ba1b8f00cc0faf83e4d8b67b48ea79676i0' />\"],[[\"*\"],\"<image href='btcfs://11b6303eb7d887d7ade459ac27959754cd55f9f9e50345ced8e1e8f47f4581fai0' />\"]]],[\"IMAGE.1\",\"attributes\",\"\",\"raw\",\"xmlns='http://www.w3.org/2000/svg'\"],[\"IMAGE.1\",\"elements\",\"Score\",\"range\",[[[0,1000],\"<image href='btcfs://11d6cc654f4c0759bfee520966937a4304db2b33880c88c2a6c649e30c7b9aaei0' />\"],[[\"*\"],\"<image href='btcfs://e1484915b27e45b120239080fe5032580550ff9ff759eb26ee86bf8aaf90068bi0' />\"]]],[\"IMAGE.1\",\"elements\",\"Value\",\"raw\",\"<image href='{value}' />\"]]";
    let dob0_output = "[{\"name\":\"Name\",\"traits\":[{\"String\":\"Ethan\"}]},{\"name\":\"Age\",\"traits\":[{\"Number\":23}]},{\"name\":\"Score\",\"traits\":[{\"Number\":136}]},{\"name\":\"_DNA\",\"traits\":[{\"String\":\"0xaabbcc\"}]},{\"name\":\"_URL\",\"traits\":[{\"String\":\"http://127.0.0.1:8090\"}]},{\"name\":\"Value\",\"traits\":[{\"String\":\"btcfs://11d6cc654f4c0759bfee520966937a4304db2b33880c88c2a6c649e30c7b9aaei0\"}]}]";

    let args = vec![
        Default::default(),
        images_base.as_bytes(),
        dob0_output.as_bytes(),
    ];
    let parameters = dobs_parse_parameters(args).expect("parse parameters");
    let result = dobs_decode(parameters).expect("decode parameters");
    println!("{}", String::from_utf8_lossy(&result));
}

// use `test_generate_basic_example` test case in spore-dob-0 repo to generate the following test
#[test]
fn test_basic_trait_schema_encode_decode() {
    let traits = vec![
        TraitSchema::new(
            "IMAGE.0",
            SVGTraitType::Attributes,
            "",
            Pattern::Raw,
            json!("width='200' height='200' xmlns='http://www.w3.org/2000/svg'")
        ),
        TraitSchema::new(
            "IMAGE.0",
            SVGTraitType::Attributes,
            "Name",
            Pattern::Options,
            serde_json::from_str("[[\"Alice\",\"fill='#0000FF'\"],[\"Bob\",\"fill='#00FF00'\"],[\"Ethan\",\"fill='#FF0000'\"],[[\"*\"],\"fill='#FFFFFF'\"]]").unwrap()
        ),
        TraitSchema::new(
            "IMAGE.0",
            SVGTraitType::Elements,
            "Age",
            Pattern::Range,
            serde_json::from_str("[[[0,50],\"<image href='btcfs://b2f4560f17679d3e3fca66209ac425c660d28a252ef72444c3325c6eb0364393i0' />\"],[[51,100],\"<image href='btcfs://eb3910b3e32a5ed9460bd0d75168c01ba1b8f00cc0faf83e4d8b67b48ea79676i0' />\"],[[\"*\"],\"<image href='btcfs://11b6303eb7d887d7ade459ac27959754cd55f9f9e50345ced8e1e8f47f4581fai0' />\"]]").unwrap()
        ),
        TraitSchema::new(
            "IMAGE.1",
            SVGTraitType::Attributes,
            "",
            Pattern::Raw,
            json!("xmlns='http://www.w3.org/2000/svg'")
        ),
        TraitSchema::new(
            "IMAGE.1",
            SVGTraitType::Elements,
            "Score",
            Pattern::Range,
            serde_json::from_str("[[[0,1000],\"<image href='btcfs://11d6cc654f4c0759bfee520966937a4304db2b33880c88c2a6c649e30c7b9aaei0' />\"],[[\"*\"],\"<image href='btcfs://e1484915b27e45b120239080fe5032580550ff9ff759eb26ee86bf8aaf90068bi0' />\"]]").unwrap()
        ),
        TraitSchema::new(
            "IMAGE.1",
            SVGTraitType::Elements,
            "Value",
            Pattern::Raw,
            json!("<image href='{value}' />")
        ),
    ];
    let encoded = traits.iter().map(TraitSchema::encode).collect::<Vec<_>>();
    println!("{}\n", serde_json::to_string_pretty(&encoded).unwrap());
    println!("pattern = {}", serde_json::to_string(&encoded).unwrap());
    let decoded = decode_trait_schema(encoded).expect("decode");
    assert_eq!(traits, decoded);
}
