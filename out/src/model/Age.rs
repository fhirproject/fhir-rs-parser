#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;


/// A duration of time during which an organism (or a process) has existed.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Age {
  /// How the value should be understood and represented - whether the actual value is
  /// greater or less than the stated value due to measurement issues; e.g. if the
  /// comparator is "<" , then the real value is < stated value.
  comparator: AgeComparator,

  /// A computer processable form of the unit in some unit representation system.
  code: String,

  /// Extensions for system
  _system: Element,

  /// The identification of the system that provides the coded form of the unit.
  system: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// The value of the measured amount. The value includes an implicit precision in
  /// the presentation of the value.
  value: f32,

  /// Extensions for code
  _code: Element,

  /// Extensions for value
  _value: Element,

  /// A human-readable form of the unit.
  unit: String,

  /// Extensions for unit
  _unit: Element,

  /// Extensions for comparator
  _comparator: Element,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum AgeComparator {
  #[serde(rename = "<")]
  LessThan,

  #[serde(rename = "<=")]
  LessThanOrEqual,

  #[serde(rename = ">=")]
  GreaterThanOrEqual,

  #[serde(rename = ">")]
  GreaterThan,

}
