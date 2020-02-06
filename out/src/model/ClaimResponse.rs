#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Attachment::Attachment;
use crate::model::ClaimResponse_Error::ClaimResponse_Error;
use crate::model::ResourceList::ResourceList;
use crate::model::ClaimResponse_AddItem::ClaimResponse_AddItem;
use crate::model::ClaimResponse_Insurance::ClaimResponse_Insurance;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::Reference::Reference;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ClaimResponse_Adjudication::ClaimResponse_Adjudication;
use crate::model::Identifier::Identifier;
use crate::model::ClaimResponse_ProcessNote::ClaimResponse_ProcessNote;
use crate::model::Period::Period;
use crate::model::ClaimResponse_Total::ClaimResponse_Total;
use crate::model::ClaimResponse_Payment::ClaimResponse_Payment;
use crate::model::Narrative::Narrative;
use crate::model::Meta::Meta;
use crate::model::ClaimResponse_Item::ClaimResponse_Item;


/// This resource provides the adjudication details from the processing of a Claim
/// resource.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponse {
  /// The time frame during which this authorization is effective.
  #[serde(rename = "preAuthPeriod")]
  pre_auth_period: Option<Period>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// The provider which is responsible for the claim, predetermination or
  /// preauthorization.
  requestor: Option<Box<Reference>>,

  /// Request for additional supporting or authorizing information.
  #[serde(rename = "communicationRequest")]
  communication_request: Option<Vec<Box<Reference>>>,

  /// Type of Party to be reimbursed: subscriber, provider, other.
  #[serde(rename = "payeeType")]
  payee_type: Option<CodeableConcept>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Original request resource reference.
  request: Option<Box<Reference>>,

  /// The outcome of the claim, predetermination, or preauthorization processing.
  outcome: Option<String>,

  /// The adjudication results which are presented at the header level rather than at
  /// the line-item or add-item levels.
  adjudication: Option<Vec<ClaimResponse_Adjudication>>,

  /// Categorized monetary totals for the adjudication.
  total: Option<Vec<ClaimResponse_Total>>,

  /// Extensions for outcome
  #[serde(rename = "_outcome")]
  _outcome: Option<Element>,

  /// The first-tier service adjudications for payor added product or service lines.
  #[serde(rename = "addItem")]
  add_item: Option<Vec<ClaimResponse_AddItem>>,

  /// Extensions for created
  #[serde(rename = "_created")]
  _created: Option<Element>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// Extensions for disposition
  #[serde(rename = "_disposition")]
  _disposition: Option<Element>,

  /// A finer grained suite of claim type codes which may convey additional
  /// information such as Inpatient vs Outpatient and/or a specialty service.
  #[serde(rename = "type")]
  fhir_type: CodeableConcept,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Reference from the Insurer which is used in later communications which refers to
  /// this adjudication.
  #[serde(rename = "preAuthRef")]
  pre_auth_ref: Option<String>,

  /// The party responsible for authorization, adjudication and reimbursement.
  insurer: Box<Reference>,

  /// Financial instruments for reimbursement for the health care products and
  /// services specified on the claim.
  insurance: Option<Vec<ClaimResponse_Insurance>>,

  /// A code, used only on a response to a preauthorization, to indicate whether the
  /// benefits payable have been reserved and for whom.
  #[serde(rename = "fundsReserve")]
  funds_reserve: Option<CodeableConcept>,

  /// A code for the form to be used for printing the content.
  #[serde(rename = "formCode")]
  form_code: Option<CodeableConcept>,

  /// Errors encountered during the processing of the adjudication.
  error: Option<Vec<ClaimResponse_Error>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// A note that describes or explains adjudication results in a human readable form.
  #[serde(rename = "processNote")]
  process_note: Option<Vec<ClaimResponse_ProcessNote>>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// A claim line. Either a simple (a product or service) or a 'group' of details
  /// which can also be a simple items or groups of sub-details.
  item: Option<Vec<ClaimResponse_Item>>,

  /// Extensions for preAuthRef
  #[serde(rename = "_preAuthRef")]
  _pre_auth_ref: Option<Element>,

  /// A code to indicate whether the nature of the request is: to request adjudication
  /// of products and services previously rendered; or requesting authorization and
  /// adjudication for provision in the future; or requesting the non-binding
  /// adjudication of the listed products and services which could be provided in the
  /// future.
  #[serde(rename = "use")]
  fhir_use: Option<String>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// Extensions for use
  #[serde(rename = "_use")]
  _use: Option<Element>,

  /// The date this resource was created.
  created: Option<String>,

  /// Payment details for the adjudication of the claim.
  payment: Option<ClaimResponse_Payment>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// The party to whom the professional services and/or products have been supplied
  /// or are being considered and for whom actual for facast reimbursement is sought.
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
  modifier_extension: Option<Vec<Extension>>,

  /// A human readable description of the status of the adjudication.
  disposition: Option<String>,

  /// A unique identifier assigned to this claim response.
  identifier: Option<Vec<Identifier>>,

  /// The status of the resource instance.
  status: Option<String>,

  /// The actual form, by reference or inclusion, for printing the content or an EOB.
  form: Option<Attachment>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// A finer grained suite of claim type codes which may convey additional
  /// information such as Inpatient vs Outpatient and/or a specialty service.
  #[serde(rename = "subType")]
  sub_type: Option<CodeableConcept>,

}
