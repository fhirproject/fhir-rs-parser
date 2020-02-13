#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Money::Money;
use crate::model::Extension::Extension;
use serde_json::value::Value;



/// Information about a medication that is used to support knowledge.

#[derive(Debug)]
pub struct MedicationKnowledge_Cost<'a> {
  pub value: &'a Value,
}

impl MedicationKnowledge_Cost<'_> {
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

  /// The category of the cost information.  For example, manufacturers' cost, patient
  /// cost, claim reimbursement cost, actual acquisition cost.
  pub fn fhir_type(&self) -> CodeableConcept {
    CodeableConcept {
      value: &self.value["type"],
    }
  }

  /// The price of the medication.
  pub fn cost(&self) -> Money {
    Money {
      value: &self.value["cost"],
    }
  }

  /// Extensions for source
  pub fn _source(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_source") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The source or owner that assigns the price to the medication.
  pub fn source(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("source") {
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

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

}
