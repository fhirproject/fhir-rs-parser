#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Identifier::Identifier;
use crate::model::Element::Element;
use crate::model::Reference::Reference;
use crate::model::Meta::Meta;
use crate::model::AdverseEvent_SuspectEntity::AdverseEvent_SuspectEntity;
use crate::model::Narrative::Narrative;
use crate::model::Extension::Extension;
use crate::model::ResourceList::ResourceList;


/// Actual or  potential/avoided event causing unintended physical injury resulting
/// from or contributed to by medical care, a research study or other healthcare
/// setting factors that requires additional monitoring, treatment, or
/// hospitalization, or that results in death.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEvent {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for actuality
  #[serde(rename = "_actuality")]
  _actuality: Option<Element>,

  /// Assessment whether this event was of real importance.
  seriousness: Option<CodeableConcept>,

  /// The date (and perhaps time) when the adverse event occurred.
  date: Option<String>,

  /// AdverseEvent.study.
  study: Option<Vec<Box<Reference>>>,

  /// Parties that may or should contribute or have contributed information to the
  /// adverse event, which can consist of one or more activities.  Such information
  /// includes information leading to the decision to perform the activity and how to
  /// perform the activity (e.g. consultant), information that the activity itself
  /// seeks to reveal (e.g. informant of clinical history), or information about what
  /// activity was performed (e.g. informant witness).
  contributor: Option<Vec<Box<Reference>>>,

  /// Describes the entity that is suspected to have caused the adverse event.
  #[serde(rename = "suspectEntity")]
  suspect_entity: Option<Vec<AdverseEvent_SuspectEntity>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// Whether the event actually happened, or just had the potential to. Note that
  /// this is independent of whether anyone was affected or harmed or how severely.
  actuality: Option<AdverseEventActuality>,

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
  modifier_extension: Option<Vec<Box<Extension>>>,

  /// Information on who recorded the adverse event.  May be the patient or a
  /// practitioner.
  recorder: Option<Box<Reference>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// The information about where the adverse event occurred.
  location: Option<Box<Reference>>,

  /// AdverseEvent.subjectMedicalHistory.
  #[serde(rename = "subjectMedicalHistory")]
  subject_medical_history: Option<Vec<Box<Reference>>>,

  /// Extensions for recordedDate
  #[serde(rename = "_recordedDate")]
  _recorded_date: Option<Element>,

  /// Business identifiers assigned to this adverse event by the performer or other
  /// systems which remain constant as the resource is updated and propagates from
  /// server to server.
  identifier: Option<Identifier>,

  /// AdverseEvent.referenceDocument.
  #[serde(rename = "referenceDocument")]
  reference_document: Option<Vec<Box<Reference>>>,

  /// Describes the severity of the adverse event, in relation to the subject.
  /// Contrast to AdverseEvent.seriousness - a severe rash might not be serious, but a
  /// mild heart problem is.
  severity: Option<CodeableConcept>,

  /// This subject or group impacted by the event.
  subject: Box<Reference>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// The date on which the existence of the AdverseEvent was first recorded.
  #[serde(rename = "recordedDate")]
  recorded_date: Option<String>,

  /// The overall type of event, intended for search and filtering purposes.
  category: Option<Vec<CodeableConcept>>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Includes information about the reaction that occurred as a result of exposure to
  /// a substance (for example, a drug or a chemical).
  #[serde(rename = "resultingCondition")]
  resulting_condition: Option<Vec<Box<Reference>>>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Estimated or actual date the AdverseEvent began, in the opinion of the reporter.
  detected: Option<String>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// The Encounter during which AdverseEvent was created or to which the creation of
  /// this record is tightly associated.
  encounter: Option<Box<Reference>>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Extensions for detected
  #[serde(rename = "_detected")]
  _detected: Option<Element>,

  /// This element defines the specific type of event that occurred or that was
  /// prevented from occurring.
  event: Option<CodeableConcept>,

  /// Describes the type of outcome from the adverse event.
  outcome: Option<CodeableConcept>,

  /// Extensions for date
  #[serde(rename = "_date")]
  _date: Option<Element>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum AdverseEventActuality {
  #[serde(rename = "actual")]
  Actual,

  #[serde(rename = "potential")]
  Potential,

}
