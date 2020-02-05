use serde::{Deserialize, Serialize};

/// An authorization for the provision of glasses and/or contact lenses to a
/// patient.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VisionPrescription_LensSpecification {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for sphere
  _sphere: Element,

  /// Extensions for diameter
  _diameter: Element,

  /// Extensions for color
  _color: Element,

  /// Brand recommendations or restrictions.
  brand: String,

  /// Back curvature measured in millimetres.
  #[serde(rename = "backCurve")]
  back_curve: decimal,

  /// Allows for adjustment on two axis.
  prism: Vec<VisionPrescription_Prism>,

  /// Power adjustment for multifocal lenses measured in dioptres (0.25 units).
  add: decimal,

  /// Notes for special requirements such as coatings and lens materials.
  note: Vec<Annotation>,

  /// The recommended maximum wear period for the lens.
  duration: Quantity,

  /// The eye for which the lens specification applies.
  eye: VisionPrescription_LensSpecificationEye,

  /// Special color or pattern.
  color: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for cylinder
  _cylinder: Element,

  /// Adjustment for astigmatism measured in integer degrees.
  axis: integer,

  /// Contact lens power measured in dioptres (0.25 units).
  power: decimal,

  /// Identifies the type of vision correction product which is required for the
  /// patient.
  product: CodeableConcept,

  /// Extensions for backCurve
  #[serde(rename = "_backCurve")]
  _back_curve: Element,

  /// Extensions for brand
  _brand: Element,

  /// Extensions for eye
  _eye: Element,

  /// Lens power measured in dioptres (0.25 units).
  sphere: decimal,

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

  /// Power adjustment for astigmatism measured in dioptres (0.25 units).
  cylinder: decimal,

  /// Extensions for axis
  _axis: Element,

  /// Extensions for add
  _add: Element,

  /// Extensions for power
  _power: Element,

  /// Contact lens diameter measured in millimetres.
  diameter: decimal,

}

#[derive(Debug, Serialize, Deserialize)]
enum VisionPrescription_LensSpecificationEye {
  #[serde(rename = "right")]
  Right,

  #[serde(rename = "left")]
  Left,

}
