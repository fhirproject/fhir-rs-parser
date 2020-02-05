use serde::{Deserialize, Serialize};

/// A formal computable definition of an operation (on the RESTful interface) or a
/// named query (using the search interaction).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OperationDefinition_ReferencedFrom {
  /// The id of the element in the referencing resource that is expected to resolve to
  /// this resource.
  #[serde(rename = "sourceId")]
  source_id: String,

  /// Extensions for source
  _source: Element,

  /// Extensions for sourceId
  #[serde(rename = "_sourceId")]
  _source_id: Element,

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

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The name of the parameter or dot-separated path of parameter names pointing to
  /// the resource parameter that is expected to contain a reference to this resource.
  source: String,

}
