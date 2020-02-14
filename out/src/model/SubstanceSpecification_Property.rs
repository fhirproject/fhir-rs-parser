#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Quantity::Quantity;
use serde_json::value::Value;



/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.

#[derive(Debug)]
pub struct SubstanceSpecification_Property<'a> {
  pub value: &'a Value,
}

impl SubstanceSpecification_Property<'_> {
  /// Parameters that were used in the measurement of a property (e.g. for viscosity:
  /// measured at 20C with a pH of 7.1).
  pub fn parameters(&self) -> Option<&str> {
    if let Some(Value::String(string)) = self.value.get("parameters") {
      return Some(string);
    }
    return None;
  }

  /// Extensions for amountString
  pub fn _amount_string(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_amountString") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A substance upon which a defining property depends (e.g. for solubility: in
  /// water, in alcohol).
  pub fn defining_substance_codeable_concept(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("definingSubstanceCodeableConcept") {
      return Some(CodeableConcept { value: val });
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

  /// Extensions for parameters
  pub fn _parameters(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_parameters") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A substance upon which a defining property depends (e.g. for solubility: in
  /// water, in alcohol).
  pub fn defining_substance_reference(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("definingSubstanceReference") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Quantitative value for this property.
  pub fn amount_quantity(&self) -> Option<Quantity> {
    if let Some(val) = self.value.get("amountQuantity") {
      return Some(Quantity { value: val });
    }
    return None;
  }

  /// Quantitative value for this property.
  pub fn amount_string(&self) -> Option<&str> {
    if let Some(Value::String(string)) = self.value.get("amountString") {
      return Some(string);
    }
    return None;
  }

  /// Property type e.g. viscosity, pH, isoelectric point.
  pub fn code(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("code") {
      return Some(CodeableConcept { value: val });
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

  /// A category for this property, e.g. Physical, Chemical, Enzymatic.
  pub fn category(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("category") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  pub fn validate(&self) -> bool {
    if let Some(_val) = self.parameters() {
    }
    if let Some(_val) = self._amount_string() {
      _val.validate();
    }
    if let Some(_val) = self.defining_substance_codeable_concept() {
      _val.validate();
    }
    if let Some(_val) = self.extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self._parameters() {
      _val.validate();
    }
    if let Some(_val) = self.defining_substance_reference() {
      _val.validate();
    }
    if let Some(_val) = self.amount_quantity() {
      _val.validate();
    }
    if let Some(_val) = self.amount_string() {
    }
    if let Some(_val) = self.code() {
      _val.validate();
    }
    if let Some(_val) = self.id() {
    }
    if let Some(_val) = self.modifier_extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.category() {
      _val.validate();
    }
    return true;
  }

}
