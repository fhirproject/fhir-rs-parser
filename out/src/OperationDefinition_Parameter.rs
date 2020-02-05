use serde::{Deserialize, Serialize};

/// A formal computable definition of an operation (on the RESTful interface) or a
/// named query (using the search interaction).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OperationDefinition_Parameter {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Describes the meaning or use of this parameter.
  documentation: String,

  /// Extensions for name
  _name: Element,

  /// The name of used to identify the parameter.
  name: String,

  /// Extensions for use
  _use: Element,

  /// Extensions for max
  _max: Element,

  /// Extensions for documentation
  _documentation: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// The minimum number of times this parameter SHALL appear in the request or
  /// response.
  min: integer,

  /// Extensions for searchType
  #[serde(rename = "_searchType")]
  _search_type: Element,

  /// Extensions for min
  _min: Element,

  /// Extensions for type
  _type: Element,

  /// Binds to a value set if this parameter is coded (code, Coding, CodeableConcept).
  binding: OperationDefinition_Binding,

  /// How the parameter is understood as a search parameter. This is only used if the
  /// parameter type is 'string'.
  #[serde(rename = "searchType")]
  search_type: OperationDefinition_ParameterSearchType,

  /// Used when the type is "Reference" or "canonical", and identifies a profile
  /// structure or implementation Guide that applies to the target of the reference
  /// this parameter refers to. If any profiles are specified, then the content must
  /// conform to at least one of them. The URL can be a local reference - to a
  /// contained StructureDefinition, or a reference to another StructureDefinition or
  /// Implementation Guide by a canonical URL. When an implementation guide is
  /// specified, the target resource SHALL conform to at least one profile defined in
  /// the implementation guide.
  #[serde(rename = "targetProfile")]
  target_profile: Vec<canonical>,

  /// The maximum number of times this element is permitted to appear in the request
  /// or response.
  max: String,

  /// Whether this is an input or an output parameter.
  use: OperationDefinition_ParameterUse,

  /// The type for this parameter.
  type: String,

  /// Identifies other resource parameters within the operation invocation that are
  /// expected to resolve to this resource.
  #[serde(rename = "referencedFrom")]
  referenced_from: Vec<OperationDefinition_ReferencedFrom>,

  /// The parts of a nested Parameter.
  part: Vec<OperationDefinition_Parameter>,

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

}

#[derive(Debug, Serialize, Deserialize)]
enum OperationDefinition_ParameterSearchType {
  #[serde(rename = "number")]
  Number,

  #[serde(rename = "date")]
  Date,

  #[serde(rename = "string")]
  String,

  #[serde(rename = "token")]
  Token,

  #[serde(rename = "reference")]
  Reference,

  #[serde(rename = "composite")]
  Composite,

  #[serde(rename = "quantity")]
  Quantity,

  #[serde(rename = "uri")]
  Uri,

  #[serde(rename = "special")]
  Special,

}

#[derive(Debug, Serialize, Deserialize)]
enum OperationDefinition_ParameterUse {
  #[serde(rename = "in")]
  In,

  #[serde(rename = "out")]
  Out,

}
