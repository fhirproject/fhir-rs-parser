#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Period::Period;
use crate::model::Extension::Extension;


/// Details for all kinds of technology mediated contact points for a person or
/// organization, including telephone, email, etc.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactPoint {
  /// Time period when the contact point was/is in use.
  period: Period,

  /// Telecommunications form for contact point - what communications system is
  /// required to make use of the contact.
  system: ContactPointSystem,

  /// Identifies the purpose for the contact point.
  #[serde(rename = "use")]
  fhir_use: ContactPointUse,

  /// Specifies a preferred order in which to use a set of contacts. ContactPoints
  /// with lower rank values are more preferred than those with higher rank values.
  rank: i32,

  /// Extensions for rank
  _rank: Element,

  /// Extensions for value
  _value: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for system
  _system: Element,

  /// The actual contact point details, in a form that is meaningful to the designated
  /// communication system (i.e. phone number or email address).
  value: String,

  /// Extensions for use
  _use: Element,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContactPointSystem {
  #[serde(rename = "phone")]
  Phone,

  #[serde(rename = "fax")]
  Fax,

  #[serde(rename = "email")]
  Email,

  #[serde(rename = "pager")]
  Pager,

  #[serde(rename = "url")]
  Url,

  #[serde(rename = "sms")]
  Sms,

  #[serde(rename = "other")]
  Other,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContactPointUse {
  #[serde(rename = "home")]
  Home,

  #[serde(rename = "work")]
  Work,

  #[serde(rename = "temp")]
  Temp,

  #[serde(rename = "old")]
  Old,

  #[serde(rename = "mobile")]
  Mobile,

}
