#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Range::Range;
use crate::model::Ratio::Ratio;
use crate::model::Element::Element;
use crate::model::Quantity::Quantity;
use crate::model::Duration::Duration;
use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;


/// Describes the intended objective(s) for a patient, group or organization care,
/// for example, weight loss, restoring an activity of daily living, obtaining herd
/// immunity via immunization, meeting a process improvement objective, etc.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Goal_Target {
  /// The target value of the focus to be achieved to signify the fulfillment of the
  /// goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range
  /// can be specified. When a low value is missing, it indicates that the goal is
  /// achieved at any focus value at or below the high value. Similarly, if the high
  /// value is missing, it indicates that the goal is achieved at any focus value at
  /// or above the low value.
  #[serde(rename = "detailCodeableConcept")]
  detail_codeable_concept: CodeableConcept,

  /// Extensions for dueDate
  #[serde(rename = "_dueDate")]
  _due_date: Element,

  /// The parameter whose value is being tracked, e.g. body weight, blood pressure, or
  /// hemoglobin A1c level.
  measure: CodeableConcept,

  /// The target value of the focus to be achieved to signify the fulfillment of the
  /// goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range
  /// can be specified. When a low value is missing, it indicates that the goal is
  /// achieved at any focus value at or below the high value. Similarly, if the high
  /// value is missing, it indicates that the goal is achieved at any focus value at
  /// or above the low value.
  #[serde(rename = "detailQuantity")]
  detail_quantity: Quantity,

  /// Indicates either the date or the duration after start by which the goal should
  /// be met.
  #[serde(rename = "dueDuration")]
  due_duration: Duration,

  /// The target value of the focus to be achieved to signify the fulfillment of the
  /// goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range
  /// can be specified. When a low value is missing, it indicates that the goal is
  /// achieved at any focus value at or below the high value. Similarly, if the high
  /// value is missing, it indicates that the goal is achieved at any focus value at
  /// or above the low value.
  #[serde(rename = "detailRatio")]
  detail_ratio: Ratio,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for detailString
  #[serde(rename = "_detailString")]
  _detail_string: Element,

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

  /// The target value of the focus to be achieved to signify the fulfillment of the
  /// goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range
  /// can be specified. When a low value is missing, it indicates that the goal is
  /// achieved at any focus value at or below the high value. Similarly, if the high
  /// value is missing, it indicates that the goal is achieved at any focus value at
  /// or above the low value.
  #[serde(rename = "detailInteger")]
  detail_integer: i32,

  /// The target value of the focus to be achieved to signify the fulfillment of the
  /// goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range
  /// can be specified. When a low value is missing, it indicates that the goal is
  /// achieved at any focus value at or below the high value. Similarly, if the high
  /// value is missing, it indicates that the goal is achieved at any focus value at
  /// or above the low value.
  #[serde(rename = "detailString")]
  detail_string: String,

  /// Indicates either the date or the duration after start by which the goal should
  /// be met.
  #[serde(rename = "dueDate")]
  due_date: String,

  /// The target value of the focus to be achieved to signify the fulfillment of the
  /// goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range
  /// can be specified. When a low value is missing, it indicates that the goal is
  /// achieved at any focus value at or below the high value. Similarly, if the high
  /// value is missing, it indicates that the goal is achieved at any focus value at
  /// or above the low value.
  #[serde(rename = "detailRange")]
  detail_range: Range,

  /// The target value of the focus to be achieved to signify the fulfillment of the
  /// goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range
  /// can be specified. When a low value is missing, it indicates that the goal is
  /// achieved at any focus value at or below the high value. Similarly, if the high
  /// value is missing, it indicates that the goal is achieved at any focus value at
  /// or above the low value.
  #[serde(rename = "detailBoolean")]
  detail_boolean: bool,

  /// Extensions for detailBoolean
  #[serde(rename = "_detailBoolean")]
  _detail_boolean: Element,

  /// Extensions for detailInteger
  #[serde(rename = "_detailInteger")]
  _detail_integer: Element,

}
