use serde::{Deserialize, Serialize};

/// The header for a message exchange that is either requesting or responding to an
/// action.  The reference(s) that are the subject of the action as well as other
/// information related to the action are typically transmitted in a bundle in which
/// the MessageHeader resource instance is the first resource in the bundle.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessageHeader_Response {
  /// Extensions for identifier
  _identifier: Element,

  /// Extensions for code
  _code: Element,

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

  /// Code that identifies the type of response to the message - whether it was
  /// successful or not, and whether it should be resent or not.
  code: MessageHeader_ResponseCode,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Full details of any issues found in the message.
  details: Reference,

  /// The MessageHeader.id of the message to which this message is a response.
  identifier: id,

}

#[derive(Debug, Serialize, Deserialize)]
enum MessageHeader_ResponseCode {
  #[serde(rename = "ok")]
  Ok,

  #[serde(rename = "transient-error")]
  TransientError,

  #[serde(rename = "fatal-error")]
  FatalError,

}
