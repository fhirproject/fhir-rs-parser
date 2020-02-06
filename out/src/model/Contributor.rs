#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::ContactDetail::ContactDetail;
use crate::model::Element::Element;


/// A contributor to the content of a knowledge asset, including authors, editors,
/// reviewers, and endorsers.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contributor {
  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// Contact details to assist a user in finding and communicating with the
  /// contributor.
  contact: Option<Vec<ContactDetail>>,

  /// The type of contributor.
  #[serde(rename = "type")]
  fhir_type: Option<ContributorType>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for type
  #[serde(rename = "_type")]
  _type: Option<Element>,

  /// The name of the individual or organization responsible for the contribution.
  name: Option<String>,

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
