#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Task_Output::Task_Output;
use crate::model::Period::Period;
use crate::model::Identifier::Identifier;
use crate::model::Annotation::Annotation;
use crate::model::Task_Input::Task_Input;
use crate::model::Task_Restriction::Task_Restriction;
use crate::model::Narrative::Narrative;
use crate::model::Meta::Meta;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::CodeableConcept::CodeableConcept;


/// A task to be performed.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
  /// Individual organization or Device currently responsible for task execution.
  owner: Option<Box<Reference>>,

  /// Principal physical location where the this task is performed.
  location: Option<Box<Reference>>,

  /// Outputs produced by the Task.
  output: Option<Vec<Task_Output>>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// The request being actioned or the resource being manipulated by this task.
  focus: Option<Box<Reference>>,

  /// Indicates the "level" of actionability associated with the Task, i.e. i+R[9]Cs
  /// this a proposed task, a planned task, an actionable task, etc.
  intent: Option<TaskIntent>,

  /// A name or code (or both) briefly describing what the task involves.
  code: Option<CodeableConcept>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// A free-text description of what is to be performed.
  description: Option<String>,

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
  modifier_extension: Option<Vec<Extension>>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Task that this particular task is part of.
  #[serde(rename = "partOf")]
  part_of: Option<Vec<Box<Reference>>>,

  /// An explanation as to why this task is held, failed, was refused, etc.
  #[serde(rename = "statusReason")]
  status_reason: Option<CodeableConcept>,

  /// The current status of the task.
  status: Option<TaskStatus>,

  /// The healthcare event  (e.g. a patient and healthcare provider interaction)
  /// during which this task was created.
  encounter: Option<Box<Reference>>,

  /// The date and time of last modification to this task.
  #[serde(rename = "lastModified")]
  last_modified: Option<String>,

  /// The kind of participant that should perform the task.
  #[serde(rename = "performerType")]
  performer_type: Option<Vec<CodeableConcept>>,

  /// Indicates how quickly the Task should be addressed with respect to other
  /// requests.
  priority: Option<String>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// A resource reference indicating why this task needs to be performed.
  #[serde(rename = "reasonReference")]
  reason_reference: Option<Box<Reference>>,

  /// The URL pointing to a *FHIR*-defined protocol, guideline, orderset or other
  /// definition that is adhered to in whole or in part by this Task.
  #[serde(rename = "instantiatesCanonical")]
  instantiates_canonical: Option<String>,

  /// BasedOn refers to a higher-level authorization that triggered the creation of
  /// the task.  It references a "request" resource such as a ServiceRequest,
  /// MedicationRequest, ServiceRequest, CarePlan, etc. which is distinct from the
  /// "request" resource the task is seeking to fulfill.  This latter resource is
  /// referenced by FocusOn.  For example, based on a ServiceRequest (= BasedOn), a
  /// task is created to fulfill a procedureRequest ( = FocusOn ) to collect a
  /// specimen from a patient.
  #[serde(rename = "basedOn")]
  based_on: Option<Vec<Box<Reference>>>,

  /// Identifies the time action was first taken against the task (start) and/or the
  /// time final action was taken against the task prior to marking it as completed
  /// (end).
  #[serde(rename = "executionPeriod")]
  execution_period: Option<Period>,

  /// Contains business-specific nuances of the business state.
  #[serde(rename = "businessStatus")]
  business_status: Option<CodeableConcept>,

  /// Insurance plans, coverage extensions, pre-authorizations and/or pre-
  /// determinations that may be relevant to the Task.
  insurance: Option<Vec<Box<Reference>>>,

  /// Extensions for priority
  #[serde(rename = "_priority")]
  _priority: Option<Element>,

  /// The business identifier for this task.
  identifier: Option<Vec<Identifier>>,

  /// Extensions for intent
  #[serde(rename = "_intent")]
  _intent: Option<Element>,

  /// The URL pointing to an *externally* maintained  protocol, guideline, orderset or
  /// other definition that is adhered to in whole or in part by this Task.
  #[serde(rename = "instantiatesUri")]
  instantiates_uri: Option<String>,

  /// The entity who benefits from the performance of the service specified in the
  /// task (e.g., the patient).
  #[serde(rename = "for")]
  fhir_for: Option<Box<Reference>>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// A description or code indicating why this task needs to be performed.
  #[serde(rename = "reasonCode")]
  reason_code: Option<CodeableConcept>,

  /// Links to Provenance records for past versions of this Task that identify key
  /// state transitions or updates that are likely to be relevant to a user looking at
  /// the current version of the task.
  #[serde(rename = "relevantHistory")]
  relevant_history: Option<Vec<Box<Reference>>>,

  /// The date and time this task was created.
  #[serde(rename = "authoredOn")]
  authored_on: Option<String>,

  /// Additional information that may be needed in the execution of the task.
  input: Option<Vec<Task_Input>>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// Free-text information captured about the task as it progresses.
  note: Option<Vec<Annotation>>,

  /// An identifier that links together multiple tasks and other requests that were
  /// created in the same context.
  #[serde(rename = "groupIdentifier")]
  group_identifier: Option<Identifier>,

  /// If the Task.focus is a request resource and the task is seeking fulfillment
  /// (i.e. is asking for the request to be actioned), this element identifies any
  /// limitations on what parts of the referenced request should be actioned.
  restriction: Option<Task_Restriction>,

  /// The creator of the task.
  requester: Option<Box<Reference>>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// Extensions for instantiatesUri
  #[serde(rename = "_instantiatesUri")]
  _instantiates_uri: Option<Element>,

  /// Extensions for authoredOn
  #[serde(rename = "_authoredOn")]
  _authored_on: Option<Element>,

  /// Extensions for lastModified
  #[serde(rename = "_lastModified")]
  _last_modified: Option<Element>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskIntent {
  #[serde(rename = "unknown")]
  Unknown,

  #[serde(rename = "proposal")]
  Proposal,

  #[serde(rename = "plan")]
  Plan,

  #[serde(rename = "order")]
  Order,

  #[serde(rename = "original-order")]
  OriginalOrder,

  #[serde(rename = "reflex-order")]
  ReflexOrder,

  #[serde(rename = "filler-order")]
  FillerOrder,

  #[serde(rename = "instance-order")]
  InstanceOrder,

  #[serde(rename = "option")]
  Option,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "requested")]
  Requested,

  #[serde(rename = "received")]
  Received,

  #[serde(rename = "accepted")]
  Accepted,

  #[serde(rename = "rejected")]
  Rejected,

  #[serde(rename = "ready")]
  Ready,

  #[serde(rename = "cancelled")]
  Cancelled,

  #[serde(rename = "in-progress")]
  InProgress,

  #[serde(rename = "on-hold")]
  OnHold,

  #[serde(rename = "failed")]
  Failed,

  #[serde(rename = "completed")]
  Completed,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

}
