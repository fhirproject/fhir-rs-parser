#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// The details of a healthcare service available at a location.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthcareService_AvailableTime {
  /// The closing time of day. Note: If the AllDay flag is set, then this time is
  /// ignored.
  #[serde(rename = "availableEndTime")]
  available_end_time: Option<String>,

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

  /// Extensions for daysOfWeek
  #[serde(rename = "_daysOfWeek")]
  _days_of_week: Option<Vec<Element>>,

  /// Is this always available? (hence times are irrelevant) e.g. 24 hour service.
  #[serde(rename = "allDay")]
  all_day: Option<bool>,

  /// The opening time of day. Note: If the AllDay flag is set, then this time is
  /// ignored.
  #[serde(rename = "availableStartTime")]
  available_start_time: Option<String>,

  /// Extensions for allDay
  #[serde(rename = "_allDay")]
  _all_day: Option<Element>,

  /// Extensions for availableStartTime
  #[serde(rename = "_availableStartTime")]
  _available_start_time: Option<Element>,

  /// Extensions for availableEndTime
  #[serde(rename = "_availableEndTime")]
  _available_end_time: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

}
