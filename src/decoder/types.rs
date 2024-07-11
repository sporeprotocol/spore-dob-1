use alloc::{string::String, vec::Vec};
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
pub enum ParsedTrait {
    String(String),
    Number(u64),
    SVG(String),
}

impl ParsedTrait {
    pub fn get_string(&self) -> Result<&String, Error> {
        if let ParsedTrait::String(value) = self {
            Ok(value)
        } else {
            Err(Error::SchemaInvalidParsedTraitType)
        }
    }

    pub fn get_number(&self) -> Result<u64, Error> {
        if let ParsedTrait::Number(value) = self {
            Ok(*value)
        } else {
            Err(Error::SchemaInvalidParsedTraitType)
        }
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
