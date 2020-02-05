use inflector::Inflector;
use maplit::hashmap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
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
}

fn main() {
  fs::remove_dir_all("out/src").unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
  });
  fs::create_dir("out/src").unwrap_or_else(|why| {
    println!("! {:?}", why.kind());
  });

  let schema_contents =
    fs::read_to_string("src/fhir.schema.json").expect("Something went wrong reading the file");
  let fhir_schema: Schema = serde_json::from_str(&schema_contents).unwrap();

  let mut reference_to_class_name_map: HashMap<String, String> = HashMap::new();

  for (class_name, reference) in fhir_schema.discriminator.mapping {
    println!("ref: {}, class_name: {}", &reference, &class_name);
    reference_to_class_name_map.insert(reference, class_name);
  }

  let builtin_type_to_class_map = hashmap! {
    "int" => "i32",
    "uri" => "String",
    "float" => "f32",
    "string" => "String",
    "code" => "String",
    "boolean" => "bool",
  };

  for (definition_name, definition) in fhir_schema.definitions {
    let contents = generate_class(
      &definition_name,
      &definition,
      &reference_to_class_name_map,
      &builtin_type_to_class_map,
    );

    if contents.len() == 0 {
      continue;
    }

    let path_string = format!("out/src/{}.rs", definition_name);
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
  }
}

fn generate_class(
  name: &str,
  definition: &Definition,
  reference_to_class_name_map: &HashMap<String, String>,
  builtin_type_to_class_map: &HashMap<&str, &str>,
) -> String {
  let mut string = String::new();

  let mut pending_enums: Vec<(String, &Vec<String>)> = Vec::new();

  string.push_str("use serde::{Deserialize, Serialize};");
  string.push_str("\n\n");

  if let Some(description) = &definition.description {
    string.push_str(&indent(&fill(&description, 80), "/// "));
  }
  string.push_str("#[derive(Debug, Serialize, Deserialize)]\n");
  string.push_str("#[serde(rename_all = \"camelCase\")]\n");
  string.push_str("struct ");
  string.push_str(name);
  string.push_str(" {\n");

  let properties = match &definition.properties {
    Some(properties) => properties,
    None => return String::new(),
  };

  for (property_name, property) in properties {
    let sanitized_name = sanitize_property_name(property_name);
    let needs_rename = &sanitized_name != property_name;
    match property {
      Property::Reference {
        description,
        fhir_ref,
      } => {
        string.push_str(&indent(&fill(description, 80), "  /// "));
        if needs_rename {
          string.push_str("  #[serde(rename = \"");
          string.push_str(property_name);
          string.push_str("\")]\n");
        }
        string.push_str("  ");
        string.push_str(&sanitized_name);
        string.push_str(": ");
        string.push_str(&class_name_from_fhir_ref(
          &fhir_ref,
          reference_to_class_name_map,
          builtin_type_to_class_map,
        ));
        string.push_str(",\n\n");
      }
      Property::PatternedTyped {
        description,
        pattern: _,
        fhir_type,
      } => {
        string.push_str(&indent(&fill(description, 80), "  /// "));
        if needs_rename {
          string.push_str("  #[serde(rename = \"");
          string.push_str(property_name);
          string.push_str("\")]\n");
        }
        string.push_str("  ");
        string.push_str(&sanitized_name);
        string.push_str(": ");
        string.push_str(&class_name_from_fhir_type(
          &fhir_type,
          reference_to_class_name_map,
          builtin_type_to_class_map,
        ));
        string.push_str(",\n\n");
      }
      Property::Array {
        description,
        items,
        fhir_type,
      } => {
        assert_eq!(fhir_type, "array");
        if let Some(Item::Ref(item_ref)) = items.get("$ref") {
          string.push_str(&indent(&fill(description, 80), "  /// "));
          if needs_rename {
            string.push_str("  #[serde(rename = \"");
            string.push_str(property_name);
            string.push_str("\")]\n");
          }
          string.push_str("  ");
          string.push_str(&sanitized_name);
          string.push_str(": Vec<");
          string.push_str(&class_name_from_fhir_ref(
            &item_ref,
            reference_to_class_name_map,
            builtin_type_to_class_map,
          ));
          string.push_str(">,\n\n");
        } else if let Some(Item::Enum(item_enum)) = items.get("$ref") {
          let enum_name = format!("{}{}", name, property_name.to_class_case());
          pending_enums.push((enum_name.clone(), item_enum));

          string.push_str(&indent(&fill(description, 80), "  /// "));
          if needs_rename {
            string.push_str("  #[serde(rename = \"");
            string.push_str(property_name);
            string.push_str("\")]\n");
          }
          string.push_str("  ");
          string.push_str(&sanitized_name);
          string.push_str(": Vec<");
          string.push_str(&enum_name);
          string.push_str(">,\n\n");
        }
      }
      Property::Typed {
        description,
        fhir_type,
      } => {
        string.push_str(&indent(&fill(description, 80), "  /// "));
        if needs_rename {
          string.push_str("  #[serde(rename = \"");
          string.push_str(property_name);
          string.push_str("\")]\n");
        }
        string.push_str("  ");
        string.push_str(&sanitized_name);
        string.push_str(": ");
        string.push_str(&class_name_from_fhir_type(
          &fhir_type,
          reference_to_class_name_map,
          builtin_type_to_class_map,
        ));
        string.push_str(",\n\n");
      }
      Property::Enum {
        description,
        fhir_enum,
      } => {
        let enum_name = format!("{}{}", name, property_name.to_class_case());
        pending_enums.push((enum_name.clone(), fhir_enum));

        string.push_str(&indent(&fill(description, 80), "  /// "));
        if needs_rename {
          string.push_str("  #[serde(rename = \"");
          string.push_str(property_name);
          string.push_str("\")]\n");
        }
        string.push_str("  ");
        string.push_str(&sanitized_name);
        string.push_str(": ");
        string.push_str(&enum_name);
        string.push_str(",\n\n");
      }
      Property::Const {
        description: _,
        fhir_const: _,
      } => {}
    }
  }

  string.push_str("}\n");

  for (enum_name, values) in pending_enums {
    string.push_str("\n#[derive(Debug, Serialize, Deserialize)]\n");
    string.push_str("enum ");
    string.push_str(&enum_name);
    string.push_str(" {\n");
    for value in values {
      string.push_str("  #[serde(rename = \"");
      string.push_str(&value);
      string.push_str("\")]\n");
      string.push_str("  ");
      string.push_str(&value.to_class_case());
      string.push_str(",\n\n");
    }
    string.push_str("}\n")
  }

  return string;
}

