use serde::{Deserialize, Serialize};

/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [[[CodeSystem]]] definitions and their use in [coded
/// elements](terminologies.html).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ValueSet_Parameter {
  /// Extensions for valueBoolean
  #[serde(rename = "_valueBoolean")]
  _value_boolean: Element,

  /// Extensions for valueDateTime
  #[serde(rename = "_valueDateTime")]
  _value_date_time: Element,

  /// Extensions for name
  _name: Element,

  /// The value of the parameter.
  #[serde(rename = "valueBoolean")]
  value_boolean: bool,

  /// The value of the parameter.
  #[serde(rename = "valueUri")]
  value_uri: String,

  /// The value of the parameter.
  #[serde(rename = "valueDateTime")]
  value_date_time: String,

  /// Extensions for valueDecimal
  #[serde(rename = "_valueDecimal")]
  _value_decimal: Element,

  /// Extensions for valueString
  #[serde(rename = "_valueString")]
  _value_string: Element,

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

  /// Extensions for valueUri
  #[serde(rename = "_valueUri")]
  _value_uri: Element,

  /// The value of the parameter.
  #[serde(rename = "valueDecimal")]
  value_decimal: number,

  /// The value of the parameter.
  #[serde(rename = "valueString")]
  value_string: String,

  /// Extensions for valueInteger
  #[serde(rename = "_valueInteger")]
  _value_integer: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// The value of the parameter.
  #[serde(rename = "valueInteger")]
  value_integer: number,

  /// The value of the parameter.
  #[serde(rename = "valueCode")]
  value_code: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Name of the input parameter to the $expand operation; may be a server-assigned
  /// name for additional default or other server-supplied parameters used to control
  /// the expansion process.
  name: String,

  /// Extensions for valueCode
  #[serde(rename = "_valueCode")]
  _value_code: Element,

}
