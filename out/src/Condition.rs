use serde::{Deserialize, Serialize};

/// A clinical condition, problem, diagnosis, or other event, situation, issue, or
/// clinical concept that has risen to a level of concern.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Condition {
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

  /// The Encounter during which this Condition was created or to which the creation
  /// of this record is tightly associated.
  encounter: Reference,

  /// The date or estimated date that the condition resolved or went into remission.
  /// This is called "abatement" because of the many overloaded connotations
  /// associated with "remission" or "resolution" - Conditions are never really
  /// resolved, but they can abate.
  #[serde(rename = "abatementPeriod")]
  abatement_period: Period,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// Estimated or actual date or date-time  the condition began, in the opinion of
  /// the clinician.
  #[serde(rename = "onsetPeriod")]
  onset_period: Period,

  /// Individual who is making the condition statement.
  asserter: Reference,

  /// A category assigned to the condition.
  category: Vec<CodeableConcept>,

  /// Estimated or actual date or date-time  the condition began, in the opinion of
  /// the clinician.
  #[serde(rename = "onsetAge")]
  onset_age: Age,

  /// The verification status to support the clinical status of the condition.
  #[serde(rename = "verificationStatus")]
  verification_status: CodeableConcept,

  /// The anatomical location where this condition manifests itself.
  #[serde(rename = "bodySite")]
  body_site: Vec<CodeableConcept>,

  /// Identification of the condition, problem or diagnosis.
  code: CodeableConcept,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Estimated or actual date or date-time  the condition began, in the opinion of
  /// the clinician.
  #[serde(rename = "onsetRange")]
  onset_range: Range,

  /// A subjective assessment of the severity of the condition as evaluated by the
  /// clinician.
  severity: CodeableConcept,

  /// Individual who recorded the record and takes responsibility for its content.
  recorder: Reference,

  /// Extensions for onsetDateTime
  #[serde(rename = "_onsetDateTime")]
  _onset_date_time: Element,

  /// The date or estimated date that the condition resolved or went into remission.
  /// This is called "abatement" because of the many overloaded connotations
  /// associated with "remission" or "resolution" - Conditions are never really
  /// resolved, but they can abate.
  #[serde(rename = "abatementDateTime")]
  abatement_date_time: String,

  /// The date or estimated date that the condition resolved or went into remission.
  /// This is called "abatement" because of the many overloaded connotations
  /// associated with "remission" or "resolution" - Conditions are never really
  /// resolved, but they can abate.
  #[serde(rename = "abatementRange")]
  abatement_range: Range,

  /// Extensions for abatementString
  #[serde(rename = "_abatementString")]
  _abatement_string: Element,

  /// Extensions for recordedDate
  #[serde(rename = "_recordedDate")]
  _recorded_date: Element,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Business identifiers assigned to this condition by the performer or other
  /// systems which remain constant as the resource is updated and propagates from
  /// server to server.
  identifier: Vec<Identifier>,

  /// The recordedDate represents when this particular Condition record was created in
  /// the system, which is often a system-generated date.
  #[serde(rename = "recordedDate")]
  recorded_date: dateTime,

  /// Extensions for abatementDateTime
  #[serde(rename = "_abatementDateTime")]
  _abatement_date_time: Element,

  /// Estimated or actual date or date-time  the condition began, in the opinion of
  /// the clinician.
  #[serde(rename = "onsetDateTime")]
  onset_date_time: String,

  /// Estimated or actual date or date-time  the condition began, in the opinion of
  /// the clinician.
  #[serde(rename = "onsetString")]
  onset_string: String,

  /// Supporting evidence / manifestations that are the basis of the Condition's
  /// verification status, such as evidence that confirmed or refuted the condition.
  evidence: Vec<Condition_Evidence>,

  /// Clinical stage or grade of a condition. May include formal severity assessments.
  stage: Vec<Condition_Stage>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for onsetString
  #[serde(rename = "_onsetString")]
  _onset_string: Element,

  /// The date or estimated date that the condition resolved or went into remission.
  /// This is called "abatement" because of the many overloaded connotations
  /// associated with "remission" or "resolution" - Conditions are never really
  /// resolved, but they can abate.
  #[serde(rename = "abatementAge")]
  abatement_age: Age,

  /// Extensions for language
  _language: Element,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// The base language in which the resource is written.
  language: String,

  /// Indicates the patient or group who the condition record is associated with.
  subject: Reference,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The clinical status of the condition.
  #[serde(rename = "clinicalStatus")]
  clinical_status: CodeableConcept,

  /// The date or estimated date that the condition resolved or went into remission.
  /// This is called "abatement" because of the many overloaded connotations
  /// associated with "remission" or "resolution" - Conditions are never really
  /// resolved, but they can abate.
  #[serde(rename = "abatementString")]
  abatement_string: String,

  /// Additional information about the Condition. This is a general notes/comments
  /// entry  for description of the Condition, its diagnosis and prognosis.
  note: Vec<Annotation>,

}
