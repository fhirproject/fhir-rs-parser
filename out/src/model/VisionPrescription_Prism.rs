#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// An authorization for the provision of glasses and/or contact lenses to a
/// patient.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisionPrescription_Prism {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The relative base, or reference lens edge, for the prism.
  base: Option<VisionPrescription_PrismBase>,

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

  /// Extensions for amount
  #[serde(rename = "_amount")]
  _amount: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Amount of prism to compensate for eye alignment in fractional units.
  amount: Option<f32>,

  /// Extensions for base
  #[serde(rename = "_base")]
  _base: Option<Element>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum VisionPrescription_PrismBase {
  #[serde(rename = "up")]
  Up,

  #[serde(rename = "down")]
  Down,

  #[serde(rename = "in")]
  In,

  #[serde(rename = "out")]
  Out,

}
