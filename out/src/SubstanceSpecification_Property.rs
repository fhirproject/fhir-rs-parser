use serde::{Deserialize, Serialize};

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SubstanceSpecification_Property {
  /// Parameters that were used in the measurement of a property (e.g. for viscosity:
  /// measured at 20C with a pH of 7.1).
  parameters: String,

  /// A category for this property, e.g. Physical, Chemical, Enzymatic.
  category: CodeableConcept,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// A substance upon which a defining property depends (e.g. for solubility: in
  /// water, in alcohol).
  #[serde(rename = "definingSubstanceCodeableConcept")]
  defining_substance_codeable_concept: CodeableConcept,

  /// Quantitative value for this property.
  #[serde(rename = "amountQuantity")]
  amount_quantity: Quantity,

  /// Quantitative value for this property.
  #[serde(rename = "amountString")]
  amount_string: String,

  /// Extensions for amountString
  #[serde(rename = "_amountString")]
  _amount_string: Element,

  /// A substance upon which a defining property depends (e.g. for solubility: in
  /// water, in alcohol).
  #[serde(rename = "definingSubstanceReference")]
  defining_substance_reference: Reference,

  /// Property type e.g. viscosity, pH, isoelectric point.
  code: CodeableConcept,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for parameters
  _parameters: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

}
