#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::StructureMap_Input::StructureMap_Input;
use crate::model::Element::Element;
use crate::model::StructureMap_Rule::StructureMap_Rule;


/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureMap_Group {
  /// Extensions for typeMode
  #[serde(rename = "_typeMode")]
  _type_mode: Element,

  /// Extensions for name
  _name: Element,

  /// A unique name for the group for the convenience of human readers.
  name: String,

  /// Extensions for extends
  _extends: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

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

  /// Another group that this group adds rules to.
  extends: String,

  /// Additional supporting documentation that explains the purpose of the group and
  /// the types of mappings within it.
  documentation: String,

  /// Extensions for documentation
  _documentation: Element,

  /// Transform Rule from source to target.
  rule: Vec<StructureMap_Rule>,

  /// A name assigned to an instance of data. The instance must be provided when the
  /// mapping is invoked.
  input: Vec<StructureMap_Input>,

  /// If this is the default rule set to apply for the source type or this combination
  /// of types.
  #[serde(rename = "typeMode")]
  type_mode: StructureMap_GroupTypeMode,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum StructureMap_GroupTypeMode {
  #[serde(rename = "none")]
  None,

  #[serde(rename = "types")]
  Types,

  #[serde(rename = "type-and-types")]
  TypeAndTypes,

}