fn class_name_from_fhir_type(
  fhir_type: &str,
  _reference_to_class_name_map: &HashMap<String, String>,
  builtin_type_to_class_map: &HashMap<&str, &str>,
) -> String {
  if let Some(builtin) = builtin_type_to_class_map.get(&fhir_type) {
    return builtin.to_string();
  }
  if let None = _reference_to_class_name_map.get(fhir_type) {
    // println!("Missing non-builtin class: {}", fhir_type);
  }

  return fhir_type.to_string();
}

fn class_name_from_fhir_ref(
  fhir_ref: &str,
  reference_to_class_name_map: &HashMap<String, String>,
  builtin_type_to_class_map: &HashMap<&str, &str>,
) -> String {
  let extracted = extract_type_from_ref(&fhir_ref);
  return class_name_from_fhir_type(
    extracted,
    reference_to_class_name_map,
    builtin_type_to_class_map,
  );
}

fn sanitize_property_name(name: &str) -> String {
  if name.len() <= 1 {
    return name.to_string();
  }
  // For some reason the inflection library can't snakecase strings
  // with a leading _, so we manually strip it, and snakecase the rest.
  if name.chars().next().unwrap() == '_' {
    let substr = &name[1..];
    return format!("_{}", substr.to_snake_case());
  }
  return name.to_snake_case();
}

fn extract_type_from_ref(fhir_ref: &str) -> &str {
  let split: Vec<&str> = fhir_ref.split("/").collect();
  return split[2];
}
