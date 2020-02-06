#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::StructureMap_Target::StructureMap_Target;
use crate::model::StructureMap_Dependent::StructureMap_Dependent;
use crate::model::Extension::Extension;
use crate::model::StructureMap_Source::StructureMap_Source;
use crate::model::Element::Element;


/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureMap_Rule {
  /// Rules contained in this rule.
  rule: Option<Vec<StructureMap_Rule>>,

  /// Documentation for this instance of data.
  documentation: Option<String>,

  /// Content to create because of this mapping rule.
  target: Option<Vec<StructureMap_Target>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Which other rules to apply in the context of this rule.
  dependent: Option<Vec<StructureMap_Dependent>>,

  /// Extensions for documentation
  #[serde(rename = "_documentation")]
  _documentation: Option<Element>,

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

  /// Source inputs to the mapping.
  source: Vec<Box<StructureMap_Source>>,

  /// Name of the rule for internal references.
  name: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

}
