#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::StructureMap_Source::StructureMap_Source;
use crate::model::Element::Element;
use crate::model::StructureMap_Target::StructureMap_Target;
use crate::model::StructureMap_Dependent::StructureMap_Dependent;


/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureMap_Rule {
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

  /// Name of the rule for internal references.
  name: String,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for name
  _name: Element,

  /// Extensions for documentation
  _documentation: Element,

  /// Which other rules to apply in the context of this rule.
  dependent: Vec<StructureMap_Dependent>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Documentation for this instance of data.
  documentation: String,

  /// Source inputs to the mapping.
  source: Vec<StructureMap_Source>,

  /// Rules contained in this rule.
  rule: Vec<StructureMap_Rule>,

  /// Content to create because of this mapping rule.
  target: Vec<StructureMap_Target>,

}
