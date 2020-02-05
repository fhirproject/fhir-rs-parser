use serde::{Deserialize, Serialize};

/// Specifies contact information for a person or organization.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContactDetail {
  /// The name of an individual to contact.
  name: String,

  /// Extensions for name
  _name: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The contact details for the individual (if a name was provided) or the
  /// organization.
  telecom: Vec<ContactPoint>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

}
