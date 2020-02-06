#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ConceptMap_Unmapped::ConceptMap_Unmapped;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::ConceptMap_Element::ConceptMap_Element;


/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMap_Group {
  /// Extensions for target
  _target: Element,

  /// The specific version of the code system, as determined by the code system
  /// authority.
  #[serde(rename = "targetVersion")]
  target_version: String,

  /// Extensions for sourceVersion
  #[serde(rename = "_sourceVersion")]
  _source_version: Element,

  /// Extensions for targetVersion
  #[serde(rename = "_targetVersion")]
  _target_version: Element,

  /// An absolute URI that identifies the target system that the concepts will be
  /// mapped to.
  target: String,

  /// What to do when there is no mapping for the source concept. "Unmapped" does not
  /// include codes that are unmatched, and the unmapped element is ignored in a code
  /// is specified to have equivalence = unmatched.
  unmapped: ConceptMap_Unmapped,

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

  /// Mappings for an individual concept in the source to one or more concepts in the
  /// target.
  element: Vec<ConceptMap_Element>,

  /// The specific version of the code system, as determined by the code system
  /// authority.
  #[serde(rename = "sourceVersion")]
  source_version: String,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for source
  _source: Element,

  /// An absolute URI that identifies the source system where the concepts to be
  /// mapped are defined.
  source: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

}
