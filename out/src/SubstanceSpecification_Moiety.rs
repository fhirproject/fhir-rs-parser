use serde::{Deserialize, Serialize};

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SubstanceSpecification_Moiety {
  /// Extensions for amountString
  #[serde(rename = "_amountString")]
  _amount_string: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

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

  /// Role that the moiety is playing.
  role: CodeableConcept,

  /// Extensions for name
  _name: Element,

  /// Optical activity type.
  #[serde(rename = "opticalActivity")]
  optical_activity: CodeableConcept,

  /// Stereochemistry type.
  stereochemistry: CodeableConcept,

  /// Extensions for molecularFormula
  #[serde(rename = "_molecularFormula")]
  _molecular_formula: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Identifier by which this moiety substance is known.
  identifier: Identifier,

  /// Molecular formula.
  #[serde(rename = "molecularFormula")]
  molecular_formula: String,

  /// Quantitative value for this moiety.
  #[serde(rename = "amountQuantity")]
  amount_quantity: Quantity,

  /// Quantitative value for this moiety.
  #[serde(rename = "amountString")]
  amount_string: String,

  /// Textual name for this moiety substance.
  name: String,

}
