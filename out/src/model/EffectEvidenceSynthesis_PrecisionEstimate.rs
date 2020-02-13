#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use serde_json::value::Value;



/// The EffectEvidenceSynthesis resource describes the difference in an outcome
/// between exposures states in a population where the effect estimate is derived
/// from a combination of research studies.

#[derive(Debug)]
pub struct EffectEvidenceSynthesis_PrecisionEstimate<'a> {
  pub value: &'a Value,
}

impl EffectEvidenceSynthesis_PrecisionEstimate<'_> {
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

  /// Extensions for level
  pub fn _level(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_level") {
      return Some(Element { value: val });
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

  /// Examples include confidence interval and interquartile range.
  pub fn fhir_type(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("type") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Extensions for to
  pub fn _to(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_to") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Upper bound of confidence interval.
  pub fn to(&self) -> Option<f64> {
    if let Some(val) = self.value.get("to") {
      return Some(val.as_f64().unwrap());
    }
    return None;
  }

  /// Use 95 for a 95% confidence interval.
  pub fn level(&self) -> Option<f64> {
    if let Some(val) = self.value.get("level") {
      return Some(val.as_f64().unwrap());
    }
    return None;
  }

  /// Extensions for from
  pub fn _from(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_from") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Lower bound of confidence interval.
  pub fn from(&self) -> Option<f64> {
    if let Some(val) = self.value.get("from") {
      return Some(val.as_f64().unwrap());
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

}
