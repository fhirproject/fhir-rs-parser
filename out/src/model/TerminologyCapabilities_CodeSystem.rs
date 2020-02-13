#![allow(unused_imports, non_camel_case_types)]

use crate::model::TerminologyCapabilities_Version::TerminologyCapabilities_Version;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;



/// A TerminologyCapabilities resource documents a set of capabilities (behaviors)
/// of a FHIR Terminology Server that may be used as a statement of actual server
/// functionality or a statement of required or desired server implementation.

#[derive(Debug)]
pub struct TerminologyCapabilities_CodeSystem<'a> {
  pub value: &'a Value,
}

impl TerminologyCapabilities_CodeSystem<'_> {
  /// For the code system, a list of versions that are supported by the server.
  pub fn version(&self) -> Option<Vec<TerminologyCapabilities_Version>> {
    if let Some(Value::Array(val)) = self.value.get("version") {
      return Some(val.into_iter().map(|e| TerminologyCapabilities_Version { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// True if subsumption is supported for this version of the code system.
  pub fn subsumption(&self) -> Option<bool> {
    if let Some(val) = self.value.get("subsumption") {
      return Some(val.as_bool().unwrap());
    }
    return None;
  }

  /// Extensions for subsumption
  pub fn _subsumption(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_subsumption") {
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

  /// URI for the Code System.
  pub fn uri(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("uri") {
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

}
