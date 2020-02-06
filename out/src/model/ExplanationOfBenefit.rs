#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Meta::Meta;
use crate::model::ExplanationOfBenefit_Insurance::ExplanationOfBenefit_Insurance;
use crate::model::ExplanationOfBenefit_AddItem::ExplanationOfBenefit_AddItem;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Reference::Reference;
use crate::model::ExplanationOfBenefit_Procedure::ExplanationOfBenefit_Procedure;
use crate::model::Attachment::Attachment;
use crate::model::Narrative::Narrative;
use crate::model::ExplanationOfBenefit_Diagnosis::ExplanationOfBenefit_Diagnosis;
use crate::model::ExplanationOfBenefit_Item::ExplanationOfBenefit_Item;
use crate::model::ResourceList::ResourceList;
use crate::model::ExplanationOfBenefit_Payment::ExplanationOfBenefit_Payment;
use crate::model::ExplanationOfBenefit_ProcessNote::ExplanationOfBenefit_ProcessNote;
use crate::model::Element::Element;
use crate::model::ExplanationOfBenefit_CareTeam::ExplanationOfBenefit_CareTeam;
use crate::model::ExplanationOfBenefit_Adjudication::ExplanationOfBenefit_Adjudication;
use crate::model::ExplanationOfBenefit_Accident::ExplanationOfBenefit_Accident;
use crate::model::ExplanationOfBenefit_BenefitBalance::ExplanationOfBenefit_BenefitBalance;
use crate::model::Identifier::Identifier;
use crate::model::ExplanationOfBenefit_Payee::ExplanationOfBenefit_Payee;
use crate::model::Period::Period;
use crate::model::Extension::Extension;
use crate::model::ExplanationOfBenefit_Total::ExplanationOfBenefit_Total;
use crate::model::ExplanationOfBenefit_Related::ExplanationOfBenefit_Related;
use crate::model::ExplanationOfBenefit_SupportingInfo::ExplanationOfBenefit_SupportingInfo;


