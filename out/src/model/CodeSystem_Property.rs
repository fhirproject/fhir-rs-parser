#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;


/// The CodeSystem resource is used to declare the existence of and describe a code
/// system or code system supplement and its key properties, and optionally define a
/// part or all of its content.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystem_Property {
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

  /// Reference to the formal meaning of the property. One possible source of meaning
  /// is the [Concept Properties](codesystem-concept-properties.html) code system.
  uri: String,

  /// Extensions for uri
  _uri: Element,

  /// A description of the property- why it is defined, and how its value might be
  /// used.
  description: String,

  /// Extensions for code
  _code: Element,

  /// Extensions for description
  _description: Element,

  /// The type of the property value. Properties of type "code" contain a code defined
  /// by the code system (e.g. a reference to another defined concept).
  #[serde(rename = "type")]
  fhir_type: CodeSystem_PropertyType,

  /// Extensions for type
  _type: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// A code that is used to identify the property. The code is used internally (in
  /// CodeSystem.concept.property.code) and also externally, such as in property
  /// filters.
  code: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum CodeSystem_PropertyType {
  #[serde(rename = "code")]
  Code,

  #[serde(rename = "Coding")]
  Coding,

  #[serde(rename = "string")]
  String,

  #[serde(rename = "integer")]
  Integer,

  #[serde(rename = "boolean")]
  Boolean,

  #[serde(rename = "dateTime")]
  DateTime,

  #[serde(rename = "decimal")]
  Decimal,

}
