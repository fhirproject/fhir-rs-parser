#![allow(unused_imports, non_camel_case_types)]

use crate::model::Reference::Reference;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use serde_json::value::Value;



/// A record of a clinical assessment performed to determine what problem(s) may
/// affect the patient and before planning the treatments or management strategies
/// that are best to manage a patient's condition. Assessments are often 1:1 with a
/// clinical consultation / encounter,  but this varies greatly depending on the
/// clinical workflow. This resource is called "ClinicalImpression" rather than
/// "ClinicalAssessment" to avoid confusion with the recording of assessment tools
/// such as Apgar score.

#[derive(Debug)]
pub struct ClinicalImpression_Finding<'a> {
  pub value: &'a Value,
}

impl ClinicalImpression_Finding<'_> {
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

  /// Extensions for basis
  pub fn _basis(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_basis") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Specific reference for finding or diagnosis, which may include ruled-out or
  /// resolved conditions.
  pub fn item_reference(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("itemReference") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Specific text or code for finding or diagnosis, which may include ruled-out or
  /// resolved conditions.
  pub fn item_codeable_concept(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("itemCodeableConcept") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Which investigations support finding or diagnosis.
  pub fn basis(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("basis") {
      return Some(string.to_string());
    }
    return None;
  }

}
