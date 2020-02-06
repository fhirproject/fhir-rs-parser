#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Reference::Reference;
use crate::model::Period::Period;
use crate::model::PlanDefinition_Condition::PlanDefinition_Condition;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Duration::Duration;
use crate::model::PlanDefinition_Participant::PlanDefinition_Participant;
use crate::model::Timing::Timing;
use crate::model::PlanDefinition_DynamicValue::PlanDefinition_DynamicValue;
use crate::model::Element::Element;
use crate::model::RelatedArtifact::RelatedArtifact;
use crate::model::PlanDefinition_RelatedAction::PlanDefinition_RelatedAction;
use crate::model::Range::Range;
use crate::model::Extension::Extension;
use crate::model::TriggerDefinition::TriggerDefinition;
use crate::model::Age::Age;
use crate::model::DataRequirement::DataRequirement;


/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical artifacts such as clinical decision
/// support rules, order sets and protocols.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinition_Action {
  /// Extensions for title
  #[serde(rename = "_title")]
  _title: Option<Element>,

  /// A brief description of the action used to provide a summary to display to the
  /// user.
  description: Option<String>,

  /// A user-visible prefix for the action.
  prefix: Option<String>,

  /// A text equivalent of the action to be performed. This provides a human-
  /// interpretable description of the action when the definition is consumed by
  /// a system that might not be capable of interpreting it dynamically.
  #[serde(rename = "textEquivalent")]
  text_equivalent: Option<String>,

  /// Extensions for textEquivalent
  #[serde(rename = "_textEquivalent")]
  _text_equivalent: Option<Element>,

  /// Didactic or other informational resources associated with the action that can be
  /// provided to the CDS recipient. Information resources can include inline text
  /// commentary and links to web resources.
  documentation: Option<Vec<RelatedArtifact>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Identifies goals that this action supports. The reference must be to a goal
  /// element defined within this plan definition.
  #[serde(rename = "goalId")]
  goal_id: Option<Vec<String>>,

  /// A code or group definition that describes the intended subject of the action and
  /// its children, if any.
  #[serde(rename = "subjectReference")]
  subject_reference: Option<Box<Reference>>,

  /// Extensions for priority
  #[serde(rename = "_priority")]
  _priority: Option<Element>,

  /// An optional value describing when the action should be performed.
  #[serde(rename = "timingRange")]
  timing_range: Option<Range>,

  /// Defines the selection behavior for the action and its children.
  #[serde(rename = "selectionBehavior")]
  selection_behavior: Option<PlanDefinition_ActionSelectionBehavior>,

  /// Extensions for precheckBehavior
  #[serde(rename = "_precheckBehavior")]
  _precheck_behavior: Option<Element>,

  /// A code that provides meaning for the action or action group. For example, a
  /// section may have a LOINC code for the section of a documentation template.
  code: Option<Vec<CodeableConcept>>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// Extensions for cardinalityBehavior
  #[serde(rename = "_cardinalityBehavior")]
  _cardinality_behavior: Option<Element>,

  /// Extensions for definitionCanonical
  #[serde(rename = "_definitionCanonical")]
  _definition_canonical: Option<Element>,

  /// A reference to a StructureMap resource that defines a transform that can be
  /// executed to produce the intent resource using the ActivityDefinition instance as
  /// the input.
  transform: Option<String>,

  /// Defines the grouping behavior for the action and its children.
  #[serde(rename = "groupingBehavior")]
  grouping_behavior: Option<PlanDefinition_ActionGroupingBehavior>,

  /// An optional value describing when the action should be performed.
  #[serde(rename = "timingDateTime")]
  timing_date_time: Option<String>,

  /// Defines the outputs of the action, if any.
  output: Option<Vec<DataRequirement>>,

  /// An optional value describing when the action should be performed.
  #[serde(rename = "timingPeriod")]
  timing_period: Option<Period>,

  /// Extensions for requiredBehavior
  #[serde(rename = "_requiredBehavior")]
  _required_behavior: Option<Element>,

  /// Defines whether the action can be selected multiple times.
  #[serde(rename = "cardinalityBehavior")]
  cardinality_behavior: Option<PlanDefinition_ActionCardinalityBehavior>,

  /// A reference to an ActivityDefinition that describes the action to be taken in
  /// detail, or a PlanDefinition that describes a series of actions to be taken.
  #[serde(rename = "definitionUri")]
  definition_uri: Option<String>,

  /// Indicates how quickly the action should be addressed with respect to other
  /// actions.
  priority: Option<String>,

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

  /// An optional value describing when the action should be performed.
  #[serde(rename = "timingDuration")]
  timing_duration: Option<Duration>,

  /// A description of when the action should be triggered.
  trigger: Option<Vec<TriggerDefinition>>,

  /// The title of the action displayed to a user.
  title: Option<String>,

  /// Extensions for groupingBehavior
  #[serde(rename = "_groupingBehavior")]
  _grouping_behavior: Option<Element>,

  /// Extensions for goalId
  #[serde(rename = "_goalId")]
  _goal_id: Option<Vec<Element>>,

  /// An expression that describes applicability criteria or start/stop conditions for
  /// the action.
  condition: Option<Vec<PlanDefinition_Condition>>,

  /// A relationship to another action such as "before" or "30-60 minutes after start
  /// of".
  #[serde(rename = "relatedAction")]
  related_action: Option<Vec<PlanDefinition_RelatedAction>>,

  /// Indicates who should participate in performing the action described.
  participant: Option<Vec<PlanDefinition_Participant>>,

  /// A description of why this action is necessary or appropriate.
  reason: Option<Vec<CodeableConcept>>,

  /// The type of action to perform (create, update, remove).
  #[serde(rename = "type")]
  fhir_type: Option<CodeableConcept>,

  /// Extensions for prefix
  #[serde(rename = "_prefix")]
  _prefix: Option<Element>,

  /// Extensions for selectionBehavior
  #[serde(rename = "_selectionBehavior")]
  _selection_behavior: Option<Element>,

  /// Defines the required behavior for the action.
  #[serde(rename = "requiredBehavior")]
  required_behavior: Option<PlanDefinition_ActionRequiredBehavior>,

  /// Defines input data requirements for the action.
  input: Option<Vec<DataRequirement>>,

  /// An optional value describing when the action should be performed.
  #[serde(rename = "timingAge")]
  timing_age: Option<Age>,

  /// Defines whether the action should usually be preselected.
  #[serde(rename = "precheckBehavior")]
  precheck_behavior: Option<PlanDefinition_ActionPrecheckBehavior>,

  /// Customizations that should be applied to the statically defined resource. For
  /// example, if the dosage of a medication must be computed based on the patient's
  /// weight, a customization would be used to specify an expression that calculated
  /// the weight, and the path on the resource that would contain the result.
  #[serde(rename = "dynamicValue")]
  dynamic_value: Option<Vec<PlanDefinition_DynamicValue>>,

  /// Sub actions that are contained within the action. The behavior of this action
  /// determines the functionality of the sub-actions. For example, a selection
  /// behavior of at-most-one indicates that of the sub-actions, at most one may be
  /// chosen as part of realizing the action definition.
  action: Option<Vec<PlanDefinition_Action>>,

  /// Extensions for definitionUri
  #[serde(rename = "_definitionUri")]
  _definition_uri: Option<Element>,

  /// A code or group definition that describes the intended subject of the action and
  /// its children, if any.
  #[serde(rename = "subjectCodeableConcept")]
  subject_codeable_concept: Option<CodeableConcept>,

  /// An optional value describing when the action should be performed.
  #[serde(rename = "timingTiming")]
  timing_timing: Option<Timing>,

  /// A reference to an ActivityDefinition that describes the action to be taken in
  /// detail, or a PlanDefinition that describes a series of actions to be taken.
  #[serde(rename = "definitionCanonical")]
  definition_canonical: Option<String>,

  /// Extensions for timingDateTime
  #[serde(rename = "_timingDateTime")]
  _timing_date_time: Option<Element>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum PlanDefinition_ActionSelectionBehavior {
  #[serde(rename = "any")]
  Any,

  #[serde(rename = "all")]
  All,

  #[serde(rename = "all-or-none")]
  AllOrNone,

  #[serde(rename = "exactly-one")]
  ExactlyOne,

  #[serde(rename = "at-most-one")]
  AtMostOne,

  #[serde(rename = "one-or-more")]
  OneOrMore,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum PlanDefinition_ActionGroupingBehavior {
  #[serde(rename = "visual-group")]
  VisualGroup,

  #[serde(rename = "logical-group")]
  LogicalGroup,

  #[serde(rename = "sentence-group")]
  SentenceGroup,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum PlanDefinition_ActionCardinalityBehavior {
  #[serde(rename = "single")]
  Single,

  #[serde(rename = "multiple")]
  Multiple,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum PlanDefinition_ActionRequiredBehavior {
  #[serde(rename = "must")]
  Must,

  #[serde(rename = "could")]
  Could,

  #[serde(rename = "must-unless-documented")]
  MustUnlessDocumented,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum PlanDefinition_ActionPrecheckBehavior {
  #[serde(rename = "yes")]
  Yes,

  #[serde(rename = "no")]
  No,

}
