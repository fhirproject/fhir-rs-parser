#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;


/// Details and position information for a physical place where services are
/// provided and resources and participants may be stored, found, contained, or
/// accommodated.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location_HoursOfOperation {
  /// Indicates which days of the week are available between the start and end Times.
  #[serde(rename = "daysOfWeek")]
  days_of_week: Option<Vec<String>>,

  /// The Location is open all day.
  #[serde(rename = "allDay")]
  all_day: Option<bool>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for allDay
  #[serde(rename = "_allDay")]
  _all_day: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Time that the Location opens.
  #[serde(rename = "openingTime")]
  opening_time: Option<String>,

  /// Extensions for openingTime
  #[serde(rename = "_openingTime")]
  _opening_time: Option<Element>,

  /// Extensions for daysOfWeek
  #[serde(rename = "_daysOfWeek")]
  _days_of_week: Option<Vec<Element>>,

  /// Time that the Location closes.
  #[serde(rename = "closingTime")]
  closing_time: Option<String>,

  /// Extensions for closingTime
  #[serde(rename = "_closingTime")]
  _closing_time: Option<Element>,

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

}
