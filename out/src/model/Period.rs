#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// A time period defined by a start and end date and optionally time.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Period {
  /// Extensions for end
  #[serde(rename = "_end")]
  _end: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The start of the period. The boundary is inclusive.
  start: Option<String>,

  /// The end of the period. If the end of the period is missing, it means no end was
  /// known or planned at the time the instance was created. The start may be in the
  /// past, and the end date in the future, which means that period is
  /// expected/planned to end at that time.
  end: Option<String>,

  /// Extensions for start
  #[serde(rename = "_start")]
  _start: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

}
