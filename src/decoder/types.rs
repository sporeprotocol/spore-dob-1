use alloc::{collections::BTreeMap, string::String, string::ToString, vec::Vec};
use serde_json::Value;

#[repr(u64)]
#[cfg_attr(test, derive(Debug))]
pub enum Error {
    ParseInvalidArgCount = 1,
    ParseInvalidDOB0Output,
    ParseInvalidSVGTraits,

    SchemaInsufficientElements,
    SchemaInvalidName,
    SchemaInvalidTraitName,
    SchemaInvalidType,
    SchemaTypeMismatch,
    SchemaInvalidPattern,
    SchemaPatternMismatch,
    SchemaInvalidArgs,
    SchemaInvalidArgsElement,
    SchemaInvalidParsedTraitType,

    DecodeInvalidOptionArgs,
    DecodeInvalidRawValue,
    DecodeInvalidRawValueTemplate,
    DecodeInvalidRawPattern,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct ParsedTrait {
    #[serde(flatten)]
    traits: BTreeMap<String, Value>,
}

impl ParsedTrait {
    pub fn get_string(&self) -> Result<&str, Error> {
        if let Some((_, v)) = self.traits.iter().next() {
            Ok(v.as_str().ok_or(Error::SchemaInvalidParsedTraitType)?)
        } else {
            Err(Error::SchemaInvalidParsedTraitType)
        }
    }

    pub fn get_number(&self) -> Result<u64, Error> {
        if let Some((_, v)) = self.traits.iter().next() {
            Ok(v.as_u64().ok_or(Error::SchemaInvalidParsedTraitType)?)
        } else {
            Err(Error::SchemaInvalidParsedTraitType)
        }
    }

    pub fn new(key: &str, value: Value) -> Self {
        let mut traits = BTreeMap::new();
        traits.insert(key.to_string(), value);
        Self { traits }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct StandardDOBOutput {
    pub name: String,
    pub traits: Vec<ParsedTrait>,
}

#[cfg_attr(test, derive(serde::Deserialize))]
pub struct Parameters {
    pub dob0_output: Vec<StandardDOBOutput>,
    pub svg_traits: Vec<TraitSchema>,
}

#[cfg_attr(test, derive(serde::Serialize, Clone, Debug))]
#[derive(serde::Deserialize, PartialEq, Eq)]
pub enum SVGTraitType {
    Attributes,
    Elements,
}

#[cfg_attr(test, derive(serde::Serialize, Clone, Debug))]
#[derive(serde::Deserialize, PartialEq)]
pub enum Pattern {
    Options,
    Range,
    Raw,
}

#[cfg_attr(test, derive(serde::Serialize, Clone, PartialEq, Debug))]
#[derive(serde::Deserialize)]
pub struct TraitSchema {
    pub name: String,
    pub type_: SVGTraitType,
    pub dob0_trait: String,
    pub pattern: Pattern,
    pub args: Value,
}
