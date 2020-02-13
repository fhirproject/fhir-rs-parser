#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Quantity::Quantity;
use crate::model::NutritionOrder_Administration::NutritionOrder_Administration;
use crate::model::Element::Element;
use serde_json::value::Value;



/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.

#[derive(Debug)]
pub struct NutritionOrder_EnteralFormula<'a> {
  pub value: &'a Value,
}

impl NutritionOrder_EnteralFormula<'_> {
  /// The product or brand name of the enteral or infant formula product such as "ACME
  /// Adult Standard Formula".
  pub fn base_formula_product_name(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("baseFormulaProductName") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The route or physiological path of administration into the patient's
  /// gastrointestinal  tract for purposes of providing the formula feeding, e.g.
  /// nasogastric tube.
  pub fn routeof_administration(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("routeofAdministration") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Free text formula administration, feeding instructions or additional
  /// instructions or information.
  pub fn administration_instruction(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("administrationInstruction") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The product or brand name of the type of modular component to be added to the
  /// formula.
  pub fn additive_product_name(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("additiveProductName") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for administrationInstruction
  pub fn _administration_instruction(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_administrationInstruction") {
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

  /// The amount of energy (calories) that the formula should provide per specified
  /// volume, typically per mL or fluid oz.  For example, an infant may require a
  /// formula that provides 24 calories per fluid ounce or an adult may require an
  /// enteral formula that provides 1.5 calorie/mL.
  pub fn caloric_density(&self) -> Option<Quantity> {
    if let Some(val) = self.value.get("caloricDensity") {
      return Some(Quantity { value: val });
    }
    return None;
  }

  /// Formula administration instructions as structured data.  This repeating
  /// structure allows for changing the administration rate or volume over time for
  /// both bolus and continuous feeding.  An example of this would be an instruction
  /// to increase the rate of continuous feeding every 2 hours.
  pub fn administration(&self) -> Option<Vec<NutritionOrder_Administration>> {
    if let Some(Value::Array(val)) = self.value.get("administration") {
      return Some(val.into_iter().map(|e| NutritionOrder_Administration { value: e }).collect::<Vec<_>>());
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

  /// The maximum total quantity of formula that may be administered to a subject over
  /// the period of time, e.g. 1440 mL over 24 hours.
  pub fn max_volume_to_deliver(&self) -> Option<Quantity> {
    if let Some(val) = self.value.get("maxVolumeToDeliver") {
      return Some(Quantity { value: val });
    }
    return None;
  }

  /// Extensions for additiveProductName
  pub fn _additive_product_name(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_additiveProductName") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for baseFormulaProductName
  pub fn _base_formula_product_name(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_baseFormulaProductName") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The type of enteral or infant formula such as an adult standard formula with
  /// fiber or a soy-based infant formula.
  pub fn base_formula_type(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("baseFormulaType") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Indicates the type of modular component such as protein, carbohydrate, fat or
  /// fiber to be provided in addition to or mixed with the base formula.
  pub fn additive_type(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("additiveType") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

}
