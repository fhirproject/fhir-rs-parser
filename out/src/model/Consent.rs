#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ResourceList::ResourceList;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::Element::Element;
use crate::model::Narrative::Narrative;
use crate::model::Meta::Meta;
use crate::model::Consent_Verification::Consent_Verification;
use crate::model::Identifier::Identifier;
use crate::model::Consent_Policy::Consent_Policy;
use crate::model::Consent_Provision::Consent_Provision;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Attachment::Attachment;


/// A record of a healthcare consumer’s  choices, which permits or denies identified
/// recipient(s) or recipient role(s) to perform one or more actions within a given
/// policy context, for specific purposes and periods of time.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Consent {
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

  /// An exception to the base policy of this consent. An exception can be an addition
  /// or removal of access permissions.
  provision: Consent_Provision,

  /// Indicates the current state of this consent.
  status: ConsentStatus,

  /// When this  Consent was issued / created / indexed.
  #[serde(rename = "dateTime")]
  date_time: String,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Either the Grantor, which is the entity responsible for granting the rights
  /// listed in a Consent Directive or the Grantee, which is the entity responsible
  /// for complying with the Consent Directive, including any obligations or
  /// limitations on authorizations and enforcement of prohibitions.
  performer: Vec<Box<Reference>>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Extensions for language
  _language: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for status
  _status: Element,

  /// Extensions for dateTime
  #[serde(rename = "_dateTime")]
  _date_time: Element,

  /// The source on which this consent statement is based. The source might be a
  /// scanned original paper form, or a reference to a consent that links back to such
  /// a source, a reference to a document repository (e.g. XDS) that stores the
  /// original consent document.
  #[serde(rename = "sourceAttachment")]
  source_attachment: Attachment,

  /// A reference to the specific base computable regulation or policy.
  #[serde(rename = "policyRule")]
  policy_rule: CodeableConcept,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// A selector of the type of consent being presented: ADR, Privacy, Treatment,
  /// Research.  This list is now extensible.
  scope: CodeableConcept,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The organization that manages the consent, and the framework within which it is
  /// executed.
  organization: Vec<Box<Reference>>,

  /// The patient/healthcare consumer to whom this consent applies.
  patient: Box<Reference>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Whether a treatment instruction (e.g. artificial respiration yes or no) was
  /// verified with the patient, his/her family or another authorized person.
  verification: Vec<Consent_Verification>,

  /// The source on which this consent statement is based. The source might be a
  /// scanned original paper form, or a reference to a consent that links back to such
  /// a source, a reference to a document repository (e.g. XDS) that stores the
  /// original consent document.
  #[serde(rename = "sourceReference")]
  source_reference: Box<Reference>,

  /// The base language in which the resource is written.
  language: String,

  /// A classification of the type of consents found in the statement. This element
  /// supports indexing and retrieval of consent statements.
  category: Vec<CodeableConcept>,

  /// Unique identifier for this copy of the Consent Statement.
  identifier: Vec<Identifier>,

  /// The references to the policies that are included in this consent scope. Policies
  /// may be organizational, but are often defined jurisdictionally, or in law.
  policy: Vec<Consent_Policy>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConsentStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "proposed")]
  Proposed,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "rejected")]
  Rejected,

  #[serde(rename = "inactive")]
  Inactive,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

}
