#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Identifier::Identifier;


/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSpecification_Moiety {
  /// Textual name for this moiety substance.
  name: String,

  /// Molecular formula.
  #[serde(rename = "molecularFormula")]
  molecular_formula: String,

  /// Quantitative value for this moiety.
  #[serde(rename = "amountQuantity")]
  amount_quantity: Quantity,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for name
  _name: Element,

  /// Quantitative value for this moiety.
  #[serde(rename = "amountString")]
  amount_string: String,

  /// Extensions for amountString
  #[serde(rename = "_amountString")]
  _amount_string: Element,

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
  modifier_extension: Vec<Extension>,

  /// Stereochemistry type.
  stereochemistry: CodeableConcept,

  /// Identifier by which this moiety substance is known.
  identifier: Identifier,

  /// Extensions for molecularFormula
  #[serde(rename = "_molecularFormula")]
  _molecular_formula: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Optical activity type.
  #[serde(rename = "opticalActivity")]
  optical_activity: CodeableConcept,

  /// Role that the moiety is playing.
  role: CodeableConcept,

}
