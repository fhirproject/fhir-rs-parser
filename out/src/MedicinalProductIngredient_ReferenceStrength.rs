use serde::{Deserialize, Serialize};

/// An ingredient of a manufactured item or pharmaceutical product.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MedicinalProductIngredient_ReferenceStrength {
  /// For when strength is measured at a particular point or distance.
  #[serde(rename = "measurementPoint")]
  measurement_point: String,

  /// The country or countries for which the strength range applies.
  country: Vec<CodeableConcept>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Strength expressed in terms of a reference substance.
  strength: Ratio,

  /// Strength expressed in terms of a reference substance.
  #[serde(rename = "strengthLowLimit")]
  strength_low_limit: Ratio,

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

  /// Extensions for measurementPoint
  #[serde(rename = "_measurementPoint")]
  _measurement_point: Element,

  /// Relevant reference substance.
  substance: CodeableConcept,

}
