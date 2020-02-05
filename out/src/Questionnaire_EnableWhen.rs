use serde::{Deserialize, Serialize};

/// A structured set of questions intended to guide the collection of answers from
/// end-users. Questionnaires provide detailed control over order, presentation,
/// phraseology and grouping to allow coherent, consistent data collection.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Questionnaire_EnableWhen {
  /// Extensions for answerDate
  #[serde(rename = "_answerDate")]
  _answer_date: Element,

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  #[serde(rename = "answerBoolean")]
  answer_boolean: bool,

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  #[serde(rename = "answerString")]
  answer_string: String,

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  #[serde(rename = "answerTime")]
  answer_time: String,

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  #[serde(rename = "answerReference")]
  answer_reference: Reference,

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  #[serde(rename = "answerDate")]
  answer_date: String,

  /// Extensions for answerString
  #[serde(rename = "_answerString")]
  _answer_string: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Specifies the criteria by which the question is enabled.
  operator: Questionnaire_EnableWhenOperator,

  /// Extensions for question
  _question: Element,

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  #[serde(rename = "answerCoding")]
  answer_coding: Coding,

  /// Extensions for operator
  _operator: Element,

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  #[serde(rename = "answerDateTime")]
  answer_date_time: String,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for answerBoolean
  #[serde(rename = "_answerBoolean")]
  _answer_boolean: Element,

  /// Extensions for answerDateTime
  #[serde(rename = "_answerDateTime")]
  _answer_date_time: Element,

  /// Extensions for answerTime
  #[serde(rename = "_answerTime")]
  _answer_time: Element,

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  #[serde(rename = "answerInteger")]
  answer_integer: number,

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  #[serde(rename = "answerQuantity")]
  answer_quantity: Quantity,

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  #[serde(rename = "answerDecimal")]
  answer_decimal: number,

  /// Extensions for answerDecimal
  #[serde(rename = "_answerDecimal")]
  _answer_decimal: Element,

  /// Extensions for answerInteger
  #[serde(rename = "_answerInteger")]
  _answer_integer: Element,

  /// The linkId for the question whose answer (or lack of answer) governs whether
  /// this item is enabled.
  question: String,

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
enum Questionnaire_EnableWhenOperator {
  #[serde(rename = "exists")]
  Exists,

  #[serde(rename = "=")]
  =,

  #[serde(rename = "!=")]
  !=,

  #[serde(rename = ">")]
  >,

  #[serde(rename = "<")]
  <,

  #[serde(rename = ">=")]
  >=,

  #[serde(rename = "<=")]
  <=,

}
