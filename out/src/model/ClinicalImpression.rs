#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Period::Period;
use crate::model::Narrative::Narrative;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ClinicalImpression_Investigation::ClinicalImpression_Investigation;
use crate::model::Identifier::Identifier;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::ResourceList::ResourceList;
use crate::model::Meta::Meta;
use crate::model::ClinicalImpression_Finding::ClinicalImpression_Finding;
use crate::model::Reference::Reference;
use crate::model::Annotation::Annotation;


/// A record of a clinical assessment performed to determine what problem(s) may
/// affect the patient and before planning the treatments or management strategies
/// that are best to manage a patient's condition. Assessments are often 1:1 with a
/// clinical consultation / encounter,  but this varies greatly depending on the
/// clinical workflow. This resource is called "ClinicalImpression" rather than
/// "ClinicalAssessment" to avoid confusion with the recording of assessment tools
/// such as Apgar score.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClinicalImpression {
  /// The point in time or period over which the subject was assessed.
  #[serde(rename = "effectivePeriod")]
  effective_period: Period,

  /// Extensions for summary
  _summary: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// A summary of the context and/or cause of the assessment - why / where it was
  /// performed, and what patient events/status prompted it.
  description: String,

  /// Extensions for date
  _date: Element,

  /// A reference to the last assessment that was conducted on this patient.
  /// Assessments are often/usually ongoing in nature; a care provider (practitioner
  /// or team) will make new assessments on an ongoing basis as new data arises or the
  /// patient's conditions changes.
  previous: Box<Reference>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Extensions for description
  _description: Element,

  /// Extensions for effectiveDateTime
  #[serde(rename = "_effectiveDateTime")]
  _effective_date_time: Element,

  /// A list of the relevant problems/conditions for a patient.
  problem: Vec<Box<Reference>>,

  /// Business identifiers assigned to this clinical impression by the performer or
  /// other systems which remain constant as the resource is updated and propagates
  /// from server to server.
  identifier: Vec<Identifier>,

  /// A text summary of the investigations and the diagnosis.
  summary: String,

  /// RiskAssessment expressing likely outcome.
  #[serde(rename = "prognosisReference")]
  prognosis_reference: Vec<Box<Reference>>,

  /// Extensions for protocol
  _protocol: Vec<Element>,

  /// Captures the reason for the current state of the ClinicalImpression.
  #[serde(rename = "statusReason")]
  status_reason: CodeableConcept,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Information supporting the clinical impression.
  #[serde(rename = "supportingInfo")]
  supporting_info: Vec<Box<Reference>>,

  /// Extensions for language
  _language: Element,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Indicates when the documentation of the assessment was complete.
  date: String,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// The Encounter during which this ClinicalImpression was created or to which the
  /// creation of this record is tightly associated.
  encounter: Box<Reference>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Extensions for status
  _status: Element,

  /// One or more sets of investigations (signs, symptoms, etc.). The actual grouping
  /// of investigations varies greatly depending on the type and context of the
  /// assessment. These investigations may include data generated during the
  /// assessment process, or data previously generated and recorded that is pertinent
  /// to the outcomes.
  investigation: Vec<ClinicalImpression_Investigation>,

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

  /// Reference to a specific published clinical protocol that was followed during
  /// this assessment, and/or that provides evidence in support of the diagnosis.
  protocol: Vec<String>,

  /// Estimate of likely outcome.
  #[serde(rename = "prognosisCodeableConcept")]
  prognosis_codeable_concept: Vec<CodeableConcept>,

  /// Specific findings or diagnoses that were considered likely or relevant to
  /// ongoing treatment.
  finding: Vec<ClinicalImpression_Finding>,

  /// Commentary about the impression, typically recorded after the impression itself
  /// was made, though supplemental notes by the original author could also appear.
  note: Vec<Annotation>,

  /// The base language in which the resource is written.
  language: String,

  /// Categorizes the type of clinical assessment performed.
  code: CodeableConcept,

  /// Identifies the workflow status of the assessment.
  status: String,

  /// The point in time or period over which the subject was assessed.
  #[serde(rename = "effectiveDateTime")]
  effective_date_time: String,

  /// The clinician performing the assessment.
  assessor: Box<Reference>,

  /// The patient or group of individuals assessed as part of this record.
  subject: Box<Reference>,

}
