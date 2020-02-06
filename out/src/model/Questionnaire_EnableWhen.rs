#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Reference::Reference;
use crate::model::Element::Element;
use crate::model::Coding::Coding;
use crate::model::Quantity::Quantity;
use crate::model::Extension::Extension;


/// A structured set of questions intended to guide the collection of answers from
/// end-users. Questionnaires provide detailed control over order, presentation,
/// phraseology and grouping to allow coherent, consistent data collection.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Questionnaire_EnableWhen {
  /// Extensions for operator
  #[serde(rename = "_operator")]
  _operator: Option<Element>,

  /// Extensions for answerBoolean
  #[serde(rename = "_answerBoolean")]
  _answer_boolean: Option<Element>,

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  #[serde(rename = "answerDateTime")]
  answer_date_time: Option<String>,

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  #[serde(rename = "answerCoding")]
  answer_coding: Option<Coding>,

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  #[serde(rename = "answerTime")]
  answer_time: Option<String>,

  /// Extensions for question
  #[serde(rename = "_question")]
  _question: Option<Element>,

  /// Specifies the criteria by which the question is enabled.
  operator: Option<Questionnaire_EnableWhenOperator>,

  /// Extensions for answerString
  #[serde(rename = "_answerString")]
  _answer_string: Option<Element>,

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  #[serde(rename = "answerDate")]
  answer_date: Option<String>,

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  #[serde(rename = "answerQuantity")]
  answer_quantity: Option<Quantity>,

  /// Extensions for answerDecimal
  #[serde(rename = "_answerDecimal")]
  _answer_decimal: Option<Element>,

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  #[serde(rename = "answerDecimal")]
  answer_decimal: Option<i32>,

  /// Extensions for answerDateTime
  #[serde(rename = "_answerDateTime")]
  _answer_date_time: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for answerTime
  #[serde(rename = "_answerTime")]
  _answer_time: Option<Element>,

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

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The linkId for the question whose answer (or lack of answer) governs whether
  /// this item is enabled.
  question: Option<String>,

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  #[serde(rename = "answerBoolean")]
  answer_boolean: Option<bool>,

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  #[serde(rename = "answerString")]
  answer_string: Option<String>,

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  #[serde(rename = "answerReference")]
  answer_reference: Option<Box<Reference>>,

  /// Extensions for answerDate
  #[serde(rename = "_answerDate")]
  _answer_date: Option<Element>,

  /// Extensions for answerInteger
  #[serde(rename = "_answerInteger")]
  _answer_integer: Option<Element>,

  /// A value that the referenced question is tested using the specified operator in
  /// order for the item to be enabled.
  #[serde(rename = "answerInteger")]
  answer_integer: Option<i32>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum Questionnaire_EnableWhenOperator {
  #[serde(rename = "exists")]
  Exists,

  #[serde(rename = "=")]
  Equal,

  #[serde(rename = "!=")]
  NotEqual,

  #[serde(rename = ">")]
  GreaterThan,

  #[serde(rename = "<")]
  LessThan,

  #[serde(rename = ">=")]
  GreaterThanOrEqual,

  #[serde(rename = "<=")]
  LessThanOrEqual,

}
