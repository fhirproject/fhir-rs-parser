use serde::{Deserialize, Serialize};

/// This resource provides the adjudication details from the processing of a Claim
/// resource.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClaimResponse {
  /// Extensions for use
  _use: Element,

  /// A unique identifier assigned to this claim response.
  identifier: Vec<Identifier>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// A code, used only on a response to a preauthorization, to indicate whether the
  /// benefits payable have been reserved and for whom.
  #[serde(rename = "fundsReserve")]
  funds_reserve: CodeableConcept,

  /// Errors encountered during the processing of the adjudication.
  error: Vec<ClaimResponse_Error>,

  /// Original request resource reference.
  request: Reference,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// The party responsible for authorization, adjudication and reimbursement.
  insurer: Reference,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// A finer grained suite of claim type codes which may convey additional
  /// information such as Inpatient vs Outpatient and/or a specialty service.
  #[serde(rename = "subType")]
  sub_type: CodeableConcept,

  /// Extensions for preAuthRef
  #[serde(rename = "_preAuthRef")]
  _pre_auth_ref: Element,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Type of Party to be reimbursed: subscriber, provider, other.
  #[serde(rename = "payeeType")]
  payee_type: CodeableConcept,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Extensions for status
  _status: Element,

  /// The provider which is responsible for the claim, predetermination or
  /// preauthorization.
  requestor: Reference,

  /// Extensions for disposition
  _disposition: Element,

  /// Reference from the Insurer which is used in later communications which refers to
  /// this adjudication.
  #[serde(rename = "preAuthRef")]
  pre_auth_ref: String,

  /// A claim line. Either a simple (a product or service) or a 'group' of details
  /// which can also be a simple items or groups of sub-details.
  item: Vec<ClaimResponse_Item>,

  /// A finer grained suite of claim type codes which may convey additional
  /// information such as Inpatient vs Outpatient and/or a specialty service.
  type: CodeableConcept,

  /// Extensions for language
  _language: Element,

  /// A note that describes or explains adjudication results in a human readable form.
  #[serde(rename = "processNote")]
  process_note: Vec<ClaimResponse_ProcessNote>,

  /// The outcome of the claim, predetermination, or preauthorization processing.
  outcome: String,

  /// The time frame during which this authorization is effective.
  #[serde(rename = "preAuthPeriod")]
  pre_auth_period: Period,

  /// A human readable description of the status of the adjudication.
  disposition: String,

  /// Extensions for created
  _created: Element,

  /// A code for the form to be used for printing the content.
  #[serde(rename = "formCode")]
  form_code: CodeableConcept,

  /// Categorized monetary totals for the adjudication.
  total: Vec<ClaimResponse_Total>,

  /// The base language in which the resource is written.
  language: String,

  /// Payment details for the adjudication of the claim.
  payment: ClaimResponse_Payment,

  /// The status of the resource instance.
  status: String,

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

  /// A code to indicate whether the nature of the request is: to request adjudication
  /// of products and services previously rendered; or requesting authorization and
  /// adjudication for provision in the future; or requesting the non-binding
  /// adjudication of the listed products and services which could be provided in the
  /// future.
  use: String,

  /// The party to whom the professional services and/or products have been supplied
  /// or are being considered and for whom actual for facast reimbursement is sought.
  patient: Reference,

  /// The date this resource was created.
  created: dateTime,

  /// The adjudication results which are presented at the header level rather than at
  /// the line-item or add-item levels.
  adjudication: Vec<ClaimResponse_Adjudication>,

  /// The actual form, by reference or inclusion, for printing the content or an EOB.
  form: Attachment,

  /// The first-tier service adjudications for payor added product or service lines.
  #[serde(rename = "addItem")]
  add_item: Vec<ClaimResponse_AddItem>,

  /// Request for additional supporting or authorizing information.
  #[serde(rename = "communicationRequest")]
  communication_request: Vec<Reference>,

  /// Financial instruments for reimbursement for the health care products and
  /// services specified on the claim.
  insurance: Vec<ClaimResponse_Insurance>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Extensions for outcome
  _outcome: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

}
