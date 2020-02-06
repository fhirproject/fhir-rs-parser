#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// A reference to a code defined by a terminology system.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coding {
  /// Indicates that this coding was chosen by a user directly - e.g. off a pick list
  /// of available items (codes or displays).
  #[serde(rename = "userSelected")]
  user_selected: Option<bool>,

  /// Extensions for display
  #[serde(rename = "_display")]
  _display: Option<Element>,

  /// Extensions for userSelected
  #[serde(rename = "_userSelected")]
  _user_selected: Option<Element>,

  /// A symbol in syntax defined by the system. The symbol may be a predefined code or
  /// an expression in a syntax defined by the coding system (e.g. post-coordination).
  code: Option<String>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The identification of the code system that defines the meaning of the symbol in
  /// the code.
  system: Option<String>,

  /// The version of the code system which was used when choosing this code. Note that
  /// a well-maintained code system does not need the version reported, because the
  /// meaning of codes is consistent across versions. However this cannot consistently
  /// be assured, and when the meaning is not guaranteed to be consistent, the version
  /// SHOULD be exchanged.
  version: Option<String>,

  /// Extensions for system
  #[serde(rename = "_system")]
  _system: Option<Element>,

  /// Extensions for version
  #[serde(rename = "_version")]
  _version: Option<Element>,

  /// A representation of the meaning of the code in the system, following the rules
  /// of the system.
  display: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Extensions for code
  #[serde(rename = "_code")]
  _code: Option<Element>,

}
