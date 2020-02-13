#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;



/// Raw data describing a biological sequence.

#[derive(Debug)]
pub struct MolecularSequence_Inner<'a> {
  pub value: &'a Value,
}

impl MolecularSequence_Inner<'_> {
  /// Structural variant inner start. If the coordinate system is either 0-based or 1-
  /// based, then start position is inclusive.
  pub fn start(&self) -> Option<i64> {
    if let Some(val) = self.value.get("start") {
      return Some(val.as_i64().unwrap());
    }
    return None;
  }

  /// Extensions for start
  pub fn _start(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_start") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Structural variant inner end. If the coordinate system is 0-based then end is
  /// exclusive and does not include the last position. If the coordinate system is 1-
  /// base, then end is inclusive and includes the last position.
  pub fn end(&self) -> Option<i64> {
    if let Some(val) = self.value.get("end") {
      return Some(val.as_i64().unwrap());
    }
    return None;
  }

  /// Extensions for end
  pub fn _end(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_end") {
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
