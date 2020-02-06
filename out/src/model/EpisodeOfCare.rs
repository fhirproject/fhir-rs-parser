#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ResourceList::ResourceList;
use crate::model::Period::Period;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Reference::Reference;
use crate::model::Narrative::Narrative;
use crate::model::Identifier::Identifier;
use crate::model::EpisodeOfCare_StatusHistory::EpisodeOfCare_StatusHistory;
use crate::model::Meta::Meta;
use crate::model::EpisodeOfCare_Diagnosis::EpisodeOfCare_Diagnosis;
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// An association between a patient and an organization / healthcare provider(s)
/// during which time encounters may occur. The managing organization assumes a
/// level of responsibility for the patient during this time.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeOfCare {
  /// The interval during which the managing organization assumes the defined
  /// responsibility.
  period: Period,

  /// Referral Request(s) that are fulfilled by this EpisodeOfCare, incoming
  /// referrals.
  #[serde(rename = "referralRequest")]
  referral_request: Vec<Box<Reference>>,

  /// Extensions for language
  _language: Element,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// A classification of the type of episode of care; e.g. specialist referral,
  /// disease management, type of funded care.
  #[serde(rename = "type")]
  fhir_type: Vec<CodeableConcept>,

  /// The patient who is the focus of this episode of care.
  patient: Box<Reference>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The practitioner that is the care manager/care coordinator for this patient.
  #[serde(rename = "careManager")]
  care_manager: Box<Reference>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// The set of accounts that may be used for billing for this EpisodeOfCare.
  account: Vec<Box<Reference>>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// The base language in which the resource is written.
  language: String,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// The EpisodeOfCare may be known by different identifiers for different contexts
  /// of use, such as when an external agency is tracking the Episode for funding
  /// purposes.
  identifier: Vec<Identifier>,

  /// planned | waitlist | active | onhold | finished | cancelled.
  status: EpisodeOfCareStatus,

  /// The history of statuses that the EpisodeOfCare has been through (without
  /// requiring processing the history of the resource).
  #[serde(rename = "statusHistory")]
  status_history: Vec<EpisodeOfCare_StatusHistory>,

  /// The organization that has assumed the specific responsibilities for the
  /// specified duration.
  #[serde(rename = "managingOrganization")]
  managing_organization: Box<Reference>,

  /// Extensions for status
  _status: Element,

  /// The list of diagnosis relevant to this episode of care.
  diagnosis: Vec<EpisodeOfCare_Diagnosis>,

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

  /// The list of practitioners that may be facilitating this episode of care for
  /// specific purposes.
  team: Vec<Box<Reference>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum EpisodeOfCareStatus {
  #[serde(rename = "planned")]
  Planned,

  #[serde(rename = "waitlist")]
  Waitlist,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "onhold")]
  Onhold,

  #[serde(rename = "finished")]
  Finished,

  #[serde(rename = "cancelled")]
  Cancelled,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

}
