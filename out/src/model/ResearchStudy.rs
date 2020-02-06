#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Meta::Meta;
use crate::model::ResourceList::ResourceList;
use crate::model::Identifier::Identifier;
use crate::model::ResearchStudy_Objective::ResearchStudy_Objective;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ResearchStudy_Arm::ResearchStudy_Arm;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::Annotation::Annotation;
use crate::model::Extension::Extension;
use crate::model::Narrative::Narrative;
use crate::model::RelatedArtifact::RelatedArtifact;
use crate::model::ContactDetail::ContactDetail;
use crate::model::Element::Element;


/// A process where a researcher or organization plans and then executes a series of
/// steps intended to increase the field of healthcare-related knowledge.  This
/// includes studies of safety, efficacy, comparative effectiveness and other
/// information about medications, devices, therapies and other interventional and
/// investigative techniques.  A ResearchStudy involves the gathering of information
/// about human or animal subjects.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudy {
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

  /// The type of study based upon the intent of the study's activities. A
  /// classification of the intent of the study.
  #[serde(rename = "primaryPurposeType")]
  primary_purpose_type: CodeableConcept,

  /// A goal that the study is aiming to achieve in terms of a scientific question to
  /// be answered by the analysis of data collected during the study.
  objective: Vec<ResearchStudy_Objective>,

  /// The current state of the study.
  status: ResearchStudyStatus,

  /// The condition that is the focus of the study.  For example, In a study to
  /// examine risk factors for Lupus, might have as an inclusion criterion "healthy
  /// volunteer", but the target condition code would be a Lupus SNOMED code.
  condition: Vec<CodeableConcept>,

  /// A short, descriptive user-friendly label for the study.
  title: String,

  /// The base language in which the resource is written.
  language: String,

  /// A full description of how the study is being conducted.
  description: String,

  /// Extensions for language
  _language: Element,

  /// Describes an expected sequence of events for one of the participants of a study.
  /// E.g. Exposure to drug A, wash-out, exposure to drug B, wash-out, follow-up.
  arm: Vec<ResearchStudy_Arm>,

  /// Citations, references and other related documents.
  #[serde(rename = "relatedArtifact")]
  related_artifact: Vec<RelatedArtifact>,

  /// An organization that initiates the investigation and is legally responsible for
  /// the study.
  sponsor: Box<Reference>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Identifies the start date and the expected (or actual, depending on status) end
  /// date for the study.
  period: Period,

  /// Extensions for title
  _title: Element,

  /// The medication(s), food(s), therapy(ies), device(s) or other concerns or
  /// interventions that the study is seeking to gain more information about.
  focus: Vec<CodeableConcept>,

  /// A facility in which study activities are conducted.
  site: Vec<Box<Reference>>,

  /// Identifiers assigned to this research study by the sponsor or other systems.
  identifier: Vec<Identifier>,

  /// A researcher in a study who oversees multiple aspects of the study, such as
  /// concept development, protocol writing, protocol submission for IRB approval,
  /// participant recruitment, informed consent, data collection, analysis,
  /// interpretation and presentation.
  #[serde(rename = "principalInvestigator")]
  principal_investigator: Box<Reference>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Extensions for status
  _status: Element,

  /// The stage in the progression of a therapy from initial experimental use in
  /// humans in clinical trials to post-market evaluation.
  phase: CodeableConcept,

  /// Contact details to assist a user in learning more about or engaging with the
  /// study.
  contact: Vec<ContactDetail>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// A description and/or code explaining the premature termination of the study.
  #[serde(rename = "reasonStopped")]
  reason_stopped: CodeableConcept,

  /// Comments made about the study by the performer, subject or other participants.
  note: Vec<Annotation>,

  /// Reference to a Group that defines the criteria for and quantity of subjects
  /// participating in the study.  E.g. " 200 female Europeans between the ages of 20
  /// and 45 with early onset diabetes".
  enrollment: Vec<Box<Reference>>,

  /// Indicates a country, state or other region where the study is taking place.
  location: Vec<CodeableConcept>,

  /// A larger research study of which this particular study is a component or step.
  #[serde(rename = "partOf")]
  part_of: Vec<Box<Reference>>,

  /// Key terms to aid in searching for or filtering the study.
  keyword: Vec<CodeableConcept>,

  /// Codes categorizing the type of study such as investigational vs. observational,
  /// type of blinding, type of randomization, safety vs. efficacy, etc.
  category: Vec<CodeableConcept>,

  /// The set of steps expected to be performed as part of the execution of the study.
  protocol: Vec<Box<Reference>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Extensions for description
  _description: Element,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResearchStudyStatus {
  #[serde(rename = "active")]
  Active,

  #[serde(rename = "administratively-completed")]
  AdministrativelyCompleted,

  #[serde(rename = "approved")]
  Approved,

  #[serde(rename = "closed-to-accrual")]
  ClosedToAccrual,

  #[serde(rename = "closed-to-accrual-and-intervention")]
  ClosedToAccrualAndIntervention,

  #[serde(rename = "completed")]
  Completed,

  #[serde(rename = "disapproved")]
  Disapproved,

  #[serde(rename = "in-review")]
  InReview,

  #[serde(rename = "temporarily-closed-to-accrual")]
  TemporarilyClosedToAccrual,

  #[serde(rename = "temporarily-closed-to-accrual-and-intervention")]
  TemporarilyClosedToAccrualAndIntervention,

  #[serde(rename = "withdrawn")]
  Withdrawn,

}
