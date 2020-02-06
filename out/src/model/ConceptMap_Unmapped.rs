#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;


/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMap_Unmapped {
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

  /// Extensions for display
  _display: Element,

  /// The canonical reference to an additional ConceptMap resource instance to use for
  /// mapping if this ConceptMap resource contains no matching mapping for the source
  /// concept.
  url: String,

  /// The display for the code. The display is only provided to help editors when
  /// editing the concept map.
  display: String,

  /// The fixed code to use when the mode = 'fixed'  - all unmapped codes are mapped
  /// to a single fixed code.
  code: String,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for code
  _code: Element,

  /// Defines which action to take if there is no match for the source concept in the
  /// target system designated for the group. One of 3 actions are possible: use the
  /// unmapped code (this is useful when doing a mapping between versions, and only a
  /// few codes have changed), use a fixed code (a default code), or alternatively, a
  /// reference to a different concept map can be provided (by canonical URL).
  mode: ConceptMap_UnmappedMode,

  /// Extensions for mode
  _mode: Element,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConceptMap_UnmappedMode {
  #[serde(rename = "provided")]
  Provided,

  #[serde(rename = "fixed")]
  Fixed,

  #[serde(rename = "other-map")]
  OtherMap,

}
