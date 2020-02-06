#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::CodeableConcept::CodeableConcept;


/// The EffectEvidenceSynthesis resource describes the difference in an outcome
/// between exposures states in a population where the effect estimate is derived
/// from a combination of research studies.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectEvidenceSynthesis_ResultsByExposure {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.    Modifier extensions
  /// SHALL NOT change the meaning of any elements on Resource or DomainResource
  /// (including cannot change the meaning of modifierExtension itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Option<Vec<Extension>>,

  /// Extensions for exposureState
  #[serde(rename = "_exposureState")]
  _exposure_state: Option<Element>,

  /// Human-readable summary of results by exposure state.
  description: Option<String>,

  /// Used to define variant exposure states such as low-risk state.
  #[serde(rename = "variantState")]
  variant_state: Option<CodeableConcept>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// Reference to a RiskEvidenceSynthesis resource.
  #[serde(rename = "riskEvidenceSynthesis")]
  risk_evidence_synthesis: Box<Reference>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Whether these results are for the exposure state or alternative exposure state.
  #[serde(rename = "exposureState")]
  exposure_state: Option<EffectEvidenceSynthesis_ResultsByExposureExposureState>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum EffectEvidenceSynthesis_ResultsByExposureExposureState {
  #[serde(rename = "exposure")]
  Exposure,

  #[serde(rename = "exposure-alternative")]
  ExposureAlternative,

}