/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefit {
  /// A unique identifier assigned to this explanation of benefit.
  identifier: Vec<Identifier>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// The members of the team who provided the products and services.
  #[serde(rename = "careTeam")]
  care_team: Vec<ExplanationOfBenefit_CareTeam>,

  /// A human readable description of the status of the adjudication.
  disposition: String,

  /// Procedures performed on the patient relevant to the billing items with the
  /// claim.
  procedure: Vec<ExplanationOfBenefit_Procedure>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// The provider which is responsible for the claim, predetermination or
  /// preauthorization.
  provider: Box<Reference>,

  /// Extensions for created
  _created: Element,

  /// A code to indicate whether and for whom funds are to be reserved for future
  /// claims.
  #[serde(rename = "fundsReserveRequested")]
  funds_reserve_requested: CodeableConcept,

  /// This indicates the relative order of a series of EOBs related to different
  /// coverages for the same suite of services.
  precedence: i32,

  /// The actual form, by reference or inclusion, for printing the content or an EOB.
  form: Attachment,

  /// A note that describes or explains adjudication results in a human readable form.
  #[serde(rename = "processNote")]
  process_note: Vec<ExplanationOfBenefit_ProcessNote>,

  /// A code, used only on a response to a preauthorization, to indicate whether the
  /// benefits payable have been reserved and for whom.
  #[serde(rename = "fundsReserve")]
  funds_reserve: CodeableConcept,

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

  /// The provider-required urgency of processing the request. Typical values include:
  /// stat, routine deferred.
  priority: CodeableConcept,

  /// The date this resource was created.
  created: String,

  /// The party to be reimbursed for cost of the products and services according to
  /// the terms of the policy.
  payee: ExplanationOfBenefit_Payee,

  /// Extensions for outcome
  _outcome: Element,

  /// Extensions for preAuthRef
  #[serde(rename = "_preAuthRef")]
  _pre_auth_ref: Vec<Element>,

  /// Extensions for precedence
  _precedence: Element,

  /// The first-tier service adjudications for payor added product or service lines.
  #[serde(rename = "addItem")]
  add_item: Vec<ExplanationOfBenefit_AddItem>,

  /// The term of the benefits documented in this response.
  #[serde(rename = "benefitPeriod")]
  benefit_period: Period,

  /// The category of claim, e.g. oral, pharmacy, vision, institutional, professional.
  #[serde(rename = "type")]
  fhir_type: CodeableConcept,

  /// Reference from the Insurer which is used in later communications which refers to
  /// this adjudication.
  #[serde(rename = "preAuthRef")]
  pre_auth_ref: Vec<String>,

  /// Payment details for the adjudication of the claim.
  payment: ExplanationOfBenefit_Payment,

  /// Facility where the services were provided.
  facility: Box<Reference>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// A reference to a referral resource.
  referral: Box<Reference>,

  /// Extensions for disposition
  _disposition: Element,

  /// The adjudication results which are presented at the header level rather than at
  /// the line-item or add-item levels.
  adjudication: Vec<ExplanationOfBenefit_Adjudication>,

  /// A finer grained suite of claim type codes which may convey additional
  /// information such as Inpatient vs Outpatient and/or a specialty service.
  #[serde(rename = "subType")]
  sub_type: CodeableConcept,

  /// Details of a accident which resulted in injuries which required the products and
  /// services listed in the claim.
  accident: ExplanationOfBenefit_Accident,

  /// Extensions for use
  _use: Element,

  /// The base language in which the resource is written.
  language: String,

  /// A claim line. Either a simple (a product or service) or a 'group' of details
  /// which can also be a simple items or groups of sub-details.
  item: Vec<ExplanationOfBenefit_Item>,

  /// The status of the resource instance.
  status: ExplanationOfBenefitStatus,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Additional information codes regarding exceptions, special considerations, the
  /// condition, situation, prior or concurrent issues.
  #[serde(rename = "supportingInfo")]
  supporting_info: Vec<ExplanationOfBenefit_SupportingInfo>,

  /// Original prescription which has been superseded by this prescription to support
  /// the dispensing of pharmacy services, medications or products.
  #[serde(rename = "originalPrescription")]
  original_prescription: Box<Reference>,

  /// The party responsible for authorization, adjudication and reimbursement.
  insurer: Box<Reference>,

  /// Information about diagnoses relevant to the claim items.
  diagnosis: Vec<ExplanationOfBenefit_Diagnosis>,

  /// Prescription to support the dispensing of pharmacy, device or vision products.
  prescription: Box<Reference>,

  /// Categorized monetary totals for the adjudication.
  total: Vec<ExplanationOfBenefit_Total>,

  /// The business identifier for the instance of the adjudication request: claim
  /// predetermination or preauthorization.
  claim: Box<Reference>,

  /// The outcome of the claim, predetermination, or preauthorization processing.
  outcome: String,

  /// Other claims which are related to this claim such as prior submissions or claims
  /// for related services or for the same event.
  related: Vec<ExplanationOfBenefit_Related>,

  /// A code to indicate whether the nature of the request is: to request adjudication
  /// of products and services previously rendered; or requesting authorization and
  /// adjudication for provision in the future; or requesting the non-binding
  /// adjudication of the listed products and services which could be provided in the
  /// future.
  #[serde(rename = "use")]
  fhir_use: String,

  /// The business identifier for the instance of the adjudication response: claim,
  /// predetermination or preauthorization response.
  #[serde(rename = "claimResponse")]
  claim_response: Box<Reference>,

  /// The period for which charges are being submitted.
  #[serde(rename = "billablePeriod")]
  billable_period: Period,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Extensions for status
  _status: Element,

  /// The timeframe during which the supplied preauthorization reference may be quoted
  /// on claims to obtain the adjudication as provided.
  #[serde(rename = "preAuthRefPeriod")]
  pre_auth_ref_period: Vec<Period>,

  /// Financial instruments for reimbursement for the health care products and
  /// services specified on the claim.
  insurance: Vec<ExplanationOfBenefit_Insurance>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Balance by Benefit Category.
  #[serde(rename = "benefitBalance")]
  benefit_balance: Vec<ExplanationOfBenefit_BenefitBalance>,

  /// Individual who created the claim, predetermination or preauthorization.
  enterer: Box<Reference>,

  /// The party to whom the professional services and/or products have been supplied
  /// or are being considered and for whom actual for forecast reimbursement is
  /// sought.
  patient: Box<Reference>,

  /// Extensions for language
  _language: Element,

  /// A code for the form to be used for printing the content.
  #[serde(rename = "formCode")]
  form_code: CodeableConcept,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExplanationOfBenefitStatus {
  #[serde(rename = "active")]
  Active,

  #[serde(rename = "cancelled")]
  Cancelled,

  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

}
