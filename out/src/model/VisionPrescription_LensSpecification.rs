#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::VisionPrescription_Prism::VisionPrescription_Prism;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::Annotation::Annotation;


/// An authorization for the provision of glasses and/or contact lenses to a
/// patient.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisionPrescription_LensSpecification {
  /// Extensions for eye
  _eye: Element,

  /// Extensions for cylinder
  _cylinder: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Identifies the type of vision correction product which is required for the
  /// patient.
  product: CodeableConcept,

  /// Lens power measured in dioptres (0.25 units).
  sphere: f32,

  /// Allows for adjustment on two axis.
  prism: Vec<VisionPrescription_Prism>,

  /// Power adjustment for multifocal lenses measured in dioptres (0.25 units).
  add: f32,

  /// Extensions for add
  _add: Element,

  /// Extensions for sphere
  _sphere: Element,

  /// Extensions for diameter
  _diameter: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Contact lens diameter measured in millimetres.
  diameter: f32,

  /// Extensions for brand
  _brand: Element,

  /// Extensions for backCurve
  #[serde(rename = "_backCurve")]
  _back_curve: Element,

  /// Adjustment for astigmatism measured in integer degrees.
  axis: i32,

  /// Extensions for power
  _power: Element,

  /// Back curvature measured in millimetres.
  #[serde(rename = "backCurve")]
  back_curve: f32,

  /// The eye for which the lens specification applies.
  eye: VisionPrescription_LensSpecificationEye,

  /// Power adjustment for astigmatism measured in dioptres (0.25 units).
  cylinder: f32,

  /// Contact lens power measured in dioptres (0.25 units).
  power: f32,

  /// Special color or pattern.
  color: String,

  /// Brand recommendations or restrictions.
  brand: String,

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

  /// Extensions for axis
  _axis: Element,

  /// The recommended maximum wear period for the lens.
  duration: Quantity,

  /// Extensions for color
  _color: Element,

  /// Notes for special requirements such as coatings and lens materials.
  note: Vec<Annotation>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum VisionPrescription_LensSpecificationEye {
  #[serde(rename = "right")]
  Right,

  #[serde(rename = "left")]
  Left,

}
