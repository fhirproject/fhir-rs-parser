#![allow(unused_imports, non_camel_case_types)]

use crate::model::Reference::Reference;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;



/// A  text note which also  contains information about who made the statement and
/// when.

#[derive(Debug)]
pub struct Annotation<'a> {
  pub value: &'a Value,
}

impl Annotation<'_> {
  /// Extensions for authorString
  pub fn _author_string(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_authorString") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The individual responsible for making the annotation.
  pub fn author_reference(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("authorReference") {
      return Some(Reference { value: val });
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

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The text of the annotation in markdown format.
  pub fn text(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("text") {
      return Some(string.to_string());
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

  /// Extensions for time
  pub fn _time(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_time") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Indicates when this particular annotation was made.
  pub fn time(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("time") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The individual responsible for making the annotation.
  pub fn author_string(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("authorString") {
      return Some(string.to_string());
    }
    return None;
  }

}
