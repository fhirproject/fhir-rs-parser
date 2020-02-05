use serde::{Deserialize, Serialize};

/// Details and position information for a physical place where services are
/// provided and resources and participants may be stored, found, contained, or
/// accommodated.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Location_HoursOfOperation {
  /// Indicates which days of the week are available between the start and end Times.
  #[serde(rename = "daysOfWeek")]
  days_of_week: Vec<String>,

  /// Extensions for daysOfWeek
  #[serde(rename = "_daysOfWeek")]
  _days_of_week: Vec<Element>,

  /// Extensions for closingTime
  #[serde(rename = "_closingTime")]
  _closing_time: Element,

  /// Extensions for allDay
  #[serde(rename = "_allDay")]
  _all_day: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The Location is open all day.
  #[serde(rename = "allDay")]
  all_day: bool,

  /// Time that the Location closes.
  #[serde(rename = "closingTime")]
  closing_time: time,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Time that the Location opens.
  #[serde(rename = "openingTime")]
  opening_time: time,

  /// Extensions for openingTime
  #[serde(rename = "_openingTime")]
  _opening_time: Element,

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
