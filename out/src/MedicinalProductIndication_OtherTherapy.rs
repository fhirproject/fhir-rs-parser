use serde::{Deserialize, Serialize};

/// Indication for the Medicinal Product.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MedicinalProductIndication_OtherTherapy {
  /// Reference to a specific medication (active substance, medicinal product or class
  /// of products) as part of an indication or contraindication.
  #[serde(rename = "medicationCodeableConcept")]
  medication_codeable_concept: CodeableConcept,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

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

  /// Reference to a specific medication (active substance, medicinal product or class
  /// of products) as part of an indication or contraindication.
  #[serde(rename = "medicationReference")]
  medication_reference: Reference,

  /// The type of relationship between the medicinal product indication or
  /// contraindication and another therapy.
  #[serde(rename = "therapyRelationshipType")]
  therapy_relationship_type: CodeableConcept,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

}
