#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Coding::Coding;
use serde_json::value::Value;



/// The metadata about a resource. This is content in the resource that is
/// maintained by the infrastructure. Changes to the content might not always be
/// associated with version changes to the resource.

#[derive(Debug)]
pub struct Meta<'a> {
  pub value: &'a Value,
}

impl Meta<'_> {
  /// A list of profiles (references to [[[StructureDefinition]]] resources) that this
  /// resource claims to conform to. The URL is a reference to
  /// [[[StructureDefinition.url]]].
  pub fn profile(&self) -> Option<Vec<&str>> {
    if let Some(Value::Array(val)) = self.value.get("profile") {
      return Some(val.into_iter().map(|e| e.as_str().unwrap()).collect::<Vec<_>>());
    }
    return None;
  }

  /// When the resource last changed - e.g. when the version changed.
  pub fn last_updated(&self) -> Option<&str> {
    if let Some(Value::String(string)) = self.value.get("lastUpdated") {
      return Some(string);
    }
    return None;
  }

  /// A uri that identifies the source system of the resource. This provides a minimal
  /// amount of [[[Provenance]]] information that can be used to track or
  /// differentiate the source of information in the resource. The source may identify
  /// another FHIR server, document, message, database, etc.
  pub fn source(&self) -> Option<&str> {
    if let Some(Value::String(string)) = self.value.get("source") {
      return Some(string);
    }
    return None;
  }

  /// Extensions for versionId
  pub fn _version_id(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_versionId") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for source
  pub fn _source(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_source") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Security labels applied to this resource. These tags connect specific resources
  /// to the overall security policy and infrastructure.
  pub fn security(&self) -> Option<Vec<Coding>> {
    if let Some(Value::Array(val)) = self.value.get("security") {
      return Some(val.into_iter().map(|e| Coding { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Tags applied to this resource. Tags are intended to be used to identify and
  /// relate resources to process and workflow, and applications are not required to
  /// consider the tags when interpreting the meaning of a resource.
  pub fn tag(&self) -> Option<Vec<Coding>> {
    if let Some(Value::Array(val)) = self.value.get("tag") {
      return Some(val.into_iter().map(|e| Coding { value: e }).collect::<Vec<_>>());
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

  /// The version specific identifier, as it appears in the version portion of the
  /// URL. This value changes when the resource is created, updated, or deleted.
  pub fn version_id(&self) -> Option<&str> {
    if let Some(Value::String(string)) = self.value.get("versionId") {
      return Some(string);
    }
    return None;
  }

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<&str> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string);
    }
    return None;
  }

  /// Extensions for lastUpdated
  pub fn _last_updated(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_lastUpdated") {
      return Some(Element { value: val });
    }
    return None;
  }

  pub fn validate(&self) -> bool {
    if let Some(_val) = self.profile() {
      _val.into_iter().for_each(|_e| {});
    }
    if let Some(_val) = self.last_updated() {
    }
    if let Some(_val) = self.source() {
    }
    if let Some(_val) = self._version_id() {
      _val.validate();
    }
    if let Some(_val) = self._source() {
      _val.validate();
    }
    if let Some(_val) = self.security() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.tag() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.version_id() {
    }
    if let Some(_val) = self.id() {
    }
    if let Some(_val) = self._last_updated() {
      _val.validate();
    }
    return true;
  }

}
