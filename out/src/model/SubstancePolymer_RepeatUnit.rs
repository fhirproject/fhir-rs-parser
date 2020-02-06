#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::SubstancePolymer_DegreeOfPolymerisation::SubstancePolymer_DegreeOfPolymerisation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::SubstancePolymer_StructuralRepresentation::SubstancePolymer_StructuralRepresentation;
use crate::model::SubstanceAmount::SubstanceAmount;


/// Todo.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstancePolymer_RepeatUnit {
  /// Todo.
  #[serde(rename = "degreeOfPolymerisation")]
  degree_of_polymerisation: Option<Vec<SubstancePolymer_DegreeOfPolymerisation>>,

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

  /// Todo.
  #[serde(rename = "repeatUnit")]
  repeat_unit: Option<String>,

  /// Todo.
  amount: Option<SubstanceAmount>,

  /// Todo.
  #[serde(rename = "structuralRepresentation")]
  structural_representation: Option<Vec<SubstancePolymer_StructuralRepresentation>>,

  /// Extensions for repeatUnit
  #[serde(rename = "_repeatUnit")]
  _repeat_unit: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Todo.
  #[serde(rename = "orientationOfPolymerisation")]
  orientation_of_polymerisation: Option<CodeableConcept>,

}
