#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Reference::Reference;
use crate::model::Meta::Meta;
use crate::model::ResourceList::ResourceList;
use crate::model::Extension::Extension;
use crate::model::Claim_Procedure::Claim_Procedure;
use crate::model::Claim_Diagnosis::Claim_Diagnosis;
use crate::model::Identifier::Identifier;
use crate::model::Claim_SupportingInfo::Claim_SupportingInfo;
use crate::model::Claim_Item::Claim_Item;
use crate::model::Narrative::Narrative;
use crate::model::Claim_Payee::Claim_Payee;
use crate::model::Claim_Related::Claim_Related;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Claim_CareTeam::Claim_CareTeam;
use crate::model::Claim_Insurance::Claim_Insurance;
use crate::model::Claim_Accident::Claim_Accident;
use crate::model::Period::Period;
use crate::model::Money::Money;
use crate::model::Element::Element;


/// A provider issued list of professional services and products which have been
/// provided, or are to be provided, to a patient which is sent to an insurer for
/// reimbursement.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claim {
  /// The members of the team who provided the products and services.
  #[serde(rename = "careTeam")]
  care_team: Vec<Claim_CareTeam>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Extensions for status
  _status: Element,

  /// A code to indicate whether the nature of the request is: to request adjudication
  /// of products and services previously rendered; or requesting authorization and
  /// adjudication for provision in the future; or requesting the non-binding
  /// adjudication of the listed products and services which could be provided in the
  /// future.
  #[serde(rename = "use")]
  fhir_use: ClaimUse,

  /// Facility where the services were provided.
  facility: Box<Reference>,

  /// The party to whom the professional services and/or products have been supplied
  /// or are being considered and for whom actual or forecast reimbursement is sought.
  patient: Box<Reference>,

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

  /// Extensions for language
  _language: Element,

  /// Extensions for created
  _created: Element,

  /// A code to indicate whether and for whom funds are to be reserved for future
  /// claims.
  #[serde(rename = "fundsReserve")]
  funds_reserve: CodeableConcept,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// Additional information codes regarding exceptions, special considerations, the
  /// condition, situation, prior or concurrent issues.
  #[serde(rename = "supportingInfo")]
  supporting_info: Vec<Claim_SupportingInfo>,

  /// A claim line. Either a simple  product or service or a 'group' of details which
  /// can each be a simple items or groups of sub-details.
  item: Vec<Claim_Item>,

  /// A reference to a referral resource.
  referral: Box<Reference>,

  /// Information about diagnoses relevant to the claim items.
  diagnosis: Vec<Claim_Diagnosis>,

  /// Prescription to support the dispensing of pharmacy, device or vision products.
  prescription: Box<Reference>,

  /// Financial instruments for reimbursement for the health care products and
  /// services specified on the claim.
  insurance: Vec<Claim_Insurance>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// A unique identifier assigned to this claim.
  identifier: Vec<Identifier>,

  /// The provider which is responsible for the claim, predetermination or
  /// preauthorization.
  provider: Box<Reference>,

  /// The base language in which the resource is written.
  language: String,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// A finer grained suite of claim type codes which may convey additional
  /// information such as Inpatient vs Outpatient and/or a specialty service.
  #[serde(rename = "subType")]
  sub_type: CodeableConcept,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// The status of the resource instance.
  status: String,

  /// The total value of the all the items in the claim.
  total: Money,

  /// The Insurer who is target of the request.
  insurer: Box<Reference>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// The date this resource was created.
  created: String,

  /// The party to be reimbursed for cost of the products and services according to
  /// the terms of the policy.
  payee: Claim_Payee,

  /// The category of claim, e.g. oral, pharmacy, vision, institutional, professional.
  #[serde(rename = "type")]
  fhir_type: CodeableConcept,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Other claims which are related to this claim such as prior submissions or claims
  /// for related services or for the same event.
  related: Vec<Claim_Related>,

  /// Extensions for use
  _use: Element,

  /// Individual who created the claim, predetermination or preauthorization.
  enterer: Box<Reference>,

  /// Details of an accident which resulted in injuries which required the products
  /// and services listed in the claim.
  accident: Claim_Accident,

  /// Procedures performed on the patient relevant to the billing items with the
  /// claim.
  procedure: Vec<Claim_Procedure>,

  /// The provider-required urgency of processing the request. Typical values include:
  /// stat, routine deferred.
  priority: CodeableConcept,

  /// The period for which charges are being submitted.
  #[serde(rename = "billablePeriod")]
  billable_period: Period,

  /// Original prescription which has been superseded by this prescription to support
  /// the dispensing of pharmacy services, medications or products.
  #[serde(rename = "originalPrescription")]
  original_prescription: Box<Reference>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClaimUse {
  #[serde(rename = "claim")]
  Claim,

  #[serde(rename = "preauthorization")]
  Preauthorization,

  #[serde(rename = "predetermination")]
  Predetermination,

}
