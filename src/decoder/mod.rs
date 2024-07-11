use alloc::{borrow::ToOwned, format, string::String, vec, vec::Vec};

pub mod types;
use serde_json::Value;
use types::{
    Error, Parameters, ParsedTrait, Pattern, SVGTraitType, StandardDOBOutput, TraitSchema,
};

pub fn dobs_parse_parameters(args: Vec<&[u8]>) -> Result<Parameters, Error> {
    // first args is always the DNA string
    if args.len() < 3 {
        return Err(Error::ParseInvalidArgCount);
    }
    let svg_traits = {
        let value = args[1];
        let traits_pool: Vec<Vec<Value>> =
            serde_json::from_slice(value).map_err(|_| Error::ParseInvalidSVGTraits)?;
        decode_trait_schema(traits_pool)?
    };
    let dob0_output: Vec<StandardDOBOutput> = {
        let output = args[2];
        if output.is_empty() {
            return Err(Error::ParseInvalidDOB0Output);
        }
        serde_json::from_slice(output).map_err(|_| Error::ParseInvalidDOB0Output)?
    };
    Ok(Parameters {
        dob0_output,
        svg_traits,
    })
}

pub fn dobs_decode(parameters: Parameters) -> Result<Vec<u8>, Error> {
    let Parameters {
        dob0_output,
        svg_traits,
    } = parameters;

    // decode svg parts
    let outputs = svg_traits
        .chunk_by(|a, b| a.name == b.name)
        .map(|parts| {
            let mut svg_attributes = vec![];
            let mut svg_elements = vec![];
            let mut name = String::new();
            for part in parts.iter() {
                name.clone_from(&part.name); // names are the same
                let value = if let Some(value) =
                    get_dob0_value_by_name(&part.dob0_trait, &dob0_output)
                {
                    match part.pattern {
                        Pattern::Options | Pattern::Range => {
                            if let Some(value) = get_dob1_value_by_dob0_value(&part.args, value)? {
                                value
                            } else {
                                continue;
                            }
                        }
                        Pattern::Raw => {
                            let value = value
                                .get_string()
                                .cloned()
                                .map_err(|_| Error::DecodeInvalidRawValue)?;
                            let template = part
                                .args
                                .as_str()
                                .ok_or(Error::DecodeInvalidRawValueTemplate)?;
                            template.to_owned().replace("{value}", &value)
                        }
                    }
                } else {
                    // cannot find parsed trait in case of non-empty value, just skip this round
                    if !part.dob0_trait.is_empty() {
                        break;
                    };
                    if part.pattern != Pattern::Raw {
                        return Err(Error::DecodeInvalidRawPattern);
                    }
                    part.args
                        .as_str()
                        .ok_or(Error::DecodeInvalidRawValueTemplate)?
                        .to_owned()
                };
                match part.type_ {
                    SVGTraitType::Attributes => svg_attributes.push(value),
                    SVGTraitType::Elements => svg_elements.push(value),
                };
            }
            // assemble svg content
            let mut svg_content = String::from("<svg");
            svg_attributes
                .into_iter()
                .for_each(|attributes| svg_content += format!(" {}", attributes).as_str());
            svg_content += ">";
            svg_elements
                .into_iter()
                .for_each(|element| svg_content += &element);
            svg_content += "</svg>";
            // generate standard dob/0 output
            let output = StandardDOBOutput {
                name: name.split('.').next().unwrap().to_owned(),
                traits: vec![ParsedTrait::SVG(svg_content)],
            };
            Ok(output)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(serde_json::to_string(&outputs).unwrap().as_bytes().to_vec())
}

pub(crate) fn decode_trait_schema(traits_pool: Vec<Vec<Value>>) -> Result<Vec<TraitSchema>, Error> {
    let traits_base = traits_pool
        .into_iter()
        .map(|schema| {
            if schema.len() < 5 {
                return Err(Error::SchemaInsufficientElements);
            }
            let name = schema[0].as_str().ok_or(Error::SchemaInvalidName)?;
            let type_ = match schema[1].as_str().ok_or(Error::SchemaInvalidType)? {
                "attributes" => SVGTraitType::Attributes,
                "elements" => SVGTraitType::Elements,
                _ => return Err(Error::SchemaTypeMismatch),
            };
            let dob0_trait = schema[2].as_str().ok_or(Error::SchemaInvalidTraitName)?;
            let pattern_str = schema[3].as_str().ok_or(Error::SchemaInvalidPattern)?;
            let pattern = match pattern_str {
                "options" => Pattern::Options,
                "range" => Pattern::Range,
                "raw" => Pattern::Raw,
                _ => return Err(Error::SchemaPatternMismatch),
            };
            let args = schema.get(4).cloned().ok_or(Error::SchemaInvalidArgs)?;
            Ok(TraitSchema {
                name: name.to_owned(),
                type_,
                dob0_trait: dob0_trait.to_owned(),
                pattern,
                args,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(traits_base)
}

fn get_dob0_value_by_name(
    trait_name: &str,
    dob0_output: &[StandardDOBOutput],
) -> Option<ParsedTrait> {
    dob0_output.iter().find_map(|output| {
        if output.name == trait_name {
            output.traits.first().cloned()
        } else {
            None
        }
    })
}

fn get_dob1_value_by_dob0_value(
    args: &Value,
    parsed_dob0_value: ParsedTrait,
) -> Result<Option<String>, Error> {
    for pattern in args.as_array().ok_or(Error::SchemaInvalidArgs)? {
        let item = pattern.as_array().ok_or(Error::SchemaInvalidArgsElement)?;
        let (Some(dob0_value), Some(dob1_value)) = (item.first(), item.get(1)) else {
            return Err(Error::SchemaInvalidArgsElement);
        };
        let dob1_value = dob1_value
            .as_str()
            .ok_or(Error::SchemaInvalidArgsElement)?
            .to_owned();
        if dob0_value.is_number() {
            let value = parsed_dob0_value.get_number()?;
            if value == dob0_value.as_u64().unwrap() {
                return Ok(Some(dob1_value));
            }
        } else if dob0_value.is_string() {
            let value = parsed_dob0_value.get_string()?;
            if value == dob0_value.as_str().unwrap() {
                return Ok(Some(dob1_value));
            }
        } else if dob0_value.is_array() {
            let range = dob0_value.as_array().unwrap();
            if Some(Some("*")) == range.first().map(|v| v.as_str()) {
                return Ok(Some(dob1_value));
            } else {
                if range.len() != 2 {
                    return Err(Error::SchemaInvalidArgsElement);
                }
                let (start, end) = (
                    range[0].as_u64().ok_or(Error::SchemaInvalidArgsElement)?,
                    range[1].as_u64().ok_or(Error::SchemaInvalidArgsElement)?,
                );
                let value = parsed_dob0_value.get_number()?;
                if start <= value && value <= end {
                    return Ok(Some(dob1_value));
                }
            }
        } else {
            return Err(Error::SchemaInvalidArgsElement);
        };
    }
    Ok(None)
}
