use serde::{Deserialize, Serialize};

/// An amount of economic utility in some recognized currency.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Money {
  /// Extensions for currency
  _currency: Element,

  /// ISO 4217 Currency Code.
  currency: String,

  /// Extensions for value
  _value: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Numerical value (with implicit precision).
  value: decimal,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

}
