#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::VisionPrescription_Prism::VisionPrescription_Prism;
use crate::model::Quantity::Quantity;
use crate::model::Element::Element;
use crate::model::Annotation::Annotation;


/// An authorization for the provision of glasses and/or contact lenses to a
/// patient.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisionPrescription_LensSpecification {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Power adjustment for multifocal lenses measured in dioptres (0.25 units).
  add: Option<f32>,

  /// Extensions for color
  #[serde(rename = "_color")]
  _color: Option<Element>,

  /// Brand recommendations or restrictions.
  brand: Option<String>,

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

  /// Extensions for backCurve
  #[serde(rename = "_backCurve")]
  _back_curve: Option<Element>,

  /// Extensions for brand
  #[serde(rename = "_brand")]
  _brand: Option<Element>,

  /// Extensions for sphere
  #[serde(rename = "_sphere")]
  _sphere: Option<Element>,

  /// Extensions for power
  #[serde(rename = "_power")]
  _power: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for cylinder
  #[serde(rename = "_cylinder")]
  _cylinder: Option<Element>,

  /// Contact lens power measured in dioptres (0.25 units).
  power: Option<f32>,

  /// Notes for special requirements such as coatings and lens materials.
  note: Option<Vec<Annotation>>,

  /// Back curvature measured in millimetres.
  #[serde(rename = "backCurve")]
  back_curve: Option<f32>,

  /// Extensions for add
  #[serde(rename = "_add")]
  _add: Option<Element>,

  /// Extensions for eye
  #[serde(rename = "_eye")]
  _eye: Option<Element>,

  /// Adjustment for astigmatism measured in integer degrees.
  axis: Option<i32>,

  /// Contact lens diameter measured in millimetres.
  diameter: Option<f32>,

  /// The eye for which the lens specification applies.
  eye: Option<VisionPrescription_LensSpecificationEye>,

  /// Allows for adjustment on two axis.
  prism: Option<Vec<VisionPrescription_Prism>>,

  /// Extensions for diameter
  #[serde(rename = "_diameter")]
  _diameter: Option<Element>,

  /// Identifies the type of vision correction product which is required for the
  /// patient.
  product: CodeableConcept,

  /// Extensions for axis
  #[serde(rename = "_axis")]
  _axis: Option<Element>,

  /// Lens power measured in dioptres (0.25 units).
  sphere: Option<f32>,

  /// The recommended maximum wear period for the lens.
  duration: Option<Quantity>,

  /// Special color or pattern.
  color: Option<String>,

  /// Power adjustment for astigmatism measured in dioptres (0.25 units).
  cylinder: Option<f32>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum VisionPrescription_LensSpecificationEye {
  #[serde(rename = "right")]
  Right,

  #[serde(rename = "left")]
  Left,

}
