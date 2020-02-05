use serde::{Deserialize, Serialize};

/// A record of an event made for purposes of maintaining a security log. Typical
/// uses include detection of intrusion attempts and monitoring for inappropriate
/// usage.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuditEvent_Network {
  /// Extensions for type
  _type: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// Extensions for address
  _address: Element,

  /// An identifier for the type of network access point that originated the audit
  /// event.
  type: AuditEvent_NetworkType,

  /// An identifier for the network access point of the user device for the audit
  /// event.
  address: String,

}

#[derive(Debug, Serialize, Deserialize)]
enum AuditEvent_NetworkType {
  #[serde(rename = "1")]
  1,

  #[serde(rename = "2")]
  2,

  #[serde(rename = "3")]
  3,

  #[serde(rename = "4")]
  4,

  #[serde(rename = "5")]
  5,

}
