use serde::{Deserialize, Serialize};

/// Describes the event of a patient being administered a vaccine or a record of an
/// immunization as reported by a patient, a clinician or another party.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Immunization_ProtocolApplied {
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

  /// Extensions for series
  _series: Element,

  /// Extensions for seriesDosesString
  #[serde(rename = "_seriesDosesString")]
  _series_doses_string: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for doseNumberString
  #[serde(rename = "_doseNumberString")]
  _dose_number_string: Element,

  /// Extensions for doseNumberPositiveInt
  #[serde(rename = "_doseNumberPositiveInt")]
  _dose_number_positive_int: Element,

  /// The recommended number of doses to achieve immunity.
  #[serde(rename = "seriesDosesString")]
  series_doses_string: String,

  /// Indicates the authority who published the protocol (e.g. ACIP) that is being
  /// followed.
  authority: Reference,

  /// The recommended number of doses to achieve immunity.
  #[serde(rename = "seriesDosesPositiveInt")]
  series_doses_positive_int: number,

  /// Nominal position in a series.
  #[serde(rename = "doseNumberString")]
  dose_number_string: String,

  /// The vaccine preventable disease the dose is being administered against.
  #[serde(rename = "targetDisease")]
  target_disease: Vec<CodeableConcept>,

  /// Extensions for seriesDosesPositiveInt
  #[serde(rename = "_seriesDosesPositiveInt")]
  _series_doses_positive_int: Element,

  /// Nominal position in a series.
  #[serde(rename = "doseNumberPositiveInt")]
  dose_number_positive_int: number,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// One possible path to achieve presumed immunity against a disease - within the
  /// context of an authority.
  series: String,

}
