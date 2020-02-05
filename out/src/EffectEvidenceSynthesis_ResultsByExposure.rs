use serde::{Deserialize, Serialize};

/// The EffectEvidenceSynthesis resource describes the difference in an outcome
/// between exposures states in a population where the effect estimate is derived
/// from a combination of research studies.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EffectEvidenceSynthesis_ResultsByExposure {
  /// Reference to a RiskEvidenceSynthesis resource.
  #[serde(rename = "riskEvidenceSynthesis")]
  risk_evidence_synthesis: Reference,

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

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for exposureState
  #[serde(rename = "_exposureState")]
  _exposure_state: Element,

  /// Whether these results are for the exposure state or alternative exposure state.
  #[serde(rename = "exposureState")]
  exposure_state: EffectEvidenceSynthesis_ResultsByExposureExposureState,

  /// Human-readable summary of results by exposure state.
  description: String,

  /// Used to define variant exposure states such as low-risk state.
  #[serde(rename = "variantState")]
  variant_state: CodeableConcept,

}

#[derive(Debug, Serialize, Deserialize)]
enum EffectEvidenceSynthesis_ResultsByExposureExposureState {
  #[serde(rename = "exposure")]
  Exposure,

  #[serde(rename = "exposure-alternative")]
  ExposureAlternative,

}
