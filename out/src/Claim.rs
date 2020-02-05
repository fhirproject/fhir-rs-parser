use serde::{Deserialize, Serialize};

/// A provider issued list of professional services and products which have been
/// provided, or are to be provided, to a patient which is sent to an insurer for
/// reimbursement.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Claim {
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

  /// Procedures performed on the patient relevant to the billing items with the
  /// claim.
  procedure: Vec<Claim_Procedure>,

  /// The period for which charges are being submitted.
  #[serde(rename = "billablePeriod")]
  billable_period: Period,

  /// Details of an accident which resulted in injuries which required the products
  /// and services listed in the claim.
  accident: Claim_Accident,

  /// A claim line. Either a simple  product or service or a 'group' of details which
  /// can each be a simple items or groups of sub-details.
  item: Vec<Claim_Item>,

  /// Extensions for use
  _use: Element,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// A code to indicate whether and for whom funds are to be reserved for future
  /// claims.
  #[serde(rename = "fundsReserve")]
  funds_reserve: CodeableConcept,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Individual who created the claim, predetermination or preauthorization.
  enterer: Reference,

  /// The members of the team who provided the products and services.
  #[serde(rename = "careTeam")]
  care_team: Vec<Claim_CareTeam>,

  /// The category of claim, e.g. oral, pharmacy, vision, institutional, professional.
  type: CodeableConcept,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Information about diagnoses relevant to the claim items.
  diagnosis: Vec<Claim_Diagnosis>,

  /// The provider which is responsible for the claim, predetermination or
  /// preauthorization.
  provider: Reference,

  /// The base language in which the resource is written.
  language: String,

  /// Additional information codes regarding exceptions, special considerations, the
  /// condition, situation, prior or concurrent issues.
  #[serde(rename = "supportingInfo")]
  supporting_info: Vec<Claim_SupportingInfo>,

  /// Extensions for language
  _language: Element,

  /// Other claims which are related to this claim such as prior submissions or claims
  /// for related services or for the same event.
  related: Vec<Claim_Related>,

  /// Original prescription which has been superseded by this prescription to support
  /// the dispensing of pharmacy services, medications or products.
  #[serde(rename = "originalPrescription")]
  original_prescription: Reference,

  /// Financial instruments for reimbursement for the health care products and
  /// services specified on the claim.
  insurance: Vec<Claim_Insurance>,

  /// The status of the resource instance.
  status: String,

  /// Extensions for created
  _created: Element,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// A unique identifier assigned to this claim.
  identifier: Vec<Identifier>,

  /// The total value of the all the items in the claim.
  total: Money,

  /// Extensions for status
  _status: Element,

  /// The date this resource was created.
  created: dateTime,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The party to whom the professional services and/or products have been supplied
  /// or are being considered and for whom actual or forecast reimbursement is sought.
  patient: Reference,

  /// The Insurer who is target of the request.
  insurer: Reference,

  /// The party to be reimbursed for cost of the products and services according to
  /// the terms of the policy.
  payee: Claim_Payee,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// A code to indicate whether the nature of the request is: to request adjudication
  /// of products and services previously rendered; or requesting authorization and
  /// adjudication for provision in the future; or requesting the non-binding
  /// adjudication of the listed products and services which could be provided in the
  /// future.
  use: ClaimUse,

  /// Facility where the services were provided.
  facility: Reference,

  /// A finer grained suite of claim type codes which may convey additional
  /// information such as Inpatient vs Outpatient and/or a specialty service.
  #[serde(rename = "subType")]
  sub_type: CodeableConcept,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Prescription to support the dispensing of pharmacy, device or vision products.
  prescription: Reference,

  /// A reference to a referral resource.
  referral: Reference,

  /// The provider-required urgency of processing the request. Typical values include:
  /// stat, routine deferred.
  priority: CodeableConcept,

}

#[derive(Debug, Serialize, Deserialize)]
enum ClaimUse {
  #[serde(rename = "claim")]
  Claim,

  #[serde(rename = "preauthorization")]
  Preauthorization,

  #[serde(rename = "predetermination")]
  Predetermination,

}
