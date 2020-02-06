#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::OperationDefinition_Binding::OperationDefinition_Binding;
use crate::model::OperationDefinition_ReferencedFrom::OperationDefinition_ReferencedFrom;


/// A formal computable definition of an operation (on the RESTful interface) or a
/// named query (using the search interaction).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationDefinition_Parameter {
  /// Binds to a value set if this parameter is coded (code, Coding, CodeableConcept).
  binding: Option<OperationDefinition_Binding>,

  /// The parts of a nested Parameter.
  part: Option<Vec<OperationDefinition_Parameter>>,

  /// Extensions for use
  #[serde(rename = "_use")]
  _use: Option<Element>,

  /// Extensions for searchType
  #[serde(rename = "_searchType")]
  _search_type: Option<Element>,

  /// The minimum number of times this parameter SHALL appear in the request or
  /// response.
  min: Option<i32>,

  /// Extensions for type
  #[serde(rename = "_type")]
  _type: Option<Element>,

  /// The type for this parameter.
  #[serde(rename = "type")]
  fhir_type: Option<String>,

  /// Describes the meaning or use of this parameter.
  documentation: Option<String>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Identifies other resource parameters within the operation invocation that are
  /// expected to resolve to this resource.
  #[serde(rename = "referencedFrom")]
  referenced_from: Option<Vec<OperationDefinition_ReferencedFrom>>,

  /// Used when the type is "Reference" or "canonical", and identifies a profile
  /// structure or implementation Guide that applies to the target of the reference
  /// this parameter refers to. If any profiles are specified, then the content must
  /// conform to at least one of them. The URL can be a local reference - to a
  /// contained StructureDefinition, or a reference to another StructureDefinition or
  /// Implementation Guide by a canonical URL. When an implementation guide is
  /// specified, the target resource SHALL conform to at least one profile defined in
  /// the implementation guide.
  #[serde(rename = "targetProfile")]
  target_profile: Option<Vec<String>>,

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

  /// Extensions for documentation
  #[serde(rename = "_documentation")]
  _documentation: Option<Element>,

  /// Extensions for max
  #[serde(rename = "_max")]
  _max: Option<Element>,

  /// The name of used to identify the parameter.
  name: Option<String>,

  /// Whether this is an input or an output parameter.
  #[serde(rename = "use")]
  fhir_use: Option<OperationDefinition_ParameterUse>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for min
  #[serde(rename = "_min")]
  _min: Option<Element>,

  /// The maximum number of times this element is permitted to appear in the request
  /// or response.
  max: Option<String>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// How the parameter is understood as a search parameter. This is only used if the
  /// parameter type is 'string'.
  #[serde(rename = "searchType")]
  search_type: Option<OperationDefinition_ParameterSearchType>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum OperationDefinition_ParameterUse {
  #[serde(rename = "in")]
  In,

  #[serde(rename = "out")]
  Out,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum OperationDefinition_ParameterSearchType {
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
