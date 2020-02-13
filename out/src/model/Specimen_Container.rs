#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Quantity::Quantity;
use crate::model::Identifier::Identifier;
use crate::model::Reference::Reference;
use serde_json::value::Value;



/// A sample to be used for analysis.

#[derive(Debug)]
pub struct Specimen_Container<'a> {
  pub value: &'a Value,
}

impl Specimen_Container<'_> {
  /// The quantity of specimen in the container; may be volume, dimensions, or other
  /// appropriate measurements, depending on the specimen type.
  pub fn specimen_quantity(&self) -> Option<Quantity> {
    if let Some(val) = self.value.get("specimenQuantity") {
      return Some(Quantity { value: val });
    }
    return None;
  }

  /// Introduced substance to preserve, maintain or enhance the specimen. Examples:
  /// Formalin, Citrate, EDTA.
  pub fn additive_reference(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("additiveReference") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// The capacity (volume or other measure) the container may contain.
  pub fn capacity(&self) -> Option<Quantity> {
    if let Some(val) = self.value.get("capacity") {
      return Some(Quantity { value: val });
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

  /// Introduced substance to preserve, maintain or enhance the specimen. Examples:
  /// Formalin, Citrate, EDTA.
  pub fn additive_codeable_concept(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("additiveCodeableConcept") {
      return Some(CodeableConcept { value: val });
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

  /// The type of container associated with the specimen (e.g. slide, aliquot, etc.).
  pub fn fhir_type(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("type") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Id for container. There may be multiple; a manufacturer's bar code, lab assigned
  /// identifier, etc. The container ID may differ from the specimen id in some
  /// circumstances.
  pub fn identifier(&self) -> Option<Vec<Identifier>> {
    if let Some(Value::Array(val)) = self.value.get("identifier") {
      return Some(val.into_iter().map(|e| Identifier { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Textual description of the container.
  pub fn description(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("description") {
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

  pub fn validate(&self) -> bool {
    if let Some(_val) = self.specimen_quantity() {
      _val.validate();
    }
    if let Some(_val) = self.additive_reference() {
      _val.validate();
    }
    if let Some(_val) = self.capacity() {
      _val.validate();
    }
    if let Some(_val) = self.id() {
    }
    if let Some(_val) = self.additive_codeable_concept() {
      _val.validate();
    }
    if let Some(_val) = self._description() {
      _val.validate();
    }
    if let Some(_val) = self.modifier_extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.fhir_type() {
      _val.validate();
    }
    if let Some(_val) = self.identifier() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.description() {
    }
    if let Some(_val) = self.extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    return true;
  }

}
