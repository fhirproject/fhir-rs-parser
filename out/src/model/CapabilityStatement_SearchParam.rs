#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR
/// Server for a particular version of FHIR that may be used as a statement of
/// actual server functionality or a statement of required or desired server
/// implementation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatement_SearchParam {
  /// Extensions for type
  _type: Element,

  /// An absolute URI that is a formal reference to where this parameter was first
  /// defined, so that a client can be confident of the meaning of the search
  /// parameter (a reference to [[[SearchParameter.url]]]). This element SHALL be
  /// populated if the search parameter refers to a SearchParameter defined by the
  /// FHIR core specification or externally defined IGs.
  definition: String,

  /// The name of the search parameter used in the interface.
  name: String,

  /// Extensions for name
  _name: Element,

  /// The type of value a search parameter refers to, and how the content is
  /// interpreted.
  #[serde(rename = "type")]
  fhir_type: CapabilityStatement_SearchParamType,

  /// Extensions for documentation
  _documentation: Element,

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

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// This allows documentation of any distinct behaviors about how the search
  /// parameter is used.  For example, text matching algorithms.
  documentation: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum CapabilityStatement_SearchParamType {
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
