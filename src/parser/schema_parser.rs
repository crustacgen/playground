use std::{collections::HashMap, format};

use crate::model::{
    self,
    schema::{ObjectType, SchemaKind, StringType},
    ReferenceOr,
};

// parses object definition to rust struct, inserts struct into hashmap, returns struct name
fn object_schema_to_string(
    schema: &ObjectType,
    property_name: &str,
    all_structs: &mut HashMap<String, String>,
) -> String {
    let before_string = format!("use serde::{{Deserialize, Serialize}};\n#[derive(Clone, Debug, Deserialize, Serialize)]\npub struct {} {{\n", property_name);
    let after_string = String::from("\n}\n");
    let property_string_it = schema.properties.iter().map(|(key, val)| match val {
        ReferenceOr::Item(x) => schema_parser_mapper(x, key, all_structs),
        _ => {
            panic!("Currently only supports string types")
        }
    });
    let property_string = property_string_it.collect::<Vec<String>>().join(",\n");
    let full_struct = before_string + &property_string + &after_string;
    all_structs.insert(property_name.to_string(), full_struct);
    property_name.to_string()
}

fn string_schema_to_string(schema: &StringType, property_name: &str) -> String {
    let content = format!("pub {}: String", property_name);
    content
}

fn schema_parser_mapper(
    schema: &Box<model::Schema>,
    property_name: &str,
    all_structs: &mut HashMap<String, String>,
) -> String {
    let schema_kind: &SchemaKind = &schema.schema_kind;
    match schema_kind {
        model::schema::SchemaKind::Type(x) => match x {
            model::schema::Type::Object(y) => {
                let struct_name = object_schema_to_string(y, property_name, all_structs);
                return format!("pub {}: {}", property_name, struct_name);
            }
            model::schema::Type::String(y) => {
                return string_schema_to_string(y, property_name);
            }
            model::schema::Type::Number(_) => {
                return String::from(format!("pub {}: f64", property_name));
            }
            model::schema::Type::Boolean {} => {
                return String::from(format!("pub {}: bool", property_name));
            }
            _ => {
                panic!("unhandeled schema type, {:?}", x);
            }
        },
        x => {
            panic!("wrong schema kind {:?}", x);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs, path::Path};

    use super::*;
    use crate::*;

    const SCHEMAS: [&str; 3] = [
        "./example/schemas/userPayload.json",
        "./example/schemas/signupSubscriber.yaml",
        "./example/schemas/userPayloadNested.json",
    ];

    //parse file to json, allowed files are yaml and json
    fn parse_test(path: &Path) -> HashMap<String, model::Schema> {
        let string_content = fs::read_to_string(path).expect("file could not be read");
        // check if file is yaml or json
        let parsed: HashMap<String, model::Schema> = match path.extension() {
            Some(ext) => match ext.to_str() {
                Some("yaml") => {
                    serde_yaml::from_str::<HashMap<String, model::Schema>>(&string_content).unwrap()
                }
                Some("json") => {
                    serde_json::from_str::<HashMap<String, model::Schema>>(&string_content).unwrap()
                }
                _ => {
                    panic!("file has no extension");
                }
            },
            None => {
                panic!("file has no extension");
            }
        };
        parsed
    }

    #[test]
    fn can_parse_schema() {
        for schema_paths in SCHEMAS {
            let definition = parse_test(Path::new(schema_paths));
            for (name, schema) in definition {
                let s = Box::new(schema);
                let structs = &mut HashMap::new();
                schema_parser_mapper(&s, &name, structs);
                let filename_without_extension = Path::new(schema_paths)
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap();
                let out_dir = format!("./test_output/{}.rs", filename_without_extension);
                write_file(
                    Path::new(&out_dir),
                    structs
                        .iter()
                        .map(|(_, v)| v.to_string())
                        .collect::<Vec<String>>()
                        .join("\n"),
                );
            }
        }
    }
}