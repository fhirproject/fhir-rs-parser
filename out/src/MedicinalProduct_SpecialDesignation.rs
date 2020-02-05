use serde::{Deserialize, Serialize};

/// Detailed definition of a medicinal product, typically for uses other than direct
/// patient care (e.g. regulatory use).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MedicinalProduct_SpecialDesignation {
  /// Condition for which the medicinal use applies.
  #[serde(rename = "indicationReference")]
  indication_reference: Reference,

  /// Date when the designation was granted.
  date: dateTime,

  /// The intended use of the product, e.g. prevention, treatment.
  #[serde(rename = "intendedUse")]
  intended_use: CodeableConcept,

  /// For example granted, pending, expired or withdrawn.
  status: CodeableConcept,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

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

  /// The type of special designation, e.g. orphan drug, minor use.
  type: CodeableConcept,

  /// Animal species for which this applies.
  species: CodeableConcept,

  /// Identifier for the designation, or procedure number.
  identifier: Vec<Identifier>,

  /// Extensions for date
  _date: Element,

  /// Condition for which the medicinal use applies.
  #[serde(rename = "indicationCodeableConcept")]
  indication_codeable_concept: CodeableConcept,

}
