#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Element::Element;
use serde_json::value::Value;



/// Captures constraints on each element within the resource, profile, or extension.

#[derive(Debug)]
pub struct ElementDefinition_Mapping<'a> {
  pub value: &'a Value,
}

impl ElementDefinition_Mapping<'_> {
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

  /// Extensions for language
  pub fn _language(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_language") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// An internal reference to the definition of a mapping.
  pub fn identity(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("identity") {
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

  /// Extensions for identity
  pub fn _identity(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_identity") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Expresses what part of the target specification corresponds to this element.
  pub fn map(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("map") {
      return Some(string.to_string());
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

  /// Extensions for map
  pub fn _map(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_map") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Identifies the computable language in which mapping.map is expressed.
  pub fn language(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("language") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Comments that provide information about the mapping or its use.
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

  pub fn validate(&self) -> bool {
    if let Some(_val) = self.modifier_extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self._language() {
      _val.validate();
    }
    if let Some(_val) = self.identity() {
    }
    if let Some(_val) = self.extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self._identity() {
      _val.validate();
    }
    if let Some(_val) = self.map() {
    }
    if let Some(_val) = self.id() {
    }
    if let Some(_val) = self._map() {
      _val.validate();
    }
    if let Some(_val) = self.language() {
    }
    if let Some(_val) = self.comment() {
    }
    if let Some(_val) = self._comment() {
      _val.validate();
    }
    return true;
  }

}
