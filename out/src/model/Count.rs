#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;


/// A measured amount (or an amount that can potentially be measured). Note that
/// measured amounts include amounts that are not precisely quantified, including
/// amounts involving arbitrary units and floating currencies.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Count {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// A human-readable form of the unit.
  unit: String,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for system
  _system: Element,

  /// The identification of the system that provides the coded form of the unit.
  system: String,

  /// Extensions for unit
  _unit: Element,

  /// How the value should be understood and represented - whether the actual value is
  /// greater or less than the stated value due to measurement issues; e.g. if the
  /// comparator is "<" , then the real value is < stated value.
  comparator: CountComparator,

  /// Extensions for value
  _value: Element,

  /// A computer processable form of the unit in some unit representation system.
  code: String,

  /// Extensions for code
  _code: Element,

  /// The value of the measured amount. The value includes an implicit precision in
  /// the presentation of the value.
  value: f32,

  /// Extensions for comparator
  _comparator: Element,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum CountComparator {
  #[serde(rename = "<")]
  LessThan,

  #[serde(rename = "<=")]
  LessThanOrEqual,

  #[serde(rename = ">=")]
  GreaterThanOrEqual,

  #[serde(rename = ">")]
  GreaterThan,

}
