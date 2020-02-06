#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// An identifier - identifies some entity uniquely and unambiguously. Typically
/// this is used for business identifiers.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
  /// The purpose of this identifier.
  #[serde(rename = "use")]
  fhir_use: Option<IdentifierUse>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The portion of the identifier typically relevant to the user and which is unique
  /// within the context of the system.
  value: Option<String>,

  /// Extensions for value
  #[serde(rename = "_value")]
  _value: Option<Element>,

  /// Extensions for use
  #[serde(rename = "_use")]
  _use: Option<Element>,

  /// Time period during which identifier is/was valid for use.
  period: Option<Period>,

  /// Organization that issued/manages the identifier.
  assigner: Option<Box<Reference>>,

  /// A coded type for the identifier that can be used to determine which identifier
  /// to use for a specific purpose.
  #[serde(rename = "type")]
  fhir_type: Option<CodeableConcept>,

  /// Establishes the namespace for the value - that is, a URL that describes a set
  /// values that are unique.
  system: Option<String>,

  /// Extensions for system
  #[serde(rename = "_system")]
  _system: Option<Element>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum IdentifierUse {
  #[serde(rename = "usual")]
  Usual,

  #[serde(rename = "official")]
  Official,

  #[serde(rename = "temp")]
  Temp,

  #[serde(rename = "secondary")]
  Secondary,

  #[serde(rename = "old")]
  Old,

}
