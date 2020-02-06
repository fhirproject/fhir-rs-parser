#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Identifier::Identifier;
use crate::model::ResourceList::ResourceList;
use crate::model::Meta::Meta;
use crate::model::Money::Money;
use crate::model::Reference::Reference;
use crate::model::Narrative::Narrative;
use crate::model::Extension::Extension;


/// This resource provides the status of the payment for goods and services
/// rendered, and the request and response resource references.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentNotice {
  /// The practitioner who is responsible for the services rendered to the patient.
  provider: Option<Box<Reference>>,

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

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// The status of the resource instance.
  status: Option<String>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// A unique identifier assigned to this payment notice.
  identifier: Option<Vec<Identifier>>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// Reference of resource for which payment is being made.
  request: Option<Box<Reference>>,

  /// A reference to the payment which is the subject of this notice.
  payment: Box<Reference>,

  /// The date when the above payment action occurred.
  #[serde(rename = "paymentDate")]
  payment_date: Option<i32>,

  /// The amount sent to the payee.
  amount: Money,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Reference of response to resource for which payment is being made.
  response: Option<Box<Reference>>,

  /// The party who will receive or has received payment that is the subject of this
  /// notification.
  payee: Option<Box<Reference>>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// Extensions for paymentDate
  #[serde(rename = "_paymentDate")]
  _payment_date: Option<Element>,

  /// A code indicating whether payment has been sent or cleared.
  #[serde(rename = "paymentStatus")]
  payment_status: Option<CodeableConcept>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// The party who is notified of the payment status.
  recipient: Box<Reference>,

  /// Extensions for created
  #[serde(rename = "_created")]
  _created: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The date when this resource was created.
  created: Option<String>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

}
