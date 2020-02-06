#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::Annotation::Annotation;
use crate::model::Invoice_Participant::Invoice_Participant;
use crate::model::Element::Element;
use crate::model::Invoice_PriceComponent::Invoice_PriceComponent;
use crate::model::ResourceList::ResourceList;
use crate::model::Money::Money;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Invoice_LineItem::Invoice_LineItem;


/// Invoice containing collected ChargeItems from an Account with calculated
/// individual and total price for Billing purpose.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// The total amount for the Invoice may be calculated as the sum of the line items
  /// with surcharges/deductions that apply in certain conditions.  The priceComponent
  /// element can be used to offer transparency to the recipient of the Invoice of how
  /// the total price was calculated.
  #[serde(rename = "totalPriceComponent")]
  total_price_component: Option<Vec<Invoice_PriceComponent>>,

  /// Account which is supposed to be balanced with this Invoice.
  account: Option<Box<Reference>>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Each line item represents one charge for goods and services rendered. Details
  /// such as date, code and amount are found in the referenced ChargeItem resource.
  #[serde(rename = "lineItem")]
  line_item: Option<Vec<Invoice_LineItem>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// In case of Invoice cancellation a reason must be given (entered in error,
  /// superseded by corrected invoice etc.).
  #[serde(rename = "cancelledReason")]
  cancelled_reason: Option<String>,

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

  /// Comments made about the invoice by the issuer, subject, or other participants.
  note: Option<Vec<Annotation>>,

  /// Date/time(s) of when this Invoice was posted.
  date: Option<String>,

  /// The individual or Organization responsible for balancing of this invoice.
  recipient: Option<Box<Reference>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// The organizationissuing the Invoice.
  issuer: Option<Box<Reference>>,

  /// Extensions for paymentTerms
  #[serde(rename = "_paymentTerms")]
  _payment_terms: Option<Element>,

  /// Payment details such as banking details, period of payment, deductibles, methods
  /// of payment.
  #[serde(rename = "paymentTerms")]
  payment_terms: Option<String>,

  /// The current state of the Invoice.
  status: Option<InvoiceStatus>,

  /// Identifier of this Invoice, often used for reference in correspondence about
  /// this invoice or for tracking of payments.
  identifier: Option<Vec<Identifier>>,

  /// Invoice total , taxes excluded.
  #[serde(rename = "totalNet")]
  total_net: Option<Money>,

  /// Extensions for date
  #[serde(rename = "_date")]
  _date: Option<Element>,

  /// Indicates who or what performed or participated in the charged service.
  participant: Option<Vec<Invoice_Participant>>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// Extensions for cancelledReason
  #[serde(rename = "_cancelledReason")]
  _cancelled_reason: Option<Element>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Invoice total, tax included.
  #[serde(rename = "totalGross")]
  total_gross: Option<Money>,

  /// Type of Invoice depending on domain, realm an usage (e.g. internal/external,
  /// dental, preliminary).
  #[serde(rename = "type")]
  fhir_type: Option<CodeableConcept>,

  /// The individual or set of individuals receiving the goods and services billed in
  /// this invoice.
  subject: Option<Box<Reference>>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum InvoiceStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "issued")]
  Issued,

  #[serde(rename = "balanced")]
  Balanced,

  #[serde(rename = "cancelled")]
  Cancelled,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

}
