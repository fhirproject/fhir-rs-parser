#![allow(unused_imports, non_camel_case_types)]

use crate::model::Money::Money;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::Quantity::Quantity;
use crate::model::CodeableConcept::CodeableConcept;
use serde_json::value::Value;



/// A provider issued list of professional services and products which have been
/// provided, or are to be provided, to a patient which is sent to an insurer for
/// reimbursement.

#[derive(Debug)]
pub struct Claim_SubDetail<'a> {
  pub value: &'a Value,
}

impl Claim_SubDetail<'_> {
  /// When the value is a group code then this item collects a set of related claim
  /// details, otherwise this contains the product, service, drug or other billing
  /// code for the item.
  pub fn product_or_service(&self) -> CodeableConcept {
    CodeableConcept {
      value: &self.value["productOrService"],
    }
  }

  /// The type of revenue or cost center providing the product and/or service.
  pub fn revenue(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("revenue") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Identifies the program under which this may be recovered.
  pub fn program_code(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("programCode") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The quantity times the unit price for an additional service or product or
  /// charge.
  pub fn net(&self) -> Option<Money> {
    if let Some(val) = self.value.get("net") {
      return Some(Money { value: val });
    }
    return None;
  }

  /// Unique Device Identifiers associated with this line item.
  pub fn udi(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("udi") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The number of repetitions of a service or product.
  pub fn quantity(&self) -> Option<Quantity> {
    if let Some(val) = self.value.get("quantity") {
      return Some(Quantity { value: val });
    }
    return None;
  }

  /// A real number that represents a multiplier used in determining the overall value
  /// of services delivered and/or goods received. The concept of a Factor allows for
  /// a discount or surcharge multiplier to be applied to a monetary amount.
  pub fn factor(&self) -> Option<f64> {
    if let Some(val) = self.value.get("factor") {
      return Some(val.as_f64().unwrap());
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

  /// Code to identify the general type of benefits under which products and services
  /// are provided.
  pub fn category(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("category") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
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

  /// If the item is not a group then this is the fee for the product or service,
  /// otherwise this is the total of the fees for the details of the group.
  pub fn unit_price(&self) -> Option<Money> {
    if let Some(val) = self.value.get("unitPrice") {
      return Some(Money { value: val });
    }
    return None;
  }

  /// Extensions for factor
  pub fn _factor(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_factor") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A number to uniquely identify item entries.
  pub fn sequence(&self) -> Option<i64> {
    if let Some(val) = self.value.get("sequence") {
      return Some(val.as_i64().unwrap());
    }
    return None;
  }

  /// Item typification or modifiers codes to convey additional context for the
  /// product or service.
  pub fn modifier(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("modifier") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
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

}
