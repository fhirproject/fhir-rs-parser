#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// For referring to data content defined in other formats.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
  /// Identifies the type of the data in the attachment and allows a method to be
  /// chosen to interpret or render the data. Includes mime type parameters such as
  /// charset where appropriate.
  #[serde(rename = "contentType")]
  content_type: String,

  /// Extensions for data
  _data: Element,

  /// Extensions for language
  _language: Element,

  /// The human language of the content. The value can be any valid value according to
  /// BCP 47.
  language: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// The number of bytes of data that make up this attachment (before base64
  /// encoding, if that is done).
  size: u32,

  /// Extensions for title
  _title: Element,

  /// The actual data of the attachment - a sequence of bytes, base64 encoded.
  data: String,

  /// Extensions for contentType
  #[serde(rename = "_contentType")]
  _content_type: Element,

  /// A label or set of text to display in place of the data.
  title: String,

  /// Extensions for size
  _size: Element,

  /// The date that the attachment was first created.
  creation: String,

  /// A location where the data can be accessed.
  url: String,

  /// The calculated hash of the data using SHA-1. Represented using base64.
  hash: String,

  /// Extensions for hash
  _hash: Element,

  /// Extensions for url
  _url: Element,

  /// Extensions for creation
  _creation: Element,

}
