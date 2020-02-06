#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Element::Element;


/// A human's name with the ability to identify parts and usage.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HumanName {
  /// The part of a name that links to the genealogy. In some cultures (e.g. Eritrea)
  /// the family name of a son is the first name of his father.
  family: Option<String>,

  /// Indicates the period of time when this name was valid for the named person.
  period: Option<Period>,

  /// Extensions for text
  #[serde(rename = "_text")]
  _text: Option<Element>,

  /// Extensions for prefix
  #[serde(rename = "_prefix")]
  _prefix: Option<Vec<Element>>,

  /// Extensions for family
  #[serde(rename = "_family")]
  _family: Option<Element>,

  /// Part of the name that is acquired as a title due to academic, legal, employment
  /// or nobility status, etc. and that appears at the start of the name.
  prefix: Option<Vec<String>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Given name.
  given: Option<Vec<String>>,

  /// Specifies the entire name as it should be displayed e.g. on an application UI.
  /// This may be provided instead of or as well as the specific parts.
  text: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Extensions for suffix
  #[serde(rename = "_suffix")]
  _suffix: Option<Vec<Element>>,

  /// Extensions for use
  #[serde(rename = "_use")]
  _use: Option<Element>,

  /// Identifies the purpose for this name.
  #[serde(rename = "use")]
  fhir_use: Option<HumanNameUse>,

  /// Part of the name that is acquired as a title due to academic, legal, employment
  /// or nobility status, etc. and that appears at the end of the name.
  suffix: Option<Vec<String>>,

  /// Extensions for given
  #[serde(rename = "_given")]
  _given: Option<Vec<Element>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum HumanNameUse {
  #[serde(rename = "usual")]
  Usual,

  #[serde(rename = "official")]
  Official,

  #[serde(rename = "temp")]
  Temp,

  #[serde(rename = "nickname")]
  Nickname,

  #[serde(rename = "anonymous")]
  Anonymous,

  #[serde(rename = "old")]
  Old,

  #[serde(rename = "maiden")]
  Maiden,

}
