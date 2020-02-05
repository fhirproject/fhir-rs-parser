use serde::{Deserialize, Serialize};

/// An assessment of the likely outcome(s) for a patient or other subject as well as
/// the likelihood of each outcome.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RiskAssessment_Prediction {
  /// Indicates the period of time or age range of the subject to which the specified
  /// probability applies.
  #[serde(rename = "whenPeriod")]
  when_period: Period,

  /// Indicates how likely the outcome is (in the specified timeframe).
  #[serde(rename = "probabilityRange")]
  probability_range: Range,

  /// Indicates the risk for this particular subject (with their specific
  /// characteristics) divided by the risk of the population in general.  (Numbers
  /// greater than 1 = higher risk than the population, numbers less than 1 = lower
  /// risk.).
  #[serde(rename = "relativeRisk")]
  relative_risk: decimal,

  /// Indicates how likely the outcome is (in the specified timeframe), expressed as a
  /// qualitative value (e.g. low, medium, or high).
  #[serde(rename = "qualitativeRisk")]
  qualitative_risk: CodeableConcept,

  /// One of the potential outcomes for the patient (e.g. remission, death,  a
  /// particular condition).
  outcome: CodeableConcept,

  /// Indicates the period of time or age range of the subject to which the specified
  /// probability applies.
  #[serde(rename = "whenRange")]
  when_range: Range,

  /// Extensions for rationale
  _rationale: Element,

  /// Extensions for probabilityDecimal
  #[serde(rename = "_probabilityDecimal")]
  _probability_decimal: Element,

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

  /// Extensions for relativeRisk
  #[serde(rename = "_relativeRisk")]
  _relative_risk: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Indicates how likely the outcome is (in the specified timeframe).
  #[serde(rename = "probabilityDecimal")]
  probability_decimal: number,

  /// Additional information explaining the basis for the prediction.
  rationale: String,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

}
