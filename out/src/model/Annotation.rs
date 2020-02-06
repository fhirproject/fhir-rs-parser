#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::Reference::Reference;


/// A  text note which also  contains information about who made the statement and
/// when.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
  /// Extensions for authorString
  #[serde(rename = "_authorString")]
  _author_string: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Indicates when this particular annotation was made.
  time: String,

  /// The individual responsible for making the annotation.
  #[serde(rename = "authorString")]
  author_string: String,

  /// Extensions for time
  _time: Element,

  /// Extensions for text
  _text: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The individual responsible for making the annotation.
  #[serde(rename = "authorReference")]
  author_reference: Box<Reference>,

  /// The text of the annotation in markdown format.
  text: String,

}
