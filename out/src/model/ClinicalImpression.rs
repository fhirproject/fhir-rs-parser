#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Reference::Reference;
use crate::model::Annotation::Annotation;
use crate::model::Period::Period;
use crate::model::ClinicalImpression_Finding::ClinicalImpression_Finding;
use crate::model::Identifier::Identifier;
use crate::model::ClinicalImpression_Investigation::ClinicalImpression_Investigation;
use crate::model::Element::Element;
use crate::model::Meta::Meta;
use crate::model::Extension::Extension;
use crate::model::Narrative::Narrative;
use crate::model::ResourceList::ResourceList;
use crate::model::CodeableConcept::CodeableConcept;


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
  /// The base language in which the resource is written.
  language: Option<String>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// One or more sets of investigations (signs, symptoms, etc.). The actual grouping
  /// of investigations varies greatly depending on the type and context of the
  /// assessment. These investigations may include data generated during the
  /// assessment process, or data previously generated and recorded that is pertinent
  /// to the outcomes.
  investigation: Option<Vec<ClinicalImpression_Investigation>>,

  /// Extensions for protocol
  #[serde(rename = "_protocol")]
  _protocol: Option<Vec<Element>>,

  /// Commentary about the impression, typically recorded after the impression itself
  /// was made, though supplemental notes by the original author could also appear.
  note: Option<Vec<Annotation>>,

  /// Business identifiers assigned to this clinical impression by the performer or
  /// other systems which remain constant as the resource is updated and propagates
  /// from server to server.
  identifier: Option<Vec<Identifier>>,

  /// Captures the reason for the current state of the ClinicalImpression.
  #[serde(rename = "statusReason")]
  status_reason: Option<CodeableConcept>,

  /// The point in time or period over which the subject was assessed.
  #[serde(rename = "effectivePeriod")]
  effective_period: Option<Period>,

  /// Reference to a specific published clinical protocol that was followed during
  /// this assessment, and/or that provides evidence in support of the diagnosis.
  protocol: Option<Vec<String>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// A summary of the context and/or cause of the assessment - why / where it was
  /// performed, and what patient events/status prompted it.
  description: Option<String>,

  /// Extensions for effectiveDateTime
  #[serde(rename = "_effectiveDateTime")]
  _effective_date_time: Option<Element>,

  /// A text summary of the investigations and the diagnosis.
  summary: Option<String>,

  /// A list of the relevant problems/conditions for a patient.
  problem: Option<Vec<Box<Reference>>>,

  /// Specific findings or diagnoses that were considered likely or relevant to
  /// ongoing treatment.
  finding: Option<Vec<ClinicalImpression_Finding>>,

  /// The Encounter during which this ClinicalImpression was created or to which the
  /// creation of this record is tightly associated.
  encounter: Option<Box<Reference>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Estimate of likely outcome.
  #[serde(rename = "prognosisCodeableConcept")]
  prognosis_codeable_concept: Option<Vec<CodeableConcept>>,

  /// Information supporting the clinical impression.
  #[serde(rename = "supportingInfo")]
  supporting_info: Option<Vec<Box<Reference>>>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// Indicates when the documentation of the assessment was complete.
  date: Option<String>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

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

  /// The point in time or period over which the subject was assessed.
  #[serde(rename = "effectiveDateTime")]
  effective_date_time: Option<String>,

  /// Identifies the workflow status of the assessment.
  status: Option<String>,

  /// RiskAssessment expressing likely outcome.
  #[serde(rename = "prognosisReference")]
  prognosis_reference: Option<Vec<Box<Reference>>>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// Extensions for summary
  #[serde(rename = "_summary")]
  _summary: Option<Element>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Categorizes the type of clinical assessment performed.
  code: Option<CodeableConcept>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The patient or group of individuals assessed as part of this record.
  subject: Box<Reference>,

  /// A reference to the last assessment that was conducted on this patient.
  /// Assessments are often/usually ongoing in nature; a care provider (practitioner
  /// or team) will make new assessments on an ongoing basis as new data arises or the
  /// patient's conditions changes.
  previous: Option<Box<Reference>>,

  /// Extensions for date
  #[serde(rename = "_date")]
  _date: Option<Element>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// The clinician performing the assessment.
  assessor: Option<Box<Reference>>,

}
