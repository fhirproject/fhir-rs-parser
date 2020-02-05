use serde::{Deserialize, Serialize};

/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical artifacts such as clinical decision
/// support rules, order sets and protocols.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlanDefinition_Action {
  /// Indicates who should participate in performing the action described.
  participant: Vec<PlanDefinition_Participant>,

  /// A reference to a StructureMap resource that defines a transform that can be
  /// executed to produce the intent resource using the ActivityDefinition instance as
  /// the input.
  transform: canonical,

  /// The title of the action displayed to a user.
  title: String,

  /// Extensions for description
  _description: Element,

  /// A code or group definition that describes the intended subject of the action and
  /// its children, if any.
  #[serde(rename = "subjectReference")]
  subject_reference: Reference,

  /// Extensions for prefix
  _prefix: Element,

  /// Didactic or other informational resources associated with the action that can be
  /// provided to the CDS recipient. Information resources can include inline text
  /// commentary and links to web resources.
  documentation: Vec<RelatedArtifact>,

  /// An expression that describes applicability criteria or start/stop conditions for
  /// the action.
  condition: Vec<PlanDefinition_Condition>,

  /// An optional value describing when the action should be performed.
  #[serde(rename = "timingRange")]
  timing_range: Range,

  /// A user-visible prefix for the action.
  prefix: String,

  /// The type of action to perform (create, update, remove).
  type: CodeableConcept,

  /// Extensions for requiredBehavior
  #[serde(rename = "_requiredBehavior")]
  _required_behavior: Element,

  /// Extensions for goalId
  #[serde(rename = "_goalId")]
  _goal_id: Vec<Element>,

  /// Identifies goals that this action supports. The reference must be to a goal
  /// element defined within this plan definition.
  #[serde(rename = "goalId")]
  goal_id: Vec<id>,

  /// A reference to an ActivityDefinition that describes the action to be taken in
  /// detail, or a PlanDefinition that describes a series of actions to be taken.
  #[serde(rename = "definitionCanonical")]
  definition_canonical: String,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Defines input data requirements for the action.
  input: Vec<DataRequirement>,

  /// Extensions for timingDateTime
  #[serde(rename = "_timingDateTime")]
  _timing_date_time: Element,

  /// Extensions for textEquivalent
  #[serde(rename = "_textEquivalent")]
  _text_equivalent: Element,

  /// A code that provides meaning for the action or action group. For example, a
  /// section may have a LOINC code for the section of a documentation template.
  code: Vec<CodeableConcept>,

  /// An optional value describing when the action should be performed.
  #[serde(rename = "timingPeriod")]
  timing_period: Period,

  /// Defines the required behavior for the action.
  #[serde(rename = "requiredBehavior")]
  required_behavior: PlanDefinition_ActionRequiredBehavior,

  /// Defines whether the action can be selected multiple times.
  #[serde(rename = "cardinalityBehavior")]
  cardinality_behavior: PlanDefinition_ActionCardinalityBehavior,

  /// Extensions for priority
  _priority: Element,

  /// An optional value describing when the action should be performed.
  #[serde(rename = "timingTiming")]
  timing_timing: Timing,

  /// Defines the grouping behavior for the action and its children.
  #[serde(rename = "groupingBehavior")]
  grouping_behavior: PlanDefinition_ActionGroupingBehavior,

  /// A reference to an ActivityDefinition that describes the action to be taken in
  /// detail, or a PlanDefinition that describes a series of actions to be taken.
  #[serde(rename = "definitionUri")]
  definition_uri: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Defines the outputs of the action, if any.
  output: Vec<DataRequirement>,

  /// Indicates how quickly the action should be addressed with respect to other
  /// actions.
  priority: String,

  /// A relationship to another action such as "before" or "30-60 minutes after start
  /// of".
  #[serde(rename = "relatedAction")]
  related_action: Vec<PlanDefinition_RelatedAction>,

  /// Extensions for groupingBehavior
  #[serde(rename = "_groupingBehavior")]
  _grouping_behavior: Element,

  /// Extensions for precheckBehavior
  #[serde(rename = "_precheckBehavior")]
  _precheck_behavior: Element,

  /// Extensions for cardinalityBehavior
  #[serde(rename = "_cardinalityBehavior")]
  _cardinality_behavior: Element,

  /// Extensions for definitionCanonical
  #[serde(rename = "_definitionCanonical")]
  _definition_canonical: Element,

  /// Sub actions that are contained within the action. The behavior of this action
  /// determines the functionality of the sub-actions. For example, a selection
  /// behavior of at-most-one indicates that of the sub-actions, at most one may be
  /// chosen as part of realizing the action definition.
  action: Vec<PlanDefinition_Action>,

  /// Defines the selection behavior for the action and its children.
  #[serde(rename = "selectionBehavior")]
  selection_behavior: PlanDefinition_ActionSelectionBehavior,

  /// Customizations that should be applied to the statically defined resource. For
  /// example, if the dosage of a medication must be computed based on the patient's
  /// weight, a customization would be used to specify an expression that calculated
  /// the weight, and the path on the resource that would contain the result.
  #[serde(rename = "dynamicValue")]
  dynamic_value: Vec<PlanDefinition_DynamicValue>,

  /// Extensions for definitionUri
  #[serde(rename = "_definitionUri")]
  _definition_uri: Element,

  /// Defines whether the action should usually be preselected.
  #[serde(rename = "precheckBehavior")]
  precheck_behavior: PlanDefinition_ActionPrecheckBehavior,

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

  /// Extensions for selectionBehavior
  #[serde(rename = "_selectionBehavior")]
  _selection_behavior: Element,

  /// A description of why this action is necessary or appropriate.
  reason: Vec<CodeableConcept>,

  /// An optional value describing when the action should be performed.
  #[serde(rename = "timingDateTime")]
  timing_date_time: String,

  /// Extensions for title
  _title: Element,

  /// A code or group definition that describes the intended subject of the action and
  /// its children, if any.
  #[serde(rename = "subjectCodeableConcept")]
  subject_codeable_concept: CodeableConcept,

  /// An optional value describing when the action should be performed.
  #[serde(rename = "timingAge")]
  timing_age: Age,

  /// An optional value describing when the action should be performed.
  #[serde(rename = "timingDuration")]
  timing_duration: Duration,

  /// A brief description of the action used to provide a summary to display to the
  /// user.
  description: String,

  /// A text equivalent of the action to be performed. This provides a human-
  /// interpretable description of the action when the definition is consumed by
  /// a system that might not be capable of interpreting it dynamically.
  #[serde(rename = "textEquivalent")]
  text_equivalent: String,

  /// A description of when the action should be triggered.
  trigger: Vec<TriggerDefinition>,

}

#[derive(Debug, Serialize, Deserialize)]
enum PlanDefinition_ActionRequiredBehavior {
  #[serde(rename = "must")]
  Must,

  #[serde(rename = "could")]
  Could,

  #[serde(rename = "must-unless-documented")]
  MustUnlessDocumented,

}

#[derive(Debug, Serialize, Deserialize)]
enum PlanDefinition_ActionCardinalityBehavior {
  #[serde(rename = "single")]
  Single,

  #[serde(rename = "multiple")]
  Multiple,

}

#[derive(Debug, Serialize, Deserialize)]
enum PlanDefinition_ActionGroupingBehavior {
  #[serde(rename = "visual-group")]
  VisualGroup,

  #[serde(rename = "logical-group")]
  LogicalGroup,

  #[serde(rename = "sentence-group")]
  SentenceGroup,

}

#[derive(Debug, Serialize, Deserialize)]
enum PlanDefinition_ActionSelectionBehavior {
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
enum PlanDefinition_ActionPrecheckBehavior {
  #[serde(rename = "yes")]
  Yes,

  #[serde(rename = "no")]
  No,

}
