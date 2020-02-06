#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::SubstanceSpecification_Isotope::SubstanceSpecification_Isotope;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Reference::Reference;
use crate::model::SubstanceSpecification_Representation::SubstanceSpecification_Representation;
use crate::model::Extension::Extension;
use crate::model::SubstanceSpecification_MolecularWeight::SubstanceSpecification_MolecularWeight;


/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSpecification_Structure {
  /// Stereochemistry type.
  stereochemistry: Option<CodeableConcept>,

  /// The molecular weight or weight range (for proteins, polymers or nucleic acids).
  #[serde(rename = "molecularWeight")]
  molecular_weight: Option<SubstanceSpecification_MolecularWeight>,

  /// Optical activity type.
  #[serde(rename = "opticalActivity")]
  optical_activity: Option<CodeableConcept>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Applicable for single substances that contain a radionuclide or a non-natural
  /// isotopic ratio.
  isotope: Option<Vec<SubstanceSpecification_Isotope>>,

  /// Supporting literature.
  source: Option<Vec<Box<Reference>>>,

  /// Molecular formula.
  #[serde(rename = "molecularFormula")]
  molecular_formula: Option<String>,

  /// Specified per moiety according to the Hill system, i.e. first C, then H, then
  /// alphabetical, each moiety separated by a dot.
  #[serde(rename = "molecularFormulaByMoiety")]
  molecular_formula_by_moiety: Option<String>,

  /// Molecular structural representation.
  representation: Option<Vec<SubstanceSpecification_Representation>>,

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
  modifier_extension: Option<Vec<Box<Extension>>>,

  /// Extensions for molecularFormula
  #[serde(rename = "_molecularFormula")]
  _molecular_formula: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for molecularFormulaByMoiety
  #[serde(rename = "_molecularFormulaByMoiety")]
  _molecular_formula_by_moiety: Option<Element>,

}
