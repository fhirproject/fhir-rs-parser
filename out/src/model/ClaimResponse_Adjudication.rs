#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::Money::Money;
use crate::model::CodeableConcept::CodeableConcept;
use serde_json::value::Value;



/// This resource provides the adjudication details from the processing of a Claim
/// resource.

#[derive(Debug)]
pub struct ClaimResponse_Adjudication<'a> {
  pub value: &'a Value,
}

impl ClaimResponse_Adjudication<'_> {
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

  /// A non-monetary value associated with the category. Mutually exclusive to the
  /// amount element above.
  pub fn value(&self) -> Option<f64> {
    if let Some(val) = self.value.get("value") {
      return Some(val.as_f64().unwrap());
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

  /// Monetary amount associated with the category.
  pub fn amount(&self) -> Option<Money> {
    if let Some(val) = self.value.get("amount") {
      return Some(Money { value: val });
    }
    return None;
  }

  /// A code supporting the understanding of the adjudication result and explaining
  /// variance from expected amount.
  pub fn reason(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("reason") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Extensions for value
  pub fn _value(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_value") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A code to indicate the information type of this adjudication record. Information
  /// types may include the value submitted, maximum values or percentages allowed or
  /// payable under the plan, amounts that: the patient is responsible for in
  /// aggregate or pertaining to this item; amounts paid by other coverages; and, the
  /// benefit payable for this item.
  pub fn category(&self) -> CodeableConcept {
    CodeableConcept {
      value: &self.value["category"],
    }
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
