#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// Describes the event of a patient being administered a vaccine or a record of an
/// immunization as reported by a patient, a clinician or another party.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Immunization_Education {
  /// Extensions for presentationDate
  #[serde(rename = "_presentationDate")]
  _presentation_date: Option<Element>,

  /// Reference pointer to the educational material given to the patient if the
  /// information was on line.
  reference: Option<String>,

  /// Date the educational material was given to the patient.
  #[serde(rename = "presentationDate")]
  presentation_date: Option<String>,

  /// Extensions for reference
  #[serde(rename = "_reference")]
  _reference: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Date the educational material was published.
  #[serde(rename = "publicationDate")]
  publication_date: Option<String>,

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

  /// Extensions for publicationDate
  #[serde(rename = "_publicationDate")]
  _publication_date: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for documentType
  #[serde(rename = "_documentType")]
  _document_type: Option<Element>,

  /// Identifier of the material presented to the patient.
  #[serde(rename = "documentType")]
  document_type: Option<String>,

}
