use serde::{Deserialize, Serialize};

/// A structured set of questions intended to guide the collection of answers from
/// end-users. Questionnaires provide detailed control over order, presentation,
/// phraseology and grouping to allow coherent, consistent data collection.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Questionnaire_Initial {
  /// The actual value to for an initial answer.
  #[serde(rename = "valueTime")]
  value_time: String,

  /// The actual value to for an initial answer.
  #[serde(rename = "valueBoolean")]
  value_boolean: bool,

  /// Extensions for valueInteger
  #[serde(rename = "_valueInteger")]
  _value_integer: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// The actual value to for an initial answer.
  #[serde(rename = "valueString")]
  value_string: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for valueString
  #[serde(rename = "_valueString")]
  _value_string: Element,

  /// The actual value to for an initial answer.
  #[serde(rename = "valueQuantity")]
  value_quantity: Quantity,

  /// The actual value to for an initial answer.
  #[serde(rename = "valueReference")]
  value_reference: Reference,

  /// The actual value to for an initial answer.
  #[serde(rename = "valueUri")]
  value_uri: String,

  /// Extensions for valueTime
  #[serde(rename = "_valueTime")]
  _value_time: Element,

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

  /// The actual value to for an initial answer.
  #[serde(rename = "valueDecimal")]
  value_decimal: number,

  /// The actual value to for an initial answer.
  #[serde(rename = "valueCoding")]
  value_coding: Coding,

  /// Extensions for valueDateTime
  #[serde(rename = "_valueDateTime")]
  _value_date_time: Element,

  /// Extensions for valueDate
  #[serde(rename = "_valueDate")]
  _value_date: Element,

  /// Extensions for valueDecimal
  #[serde(rename = "_valueDecimal")]
  _value_decimal: Element,

  /// Extensions for valueUri
  #[serde(rename = "_valueUri")]
  _value_uri: Element,

  /// Extensions for valueBoolean
  #[serde(rename = "_valueBoolean")]
  _value_boolean: Element,

  /// The actual value to for an initial answer.
  #[serde(rename = "valueDate")]
  value_date: String,

  /// The actual value to for an initial answer.
  #[serde(rename = "valueAttachment")]
  value_attachment: Attachment,

  /// The actual value to for an initial answer.
  #[serde(rename = "valueDateTime")]
  value_date_time: String,

  /// The actual value to for an initial answer.
  #[serde(rename = "valueInteger")]
  value_integer: number,

}
