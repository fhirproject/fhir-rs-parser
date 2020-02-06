#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::RiskEvidenceSynthesis_PrecisionEstimate::RiskEvidenceSynthesis_PrecisionEstimate;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Element::Element;


/// The RiskEvidenceSynthesis resource describes the likelihood of an outcome in a
/// population plus exposure state where the risk estimate is derived from a
/// combination of research studies.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskEvidenceSynthesis_RiskEstimate {
  /// Extensions for denominatorCount
  #[serde(rename = "_denominatorCount")]
  _denominator_count: Option<Element>,

  /// The sample size for the group that was measured for this risk estimate.
  #[serde(rename = "denominatorCount")]
  denominator_count: Option<i32>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// The number of group members with the outcome of interest.
  #[serde(rename = "numeratorCount")]
  numerator_count: Option<i32>,

  /// Extensions for numeratorCount
  #[serde(rename = "_numeratorCount")]
  _numerator_count: Option<Element>,

  /// A description of the precision of the estimate for the effect.
  #[serde(rename = "precisionEstimate")]
  precision_estimate: Option<Vec<RiskEvidenceSynthesis_PrecisionEstimate>>,

  /// Human-readable summary of risk estimate.
  description: Option<String>,

  /// Examples include proportion and mean.
  #[serde(rename = "type")]
  fhir_type: Option<CodeableConcept>,

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
  modifier_extension: Option<Vec<Box<Extension>>>,

  /// The point estimate of the risk estimate.
  value: Option<f32>,

  /// Extensions for value
  #[serde(rename = "_value")]
  _value: Option<Element>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// Specifies the UCUM unit for the outcome.
  #[serde(rename = "unitOfMeasure")]
  unit_of_measure: Option<CodeableConcept>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

}
