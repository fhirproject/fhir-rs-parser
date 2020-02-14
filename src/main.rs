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
    "int" => ("i64", None),
    "integer" => ("i64", None),
    "number" => ("f64", None), // todo
    "uri" => ("&str", None),
    "url" => ("&str", None),
    "markdown" => ("&str", None),
    "xhtml" => ("&str", None),
    "decimal" => ("f64", None),
    "positiveInt" => ("i64", None),
    "canonical" => ("&str", None),
    "float" => ("f64", None),
    "string" => ("&str", None),
    "code" => ("&str", None),
    "boolean" => ("bool", None),
    "unsignedInt" => ("u64", None),
    "id" => ("&str", None),
    "time" => ("&str", None), // todo
    "time" => ("&str", None), // todo
    "instant" => ("&str", None), // todo
    "date" => ("&str", None), // todo
    "base64Binary" => ("&str", None),
    "dateTime" => ("&str", None), // todo

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
    let contents = generate_trait(
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

fn generate_trait(
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

  let mut inner_string = String::new();
  inner_string.push_str("use serde_json::value::Value;\n\n");

  let mut validation_string = String::new();

  if let Some(one_of) = &definition.one_of {
    inner_string.push_str("\n#[derive(Debug)]\n");
    inner_string.push_str("pub struct ");
    inner_string.push_str(name);
    inner_string.push_str("<'a> {\n");
    inner_string.push_str("  pub value: &'a Value,\n");
    inner_string.push_str("}\n\n");

    let mut impl_string = String::new();
    impl_string.push_str("impl ");
    impl_string.push_str(name);
    impl_string.push_str("<'_> {\n");
    impl_string.push_str("  pub fn resource(&self) -> Option<");
    impl_string.push_str(name);
    impl_string.push_str("Enum> {\n");
    impl_string.push_str("    let fhir_type = self.value[\"resourceType\"].as_str().unwrap();\n");
    impl_string.push_str("    match fhir_type {\n");

    let mut validation_string = String::new();
    validation_string.push_str("  pub fn validate(&self) -> bool {\n");
    validation_string.push_str("    if let Some(resource) = self.resource() {\n");
    validation_string.push_str("      match resource {\n");

    inner_string.push_str("#[derive(Debug)]\n");
    inner_string.push_str("pub enum ");
    inner_string.push_str(name);
    inner_string.push_str("Enum<'a> {\n");
    for hash_map in one_of {
      let fhir_ref = &hash_map["$ref"];
      let original_name = extract_type_from_ref(&fhir_ref);
      let type_definition = type_definition_from_fhir_ref(
        &fhir_ref,
        &reference_to_class_name_map,
        builtin_type_to_class_map,
      );
      if let Some(import) = type_definition.import {
        pending_imports.insert(import);
      }
      inner_string.push_str("  Resource");
      inner_string.push_str(&original_name);
      inner_string.push_str("(");
      inner_string.push_str(&type_definition.name);
      inner_string.push_str("<'a>),\n");
      impl_string.push_str("      \"");
      impl_string.push_str(&original_name);
      impl_string.push_str("\" => Some(");
      impl_string.push_str(name);
      impl_string.push_str("Enum::Resource");
      impl_string.push_str(&type_definition.name);
      impl_string.push_str("(");
      impl_string.push_str(&type_definition.name);
      impl_string.push_str(" { value: self.value })),\n");

      validation_string.push_str("        ");
      validation_string.push_str(name);
      validation_string.push_str("Enum::Resource");
      validation_string.push_str(&type_definition.name);
      validation_string.push_str("(val) => { return val.validate(); },\n");
    }
    validation_string.push_str("      }\n");
    validation_string.push_str("    }\n");
    validation_string.push_str("    return false;\n");
    validation_string.push_str("  }\n");
    impl_string.push_str("      _ => None,\n");
    impl_string.push_str("    }\n");
    impl_string.push_str("  }\n\n");
    impl_string.push_str(&validation_string);
    impl_string.push_str("}\n\n");
    inner_string.push_str("}\n\n");
    inner_string.push_str(&impl_string);
  } else {
    inner_string.push_str("\n\n");

    if let Some(description) = &definition.description {
      inner_string.push_str(&indent(&fill(&description, 80), "/// "));
    }
    inner_string.push_str("\n#[derive(Debug)]\n");
    inner_string.push_str("pub struct ");
    inner_string.push_str(name);
    inner_string.push_str("<'a> {\n");
    inner_string.push_str("  pub value: &'a Value,\n}\n\n");

    inner_string.push_str("impl ");
    inner_string.push_str(name);
    inner_string.push_str("<'_> {\n");

    validation_string.push_str("  pub fn validate(&self) -> bool {\n");

    let properties = match &definition.properties {
      Some(properties) => properties,
      None => return String::new(),
    };

    let required_property_names: HashSet<String> = match &definition.required {
      Some(strings) => HashSet::from_iter(strings.iter().cloned()),
      None => HashSet::new(),
    };

    for (property_name, property) in properties {
      let required = required_property_names.contains(&property_name[..]);
      match property {
        Property::Reference {
          description,
          fhir_ref,
        } => {
          let type_definition = type_definition_from_fhir_ref(
            &fhir_ref,
            reference_to_class_name_map,
            builtin_type_to_class_map,
          );

          if let Some(import) = &type_definition.import {
            pending_imports.insert(import.clone());
          }

          write_property(
            &mut inner_string,
            &mut validation_string,
            &property_name,
            &type_definition,
            &description,
            None,
            required,
            false,
            "  ",
            &property_replacement_map,
          );
        }
        Property::PatternedTyped {
          description,
          pattern,
          fhir_type,
        } => {
          let type_definition = type_definition_from_fhir_type(
            &fhir_type,
            reference_to_class_name_map,
            builtin_type_to_class_map,
          );

          if let Some(import) = &type_definition.import {
            pending_imports.insert(import.clone());
          }

          write_property(
            &mut inner_string,
            &mut validation_string,
            &property_name,
            &type_definition,
            &description,
            Some(pattern.to_string()),
            required,
            false,
            "  ",
            &property_replacement_map,
          );
        }
        Property::Array {
          description,
          items,
          fhir_type,
        } => {
          assert_eq!(fhir_type, "array");
          if let Some(Item::Ref(item_ref)) = items.get("$ref") {
            let type_definition = type_definition_from_fhir_ref(
              &item_ref,
              reference_to_class_name_map,
              builtin_type_to_class_map,
            );
            if let Some(import) = &type_definition.import {
              pending_imports.insert(import.clone());
            }
            write_property(
              &mut inner_string,
              &mut validation_string,
              &property_name,
              &type_definition,
              &description,
              None,
              required,
              true,
              "  ",
              &property_replacement_map,
            );
          } else if let Some(Item::Enum(item_enum)) = items.get("$ref") {
            let type_definition = TypeDefinition {
              name: format!("{}{}", name, property_name.to_class_case()),
              builtin: false,
              string_enum: true,
              import: None,
            };
            pending_enums.push((type_definition.name.clone(), item_enum));
            write_property(
              &mut inner_string,
              &mut validation_string,
              &property_name,
              &type_definition,
              &description,
              None,
              required,
              false,
              "  ",
              &property_replacement_map,
            );
          }
        }
        Property::Typed {
          description,
          fhir_type,
        } => {
          let type_definition = type_definition_from_fhir_type(
            &fhir_type,
            reference_to_class_name_map,
            builtin_type_to_class_map,
          );
          if let Some(import) = &type_definition.import {
            pending_imports.insert(import.clone());
          }
          write_property(
            &mut inner_string,
            &mut validation_string,
            &property_name,
            &type_definition,
            &description,
            None,
            required,
            false,
            "  ",
            &property_replacement_map,
          );
        }
        Property::Enum {
          description,
          fhir_enum,
        } => {
          let type_definition = TypeDefinition {
            name: format!("{}{}", name, property_name.to_class_case()),
            builtin: false,
            string_enum: true,
            import: None,
          };
          pending_enums.push((type_definition.name.clone(), fhir_enum));
          write_property(
            &mut inner_string,
            &mut validation_string,
            &property_name,
            &type_definition,
            &description,
            None,
            required,
            false,
            "  ",
            &property_replacement_map,
          );
        }
        Property::Const {
          description: _,
          fhir_const: _,
        } => {}
      }
    }

    validation_string.push_str("    return true;\n  }\n\n");

    inner_string.push_str(&validation_string);

    inner_string.push_str("}\n");
  }

  for (enum_name, values) in pending_enums {
    inner_string.push_str("\n#[derive(Debug)]\n");
    inner_string.push_str("pub enum ");
    inner_string.push_str(&enum_name);
    inner_string.push_str(" {\n");
    let mut enum_impl_string = String::new();
    enum_impl_string.push_str("impl ");
    enum_impl_string.push_str(&enum_name);
    enum_impl_string.push_str(" {\n");
    enum_impl_string.push_str("    pub fn from_string(string: &str) -> Option<");
    enum_impl_string.push_str(&enum_name);
    enum_impl_string.push_str("> {\n");
    enum_impl_string.push_str("      match string {\n");
    for value in values {
      let sanitized_name = sanitize_name(value, property_replacement_map).to_class_case();
      inner_string.push_str("  ");
      inner_string.push_str(&sanitized_name);
      inner_string.push_str(",\n");
      enum_impl_string.push_str("        \"");
      enum_impl_string.push_str(&value);
      enum_impl_string.push_str("\" => Some(");
      enum_impl_string.push_str(&enum_name);
      enum_impl_string.push_str("::");
      enum_impl_string.push_str(&sanitized_name);
      enum_impl_string.push_str("),\n");
    }
    inner_string.push_str("}\n\n");
    enum_impl_string.push_str("        _ => None,\n");
    enum_impl_string.push_str("    }\n  }\n}\n\n");

    inner_string.push_str(&enum_impl_string);
  }

  for import in pending_imports {
    if !import.ends_with(&format!("::{}", name)) {
      string.push_str(&format!("use {};\n", import));
    }
  }
  string.push_str(&inner_string);

  return string;
}

fn write_property(
  inner_string: &mut String,
  validation_string: &mut String,
  property_name: &str,
  type_definition: &TypeDefinition,
  description: &str,
  _pattern: Option<String>,
  required: bool,
  array: bool,
  indentation_level: &str,
  property_replacement_map: &HashMap<&str, &str>,
) {
  let sanitized_name = sanitize_property_name(property_name, &property_replacement_map);

  let mut type_name = String::new();
  {
    if !required {
      type_name.push_str("Option<");
    }
    if array {
      type_name.push_str("Vec<")
    }
    type_name.push_str(&type_definition.name);
    if array {
      type_name.push_str(">")
    }
    if !required {
      type_name.push_str(">");
    }
  }

  if required {
    validation_string.push_str("    let _ = self.");
    validation_string.push_str(&sanitized_name);
    validation_string.push_str("()");
    if array {
      if !type_definition.builtin {
        validation_string.push_str(".into_iter().for_each(|e| { e.validate(); })")
      } else {
        validation_string.push_str(".into_iter().for_each(|_e| {})")
      }
    } else if !type_definition.builtin && !type_definition.string_enum {
      validation_string.push_str(".validate()");
    }
    validation_string.push_str(";\n");
  } else {
    validation_string.push_str("    if let Some(_val) = self.");
    validation_string.push_str(&sanitized_name);
    validation_string.push_str("() {\n");
    if array {
      if !type_definition.builtin {
        validation_string.push_str("      _val.into_iter().for_each(|e| { e.validate(); });\n");
      } else {
        validation_string.push_str("      _val.into_iter().for_each(|_e| {});\n");
      }
    } else if !type_definition.builtin && !type_definition.string_enum {
      validation_string.push_str("      _val.validate();\n");
    }
    validation_string.push_str("    }\n");
  }

  let mut getter = String::new();
  // getter
  getter.push_str("  pub fn ");
  getter.push_str(&sanitized_name);
  getter.push_str("(&self) -> ");
  getter.push_str(&type_name);

  // generated impl
  inner_string.push_str(&indent(
    &fill(&sanitize_description(description), 80),
    &format!("{}{}", indentation_level, "/// "),
  ));
  inner_string.push_str(&getter);
  inner_string.push_str(" {\n");
  if array {
    if type_definition.builtin {
      if required {
        if type_definition.name == "&str" {
          inner_string.push_str("    self.value.get(\"");
          inner_string.push_str(&property_name);
          inner_string.push_str("\").unwrap().as_array().unwrap().into_iter().map(|e| e.as_str().unwrap()).collect::<Vec<_>>()\n");
        } else {
          inner_string.push_str("    self.value.get(\"");
          inner_string.push_str(&property_name);
          inner_string.push_str("\").unwrap().as_array().unwrap().into_iter().map(|e| e.as_");
          inner_string.push_str(&type_definition.name);
          inner_string.push_str("().unwrap()).collect::<Vec<_>>()\n");
        }
      } else {
        if type_definition.name == "&str" {
          inner_string.push_str("    if let Some(Value::Array(val)) = self.value.get(\"");
          inner_string.push_str(&property_name);
          inner_string
            .push_str("\") {\n      return Some(val.into_iter().map(|e| e.as_str().unwrap()).collect::<Vec<_>>());\n    }\n    return None;\n");
        } else {
          inner_string.push_str("    if let Some(Value::Array(val)) = self.value.get(\"");
          inner_string.push_str(&property_name);
          inner_string.push_str("\") {\n      return Some(val.into_iter().map(|e| e.as_");
          inner_string.push_str(&type_definition.name);
          inner_string.push_str("().unwrap()).collect::<Vec<_>>());\n    }\n    return None;\n");
        }
      }
    } else {
      if required {
        if type_definition.string_enum {
          inner_string.push_str("    self.value.get(\"");
          inner_string.push_str(&property_name);
          inner_string.push_str("\").unwrap().as_array().unwrap().into_iter().map(|e| ");
          inner_string.push_str(&type_definition.name);
          inner_string.push_str("::from_string(&e).unwrap()).collect::<Vec<_>>()\n");
        } else {
          inner_string.push_str("    self.value.get(\"");
          inner_string.push_str(&property_name);
          inner_string.push_str("\").unwrap().as_array().unwrap().into_iter().map(|e| ");
          inner_string.push_str(&type_definition.name);
          inner_string.push_str(" { value: e }).collect::<Vec<_>>()\n");
        }
      } else {
        if type_definition.string_enum {
          inner_string.push_str("    if let Some(Value::Array(val)) = self.value.get(\"");
          inner_string.push_str(&property_name);
          inner_string.push_str("\") {\n      return Some(val.into_iter().map(|e| ");
          inner_string.push_str(&type_definition.name);
          inner_string.push_str(
            "::from_string(&e).unwrap()).collect::<Vec<_>>());\n    }\n    return None;\n",
          );
        } else {
          inner_string.push_str("    if let Some(Value::Array(val)) = self.value.get(\"");
          inner_string.push_str(&property_name);
          inner_string.push_str("\") {\n      return Some(val.into_iter().map(|e| ");
          inner_string.push_str(&type_definition.name);
          inner_string.push_str(" { value: e }).collect::<Vec<_>>());\n    }\n    return None;\n");
        }
      }
    }
  // end arrays
  } else if type_definition.builtin {
    if required {
      if type_definition.name == "&str" {
        inner_string.push_str("    self.value.get(\"");
        inner_string.push_str(&property_name);
        inner_string.push_str("\").unwrap().as_str().unwrap()\n");
      } else {
        inner_string.push_str("    self.value.get(\"");
        inner_string.push_str(&property_name);
        inner_string.push_str("\").unwrap().as_");
        inner_string.push_str(&type_definition.name);
        inner_string.push_str("().unwrap()\n");
      }
    } else {
      if type_definition.name == "&str" {
        inner_string.push_str("    if let Some(Value::String(string)) = self.value.get(\"");
        inner_string.push_str(&property_name);
        inner_string.push_str("\") {\n      return Some(string);\n    }\n    return None;\n");
      } else {
        inner_string.push_str("    if let Some(val) = self.value.get(\"");
        inner_string.push_str(&property_name);
        inner_string.push_str("\") {\n      return Some(val.as_");
        inner_string.push_str(&type_definition.name);
        inner_string.push_str("().unwrap());\n    }\n    return None;\n");
      }
    }
  } else {
    if required {
      if type_definition.string_enum {
        inner_string.push_str("    ");
        inner_string.push_str(&type_definition.name);
        inner_string.push_str("::from_string(");
        inner_string.push_str("&self.value[\"");
        inner_string.push_str(&property_name);
        inner_string.push_str("\"].as_str().unwrap()).unwrap()\n");
      } else {
        inner_string.push_str("    ");
        inner_string.push_str(&type_definition.name);
        inner_string.push_str(" {\n");
        inner_string.push_str("      value: &self.value[\"");
        inner_string.push_str(&property_name);
        inner_string.push_str("\"],\n    }\n");
      }
    } else {
      if type_definition.string_enum {
        inner_string.push_str("    if let Some(Value::String(val)) = self.value.get(\"");
        inner_string.push_str(&property_name);
        inner_string.push_str("\") {\n      return Some(");
        inner_string.push_str(&type_definition.name);
        inner_string.push_str("::from_string(&val).unwrap());\n    }\n    return None;\n");
      } else {
        inner_string.push_str("    if let Some(val) = self.value.get(\"");
        inner_string.push_str(&property_name);
        inner_string.push_str("\") {\n      return Some(");
        inner_string.push_str(&type_definition.name);
        inner_string.push_str(" { value: val });\n    }\n    return None;\n");
      }
    }
  }
  // todo arrays!
  inner_string.push_str("  }\n\n");
}

struct TypeDefinition {
  name: String,
  builtin: bool,
  string_enum: bool,
  import: Option<String>,
}

fn type_definition_from_fhir_type(
  fhir_type: &str,
  reference_to_class_name_map: &HashMap<String, String>,
  builtin_type_to_class_map: &HashMap<&str, (&str, Option<&str>)>,
) -> TypeDefinition {
  if let Some(builtin) = builtin_type_to_class_map.get(&fhir_type) {
    return TypeDefinition {
      name: builtin.0.to_string(),
      builtin: true,
      string_enum: false,
      import: match builtin.1 {
        Some(import) => Some(import.to_string()),
        None => None,
      },
    };
  }
  if let None = reference_to_class_name_map.get(fhir_type) {
    // println!("Missing non-builtin class: {}", fhir_type);
  }

  return TypeDefinition {
    name: fhir_type.to_string(),
    builtin: false,
    string_enum: false,
    import: Some(format!("crate::model::{}::{}", fhir_type, fhir_type)),
  };
}

fn type_definition_from_fhir_ref(
  fhir_ref: &str,
  reference_to_class_name_map: &HashMap<String, String>,
  builtin_type_to_class_map: &HashMap<&str, (&str, Option<&str>)>,
) -> TypeDefinition {
  let extracted = extract_type_from_ref(&fhir_ref);
  return type_definition_from_fhir_type(
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
