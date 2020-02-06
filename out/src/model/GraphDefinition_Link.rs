#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::GraphDefinition_Target::GraphDefinition_Target;
use crate::model::Extension::Extension;
use crate::model::Element::Element;


/// A formal computable definition of a graph of resources - that is, a coherent set
/// of resources that form a graph by following references. The Graph Definition
/// resource defines a set and makes rules about the set.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphDefinition_Link {
  /// Minimum occurrences for this link.
  min: i32,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for description
  _description: Element,

  /// Extensions for min
  _min: Element,

  /// Extensions for max
  _max: Element,

  /// Which slice (if profiled).
  #[serde(rename = "sliceName")]
  slice_name: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// A FHIR expression that identifies one of FHIR References to other resources.
  path: String,

  /// Maximum occurrences for this link.
  max: String,

  /// Information about why this link is of interest in this graph definition.
  description: String,

  /// Potential target for the link.
  target: Vec<GraphDefinition_Target>,

  /// Extensions for sliceName
  #[serde(rename = "_sliceName")]
  _slice_name: Element,

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

  /// Extensions for path
  _path: Element,

}
