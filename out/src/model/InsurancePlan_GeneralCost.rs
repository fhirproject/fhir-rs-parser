#![allow(unused_imports, non_camel_case_types)]

use crate::model::Money::Money;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;
use serde_json::value::Value;



/// Details of a Health Insurance product/plan provided by an organization.

#[derive(Debug)]
pub struct InsurancePlan_GeneralCost<'a> {
  pub value: &'a Value,
}

impl InsurancePlan_GeneralCost<'_> {
  /// Value of the cost.
  pub fn cost(&self) -> Option<Money> {
    if let Some(val) = self.value.get("cost") {
      return Some(Money { value: val });
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

  /// Number of participants enrolled in the plan.
  pub fn group_size(&self) -> Option<i64> {
    if let Some(val) = self.value.get("groupSize") {
      return Some(val.as_i64().unwrap());
    }
    return None;
  }

  /// Type of cost.
  pub fn fhir_type(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("type") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Extensions for groupSize
  pub fn _group_size(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_groupSize") {
      return Some(Element { value: val });
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

  /// Additional information about the general costs associated with this plan.
  pub fn comment(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("comment") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for comment
  pub fn _comment(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_comment") {
      return Some(Element { value: val });
    }
    return None;
  }

}
