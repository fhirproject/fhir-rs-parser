#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::GraphDefinition_Target::GraphDefinition_Target;
use serde_json::value::Value;



/// A formal computable definition of a graph of resources - that is, a coherent set
/// of resources that form a graph by following references. The Graph Definition
/// resource defines a set and makes rules about the set.

#[derive(Debug)]
pub struct GraphDefinition_Link<'a> {
  pub value: &'a Value,
}

impl GraphDefinition_Link<'_> {
  /// Which slice (if profiled).
  pub fn slice_name(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("sliceName") {
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

  /// Minimum occurrences for this link.
  pub fn min(&self) -> Option<i64> {
    if let Some(val) = self.value.get("min") {
      return Some(val.as_i64().unwrap());
    }
    return None;
  }

  /// Extensions for min
  pub fn _min(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_min") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Information about why this link is of interest in this graph definition.
  pub fn description(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("description") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for path
  pub fn _path(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_path") {
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

  /// Extensions for max
  pub fn _max(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_max") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for sliceName
  pub fn _slice_name(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_sliceName") {
      return Some(Element { value: val });
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

  /// Potential target for the link.
  pub fn target(&self) -> Option<Vec<GraphDefinition_Target>> {
    if let Some(Value::Array(val)) = self.value.get("target") {
      return Some(val.into_iter().map(|e| GraphDefinition_Target { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A FHIR expression that identifies one of FHIR References to other resources.
  pub fn path(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("path") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Maximum occurrences for this link.
  pub fn max(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("max") {
      return Some(string.to_string());
    }
    return None;
  }

}
