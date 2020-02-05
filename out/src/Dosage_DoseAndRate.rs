use serde::{Deserialize, Serialize};

/// Indicates how the medication is/was taken or should be taken by the patient.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Dosage_DoseAndRate {
  /// Amount of medication per unit of time.
  #[serde(rename = "rateQuantity")]
  rate_quantity: Quantity,

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

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Amount of medication per dose.
  #[serde(rename = "doseRange")]
  dose_range: Range,

  /// Amount of medication per dose.
  #[serde(rename = "doseQuantity")]
  dose_quantity: Quantity,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Amount of medication per unit of time.
  #[serde(rename = "rateRatio")]
  rate_ratio: Ratio,

  /// Amount of medication per unit of time.
  #[serde(rename = "rateRange")]
  rate_range: Range,

  /// The kind of dose or rate specified, for example, ordered or calculated.
  type: CodeableConcept,

}
