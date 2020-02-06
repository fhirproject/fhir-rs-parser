#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::UsageContext::UsageContext;
use crate::model::Duration::Duration;
use crate::model::Timing::Timing;
use crate::model::Period::Period;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::TriggerDefinition::TriggerDefinition;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::DataRequirement::DataRequirement;
use crate::model::Expression::Expression;


/// The EvidenceVariable resource describes a "PICO" element that knowledge
/// (evidence, assertion, recommendation) is about.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceVariable_Characteristic {
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

  /// Define members of the evidence element using Codes (such as condition,
  /// medication, or observation), Expressions ( using an expression language such as
  /// FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the
  /// last year).
  #[serde(rename = "definitionReference")]
  definition_reference: Box<Reference>,

  /// Extensions for description
  _description: Element,

  /// Define members of the evidence element using Codes (such as condition,
  /// medication, or observation), Expressions ( using an expression language such as
  /// FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the
  /// last year).
  #[serde(rename = "definitionExpression")]
  definition_expression: Expression,

  /// Extensions for exclude
  _exclude: Element,

  /// Define members of the evidence element using Codes (such as condition,
  /// medication, or observation), Expressions ( using an expression language such as
  /// FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the
  /// last year).
  #[serde(rename = "definitionCodeableConcept")]
  definition_codeable_concept: CodeableConcept,

  /// Define members of the evidence element using Codes (such as condition,
  /// medication, or observation), Expressions ( using an expression language such as
  /// FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the
  /// last year).
  #[serde(rename = "definitionDataRequirement")]
  definition_data_requirement: DataRequirement,

  /// Extensions for participantEffectiveDateTime
  #[serde(rename = "_participantEffectiveDateTime")]
  _participant_effective_date_time: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Indicates what effective period the study covers.
  #[serde(rename = "participantEffectiveDuration")]
  participant_effective_duration: Duration,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Indicates what effective period the study covers.
  #[serde(rename = "participantEffectiveDateTime")]
  participant_effective_date_time: String,

  /// Define members of the evidence element using Codes (such as condition,
  /// medication, or observation), Expressions ( using an expression language such as
  /// FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the
  /// last year).
  #[serde(rename = "definitionCanonical")]
  definition_canonical: String,

  /// Define members of the evidence element using Codes (such as condition,
  /// medication, or observation), Expressions ( using an expression language such as
  /// FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the
  /// last year).
  #[serde(rename = "definitionTriggerDefinition")]
  definition_trigger_definition: TriggerDefinition,

  /// Extensions for definitionCanonical
  #[serde(rename = "_definitionCanonical")]
  _definition_canonical: Element,

  /// Indicates what effective period the study covers.
  #[serde(rename = "participantEffectiveTiming")]
  participant_effective_timing: Timing,

  /// Indicates duration from the participant's study entry.
  #[serde(rename = "timeFromStart")]
  time_from_start: Duration,

  /// Indicates how elements are aggregated within the study effective period.
  #[serde(rename = "groupMeasure")]
  group_measure: EvidenceVariable_CharacteristicGroupMeasure,

  /// Indicates what effective period the study covers.
  #[serde(rename = "participantEffectivePeriod")]
  participant_effective_period: Period,

  /// A short, natural language description of the characteristic that could be used
  /// to communicate the criteria to an end-user.
  description: String,

  /// When true, members with this characteristic are excluded from the element.
  exclude: bool,

  /// Extensions for groupMeasure
  #[serde(rename = "_groupMeasure")]
  _group_measure: Element,

  /// Use UsageContext to define the members of the population, such as Age Ranges,
  /// Genders, Settings.
  #[serde(rename = "usageContext")]
  usage_context: Vec<UsageContext>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum EvidenceVariable_CharacteristicGroupMeasure {
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
