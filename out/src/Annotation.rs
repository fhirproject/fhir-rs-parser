use serde::{Deserialize, Serialize};

/// A  text note which also  contains information about who made the statement and
/// when.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Annotation {
  /// The text of the annotation in markdown format.
  text: markdown,

  /// Extensions for time
  _time: Element,

  /// Indicates when this particular annotation was made.
  time: dateTime,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for text
  _text: Element,

  /// The individual responsible for making the annotation.
  #[serde(rename = "authorString")]
  author_string: String,

  /// The individual responsible for making the annotation.
  #[serde(rename = "authorReference")]
  author_reference: Reference,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for authorString
  #[serde(rename = "_authorString")]
  _author_string: Element,

}
