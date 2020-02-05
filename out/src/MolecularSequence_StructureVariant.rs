use serde::{Deserialize, Serialize};

/// Raw data describing a biological sequence.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MolecularSequence_StructureVariant {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Structural variant outer.
  outer: MolecularSequence_Outer,

  /// Structural variant inner.
  inner: MolecularSequence_Inner,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Information about chromosome structure variation DNA change type.
  #[serde(rename = "variantType")]
  variant_type: CodeableConcept,

  /// Extensions for length
  _length: Element,

  /// Extensions for exact
  _exact: Element,

  /// Used to indicate if the outer and inner start-end values have the same meaning.
  exact: bool,

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

  /// Length of the variant chromosome.
  length: integer,

}
