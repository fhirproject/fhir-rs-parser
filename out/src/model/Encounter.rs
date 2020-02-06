#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Encounter_Participant::Encounter_Participant;
use crate::model::Identifier::Identifier;
use crate::model::Encounter_ClassHistory::Encounter_ClassHistory;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Encounter_StatusHistory::Encounter_StatusHistory;
use crate::model::Duration::Duration;
use crate::model::Encounter_Location::Encounter_Location;
use crate::model::Meta::Meta;
use crate::model::Coding::Coding;
use crate::model::Reference::Reference;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Encounter_Hospitalization::Encounter_Hospitalization;
use crate::model::Encounter_Diagnosis::Encounter_Diagnosis;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::ResourceList::ResourceList;


/// An interaction between a patient and healthcare provider(s) for the purpose of
/// providing healthcare service(s) or assessing the health status of a patient.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encounter {
  /// The base language in which the resource is written.
  language: String,

  /// The patient or group present at the encounter.
  subject: Box<Reference>,

  /// Reason the encounter takes place, expressed as a code. For admissions, this can
  /// be used for a coded admission diagnosis.
  #[serde(rename = "reasonCode")]
  reason_code: Vec<CodeableConcept>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Reason the encounter takes place, expressed as a code. For admissions, this can
  /// be used for a coded admission diagnosis.
  #[serde(rename = "reasonReference")]
  reason_reference: Vec<Box<Reference>>,

  /// Extensions for language
  _language: Element,

  /// Broad categorization of the service that is to be provided (e.g. cardiology).
  #[serde(rename = "serviceType")]
  service_type: CodeableConcept,

  /// The list of diagnosis relevant to this encounter.
  diagnosis: Vec<Encounter_Diagnosis>,

  /// The set of accounts that may be used for billing for this Encounter.
  account: Vec<Box<Reference>>,

  /// The start and end time of the encounter.
  period: Period,

  /// List of locations where  the patient has been during this encounter.
  location: Vec<Encounter_Location>,

  /// Details about the admission to a healthcare service.
  hospitalization: Encounter_Hospitalization,

  /// The status history permits the encounter resource to contain the status history
  /// without needing to read through the historical versions of the resource, or even
  /// have the server store them.
  #[serde(rename = "statusHistory")]
  status_history: Vec<Encounter_StatusHistory>,

  /// Where a specific encounter should be classified as a part of a specific
  /// episode(s) of care this field should be used. This association can facilitate
  /// grouping of related encounters together for a specific purpose, such as
  /// government reporting, issue tracking, association via a common problem.  The
  /// association is recorded on the encounter as these are typically created after
  /// the episode of care and grouped on entry rather than editing the episode of care
  /// to append another encounter to it (the episode of care could span years).
  #[serde(rename = "episodeOfCare")]
  episode_of_care: Vec<Box<Reference>>,

  /// Specific type of encounter (e.g. e-mail consultation, surgical day-care, skilled
  /// nursing, rehabilitation).
  #[serde(rename = "type")]
  fhir_type: Vec<CodeableConcept>,

  /// Concepts representing classification of patient encounter such as ambulatory
  /// (outpatient), inpatient, emergency, home health or others due to local
  /// variations.
  class: Coding,

  /// The list of people responsible for providing the service.
  participant: Vec<Encounter_Participant>,

  /// The organization that is primarily responsible for this Encounter's services.
  /// This MAY be the same as the organization on the Patient record, however it could
  /// be different, such as if the actor performing the services was from an external
  /// organization (which may be billed seperately) for an external consultation.
  /// Refer to the example bundle showing an abbreviated set of Encounters for a
  /// colonoscopy.
  #[serde(rename = "serviceProvider")]
  service_provider: Box<Reference>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// The request this encounter satisfies (e.g. incoming referral or procedure
  /// request).
  #[serde(rename = "basedOn")]
  based_on: Vec<Box<Reference>>,

  /// The appointment that scheduled this encounter.
  appointment: Vec<Box<Reference>>,

  /// Indicates the urgency of the encounter.
  priority: CodeableConcept,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// Identifier(s) by which this encounter is known.
  identifier: Vec<Identifier>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Extensions for status
  _status: Element,

  /// Another Encounter of which this encounter is a part of (administratively or in
  /// time).
  #[serde(rename = "partOf")]
  part_of: Box<Reference>,

  /// planned | arrived | triaged | in-progress | onleave | finished | cancelled +.
  status: EncounterStatus,

  /// The class history permits the tracking of the encounters transitions without
  /// needing to go  through the resource history.  This would be used for a case
  /// where an admission starts of as an emergency encounter, then transitions into an
  /// inpatient scenario. Doing this and not restarting a new encounter ensures that
  /// any lab/diagnostic results can more easily follow the patient and not require
  /// re-processing and not get lost or cancelled during a kind of discharge from
  /// emergency to inpatient.
  #[serde(rename = "classHistory")]
  class_history: Vec<Encounter_ClassHistory>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Quantity of time the encounter lasted. This excludes the time during leaves of
  /// absence.
  length: Duration,

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

}

#[derive(Debug, Serialize, Deserialize)]
pub enum EncounterStatus {
  #[serde(rename = "planned")]
  Planned,

  #[serde(rename = "arrived")]
  Arrived,

  #[serde(rename = "triaged")]
  Triaged,

  #[serde(rename = "in-progress")]
  InProgress,

  #[serde(rename = "onleave")]
  Onleave,

  #[serde(rename = "finished")]
  Finished,

  #[serde(rename = "cancelled")]
  Cancelled,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

  #[serde(rename = "unknown")]
  Unknown,

}
