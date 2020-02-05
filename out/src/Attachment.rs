use serde::{Deserialize, Serialize};

/// For referring to data content defined in other formats.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Attachment {
  /// The actual data of the attachment - a sequence of bytes, base64 encoded.
  data: base64Binary,

  /// Extensions for title
  _title: Element,

  /// Extensions for hash
  _hash: Element,

  /// The number of bytes of data that make up this attachment (before base64
  /// encoding, if that is done).
  size: unsignedInt,

  /// A label or set of text to display in place of the data.
  title: String,

  /// Identifies the type of the data in the attachment and allows a method to be
  /// chosen to interpret or render the data. Includes mime type parameters such as
  /// charset where appropriate.
  #[serde(rename = "contentType")]
  content_type: String,

  /// A location where the data can be accessed.
  url: url,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for size
  _size: Element,

  /// Extensions for data
  _data: Element,

  /// The calculated hash of the data using SHA-1. Represented using base64.
  hash: base64Binary,

  /// Extensions for contentType
  #[serde(rename = "_contentType")]
  _content_type: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// The date that the attachment was first created.
  creation: dateTime,

  /// Extensions for url
  _url: Element,

  /// Extensions for creation
  _creation: Element,

  /// The human language of the content. The value can be any valid value according to
  /// BCP 47.
  language: String,

  /// Extensions for language
  _language: Element,

}
