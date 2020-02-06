#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ResourceList::ResourceList;
use crate::model::Extension::Extension;
use crate::model::Narrative::Narrative;
use crate::model::Element::Element;
use crate::model::Reference::Reference;
use crate::model::Goal_Target::Goal_Target;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;


/// Describes the intended objective(s) for a patient, group or organization care,
/// for example, weight loss, restoring an activity of daily living, obtaining herd
/// immunity via immunization, meeting a process improvement objective, etc.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
  /// Identifies the patient, group or organization for whom the goal is being
  /// established.
  subject: Box<Reference>,

  /// Any comments related to the goal.
  note: Vec<Annotation>,

  /// Captures the reason for the current status.
  #[serde(rename = "statusReason")]
  status_reason: String,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The state of the goal throughout its lifecycle.
  #[serde(rename = "lifecycleStatus")]
  lifecycle_status: GoalLifecycleStatus,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Extensions for lifecycleStatus
  #[serde(rename = "_lifecycleStatus")]
  _lifecycle_status: Element,

  /// Extensions for language
  _language: Element,

  /// The identified conditions and other health record elements that are intended to
  /// be addressed by the goal.
  addresses: Vec<Box<Reference>>,

  /// Extensions for startDate
  #[serde(rename = "_startDate")]
  _start_date: Element,

  /// Details of what's changed (or not changed).
  #[serde(rename = "outcomeReference")]
  outcome_reference: Vec<Box<Reference>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.    Modifier
  /// extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// The base language in which the resource is written.
  language: String,

  /// Indicates what should be done by when.
  target: Vec<Goal_Target>,

  /// Identifies the change (or lack of change) at the point when the status of the
  /// goal is assessed.
  #[serde(rename = "outcomeCode")]
  outcome_code: Vec<CodeableConcept>,

  /// Describes the progression, or lack thereof, towards the goal against the target.
  #[serde(rename = "achievementStatus")]
  achievement_status: CodeableConcept,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// The date or event after which the goal should begin being pursued.
  #[serde(rename = "startDate")]
  start_date: String,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Business identifiers assigned to this goal by the performer or other systems
  /// which remain constant as the resource is updated and propagates from server to
  /// server.
  identifier: Vec<Identifier>,

  /// Human-readable and/or coded description of a specific desired objective of care,
  /// such as "control blood pressure" or "negotiate an obstacle course" or "dance
  /// with child at wedding".
  description: CodeableConcept,

  /// Extensions for statusDate
  #[serde(rename = "_statusDate")]
  _status_date: Element,

  /// Extensions for statusReason
  #[serde(rename = "_statusReason")]
  _status_reason: Element,

  /// Indicates a category the goal falls within.
  category: Vec<CodeableConcept>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// The date or event after which the goal should begin being pursued.
  #[serde(rename = "startCodeableConcept")]
  start_codeable_concept: CodeableConcept,

  /// Indicates whose goal this is - patient goal, practitioner goal, etc.
  #[serde(rename = "expressedBy")]
  expressed_by: Box<Reference>,

  /// Identifies the mutually agreed level of importance associated with
  /// reaching/sustaining the goal.
  priority: CodeableConcept,

  /// Identifies when the current status.  I.e. When initially created, when achieved,
  /// when cancelled, etc.
  #[serde(rename = "statusDate")]
  status_date: i32,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum GoalLifecycleStatus {
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
