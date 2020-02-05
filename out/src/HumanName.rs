use serde::{Deserialize, Serialize};

/// A human's name with the ability to identify parts and usage.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HumanName {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Specifies the entire name as it should be displayed e.g. on an application UI.
  /// This may be provided instead of or as well as the specific parts.
  text: String,

  /// Extensions for family
  _family: Element,

  /// Extensions for use
  _use: Element,

  /// Part of the name that is acquired as a title due to academic, legal, employment
  /// or nobility status, etc. and that appears at the start of the name.
  prefix: Vec<String>,

  /// Part of the name that is acquired as a title due to academic, legal, employment
  /// or nobility status, etc. and that appears at the end of the name.
  suffix: Vec<String>,

  /// Identifies the purpose for this name.
  use: HumanNameUse,

  /// Extensions for given
  _given: Vec<Element>,

  /// Extensions for prefix
  _prefix: Vec<Element>,

  /// The part of a name that links to the genealogy. In some cultures (e.g. Eritrea)
  /// the family name of a son is the first name of his father.
  family: String,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for text
  _text: Element,

  /// Given name.
  given: Vec<String>,

  /// Indicates the period of time when this name was valid for the named person.
  period: Period,

  /// Extensions for suffix
  _suffix: Vec<Element>,

}

#[derive(Debug, Serialize, Deserialize)]
enum HumanNameUse {
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
