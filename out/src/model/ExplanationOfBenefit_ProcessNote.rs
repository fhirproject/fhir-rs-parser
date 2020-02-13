#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;
use serde_json::value::Value;



/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.

#[derive(Debug)]
pub struct ExplanationOfBenefit_ProcessNote<'a> {
  pub value: &'a Value,
}

impl ExplanationOfBenefit_ProcessNote<'_> {
  /// A number to uniquely identify a note entry.
  pub fn number(&self) -> Option<i64> {
    if let Some(val) = self.value.get("number") {
      return Some(val.as_i64().unwrap());
    }
    return None;
  }

  /// The business purpose of the note text.
  pub fn fhir_type(&self) -> Option<ExplanationOfBenefit_ProcessNoteType> {
    if let Some(Value::String(val)) = self.value.get("type") {
      return Some(ExplanationOfBenefit_ProcessNoteType::from_string(&val).unwrap());
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

  /// Extensions for number
  pub fn _number(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_number") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for text
  pub fn _text(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_text") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A code to define the language used in the text of the note.
  pub fn language(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("language") {
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

  /// Extensions for type
  pub fn _type(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_type") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The explanation or description associated with the processing.
  pub fn text(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("text") {
      return Some(string.to_string());
    }
    return None;
  }

}

#[derive(Debug)]
pub enum ExplanationOfBenefit_ProcessNoteType {
  Display,
  Print,
  Printoper,
}

impl ExplanationOfBenefit_ProcessNoteType {
    pub fn from_string(string: &str) -> Option<ExplanationOfBenefit_ProcessNoteType> {
      match string {
        "display" => Some(ExplanationOfBenefit_ProcessNoteType::Display),
        "print" => Some(ExplanationOfBenefit_ProcessNoteType::Print),
        "printoper" => Some(ExplanationOfBenefit_ProcessNoteType::Printoper),
        _ => None,
    }
  }
}

