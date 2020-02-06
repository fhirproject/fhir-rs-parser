#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::RiskEvidenceSynthesis_PrecisionEstimate::RiskEvidenceSynthesis_PrecisionEstimate;


/// The RiskEvidenceSynthesis resource describes the likelihood of an outcome in a
/// population plus exposure state where the risk estimate is derived from a
/// combination of research studies.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskEvidenceSynthesis_RiskEstimate {
  /// Extensions for description
  _description: Element,

  /// The sample size for the group that was measured for this risk estimate.
  #[serde(rename = "denominatorCount")]
  denominator_count: i32,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for value
  _value: Element,

  /// Extensions for numeratorCount
  #[serde(rename = "_numeratorCount")]
  _numerator_count: Element,

  /// A description of the precision of the estimate for the effect.
  #[serde(rename = "precisionEstimate")]
  precision_estimate: Vec<RiskEvidenceSynthesis_PrecisionEstimate>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Examples include proportion and mean.
  #[serde(rename = "type")]
  fhir_type: CodeableConcept,

  /// Extensions for denominatorCount
  #[serde(rename = "_denominatorCount")]
  _denominator_count: Element,

  /// Specifies the UCUM unit for the outcome.
  #[serde(rename = "unitOfMeasure")]
  unit_of_measure: CodeableConcept,

  /// Human-readable summary of risk estimate.
  description: String,

  /// The number of group members with the outcome of interest.
  #[serde(rename = "numeratorCount")]
  numerator_count: i32,

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
  modifier_extension: Vec<Extension>,

  /// The point estimate of the risk estimate.
  value: f32,

}
