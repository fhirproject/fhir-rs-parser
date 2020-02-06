#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Quantity::Quantity;
use crate::model::NutritionOrder_Administration::NutritionOrder_Administration;
use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;


/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrder_EnteralFormula {
  /// The product or brand name of the type of modular component to be added to the
  /// formula.
  #[serde(rename = "additiveProductName")]
  additive_product_name: Option<String>,

  /// The amount of energy (calories) that the formula should provide per specified
  /// volume, typically per mL or fluid oz.  For example, an infant may require a
  /// formula that provides 24 calories per fluid ounce or an adult may require an
  /// enteral formula that provides 1.5 calorie/mL.
  #[serde(rename = "caloricDensity")]
  caloric_density: Option<Quantity>,

  /// Extensions for baseFormulaProductName
  #[serde(rename = "_baseFormulaProductName")]
  _base_formula_product_name: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The type of enteral or infant formula such as an adult standard formula with
  /// fiber or a soy-based infant formula.
  #[serde(rename = "baseFormulaType")]
  base_formula_type: Option<CodeableConcept>,

  /// Indicates the type of modular component such as protein, carbohydrate, fat or
  /// fiber to be provided in addition to or mixed with the base formula.
  #[serde(rename = "additiveType")]
  additive_type: Option<CodeableConcept>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Formula administration instructions as structured data.  This repeating
  /// structure allows for changing the administration rate or volume over time for
  /// both bolus and continuous feeding.  An example of this would be an instruction
  /// to increase the rate of continuous feeding every 2 hours.
  administration: Option<Vec<NutritionOrder_Administration>>,

  /// The route or physiological path of administration into the patient's
  /// gastrointestinal  tract for purposes of providing the formula feeding, e.g.
  /// nasogastric tube.
  #[serde(rename = "routeofAdministration")]
  routeof_administration: Option<CodeableConcept>,

  /// The product or brand name of the enteral or infant formula product such as "ACME
  /// Adult Standard Formula".
  #[serde(rename = "baseFormulaProductName")]
  base_formula_product_name: Option<String>,

  /// The maximum total quantity of formula that may be administered to a subject over
  /// the period of time, e.g. 1440 mL over 24 hours.
  #[serde(rename = "maxVolumeToDeliver")]
  max_volume_to_deliver: Option<Quantity>,

  /// Free text formula administration, feeding instructions or additional
  /// instructions or information.
  #[serde(rename = "administrationInstruction")]
  administration_instruction: Option<String>,

  /// Extensions for administrationInstruction
  #[serde(rename = "_administrationInstruction")]
  _administration_instruction: Option<Element>,

  /// Extensions for additiveProductName
  #[serde(rename = "_additiveProductName")]
  _additive_product_name: Option<Element>,

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
  #[serde(rename = "modifierExtension")]
  modifier_extension: Option<Vec<Extension>>,

}
