#![allow(unused_imports, non_camel_case_types)]

use crate::model::Reference::Reference;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Invoice_PriceComponent::Invoice_PriceComponent;
use serde_json::value::Value;



/// Invoice containing collected ChargeItems from an Account with calculated
/// individual and total price for Billing purpose.

#[derive(Debug)]
pub struct Invoice_LineItem<'a> {
  pub value: &'a Value,
}

impl Invoice_LineItem<'_> {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<&str> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string);
    }
    return None;
  }

  /// Sequence in which the items appear on the invoice.
  pub fn sequence(&self) -> Option<i64> {
    if let Some(val) = self.value.get("sequence") {
      return Some(val.as_i64().unwrap());
    }
    return None;
  }

  /// The ChargeItem contains information such as the billing code, date, amount etc.
  /// If no further details are required for the lineItem, inline billing codes can be
  /// added using the CodeableConcept data type instead of the Reference.
  pub fn charge_item_codeable_concept(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("chargeItemCodeableConcept") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// The ChargeItem contains information such as the billing code, date, amount etc.
  /// If no further details are required for the lineItem, inline billing codes can be
  /// added using the CodeableConcept data type instead of the Reference.
  pub fn charge_item_reference(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("chargeItemReference") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// The price for a ChargeItem may be calculated as a base price with
  /// surcharges/deductions that apply in certain conditions. A ChargeItemDefinition
  /// resource that defines the prices, factors and conditions that apply to a billing
  /// code is currently under development. The priceComponent element can be used to
  /// offer transparency to the recipient of the Invoice as to how the prices have
  /// been calculated.
  pub fn price_component(&self) -> Option<Vec<Invoice_PriceComponent>> {
    if let Some(Value::Array(val)) = self.value.get("priceComponent") {
      return Some(val.into_iter().map(|e| Invoice_PriceComponent { value: e }).collect::<Vec<_>>());
    }
    return None;
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

  /// Extensions for sequence
  pub fn _sequence(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_sequence") {
      return Some(Element { value: val });
    }
    return None;
  }

  pub fn validate(&self) -> bool {
    if let Some(_val) = self.id() {
    }
    if let Some(_val) = self.sequence() {
    }
    if let Some(_val) = self.charge_item_codeable_concept() {
      _val.validate();
    }
    if let Some(_val) = self.charge_item_reference() {
      _val.validate();
    }
    if let Some(_val) = self.price_component() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.modifier_extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self._sequence() {
      _val.validate();
    }
    return true;
  }

}
