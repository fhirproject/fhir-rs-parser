#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Meta::Meta;
use crate::model::Money::Money;
use crate::model::ResourceList::ResourceList;
use crate::model::Element::Element;
use crate::model::Identifier::Identifier;
use crate::model::Reference::Reference;
use crate::model::Extension::Extension;
use crate::model::PaymentReconciliation_Detail::PaymentReconciliation_Detail;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Period::Period;
use crate::model::PaymentReconciliation_ProcessNote::PaymentReconciliation_ProcessNote;
use crate::model::Narrative::Narrative;


/// This resource provides the details including amount of a payment and allocates
/// the payment items being paid.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentReconciliation {
  /// Extensions for disposition
  #[serde(rename = "_disposition")]
  _disposition: Option<Element>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// The date when the resource was created.
  created: Option<String>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Extensions for paymentDate
  #[serde(rename = "_paymentDate")]
  _payment_date: Option<Element>,

  /// Extensions for created
  #[serde(rename = "_created")]
  _created: Option<Element>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Issuer's unique identifier for the payment instrument.
  #[serde(rename = "paymentIdentifier")]
  payment_identifier: Option<Identifier>,

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

  /// The status of the resource instance.
  status: Option<String>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// A note that describes or explains the processing in a human readable form.
  #[serde(rename = "processNote")]
  process_note: Option<Vec<PaymentReconciliation_ProcessNote>>,

  /// A code for the form to be used for printing the content.
  #[serde(rename = "formCode")]
  form_code: Option<CodeableConcept>,

  /// The party who generated the payment.
  #[serde(rename = "paymentIssuer")]
  payment_issuer: Option<Box<Reference>>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// The outcome of a request for a reconciliation.
  outcome: Option<PaymentReconciliationOutcome>,

  /// Total payment amount as indicated on the financial instrument.
  #[serde(rename = "paymentAmount")]
  payment_amount: Money,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// A unique identifier assigned to this payment reconciliation.
  identifier: Option<Vec<Identifier>>,

  /// Extensions for outcome
  #[serde(rename = "_outcome")]
  _outcome: Option<Element>,

  /// The practitioner who is responsible for the services rendered to the patient.
  requestor: Option<Box<Reference>>,

  /// A human readable description of the status of the request for the
  /// reconciliation.
  disposition: Option<String>,

  /// The date of payment as indicated on the financial instrument.
  #[serde(rename = "paymentDate")]
  payment_date: Option<i32>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// Original request resource reference.
  request: Option<Box<Reference>>,

  /// Distribution of the payment amount for a previously acknowledged payable.
  detail: Option<Vec<PaymentReconciliation_Detail>>,

  /// The period of time for which payments have been gathered into this bulk payment
  /// for settlement.
  period: Option<Period>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentReconciliationOutcome {
  #[serde(rename = "queued")]
  Queued,

  #[serde(rename = "complete")]
  Complete,

  #[serde(rename = "error")]
  Error,

  #[serde(rename = "partial")]
  Partial,

}
