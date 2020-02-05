use serde::{Deserialize, Serialize};

/// A formal computable definition of a graph of resources - that is, a coherent set
/// of resources that form a graph by following references. The Graph Definition
/// resource defines a set and makes rules about the set.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphDefinition_Link {
  /// Maximum occurrences for this link.
  max: String,

  /// Extensions for description
  _description: Element,

  /// Information about why this link is of interest in this graph definition.
  description: String,

  /// Potential target for the link.
  target: Vec<GraphDefinition_Target>,

  /// Extensions for max
  _max: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// A FHIR expression that identifies one of FHIR References to other resources.
  path: String,

  /// Which slice (if profiled).
  #[serde(rename = "sliceName")]
  slice_name: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for sliceName
  #[serde(rename = "_sliceName")]
  _slice_name: Element,

  /// Extensions for path
  _path: Element,

  /// Minimum occurrences for this link.
  min: integer,

  /// Extensions for min
  _min: Element,

}
