#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use crate::model::Extension::Extension;


/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSpecification_Property {
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

  /// A category for this property, e.g. Physical, Chemical, Enzymatic.
  category: Option<CodeableConcept>,

  /// Quantitative value for this property.
  #[serde(rename = "amountString")]
  amount_string: Option<String>,

  /// Property type e.g. viscosity, pH, isoelectric point.
  code: Option<CodeableConcept>,

  /// Quantitative value for this property.
  #[serde(rename = "amountQuantity")]
  amount_quantity: Option<Quantity>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// A substance upon which a defining property depends (e.g. for solubility: in
  /// water, in alcohol).
  #[serde(rename = "definingSubstanceReference")]
  defining_substance_reference: Option<Box<Reference>>,

  /// Parameters that were used in the measurement of a property (e.g. for viscosity:
  /// measured at 20C with a pH of 7.1).
  parameters: Option<String>,

  /// Extensions for amountString
  #[serde(rename = "_amountString")]
  _amount_string: Option<Element>,

  /// Extensions for parameters
  #[serde(rename = "_parameters")]
  _parameters: Option<Element>,

  /// A substance upon which a defining property depends (e.g. for solubility: in
  /// water, in alcohol).
  #[serde(rename = "definingSubstanceCodeableConcept")]
  defining_substance_codeable_concept: Option<CodeableConcept>,

}
