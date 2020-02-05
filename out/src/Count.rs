use serde::{Deserialize, Serialize};

/// A measured amount (or an amount that can potentially be measured). Note that
/// measured amounts include amounts that are not precisely quantified, including
/// amounts involving arbitrary units and floating currencies.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Count {
  /// Extensions for code
  _code: Element,

  /// Extensions for comparator
  _comparator: Element,

  /// How the value should be understood and represented - whether the actual value is
  /// greater or less than the stated value due to measurement issues; e.g. if the
  /// comparator is "<" , then the real value is < stated value.
  comparator: CountComparator,

  /// The value of the measured amount. The value includes an implicit precision in
  /// the presentation of the value.
  value: decimal,

  /// The identification of the system that provides the coded form of the unit.
  system: String,

  /// A computer processable form of the unit in some unit representation system.
  code: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for value
  _value: Element,

  /// Extensions for unit
  _unit: Element,

  /// A human-readable form of the unit.
  unit: String,

  /// Extensions for system
  _system: Element,

}

#[derive(Debug, Serialize, Deserialize)]
enum CountComparator {
  #[serde(rename = "<")]
  <,

  #[serde(rename = "<=")]
  <=,

  #[serde(rename = ">=")]
  >=,

  #[serde(rename = ">")]
  >,

}
