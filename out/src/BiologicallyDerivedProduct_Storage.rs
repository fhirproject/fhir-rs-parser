use serde::{Deserialize, Serialize};

/// A material substance originating from a biological entity intended to be
/// transplanted or infused
/// into another (possibly the same) biological entity.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BiologicallyDerivedProduct_Storage {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Temperature scale used.
  scale: BiologicallyDerivedProduct_StorageScale,

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
  /// resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// Extensions for description
  _description: Element,

  /// Storage timeperiod.
  duration: Period,

  /// Extensions for temperature
  _temperature: Element,

  /// Storage temperature.
  temperature: decimal,

  /// Extensions for scale
  _scale: Element,

  /// Description of storage.
  description: String,

}

#[derive(Debug, Serialize, Deserialize)]
enum BiologicallyDerivedProduct_StorageScale {
  #[serde(rename = "farenheit")]
  Farenheit,

  #[serde(rename = "celsius")]
  Celsius,

  #[serde(rename = "kelvin")]
  Kelvin,

}
