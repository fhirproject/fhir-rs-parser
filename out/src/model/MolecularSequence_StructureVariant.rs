#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::MolecularSequence_Outer::MolecularSequence_Outer;
use crate::model::MolecularSequence_Inner::MolecularSequence_Inner;
use crate::model::Extension::Extension;
use crate::model::Element::Element;


/// Raw data describing a biological sequence.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequence_StructureVariant {
  /// Information about chromosome structure variation DNA change type.
  #[serde(rename = "variantType")]
  variant_type: Option<CodeableConcept>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Extensions for exact
  #[serde(rename = "_exact")]
  _exact: Option<Element>,

  /// Length of the variant chromosome.
  length: Option<i32>,

  /// Used to indicate if the outer and inner start-end values have the same meaning.
  exact: Option<bool>,

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

  /// Structural variant outer.
  outer: Option<MolecularSequence_Outer>,

  /// Structural variant inner.
  inner: Option<MolecularSequence_Inner>,

  /// Extensions for length
  #[serde(rename = "_length")]
  _length: Option<Element>,

}
