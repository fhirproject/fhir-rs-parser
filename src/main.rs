use inflector::Inflector;
use maplit::hashmap;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::path::Path;
use textwrap::{fill, indent};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Schema {
  #[serde(rename = "$schema")]
  schema: String,

  id: String,

  description: String,

  discriminator: Discriminator,

  one_of: Vec<ClassReference>,

  definitions: HashMap<String, Definition>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ClassReference {
  #[serde(rename = "$ref")]
  fhir_ref: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Discriminator {
  property_name: String,

  mapping: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
enum Item {
  Ref(String),
  Enum(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
enum Property {
  Reference {
    description: String,
    #[serde(rename = "$ref")]
    fhir_ref: String,
  },
  Array {
    description: String,
    items: HashMap<String, Item>,
    #[serde(rename = "type")]
    fhir_type: String,
  },
  PatternedTyped {
    description: String,
    pattern: String,
    #[serde(rename = "type")]
    fhir_type: String,
  },
  Typed {
    description: String,
    #[serde(rename = "type")]
    fhir_type: String,
  },
  Enum {
    description: String,
    #[serde(rename = "enum")]
    fhir_enum: Vec<String>,
  },
  Const {
    description: String,
    #[serde(rename = "const")]
    fhir_const: String,
  },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Definition {
  pattern: Option<String>,

  #[serde(rename = "type")]
  fhir_type: Option<String>,

  description: Option<String>,

  properties: Option<HashMap<String, Property>>,

  additional_properties: Option<bool>,

  required: Option<Vec<String>>,

  one_of: Option<Vec<HashMap<String, String>>>,
}

fn main() {
  fs::remove_dir_all("out/src/model").unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
  });
  fs::create_dir("out/src/model").unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
  });

  let schema_contents =
    fs::read_to_string("src/fhir.schema.json").expect("Something went wrong reading the file");
  let fhir_schema: Schema = serde_json::from_str(&schema_contents).unwrap();

  let mut reference_to_class_name_map: HashMap<String, String> = HashMap::new();

  for (class_name, reference) in &fhir_schema.discriminator.mapping {
    println!("ref: {}, class_name: {}", &reference, &class_name);
    reference_to_class_name_map.insert(reference.to_string(), class_name.to_string());
  }

  let builtin_type_to_class_map: HashMap<&str, (&str, Option<&str>)> = hashmap! {
    "int" => ("i32", None),
    "integer" => ("i32", None),
    "number" => ("i32", None), // todo
    "uri" => ("String", None),
    "url" => ("String", None),
    "markdown" => ("String", None),
    "xhtml" => ("String", None),
    "decimal" => ("f32", None),
    "positiveInt" => ("i32", None),
    "canonical" => ("String", None),
    "float" => ("f32", None),
    "string" => ("String", None),
    "code" => ("String", None),
    "boolean" => ("bool", None),
    "unsignedInt" => ("u32", None),
    "id" => ("String", None),
    "time" => ("String", None), // todo
    "time" => ("String", None), // todo
    "instant" => ("String", None), // todo
    "date" => ("i32", None), // todo
    "base64Binary" => ("String", None),
    "dateTime" => ("String", None), // todo
    // These structs are way too heavyweight to remain on the stack,
    // so we break them into heap references.
    "Reference" => ("Box<Reference>", Some("crate::model::Reference::Reference")),
    "ElementDefinition" => ("Box<ElementDefinition>", Some("crate::model::ElementDefinition::ElementDefinition")),
    "StructureDefinition" => ("Box<StructureDefinition>", Some("crate::model::StructureDefinition::StructureDefinition")),
    "Extension" => ("Box<Extension>", Some("crate::model::Extension::Extension")),
    "ImplementationGuide" => ("Box<ImplementationGuide>", Some("crate::model::ImplementationGuide::ImplementationGuide")),
    "ActivityDefinition" => ("Box<ActivityDefinition>", Some("crate::model::ActivityDefinition::ActivityDefinition")),
    "StructureMap_Source" => ("Box<StructureMap_Source>", Some("crate::model::StructureMap_Source::StructureMap_Source")),
    "Measure" => ("Box<Measure>", Some("crate::model::Measure::Measure")),
    "ResearchElementDefinition" => ("Box<ResearchElementDefinition>", Some("crate::model::ResearchElementDefinition::ResearchElementDefinition")),
    "ElementDefinition_Example" => ("Box<ElementDefinition_Example>", Some("crate::model::ElementDefinition_Example::ElementDefinition_Example")),
    "Observation" => ("Box<Observation>", Some("crate::model::Observation::Observation")),

  };

  let property_replacement_map = hashmap! {
    "type" => "fhir_type",
    "r#type" => "fhir_r_type",
    "use" => "fhir_use",
    "<" => "less_than",
    ">" => "greater_than",
    "<=" => "less_than_or_equal",
    ">=" => "greater_than_or_equal",
    "=" => "equal",
    "!=" => "not_equal",
    "0" => "zero",
    "1" => "one",
    "2" => "two",
    "3" => "three",
    "4" => "four",
    "5" => "five",
    "6" => "six",
    "7" => "seven",
    "8" => "eight",
    "9" => "nine",
    "10" => "ten",
    "11" => "eleven",
    "12" => "twelve",
    "r#for" => "fhir_r_for",
    "for" => "fhir_for",
    "r#ref" => "fhir_r_ref",
    "ref" => "fhir_ref",
    "r#abstract" => "fhir_r_abstract",
    "abstract" => "fhir_abstract",
  };

  let mut model_mod_contents = String::new();
  model_mod_contents.push_str("#![allow(non_snake_case)]\n\n");

  for (definition_name, definition) in &fhir_schema.definitions {
    let contents = generate_class(
      &definition_name,
      &definition,
      &reference_to_class_name_map,
      &builtin_type_to_class_map,
      &property_replacement_map,
    );

    let path_string = format!("out/src/model/{}.rs", definition_name);
    let wrote_file = write_string_to_file(&contents, &path_string);

    if wrote_file {
      model_mod_contents.push_str("pub mod ");
      model_mod_contents.push_str(&definition_name);
      model_mod_contents.push_str(";\n");
    }
  }

  write_string_to_file(&model_mod_contents, "out/src/model/mod.rs");

  let mut parser_string = String::new();

  parser_string.push_str("#![allow(unused_imports, non_camel_case_types)]\n\n");

  parser_string.push_str("use serde::{Deserialize, Serialize};\nuse crate::model;\n\n");

  parser_string.push_str("#[derive(Debug, Serialize, Deserialize)]\n");
  parser_string.push_str("#[serde(tag = \"resourceType\")]\n");
  parser_string.push_str("pub enum FHIRResource {\n");
  for (resource_name, _) in &fhir_schema.discriminator.mapping {
    parser_string.push_str("  #[serde(rename = \"");
    parser_string.push_str(&resource_name);
    parser_string.push_str("\")]\n");
    parser_string.push_str("  Parsed");
    parser_string.push_str(&resource_name);
    parser_string.push_str("(model::");
    parser_string.push_str(&resource_name);
    parser_string.push_str("::");
    parser_string.push_str(&resource_name);
    parser_string.push_str("),\n\n");
  }
  parser_string.push_str("}");

  write_string_to_file(&parser_string, "out/src/parser.rs");
}

fn write_string_to_file(contents: &str, path_string: &str) -> bool {
  if contents.len() == 0 {
    return false;
  }
  let path = Path::new(&path_string);
  // Open a file in write-only mode, returns `io::Result<File>`
  let mut file = match File::create(&path) {
    Err(why) => panic!("couldn't create {}: {}", path_string, why.description()),
    Ok(file) => file,
  };

  // Write the file contents string to `file`, returns `io::Result<()>`
  match file.write_all(contents.as_bytes()) {
    Err(why) => panic!("couldn't write to {}: {}", path_string, why.description()),
    Ok(_) => println!("successfully wrote to {}", path_string),
  }
  return true;
}

fn generate_class(
  name: &str,
  definition: &Definition,
  reference_to_class_name_map: &HashMap<String, String>,
  builtin_type_to_class_map: &HashMap<&str, (&str, Option<&str>)>,
  property_replacement_map: &HashMap<&str, &str>,
) -> String {
  let mut string = String::new();

  let mut pending_enums: Vec<(String, &Vec<String>)> = Vec::new();
  let mut pending_imports: HashSet<String> = HashSet::new();

  string.push_str("#![allow(unused_imports, non_camel_case_types)]\n\n");

  string.push_str("use serde::{Deserialize, Serialize};\n");

  let mut inner_string = String::new();

  if let Some(one_of) = &definition.one_of {
    inner_string.push_str("\n#[derive(Debug, Serialize, Deserialize)]\n");
    inner_string.push_str("#[serde(tag = \"resourceType\")]\n");
    inner_string.push_str("pub enum ");
    inner_string.push_str(name);
    inner_string.push_str(" {\n");
    for hash_map in one_of {
      let fhir_ref = &hash_map["$ref"];
      let original_name = extract_type_from_ref(&fhir_ref);
      let (fhir_name, import) = class_name_from_fhir_ref(
        &fhir_ref,
        &reference_to_class_name_map,
        builtin_type_to_class_map,
      );
      if let Some(import) = import {
        pending_imports.insert(import);
      }
      inner_string.push_str("  #[serde(rename = \"");
      inner_string.push_str(&original_name);
      inner_string.push_str("\")]\n");
      inner_string.push_str("  Resource");
      inner_string.push_str(&original_name);
      inner_string.push_str("(");
      inner_string.push_str(&fhir_name);
      inner_string.push_str("),\n\n");
    }
    inner_string.push_str("}\n")
  } else {
    inner_string.push_str("\n\n");

    if let Some(description) = &definition.description {
      inner_string.push_str(&indent(&fill(&description, 80), "/// "));
    }
    inner_string.push_str("#[derive(Debug, Serialize, Deserialize)]\n");
    inner_string.push_str("#[serde(rename_all = \"camelCase\")]\n");
    inner_string.push_str("pub struct ");
    inner_string.push_str(name);
    inner_string.push_str(" {\n");

    let properties = match &definition.properties {
      Some(properties) => properties,
      None => return String::new(),
    };

    let required_property_names: HashSet<String> = match &definition.required {
      Some(strings) => HashSet::from_iter(strings.iter().cloned()),
      None => HashSet::new(),
    };

    for (property_name, property) in properties {
      let sanitized_name = sanitize_property_name(property_name, &property_replacement_map);
      let needs_rename = &sanitized_name != property_name || sanitized_name.starts_with("_");
      let required = required_property_names.contains(&property_name[..]);
      match property {
        Property::Reference {
          description,
          fhir_ref,
        } => {
          let (class_name, import) = class_name_from_fhir_ref(
            &fhir_ref,
            reference_to_class_name_map,
            builtin_type_to_class_map,
          );

          if let Some(import) = import {
            pending_imports.insert(import);
          }

          inner_string.push_str(&indent(
            &fill(&sanitize_description(description), 80),
            "  /// ",
          ));
          if needs_rename {
            inner_string.push_str("  #[serde(rename = \"");
            inner_string.push_str(property_name);
            inner_string.push_str("\")]\n");
          }
          inner_string.push_str("  ");
          inner_string.push_str(&sanitized_name);
          inner_string.push_str(": ");
          if !required {
            inner_string.push_str("Option<");
          }
          inner_string.push_str(&class_name);
          if !required {
            inner_string.push_str(">");
          }
          inner_string.push_str(",\n\n");
        }
        Property::PatternedTyped {
          description,
          pattern: _,
          fhir_type,
        } => {
          let (class_name, import) = class_name_from_fhir_type(
            &fhir_type,
            reference_to_class_name_map,
            builtin_type_to_class_map,
          );

          if let Some(import) = import {
            pending_imports.insert(import);
          }

          inner_string.push_str(&indent(
            &fill(&sanitize_description(description), 80),
            "  /// ",
          ));
          if needs_rename {
            inner_string.push_str("  #[serde(rename = \"");
            inner_string.push_str(property_name);
            inner_string.push_str("\")]\n");
          }
          inner_string.push_str("  ");
          inner_string.push_str(&sanitized_name);
          inner_string.push_str(": ");
          if !required {
            inner_string.push_str("Option<");
          }
          inner_string.push_str(&class_name);
          if !required {
            inner_string.push_str(">");
          }
          inner_string.push_str(",\n\n");
        }
        Property::Array {
          description,
          items,
          fhir_type,
        } => {
          assert_eq!(fhir_type, "array");
          if let Some(Item::Ref(item_ref)) = items.get("$ref") {
            let (class_name, import) = class_name_from_fhir_ref(
              &item_ref,
              reference_to_class_name_map,
              builtin_type_to_class_map,
            );
            if let Some(import) = import {
              pending_imports.insert(import);
            }
            inner_string.push_str(&indent(
              &fill(&sanitize_description(description), 80),
              "  /// ",
            ));
            if needs_rename {
              inner_string.push_str("  #[serde(rename = \"");
              inner_string.push_str(property_name);
              inner_string.push_str("\")]\n");
            }
            inner_string.push_str("  ");
            inner_string.push_str(&sanitized_name);
            inner_string.push_str(": ");
            if !required {
              inner_string.push_str("Option<");
            }
            inner_string.push_str("Vec<");
            inner_string.push_str(&class_name);
            if !required {
              inner_string.push_str(">");
            }
            inner_string.push_str(">,\n\n");
          } else if let Some(Item::Enum(item_enum)) = items.get("$ref") {
            let enum_name = format!("{}{}", name, property_name.to_class_case());
            pending_enums.push((enum_name.clone(), item_enum));

            inner_string.push_str(&indent(
              &fill(&sanitize_description(description), 80),
              "  /// ",
            ));
            if needs_rename {
              inner_string.push_str("  #[serde(rename = \"");
              inner_string.push_str(property_name);
              inner_string.push_str("\")]\n");
            }
            inner_string.push_str("  ");
            inner_string.push_str(&sanitized_name);
            inner_string.push_str(": ");
            if !required {
              inner_string.push_str("Option<");
            }
            inner_string.push_str("Vec<");
            inner_string.push_str(&enum_name);
            if !required {
              inner_string.push_str(">");
            }
            inner_string.push_str(">,\n\n");
          }
        }
        Property::Typed {
          description,
          fhir_type,
        } => {
          let (class_name, import) = class_name_from_fhir_type(
            &fhir_type,
            reference_to_class_name_map,
            builtin_type_to_class_map,
          );
          if let Some(import) = import {
            pending_imports.insert(import);
          }
          inner_string.push_str(&indent(&fill(description, 80), "  /// "));
          if needs_rename {
            inner_string.push_str("  #[serde(rename = \"");
            inner_string.push_str(property_name);
            inner_string.push_str("\")]\n");
          }
          inner_string.push_str("  ");
          inner_string.push_str(&sanitized_name);
          inner_string.push_str(": ");
          if !required {
            inner_string.push_str("Option<");
          }
          inner_string.push_str(&class_name);
          if !required {
            inner_string.push_str(">");
          }
          inner_string.push_str(",\n\n");
        }
        Property::Enum {
          description,
          fhir_enum,
        } => {
          let enum_name = format!("{}{}", name, property_name.to_class_case());
          pending_enums.push((enum_name.clone(), fhir_enum));

          inner_string.push_str(&indent(&fill(description, 80), "  /// "));
          if needs_rename {
            inner_string.push_str("  #[serde(rename = \"");
            inner_string.push_str(property_name);
            inner_string.push_str("\")]\n");
          }
          inner_string.push_str("  ");
          inner_string.push_str(&sanitized_name);
          inner_string.push_str(": ");
          if !required {
            inner_string.push_str("Option<");
          }
          inner_string.push_str(&enum_name);
          if !required {
            inner_string.push_str(">");
          }
          inner_string.push_str(",\n\n");
        }
        Property::Const {
          description: _,
          fhir_const: _,
        } => {}
      }
    }

    inner_string.push_str("}\n");
  }

  for (enum_name, values) in pending_enums {
    inner_string.push_str("\n#[derive(Debug, Serialize, Deserialize)]\n");
    inner_string.push_str("pub enum ");
    inner_string.push_str(&enum_name);
    inner_string.push_str(" {\n");
    for value in values {
      inner_string.push_str("  #[serde(rename = \"");
      inner_string.push_str(&value);
      inner_string.push_str("\")]\n");
      inner_string.push_str("  ");
      inner_string.push_str(&sanitize_name(value, property_replacement_map).to_class_case());
      inner_string.push_str(",\n\n");
    }
    inner_string.push_str("}\n")
  }

  for import in pending_imports {
    if !import.ends_with(&format!("::{}", name)) {
      string.push_str(&format!("use {};\n", import));
    }
  }
  string.push_str(&inner_string);

  return string;
}

fn class_name_from_fhir_type(
  fhir_type: &str,
  reference_to_class_name_map: &HashMap<String, String>,
  builtin_type_to_class_map: &HashMap<&str, (&str, Option<&str>)>,
) -> (String, Option<String>) {
  if let Some(builtin) = builtin_type_to_class_map.get(&fhir_type) {
    return (
      builtin.0.to_string(),
      match builtin.1 {
        Some(import) => Some(import.to_string()),
        None => None,
      },
    );
  }
  if let None = reference_to_class_name_map.get(fhir_type) {
    // println!("Missing non-builtin class: {}", fhir_type);
  }

  return (
    fhir_type.to_string(),
    Some(format!("crate::model::{}::{}", fhir_type, fhir_type)),
  );
}

fn class_name_from_fhir_ref(
  fhir_ref: &str,
  reference_to_class_name_map: &HashMap<String, String>,
  builtin_type_to_class_map: &HashMap<&str, (&str, Option<&str>)>,
) -> (String, Option<String>) {
  let extracted = extract_type_from_ref(&fhir_ref);
  return class_name_from_fhir_type(
    extracted,
    reference_to_class_name_map,
    builtin_type_to_class_map,
  );
}

fn snake_case_string(string: &str) -> String {
  if string.len() <= 1 {
    return string.to_string();
  }
  // For some reason the inflection library can't snakecase strings
  // with a leading _, so we manually strip it, and snakecase the rest.
  if string.chars().next().unwrap() == '_' {
    let substr = &string[1..];
    return format!("_{}", substr.to_snake_case());
  }
  return string.to_snake_case();
}

fn sanitize_name(name: &str, property_replacement_map: &HashMap<&str, &str>) -> String {
  if let Some(replacement) = property_replacement_map.get(name) {
    return replacement.to_string();
  }
  if name.chars().all(is_numeric_char) || name.chars().next().unwrap().is_numeric() {
    return strip_symbols(&format!("FHIR_{}", name));
  }
  return strip_symbols(name);
}

fn strip_symbols(string: &str) -> String {
  string.replace("/", "_").replace(".", "_")
}

fn is_numeric_char(c: char) -> bool {
  c.is_numeric() || c == '.'
}

fn sanitize_property_name(name: &str, property_replacement_map: &HashMap<&str, &str>) -> String {
  return snake_case_string(&sanitize_name(name, property_replacement_map));
}

fn sanitize_description(string: &str) -> String {
  string.replace("\n", "  ").replace("\r", "  ")
}

fn extract_type_from_ref(fhir_ref: &str) -> &str {
  let split: Vec<&str> = fhir_ref.split("/").collect();
  return split[2];
}
