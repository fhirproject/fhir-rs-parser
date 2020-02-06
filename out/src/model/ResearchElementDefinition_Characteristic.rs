#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Expression::Expression;
use crate::model::Element::Element;
use crate::model::DataRequirement::DataRequirement;
use crate::model::Period::Period;
use crate::model::UsageContext::UsageContext;
use crate::model::Duration::Duration;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Timing::Timing;


/// The ResearchElementDefinition resource describes a "PICO" element that knowledge
/// (evidence, assertion, recommendation) is about.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchElementDefinition_Characteristic {
  /// Indicates what effective period the study covers.
  #[serde(rename = "participantEffectivePeriod")]
  participant_effective_period: Option<Period>,

  /// Extensions for exclude
  #[serde(rename = "_exclude")]
  _exclude: Option<Element>,

  /// Indicates how elements are aggregated within the study effective period.
  #[serde(rename = "studyEffectiveGroupMeasure")]
  study_effective_group_measure: Option<ResearchElementDefinition_CharacteristicStudyEffectiveGroupMeasure>,

  /// A narrative description of the time period the study covers.
  #[serde(rename = "studyEffectiveDescription")]
  study_effective_description: Option<String>,

  /// Indicates what effective period the study covers.
  #[serde(rename = "participantEffectiveDateTime")]
  participant_effective_date_time: Option<String>,

  /// Define members of the research element using Codes (such as condition,
  /// medication, or observation), Expressions ( using an expression language such as
  /// FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the
  /// last year).
  #[serde(rename = "definitionExpression")]
  definition_expression: Option<Expression>,

  /// Extensions for participantEffectiveGroupMeasure
  #[serde(rename = "_participantEffectiveGroupMeasure")]
  _participant_effective_group_measure: Option<Element>,

  /// Define members of the research element using Codes (such as condition,
  /// medication, or observation), Expressions ( using an expression language such as
  /// FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the
  /// last year).
  #[serde(rename = "definitionDataRequirement")]
  definition_data_requirement: Option<DataRequirement>,

  /// Extensions for participantEffectiveDescription
  #[serde(rename = "_participantEffectiveDescription")]
  _participant_effective_description: Option<Element>,

  /// Extensions for participantEffectiveDateTime
  #[serde(rename = "_participantEffectiveDateTime")]
  _participant_effective_date_time: Option<Element>,

  /// Extensions for studyEffectiveDescription
  #[serde(rename = "_studyEffectiveDescription")]
  _study_effective_description: Option<Element>,

  /// Indicates what effective period the study covers.
  #[serde(rename = "studyEffectivePeriod")]
  study_effective_period: Option<Period>,

  /// Use UsageContext to define the members of the population, such as Age Ranges,
  /// Genders, Settings.
  #[serde(rename = "usageContext")]
  usage_context: Option<Vec<UsageContext>>,

  /// Indicates duration from the study initiation.
  #[serde(rename = "studyEffectiveTimeFromStart")]
  study_effective_time_from_start: Option<Duration>,

  /// Indicates how elements are aggregated within the study effective period.
  #[serde(rename = "participantEffectiveGroupMeasure")]
  participant_effective_group_measure: Option<ResearchElementDefinition_CharacteristicParticipantEffectiveGroupMeasure>,

  /// Indicates what effective period the study covers.
  #[serde(rename = "studyEffectiveDuration")]
  study_effective_duration: Option<Duration>,

  /// Define members of the research element using Codes (such as condition,
  /// medication, or observation), Expressions ( using an expression language such as
  /// FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the
  /// last year).
  #[serde(rename = "definitionCodeableConcept")]
  definition_codeable_concept: Option<CodeableConcept>,

  /// Specifies the UCUM unit for the outcome.
  #[serde(rename = "unitOfMeasure")]
  unit_of_measure: Option<CodeableConcept>,

  /// Indicates what effective period the study covers.
  #[serde(rename = "studyEffectiveDateTime")]
  study_effective_date_time: Option<String>,

  /// A narrative description of the time period the study covers.
  #[serde(rename = "participantEffectiveDescription")]
  participant_effective_description: Option<String>,

  /// Extensions for studyEffectiveGroupMeasure
  #[serde(rename = "_studyEffectiveGroupMeasure")]
  _study_effective_group_measure: Option<Element>,

  /// When true, members with this characteristic are excluded from the element.
  exclude: Option<bool>,

  /// Indicates what effective period the study covers.
  #[serde(rename = "studyEffectiveTiming")]
  study_effective_timing: Option<Timing>,

  /// Define members of the research element using Codes (such as condition,
  /// medication, or observation), Expressions ( using an expression language such as
  /// FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the
  /// last year).
  #[serde(rename = "definitionCanonical")]
  definition_canonical: Option<String>,

  /// Indicates what effective period the study covers.
  #[serde(rename = "participantEffectiveDuration")]
  participant_effective_duration: Option<Duration>,

  /// Extensions for definitionCanonical
  #[serde(rename = "_definitionCanonical")]
  _definition_canonical: Option<Element>,

  /// Extensions for studyEffectiveDateTime
  #[serde(rename = "_studyEffectiveDateTime")]
  _study_effective_date_time: Option<Element>,

  /// Indicates duration from the participant's study entry.
  #[serde(rename = "participantEffectiveTimeFromStart")]
  participant_effective_time_from_start: Option<Duration>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

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

  /// Indicates what effective period the study covers.
  #[serde(rename = "participantEffectiveTiming")]
  participant_effective_timing: Option<Timing>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResearchElementDefinition_CharacteristicStudyEffectiveGroupMeasure {
  #[serde(rename = "mean")]
  Mean,

  #[serde(rename = "median")]
  Median,

  #[serde(rename = "mean-of-mean")]
  MeanOfMean,

  #[serde(rename = "mean-of-median")]
  MeanOfMedian,

  #[serde(rename = "median-of-mean")]
  MedianOfMean,

  #[serde(rename = "median-of-median")]
  MedianOfMedian,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResearchElementDefinition_CharacteristicParticipantEffectiveGroupMeasure {
  #[serde(rename = "mean")]
  Mean,

  #[serde(rename = "median")]
  Median,

  #[serde(rename = "mean-of-mean")]
  MeanOfMean,

  #[serde(rename = "mean-of-median")]
  MeanOfMedian,

  #[serde(rename = "median-of-mean")]
  MedianOfMean,

  #[serde(rename = "median-of-median")]
  MedianOfMedian,

}
