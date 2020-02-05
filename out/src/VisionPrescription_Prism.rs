use serde::{Deserialize, Serialize};

/// An authorization for the provision of glasses and/or contact lenses to a
/// patient.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VisionPrescription_Prism {
  /// The relative base, or reference lens edge, for the prism.
  base: VisionPrescription_PrismBase,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Amount of prism to compensate for eye alignment in fractional units.
  amount: decimal,

  /// Extensions for amount
  _amount: Element,

  /// Extensions for base
  _base: Element,

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

}

#[derive(Debug, Serialize, Deserialize)]
enum VisionPrescription_PrismBase {
  #[serde(rename = "up")]
  Up,

  #[serde(rename = "down")]
  Down,

  #[serde(rename = "in")]
  In,

  #[serde(rename = "out")]
  Out,

}
