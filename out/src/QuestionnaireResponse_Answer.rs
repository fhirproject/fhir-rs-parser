use serde::{Deserialize, Serialize};

/// A structured set of questions and their answers. The questions are ordered and
/// grouped into coherent subsets, corresponding to the structure of the grouping of
/// the questionnaire being responded to.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct QuestionnaireResponse_Answer {
  /// The answer (or one of the answers) provided by the respondent to the question.
  #[serde(rename = "valueBoolean")]
  value_boolean: bool,

  /// The answer (or one of the answers) provided by the respondent to the question.
  #[serde(rename = "valueUri")]
  value_uri: String,

  /// The answer (or one of the answers) provided by the respondent to the question.
  #[serde(rename = "valueReference")]
  value_reference: Reference,

  /// Extensions for valueDateTime
  #[serde(rename = "_valueDateTime")]
  _value_date_time: Element,

  /// The answer (or one of the answers) provided by the respondent to the question.
  #[serde(rename = "valueString")]
  value_string: String,

  /// The answer (or one of the answers) provided by the respondent to the question.
  #[serde(rename = "valueTime")]
  value_time: String,

  /// Nested groups and/or questions found within this particular answer.
  item: Vec<QuestionnaireResponse_Item>,

  /// The answer (or one of the answers) provided by the respondent to the question.
  #[serde(rename = "valueQuantity")]
  value_quantity: Quantity,

  /// Extensions for valueInteger
  #[serde(rename = "_valueInteger")]
  _value_integer: Element,

  /// Extensions for valueTime
  #[serde(rename = "_valueTime")]
  _value_time: Element,

  /// Extensions for valueUri
  #[serde(rename = "_valueUri")]
  _value_uri: Element,

  /// Extensions for valueDate
  #[serde(rename = "_valueDate")]
  _value_date: Element,

  /// The answer (or one of the answers) provided by the respondent to the question.
  #[serde(rename = "valueDateTime")]
  value_date_time: String,

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
  /// resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// The answer (or one of the answers) provided by the respondent to the question.
  #[serde(rename = "valueInteger")]
  value_integer: number,

  /// The answer (or one of the answers) provided by the respondent to the question.
  #[serde(rename = "valueDecimal")]
  value_decimal: number,

  /// The answer (or one of the answers) provided by the respondent to the question.
  #[serde(rename = "valueDate")]
  value_date: String,

  /// The answer (or one of the answers) provided by the respondent to the question.
  #[serde(rename = "valueCoding")]
  value_coding: Coding,

  /// Extensions for valueString
  #[serde(rename = "_valueString")]
  _value_string: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for valueDecimal
  #[serde(rename = "_valueDecimal")]
  _value_decimal: Element,

  /// The answer (or one of the answers) provided by the respondent to the question.
  #[serde(rename = "valueAttachment")]
  value_attachment: Attachment,

  /// Extensions for valueBoolean
  #[serde(rename = "_valueBoolean")]
  _value_boolean: Element,

}
