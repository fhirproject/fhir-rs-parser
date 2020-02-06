#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::ContactDetail::ContactDetail;


/// A contributor to the content of a knowledge asset, including authors, editors,
/// reviewers, and endorsers.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contributor {
  /// Extensions for type
  _type: Element,

  /// The type of contributor.
  #[serde(rename = "type")]
  fhir_type: ContributorType,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for name
  _name: Element,

  /// Contact details to assist a user in finding and communicating with the
  /// contributor.
  contact: Vec<ContactDetail>,

  /// The name of the individual or organization responsible for the contribution.
  name: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContributorType {
  #[serde(rename = "author")]
  Author,

  #[serde(rename = "editor")]
  Editor,

  #[serde(rename = "reviewer")]
  Reviewer,

  #[serde(rename = "endorser")]
  Endorser,

}
