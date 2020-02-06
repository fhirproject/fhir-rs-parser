#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::Duration::Duration;
use crate::model::Period::Period;
use crate::model::Range::Range;


/// Specifies an event that may occur multiple times. Timing schedules are used to
/// record when things are planned, expected or requested to occur. The most common
/// usage is in dosage instructions for medications. They are also used when
/// planning care of various kinds, and may be used for reporting the schedule to
/// which past regular activities were carried out.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timing_Repeat {
  /// Extensions for periodMax
  #[serde(rename = "_periodMax")]
  _period_max: Element,

  /// Either a duration for the length of the timing schedule, a range of possible
  /// length, or outer bounds for start and/or end limits of the timing schedule.
  #[serde(rename = "boundsRange")]
  bounds_range: Range,

  /// Extensions for timeOfDay
  #[serde(rename = "_timeOfDay")]
  _time_of_day: Vec<Element>,

  /// Indicates the duration of time over which repetitions are to occur; e.g. to
  /// express "3 times per day", 3 would be the frequency and "1 day" would be the
  /// period. If periodMax is present, this element indicates the lower bound of the
  /// allowed range of the period length.
  period: f32,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for period
  _period: Element,

  /// Extensions for duration
  _duration: Element,

  /// Extensions for durationUnit
  #[serde(rename = "_durationUnit")]
  _duration_unit: Element,

  /// A total count of the desired number of repetitions across the duration of the
  /// entire timing specification. If countMax is present, this element indicates the
  /// lower bound of the allowed range of count values.
  count: i32,

  /// If one or more days of week is provided, then the action happens only on the
  /// specified day(s).
  #[serde(rename = "dayOfWeek")]
  day_of_week: Vec<String>,

  /// The number of minutes from the event. If the event code does not indicate
  /// whether the minutes is before or after the event, then the offset is assumed to
  /// be after the event.
  offset: u32,

  /// Extensions for offset
  _offset: Element,

  /// Extensions for frequencyMax
  #[serde(rename = "_frequencyMax")]
  _frequency_max: Element,

  /// If present, indicates that the duration is a range - so to perform the action
  /// between [duration] and [durationMax] time length.
  #[serde(rename = "durationMax")]
  duration_max: f32,

  /// Extensions for frequency
  _frequency: Element,

  /// Specified time of day for action to take place.
  #[serde(rename = "timeOfDay")]
  time_of_day: Vec<String>,

  /// The units of time for the duration, in UCUM units.
  #[serde(rename = "durationUnit")]
  duration_unit: Timing_RepeatDurationUnit,

  /// Extensions for dayOfWeek
  #[serde(rename = "_dayOfWeek")]
  _day_of_week: Vec<Element>,

  /// Either a duration for the length of the timing schedule, a range of possible
  /// length, or outer bounds for start and/or end limits of the timing schedule.
  #[serde(rename = "boundsDuration")]
  bounds_duration: Duration,

  /// Either a duration for the length of the timing schedule, a range of possible
  /// length, or outer bounds for start and/or end limits of the timing schedule.
  #[serde(rename = "boundsPeriod")]
  bounds_period: Period,

  /// If present, indicates that the period is a range from [period] to [periodMax],
  /// allowing expressing concepts such as "do this once every 3-5 days.
  #[serde(rename = "periodMax")]
  period_max: f32,

  /// Extensions for periodUnit
  #[serde(rename = "_periodUnit")]
  _period_unit: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for durationMax
  #[serde(rename = "_durationMax")]
  _duration_max: Element,

  /// Extensions for when
  _when: Vec<Element>,

  /// If present, indicates that the frequency is a range - so to repeat between
  /// [frequency] and [frequencyMax] times within the period or period range.
  #[serde(rename = "frequencyMax")]
  frequency_max: i32,

  /// Extensions for count
  _count: Element,

  /// Extensions for countMax
  #[serde(rename = "_countMax")]
  _count_max: Element,

  /// If present, indicates that the count is a range - so to perform the action
  /// between [count] and [countMax] times.
  #[serde(rename = "countMax")]
  count_max: i32,

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

  /// How long this thing happens for when it happens. If durationMax is present, this
  /// element indicates the lower bound of the allowed range of the duration.
  duration: f32,

  /// The number of times to repeat the action within the specified period. If
  /// frequencyMax is present, this element indicates the lower bound of the allowed
  /// range of the frequency.
  frequency: i32,

  /// The units of time for the period in UCUM units.
  #[serde(rename = "periodUnit")]
  period_unit: Timing_RepeatPeriodUnit,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum Timing_RepeatDurationUnit {
  #[serde(rename = "s")]
  S,

  #[serde(rename = "min")]
  Min,

  #[serde(rename = "h")]
  H,

  #[serde(rename = "d")]
  D,

  #[serde(rename = "wk")]
  Wk,

  #[serde(rename = "mo")]
  Mo,

  #[serde(rename = "a")]
  A,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum Timing_RepeatPeriodUnit {
  #[serde(rename = "s")]
  S,

  #[serde(rename = "min")]
  Min,

  #[serde(rename = "h")]
  H,

  #[serde(rename = "d")]
  D,

  #[serde(rename = "wk")]
  Wk,

  #[serde(rename = "mo")]
  Mo,

  #[serde(rename = "a")]
  A,

}
