#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureMap_Input {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for mode
  #[serde(rename = "_mode")]
  _mode: Option<Element>,

  /// Mode for this instance of data.
  mode: Option<StructureMap_InputMode>,

  /// Documentation for this instance of data.
  documentation: Option<String>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// Extensions for type
  #[serde(rename = "_type")]
  _type: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.    Modifier extensions
  /// SHALL NOT change the meaning of any elements on Resource or DomainResource
  /// (including cannot change the meaning of modifierExtension itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Option<Vec<Box<Extension>>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for documentation
  #[serde(rename = "_documentation")]
  _documentation: Option<Element>,

  /// Name for this instance of data.
  name: Option<String>,

  /// Type for this instance of data.
  #[serde(rename = "type")]
  fhir_type: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum StructureMap_InputMode {
  #[serde(rename = "source")]
  Source,

  #[serde(rename = "target")]
  Target,

}
