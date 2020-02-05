use serde::{Deserialize, Serialize};

/// Describes the intended objective(s) for a patient, group or organization care,
/// for example, weight loss, restoring an activity of daily living, obtaining herd
/// immunity via immunization, meeting a process improvement objective, etc.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Goal {
  /// Indicates a category the goal falls within.
  category: Vec<CodeableConcept>,

  /// Captures the reason for the current status.
  #[serde(rename = "statusReason")]
  status_reason: String,

  /// Business identifiers assigned to this goal by the performer or other systems
  /// which remain constant as the resource is updated and propagates from server to
  /// server.
  identifier: Vec<Identifier>,

  /// The date or event after which the goal should begin being pursued.
  #[serde(rename = "startCodeableConcept")]
  start_codeable_concept: CodeableConcept,

  /// Any comments related to the goal.
  note: Vec<Annotation>,

  /// Details of what's changed (or not changed).
  #[serde(rename = "outcomeReference")]
  outcome_reference: Vec<Reference>,

  /// Indicates what should be done by when.
  target: Vec<Goal_Target>,

  /// Identifies the change (or lack of change) at the point when the status of the
  /// goal is assessed.
  #[serde(rename = "outcomeCode")]
  outcome_code: Vec<CodeableConcept>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Extensions for statusDate
  #[serde(rename = "_statusDate")]
  _status_date: Element,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// The state of the goal throughout its lifecycle.
  #[serde(rename = "lifecycleStatus")]
  lifecycle_status: GoalLifecycleStatus,

  /// Extensions for language
  _language: Element,

  /// Extensions for statusReason
  #[serde(rename = "_statusReason")]
  _status_reason: Element,

  /// Identifies when the current status.  I.e. When initially created, when achieved,
  /// when cancelled, etc.
  #[serde(rename = "statusDate")]
  status_date: date,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Indicates whose goal this is - patient goal, practitioner goal, etc.
  #[serde(rename = "expressedBy")]
  expressed_by: Reference,

  /// The date or event after which the goal should begin being pursued.
  #[serde(rename = "startDate")]
  start_date: String,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Describes the progression, or lack thereof, towards the goal against the target.
  #[serde(rename = "achievementStatus")]
  achievement_status: CodeableConcept,

  /// Extensions for lifecycleStatus
  #[serde(rename = "_lifecycleStatus")]
  _lifecycle_status: Element,

  /// The base language in which the resource is written.
  language: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// Extensions for startDate
  #[serde(rename = "_startDate")]
  _start_date: Element,

  /// Identifies the mutually agreed level of importance associated with
  /// reaching/sustaining the goal.
  priority: CodeableConcept,

  /// Human-readable and/or coded description of a specific desired objective of care,
  /// such as "control blood pressure" or "negotiate an obstacle course" or "dance
  /// with child at wedding".
  description: CodeableConcept,

  /// The identified conditions and other health record elements that are intended to
  /// be addressed by the goal.
  addresses: Vec<Reference>,

  /// Identifies the patient, group or organization for whom the goal is being
  /// established.
  subject: Reference,

}

#[derive(Debug, Serialize, Deserialize)]
enum GoalLifecycleStatus {
  #[serde(rename = "proposed")]
  Proposed,

  #[serde(rename = "planned")]
  Planned,

  #[serde(rename = "accepted")]
  Accepted,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "on-hold")]
  OnHold,

  #[serde(rename = "completed")]
  Completed,

  #[serde(rename = "cancelled")]
  Cancelled,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

  #[serde(rename = "rejected")]
  Rejected,

}
