use serde::{Deserialize, Serialize};

/// A record of a clinical assessment performed to determine what problem(s) may
/// affect the patient and before planning the treatments or management strategies
/// that are best to manage a patient's condition. Assessments are often 1:1 with a
/// clinical consultation / encounter,  but this varies greatly depending on the
/// clinical workflow. This resource is called "ClinicalImpression" rather than
/// "ClinicalAssessment" to avoid confusion with the recording of assessment tools
/// such as Apgar score.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClinicalImpression {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The point in time or period over which the subject was assessed.
  #[serde(rename = "effectiveDateTime")]
  effective_date_time: String,

  /// Categorizes the type of clinical assessment performed.
  code: CodeableConcept,

  /// A reference to the last assessment that was conducted on this patient.
  /// Assessments are often/usually ongoing in nature; a care provider (practitioner
  /// or team) will make new assessments on an ongoing basis as new data arises or the
  /// patient's conditions changes.
  previous: Reference,

  /// Extensions for status
  _status: Element,

  /// Extensions for protocol
  _protocol: Vec<Element>,

  /// Extensions for effectiveDateTime
  #[serde(rename = "_effectiveDateTime")]
  _effective_date_time: Element,

  /// Reference to a specific published clinical protocol that was followed during
  /// this assessment, and/or that provides evidence in support of the diagnosis.
  protocol: Vec<String>,

  /// One or more sets of investigations (signs, symptoms, etc.). The actual grouping
  /// of investigations varies greatly depending on the type and context of the
  /// assessment. These investigations may include data generated during the
  /// assessment process, or data previously generated and recorded that is pertinent
  /// to the outcomes.
  investigation: Vec<ClinicalImpression_Investigation>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Business identifiers assigned to this clinical impression by the performer or
  /// other systems which remain constant as the resource is updated and propagates
  /// from server to server.
  identifier: Vec<Identifier>,

  /// Extensions for language
  _language: Element,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Extensions for summary
  _summary: Element,

  /// Captures the reason for the current state of the ClinicalImpression.
  #[serde(rename = "statusReason")]
  status_reason: CodeableConcept,

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

  /// A text summary of the investigations and the diagnosis.
  summary: String,

  /// The base language in which the resource is written.
  language: String,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// The Encounter during which this ClinicalImpression was created or to which the
  /// creation of this record is tightly associated.
  encounter: Reference,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Commentary about the impression, typically recorded after the impression itself
  /// was made, though supplemental notes by the original author could also appear.
  note: Vec<Annotation>,

  /// Indicates when the documentation of the assessment was complete.
  date: dateTime,

  /// Identifies the workflow status of the assessment.
  status: String,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// The point in time or period over which the subject was assessed.
  #[serde(rename = "effectivePeriod")]
  effective_period: Period,

  /// Specific findings or diagnoses that were considered likely or relevant to
  /// ongoing treatment.
  finding: Vec<ClinicalImpression_Finding>,

  /// Estimate of likely outcome.
  #[serde(rename = "prognosisCodeableConcept")]
  prognosis_codeable_concept: Vec<CodeableConcept>,

  /// RiskAssessment expressing likely outcome.
  #[serde(rename = "prognosisReference")]
  prognosis_reference: Vec<Reference>,

  /// Extensions for date
  _date: Element,

  /// A list of the relevant problems/conditions for a patient.
  problem: Vec<Reference>,

  /// Information supporting the clinical impression.
  #[serde(rename = "supportingInfo")]
  supporting_info: Vec<Reference>,

  /// The clinician performing the assessment.
  assessor: Reference,

  /// Extensions for description
  _description: Element,

  /// A summary of the context and/or cause of the assessment - why / where it was
  /// performed, and what patient events/status prompted it.
  description: String,

  /// The patient or group of individuals assessed as part of this record.
  subject: Reference,

}
