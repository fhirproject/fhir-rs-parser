#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Range::Range;
use crate::model::Extension::Extension;
use crate::model::Period::Period;


/// An assessment of the likely outcome(s) for a patient or other subject as well as
/// the likelihood of each outcome.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskAssessment_Prediction {
  /// Indicates the risk for this particular subject (with their specific
  /// characteristics) divided by the risk of the population in general.  (Numbers
  /// greater than 1 = higher risk than the population, numbers less than 1 = lower
  /// risk.).
  #[serde(rename = "relativeRisk")]
  relative_risk: Option<f32>,

  /// Additional information explaining the basis for the prediction.
  rationale: Option<String>,

  /// Extensions for rationale
  #[serde(rename = "_rationale")]
  _rationale: Option<Element>,

  /// One of the potential outcomes for the patient (e.g. remission, death,  a
  /// particular condition).
  outcome: Option<CodeableConcept>,

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

  /// Extensions for relativeRisk
  #[serde(rename = "_relativeRisk")]
  _relative_risk: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for probabilityDecimal
  #[serde(rename = "_probabilityDecimal")]
  _probability_decimal: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Indicates how likely the outcome is (in the specified timeframe).
  #[serde(rename = "probabilityRange")]
  probability_range: Option<Range>,

  /// Indicates how likely the outcome is (in the specified timeframe).
  #[serde(rename = "probabilityDecimal")]
  probability_decimal: Option<i32>,

  /// Indicates how likely the outcome is (in the specified timeframe), expressed as a
  /// qualitative value (e.g. low, medium, or high).
  #[serde(rename = "qualitativeRisk")]
  qualitative_risk: Option<CodeableConcept>,

  /// Indicates the period of time or age range of the subject to which the specified
  /// probability applies.
  #[serde(rename = "whenRange")]
  when_range: Option<Range>,

  /// Indicates the period of time or age range of the subject to which the specified
  /// probability applies.
  #[serde(rename = "whenPeriod")]
  when_period: Option<Period>,

}
