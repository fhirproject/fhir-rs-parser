use serde::{Deserialize, Serialize};

/// A patient's point-in-time set of recommendations (i.e. forecasting) according to
/// a published schedule with optional supporting justification.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImmunizationRecommendation_Recommendation {
  /// Nominal position of the recommended dose in a series (e.g. dose 2 is the next
  /// recommended dose).
  #[serde(rename = "doseNumberPositiveInt")]
  dose_number_positive_int: number,

  /// Extensions for doseNumberString
  #[serde(rename = "_doseNumberString")]
  _dose_number_string: Element,

  /// Extensions for seriesDosesPositiveInt
  #[serde(rename = "_seriesDosesPositiveInt")]
  _series_doses_positive_int: Element,

  /// Patient Information that supports the status and recommendation.  This includes
  /// patient observations, adverse reactions and allergy/intolerance information.
  #[serde(rename = "supportingPatientInformation")]
  supporting_patient_information: Vec<Reference>,

  /// Nominal position of the recommended dose in a series (e.g. dose 2 is the next
  /// recommended dose).
  #[serde(rename = "doseNumberString")]
  dose_number_string: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for doseNumberPositiveInt
  #[serde(rename = "_doseNumberPositiveInt")]
  _dose_number_positive_int: Element,

  /// One possible path to achieve presumed immunity against a disease - within the
  /// context of an authority.
  series: String,

  /// The reason for the assigned forecast status.
  #[serde(rename = "forecastReason")]
  forecast_reason: Vec<CodeableConcept>,

  /// The recommended number of doses to achieve immunity.
  #[serde(rename = "seriesDosesString")]
  series_doses_string: String,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for description
  _description: Element,

  /// Extensions for series
  _series: Element,

  /// Vaccine date recommendations.  For example, earliest date to administer, latest
  /// date to administer, etc.
  #[serde(rename = "dateCriterion")]
  date_criterion: Vec<ImmunizationRecommendation_DateCriterion>,

  /// The recommended number of doses to achieve immunity.
  #[serde(rename = "seriesDosesPositiveInt")]
  series_doses_positive_int: number,

  /// Contains the description about the protocol under which the vaccine was
  /// administered.
  description: String,

  /// Immunization event history and/or evaluation that supports the status and
  /// recommendation.
  #[serde(rename = "supportingImmunization")]
  supporting_immunization: Vec<Reference>,

  /// Extensions for seriesDosesString
  #[serde(rename = "_seriesDosesString")]
  _series_doses_string: Element,

  /// Vaccine(s) which should not be used to fulfill the recommendation.
  #[serde(rename = "contraindicatedVaccineCode")]
  contraindicated_vaccine_code: Vec<CodeableConcept>,

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

  /// The targeted disease for the recommendation.
  #[serde(rename = "targetDisease")]
  target_disease: CodeableConcept,

  /// Vaccine(s) or vaccine group that pertain to the recommendation.
  #[serde(rename = "vaccineCode")]
  vaccine_code: Vec<CodeableConcept>,

  /// Indicates the patient status with respect to the path to immunity for the target
  /// disease.
  #[serde(rename = "forecastStatus")]
  forecast_status: CodeableConcept,

}
