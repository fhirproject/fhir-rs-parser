#![allow(unused_imports, non_camel_case_types)]

use crate::model::Money::Money;
use crate::model::Meta::Meta;
use crate::model::Reference::Reference;
use crate::model::Identifier::Identifier;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Narrative::Narrative;
use crate::model::ResourceList::ResourceList;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;



/// This resource provides the status of the payment for goods and services
/// rendered, and the request and response resource references.

#[derive(Debug)]
pub struct PaymentNotice<'a> {
  pub value: &'a Value,
}

impl PaymentNotice<'_> {
  /// The base language in which the resource is written.
  pub fn language(&self) -> Option<&str> {
    if let Some(Value::String(string)) = self.value.get("language") {
      return Some(string);
    }
    return None;
  }

  /// Extensions for language
  pub fn _language(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_language") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  pub fn text(&self) -> Option<Narrative> {
    if let Some(val) = self.value.get("text") {
      return Some(Narrative { value: val });
    }
    return None;
  }

  /// Extensions for implicitRules
  pub fn _implicit_rules(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_implicitRules") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  pub fn contained(&self) -> Option<Vec<ResourceList>> {
    if let Some(Value::Array(val)) = self.value.get("contained") {
      return Some(val.into_iter().map(|e| ResourceList { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  pub fn extension(&self) -> Option<Vec<Extension>> {
    if let Some(Value::Array(val)) = self.value.get("extension") {
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

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
  pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
    if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The amount sent to the payee.
  pub fn amount(&self) -> Money {
    Money {
      value: &self.value["amount"],
    }
  }

  /// The date when this resource was created.
  pub fn created(&self) -> Option<&str> {
    if let Some(Value::String(string)) = self.value.get("created") {
      return Some(string);
    }
    return None;
  }

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  pub fn meta(&self) -> Option<Meta> {
    if let Some(val) = self.value.get("meta") {
      return Some(Meta { value: val });
    }
    return None;
  }

  /// Extensions for created
  pub fn _created(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_created") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Reference of response to resource for which payment is being made.
  pub fn response(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("response") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// The date when the above payment action occurred.
  pub fn payment_date(&self) -> Option<&str> {
    if let Some(Value::String(string)) = self.value.get("paymentDate") {
      return Some(string);
    }
    return None;
  }

  /// The party who will receive or has received payment that is the subject of this
  /// notification.
  pub fn payee(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("payee") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// The party who is notified of the payment status.
  pub fn recipient(&self) -> Reference {
    Reference {
      value: &self.value["recipient"],
    }
  }

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  pub fn id(&self) -> Option<&str> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string);
    }
    return None;
  }

  /// The practitioner who is responsible for the services rendered to the patient.
  pub fn provider(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("provider") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Extensions for status
  pub fn _status(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_status") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A unique identifier assigned to this payment notice.
  pub fn identifier(&self) -> Option<Vec<Identifier>> {
    if let Some(Value::Array(val)) = self.value.get("identifier") {
      return Some(val.into_iter().map(|e| Identifier { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Reference of resource for which payment is being made.
  pub fn request(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("request") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// The status of the resource instance.
  pub fn status(&self) -> Option<&str> {
    if let Some(Value::String(string)) = self.value.get("status") {
      return Some(string);
    }
    return None;
  }

  /// A reference to the payment which is the subject of this notice.
  pub fn payment(&self) -> Reference {
    Reference {
      value: &self.value["payment"],
    }
  }

  /// Extensions for paymentDate
  pub fn _payment_date(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_paymentDate") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A code indicating whether payment has been sent or cleared.
  pub fn payment_status(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("paymentStatus") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  pub fn implicit_rules(&self) -> Option<&str> {
    if let Some(Value::String(string)) = self.value.get("implicitRules") {
      return Some(string);
    }
    return None;
  }

  pub fn validate(&self) -> bool {
    if let Some(_val) = self.language() {
    }
    if let Some(_val) = self._language() {
      _val.validate();
    }
    if let Some(_val) = self.text() {
      _val.validate();
    }
    if let Some(_val) = self._implicit_rules() {
      _val.validate();
    }
    if let Some(_val) = self.contained() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.modifier_extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    let _ = self.amount().validate();
    if let Some(_val) = self.created() {
    }
    if let Some(_val) = self.meta() {
      _val.validate();
    }
    if let Some(_val) = self._created() {
      _val.validate();
    }
    if let Some(_val) = self.response() {
      _val.validate();
    }
    if let Some(_val) = self.payment_date() {
    }
    if let Some(_val) = self.payee() {
      _val.validate();
    }
    let _ = self.recipient().validate();
    if let Some(_val) = self.id() {
    }
    if let Some(_val) = self.provider() {
      _val.validate();
    }
    if let Some(_val) = self._status() {
      _val.validate();
    }
    if let Some(_val) = self.identifier() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.request() {
      _val.validate();
    }
    if let Some(_val) = self.status() {
    }
    let _ = self.payment().validate();
    if let Some(_val) = self._payment_date() {
      _val.validate();
    }
    if let Some(_val) = self.payment_status() {
      _val.validate();
    }
    if let Some(_val) = self.implicit_rules() {
    }
    return true;
  }

}
