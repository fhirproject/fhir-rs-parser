#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Range::Range;
use serde_json::value::Value;



/// A populatioof people with some set of grouping criteria.

#[derive(Debug)]
pub struct Population<'a> {
  pub value: &'a Value,
}

impl Population<'_> {
  /// Race of the specific population.
  pub fn race(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("race") {
      return Some(CodeableConcept { value: val });
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

  /// The existing physiological conditions of the specific population to which this
  /// applies.
  pub fn physiological_condition(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("physiologicalCondition") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// The gender of the specific population.
  pub fn gender(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("gender") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// The age of the specific population.
  pub fn age_range(&self) -> Option<Range> {
    if let Some(val) = self.value.get("ageRange") {
      return Some(Range { value: val });
    }
    return None;
  }

  /// The age of the specific population.
  pub fn age_codeable_concept(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("ageCodeableConcept") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<&str> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string);
    }
    return None;
  }

  pub fn validate(&self) -> bool {
    if let Some(_val) = self.race() {
      _val.validate();
    }
    if let Some(_val) = self.modifier_extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.physiological_condition() {
      _val.validate();
    }
    if let Some(_val) = self.gender() {
      _val.validate();
    }
    if let Some(_val) = self.age_range() {
      _val.validate();
    }
    if let Some(_val) = self.age_codeable_concept() {
      _val.validate();
    }
    if let Some(_val) = self.id() {
    }
    return true;
  }

}
