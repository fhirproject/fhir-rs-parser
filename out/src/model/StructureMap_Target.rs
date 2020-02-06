#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::StructureMap_Parameter::StructureMap_Parameter;


/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureMap_Target {
  /// How to interpret the context.
  #[serde(rename = "contextType")]
  context_type: StructureMap_TargetContextType,

  /// Field to create in the context.
  element: String,

  /// Extensions for listMode
  #[serde(rename = "_listMode")]
  _list_mode: Vec<Element>,

  /// Extensions for element
  _element: Element,

  /// Extensions for variable
  _variable: Element,

  /// Extensions for listRuleId
  #[serde(rename = "_listRuleId")]
  _list_rule_id: Element,

  /// Extensions for transform
  _transform: Element,

  /// Extensions for context
  _context: Element,

  /// Parameters to the transform.
  parameter: Vec<StructureMap_Parameter>,

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
  modifier_extension: Vec<Extension>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for contextType
  #[serde(rename = "_contextType")]
  _context_type: Element,

  /// How the data is copied / created.
  transform: StructureMap_TargetTransform,

  /// Type or variable this rule applies to.
  context: String,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Internal rule reference for shared list items.
  #[serde(rename = "listRuleId")]
  list_rule_id: String,

  /// Named context for field, if desired, and a field is specified.
  variable: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum StructureMap_TargetContextType {
  #[serde(rename = "type")]
  FhirType,

  #[serde(rename = "variable")]
  Variable,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum StructureMap_TargetTransform {
  #[serde(rename = "create")]
  Create,

  #[serde(rename = "copy")]
  Copy,

  #[serde(rename = "truncate")]
  Truncate,

  #[serde(rename = "escape")]
  Escape,

  #[serde(rename = "cast")]
  Cast,

  #[serde(rename = "append")]
  Append,

  #[serde(rename = "translate")]
  Translate,

  #[serde(rename = "reference")]
  Reference,

  #[serde(rename = "dateOp")]
  DateOp,

  #[serde(rename = "uuid")]
  Uuid,

  #[serde(rename = "pointer")]
  Pointer,

  #[serde(rename = "evaluate")]
  Evaluate,

  #[serde(rename = "cc")]
  Cc,

  #[serde(rename = "c")]
  C,

  #[serde(rename = "qty")]
  Qty,

  #[serde(rename = "id")]
  Id,

  #[serde(rename = "cp")]
  Cp,

}
