#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Reference::Reference;
use crate::model::Element::Element;
use crate::model::Coding::Coding;
use crate::model::Extension::Extension;


/// A structured set of questions intended to guide the collection of answers from
/// end-users. Questionnaires provide detailed control over order, presentation,
/// phraseology and grouping to allow coherent, consistent data collection.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Questionnaire_AnswerOption {
  /// A potential answer that's allowed as the answer to this question.
  #[serde(rename = "valueCoding")]
  value_coding: Coding,

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
  /// resource are required to check for modifier extensions.    Modifier extensions
  /// SHALL NOT change the meaning of any elements on Resource or DomainResource
  /// (including cannot change the meaning of modifierExtension itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// A potential answer that's allowed as the answer to this question.
  #[serde(rename = "valueTime")]
  value_time: String,

  /// Extensions for valueTime
  #[serde(rename = "_valueTime")]
  _value_time: Element,

  /// Extensions for valueInteger
  #[serde(rename = "_valueInteger")]
  _value_integer: Element,

  /// A potential answer that's allowed as the answer to this question.
  #[serde(rename = "valueReference")]
  value_reference: Box<Reference>,

  /// Extensions for valueString
  #[serde(rename = "_valueString")]
  _value_string: Element,

  /// Extensions for valueDate
  #[serde(rename = "_valueDate")]
  _value_date: Element,

  /// A potential answer that's allowed as the answer to this question.
  #[serde(rename = "valueString")]
  value_string: String,

  /// Indicates whether the answer value is selected when the list of possible answers
  /// is initially shown.
  #[serde(rename = "initialSelected")]
  initial_selected: bool,

  /// A potential answer that's allowed as the answer to this question.
  #[serde(rename = "valueDate")]
  value_date: String,

  /// Extensions for initialSelected
  #[serde(rename = "_initialSelected")]
  _initial_selected: Element,

  /// A potential answer that's allowed as the answer to this question.
  #[serde(rename = "valueInteger")]
  value_integer: i32,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

}
