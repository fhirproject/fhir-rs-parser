#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Expression::Expression;
use crate::model::Extension::Extension;
use serde_json::value::Value;



/// A group of related requests that can be used to capture intended activities that
/// have inter-dependencies such as "give this medication after that one".

#[derive(Debug)]
pub struct RequestGroup_Condition<'a> {
  pub value: &'a Value,
}

impl RequestGroup_Condition<'_> {
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

  /// The kind of condition.
  pub fn kind(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("kind") {
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

  /// Extensions for kind
  pub fn _kind(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_kind") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// An expression that returns true or false, indicating whether or not the
  /// condition is satisfied.
  pub fn expression(&self) -> Option<Expression> {
    if let Some(val) = self.value.get("expression") {
      return Some(Expression { value: val });
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

  pub fn validate(&self) -> bool {
    if let Some(_val) = self.extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.kind() {
    }
    if let Some(_val) = self.id() {
    }
    if let Some(_val) = self._kind() {
      _val.validate();
    }
    if let Some(_val) = self.expression() {
      _val.validate();
    }
    if let Some(_val) = self.modifier_extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    return true;
  }

}