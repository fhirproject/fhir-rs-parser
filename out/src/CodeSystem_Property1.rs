use serde::{Deserialize, Serialize};

/// The CodeSystem resource is used to declare the existence of and describe a code
/// system or code system supplement and its key properties, and optionally define a
/// part or all of its content.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CodeSystem_Property1 {
  /// Extensions for valueDateTime
  #[serde(rename = "_valueDateTime")]
  _value_date_time: Element,

  /// Extensions for valueInteger
  #[serde(rename = "_valueInteger")]
  _value_integer: Element,

  /// The value of this property.
  #[serde(rename = "valueBoolean")]
  value_boolean: bool,

  /// The value of this property.
  #[serde(rename = "valueInteger")]
  value_integer: number,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for valueString
  #[serde(rename = "_valueString")]
  _value_string: Element,

  /// The value of this property.
  #[serde(rename = "valueDecimal")]
  value_decimal: number,

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

  /// A code that is a reference to CodeSystem.property.code.
  code: String,

  /// The value of this property.
  #[serde(rename = "valueString")]
  value_string: String,

  /// The value of this property.
  #[serde(rename = "valueDateTime")]
  value_date_time: String,

  /// Extensions for valueCode
  #[serde(rename = "_valueCode")]
  _value_code: Element,

  /// The value of this property.
  #[serde(rename = "valueCode")]
  value_code: String,

  /// Extensions for valueBoolean
  #[serde(rename = "_valueBoolean")]
  _value_boolean: Element,

  /// The value of this property.
  #[serde(rename = "valueCoding")]
  value_coding: Coding,

  /// Extensions for code
  _code: Element,

  /// Extensions for valueDecimal
  #[serde(rename = "_valueDecimal")]
  _value_decimal: Element,

}
