#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Reference::Reference;
use crate::model::Extension::Extension;


/// A  text note which also  contains information about who made the statement and
/// when.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// The individual responsible for making the annotation.
  #[serde(rename = "authorString")]
  author_string: Option<String>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Indicates when this particular annotation was made.
  time: Option<String>,

  /// Extensions for time
  #[serde(rename = "_time")]
  _time: Option<Element>,

  /// The text of the annotation in markdown format.
  text: Option<String>,

  /// Extensions for text
  #[serde(rename = "_text")]
  _text: Option<Element>,

  /// The individual responsible for making the annotation.
  #[serde(rename = "authorReference")]
  author_reference: Option<Box<Reference>>,

  /// Extensions for authorString
  #[serde(rename = "_authorString")]
  _author_string: Option<Element>,

}
