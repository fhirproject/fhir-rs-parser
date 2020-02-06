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
  /// The actual contact point details, in a form that is meaningful to the designated
  /// communication system (i.e. phone number or email address).
  value: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Telecommunications form for contact point - what communications system is
  /// required to make use of the contact.
  system: Option<ContactPointSystem>,

  /// Extensions for system
  #[serde(rename = "_system")]
  _system: Option<Element>,

  /// Time period when the contact point was/is in use.
  period: Option<Period>,

  /// Extensions for value
  #[serde(rename = "_value")]
  _value: Option<Element>,

  /// Extensions for rank
  #[serde(rename = "_rank")]
  _rank: Option<Element>,

  /// Extensions for use
  #[serde(rename = "_use")]
  _use: Option<Element>,

  /// Specifies a preferred order in which to use a set of contacts. ContactPoints
  /// with lower rank values are more preferred than those with higher rank values.
  rank: Option<i32>,

  /// Identifies the purpose for the contact point.
  #[serde(rename = "use")]
  fhir_use: Option<ContactPointUse>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

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
