use serde::{Deserialize, Serialize};

/// Describes the event of a patient being administered a vaccine or a record of an
/// immunization as reported by a patient, a clinician or another party.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Immunization_Education {
  /// Identifier of the material presented to the patient.
  #[serde(rename = "documentType")]
  document_type: String,

  /// Extensions for presentationDate
  #[serde(rename = "_presentationDate")]
  _presentation_date: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for reference
  _reference: Element,

  /// Date the educational material was published.
  #[serde(rename = "publicationDate")]
  publication_date: dateTime,

  /// Date the educational material was given to the patient.
  #[serde(rename = "presentationDate")]
  presentation_date: dateTime,

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

  /// Extensions for publicationDate
  #[serde(rename = "_publicationDate")]
  _publication_date: Element,

  /// Extensions for documentType
  #[serde(rename = "_documentType")]
  _document_type: Element,

  /// Reference pointer to the educational material given to the patient if the
  /// information was on line.
  reference: String,

}
