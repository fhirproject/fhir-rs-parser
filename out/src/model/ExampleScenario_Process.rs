#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::ExampleScenario_Step::ExampleScenario_Step;
use serde_json::value::Value;



/// Example of workflow instance.

#[derive(Debug)]
pub struct ExampleScenario_Process<'a> {
  pub value: &'a Value,
}

impl ExampleScenario_Process<'_> {
  /// Extensions for postConditions
  pub fn _post_conditions(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_postConditions") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Each step of the process.
  pub fn step(&self) -> Option<Vec<ExampleScenario_Step>> {
    if let Some(Value::Array(val)) = self.value.get("step") {
      return Some(val.into_iter().map(|e| ExampleScenario_Step { value: e }).collect::<Vec<_>>());
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

  /// Extensions for preConditions
  pub fn _pre_conditions(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_preConditions") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for title
  pub fn _title(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_title") {
      return Some(Element { value: val });
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

  /// Extensions for description
  pub fn _description(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_description") {
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

  /// The diagram title of the group of operations.
  pub fn title(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("title") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Description of final status after the process ends.
  pub fn post_conditions(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("postConditions") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Description of initial status before the process starts.
  pub fn pre_conditions(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("preConditions") {
      return Some(string.to_string());
    }
    return None;
  }

  /// A longer description of the group of operations.
  pub fn description(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("description") {
      return Some(string.to_string());
    }
    return None;
  }

}
