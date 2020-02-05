use serde::{Deserialize, Serialize};

/// A task to be performed.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Task {
  /// Links to Provenance records for past versions of this Task that identify key
  /// state transitions or updates that are likely to be relevant to a user looking at
  /// the current version of the task.
  #[serde(rename = "relevantHistory")]
  relevant_history: Vec<Reference>,

  /// If the Task.focus is a request resource and the task is seeking fulfillment
  /// (i.e. is asking for the request to be actioned), this element identifies any
  /// limitations on what parts of the referenced request should be actioned.
  restriction: Task_Restriction,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// A resource reference indicating why this task needs to be performed.
  #[serde(rename = "reasonReference")]
  reason_reference: Reference,

  /// Additional information that may be needed in the execution of the task.
  input: Vec<Task_Input>,

  /// Extensions for lastModified
  #[serde(rename = "_lastModified")]
  _last_modified: Element,

  /// Principal physical location where the this task is performed.
  location: Reference,

  /// The creator of the task.
  requester: Reference,

  /// Extensions for instantiatesUri
  #[serde(rename = "_instantiatesUri")]
  _instantiates_uri: Element,

  /// Extensions for intent
  _intent: Element,

  /// The URL pointing to an *externally* maintained  protocol, guideline, orderset or
  /// other definition that is adhered to in whole or in part by this Task.
  #[serde(rename = "instantiatesUri")]
  instantiates_uri: String,

  /// The URL pointing to a *FHIR*-defined protocol, guideline, orderset or other
  /// definition that is adhered to in whole or in part by this Task.
  #[serde(rename = "instantiatesCanonical")]
  instantiates_canonical: canonical,

  /// The entity who benefits from the performance of the service specified in the
  /// task (e.g., the patient).
  for: Reference,

  /// Indicates the "level" of actionability associated with the Task, i.e. i+R[9]Cs
  /// this a proposed task, a planned task, an actionable task, etc.
  intent: TaskIntent,

  /// Extensions for authoredOn
  #[serde(rename = "_authoredOn")]
  _authored_on: Element,

  /// Free-text information captured about the task as it progresses.
  note: Vec<Annotation>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// The current status of the task.
  status: TaskStatus,

  /// The request being actioned or the resource being manipulated by this task.
  focus: Reference,

  /// Extensions for status
  _status: Element,

  /// A free-text description of what is to be performed.
  description: String,

  /// Outputs produced by the Task.
  output: Vec<Task_Output>,

  /// Insurance plans, coverage extensions, pre-authorizations and/or pre-
  /// determinations that may be relevant to the Task.
  insurance: Vec<Reference>,

  /// An explanation as to why this task is held, failed, was refused, etc.
  #[serde(rename = "statusReason")]
  status_reason: CodeableConcept,

  /// Contains business-specific nuances of the business state.
  #[serde(rename = "businessStatus")]
  business_status: CodeableConcept,

  /// The business identifier for this task.
  identifier: Vec<Identifier>,

  /// Indicates how quickly the Task should be addressed with respect to other
  /// requests.
  priority: String,

  /// Task that this particular task is part of.
  #[serde(rename = "partOf")]
  part_of: Vec<Reference>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// Extensions for language
  _language: Element,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// BasedOn refers to a higher-level authorization that triggered the creation of
  /// the task.  It references a "request" resource such as a ServiceRequest,
  /// MedicationRequest, ServiceRequest, CarePlan, etc. which is distinct from the
  /// "request" resource the task is seeking to fulfill.  This latter resource is
  /// referenced by FocusOn.  For example, based on a ServiceRequest (= BasedOn), a
  /// task is created to fulfill a procedureRequest ( = FocusOn ) to collect a
  /// specimen from a patient.
  #[serde(rename = "basedOn")]
  based_on: Vec<Reference>,

  /// Extensions for description
  _description: Element,

  /// Extensions for priority
  _priority: Element,

  /// The healthcare event  (e.g. a patient and healthcare provider interaction)
  /// during which this task was created.
  encounter: Reference,

  /// The base language in which the resource is written.
  language: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

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

  /// A name or code (or both) briefly describing what the task involves.
  code: CodeableConcept,

  /// An identifier that links together multiple tasks and other requests that were
  /// created in the same context.
  #[serde(rename = "groupIdentifier")]
  group_identifier: Identifier,

  /// Identifies the time action was first taken against the task (start) and/or the
  /// time final action was taken against the task prior to marking it as completed
  /// (end).
  #[serde(rename = "executionPeriod")]
  execution_period: Period,

  /// The date and time this task was created.
  #[serde(rename = "authoredOn")]
  authored_on: dateTime,

  /// The date and time of last modification to this task.
  #[serde(rename = "lastModified")]
  last_modified: dateTime,

  /// The kind of participant that should perform the task.
  #[serde(rename = "performerType")]
  performer_type: Vec<CodeableConcept>,

  /// Individual organization or Device currently responsible for task execution.
  owner: Reference,

  /// A description or code indicating why this task needs to be performed.
  #[serde(rename = "reasonCode")]
  reason_code: CodeableConcept,

}

#[derive(Debug, Serialize, Deserialize)]
enum TaskIntent {
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
enum TaskStatus {
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
