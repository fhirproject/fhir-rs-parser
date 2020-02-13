#![allow(unused_imports, non_camel_case_types)]

use crate::model::Money::Money;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use serde_json::value::Value;



/// This resource provides the adjudication details from the processing of a Claim
/// resource.

#[derive(Debug)]
pub struct ClaimResponse_Payment<'a> {
  pub value: &'a Value,
}

impl ClaimResponse_Payment<'_> {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  pub fn extension(&self) -> Option<Vec<Extension>> {
    if let Some(Value::Array(val)) = self.value.get("extension") {
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Reason for the payment adjustment.
  pub fn adjustment_reason(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("adjustmentReason") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Estimated date the payment will be issued or the actual issue date of payment.
  pub fn date(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("date") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Whether this represents partial or complete payment of the benefits payable.
  pub fn fhir_type(&self) -> CodeableConcept {
    CodeableConcept {
      value: &self.value["type"],
    }
  }

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.    Modifier extensions
  /// SHALL NOT change the meaning of any elements on Resource or DomainResource
  /// (including cannot change the meaning of modifierExtension itself).
  pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
    if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for date
  pub fn _date(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_date") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Total amount of all adjustments to this payment included in this transaction
  /// which are not related to this claim's adjudication.
  pub fn adjustment(&self) -> Option<Money> {
    if let Some(val) = self.value.get("adjustment") {
      return Some(Money { value: val });
    }
    return None;
  }

  /// Issuer's unique identifier for the payment instrument.
  pub fn identifier(&self) -> Option<Identifier> {
    if let Some(val) = self.value.get("identifier") {
      return Some(Identifier { value: val });
    }
    return None;
  }

  /// Benefits payable less any payment adjustment.
  pub fn amount(&self) -> Money {
    Money {
      value: &self.value["amount"],
    }
  }

  pub fn validate(&self) -> bool {
    if let Some(_val) = self.id() {
    }
    if let Some(_val) = self.extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.adjustment_reason() {
      _val.validate();
    }
    if let Some(_val) = self.date() {
    }
    let _ = self.fhir_type().validate();
    if let Some(_val) = self.modifier_extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self._date() {
      _val.validate();
    }
    if let Some(_val) = self.adjustment() {
      _val.validate();
    }
    if let Some(_val) = self.identifier() {
      _val.validate();
    }
    let _ = self.amount().validate();
    return true;
  }

}
