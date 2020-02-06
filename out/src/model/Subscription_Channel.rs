#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// The subscription resource is used to define a push-based subscription from a
/// server to another system. Once a subscription is registered with the server, the
/// server checks every resource that is created or updated, and if the resource
/// matches the given criteria, it sends a message on the defined "channel" so that
/// another system can take an appropriate action.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscription_Channel {
  /// The type of channel to send notifications on.
  #[serde(rename = "type")]
  fhir_type: Option<Subscription_ChannelType>,

  /// Extensions for header
  #[serde(rename = "_header")]
  _header: Option<Vec<Element>>,

  /// Extensions for payload
  #[serde(rename = "_payload")]
  _payload: Option<Element>,

  /// Extensions for type
  #[serde(rename = "_type")]
  _type: Option<Element>,

  /// The url that describes the actual end-point to send messages to.
  endpoint: Option<String>,

  /// Extensions for endpoint
  #[serde(rename = "_endpoint")]
  _endpoint: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.    Modifier extensions
  /// SHALL NOT change the meaning of any elements on Resource or DomainResource
  /// (including cannot change the meaning of modifierExtension itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Option<Vec<Box<Extension>>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The mime type to send the payload in - either application/fhir+xml, or
  /// application/fhir+json. If the payload is not present, then there is no payload
  /// in the notification, just a notification. The mime type "text/plain" may also be
  /// used for Email and SMS subscriptions.
  payload: Option<String>,

  /// Additional headers / information to send as part of the notification.
  header: Option<Vec<String>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum Subscription_ChannelType {
  #[serde(rename = "rest-hook")]
  RestHook,

  #[serde(rename = "websocket")]
  Websocket,

  #[serde(rename = "email")]
  Email,

  #[serde(rename = "sms")]
  Sms,

  #[serde(rename = "message")]
  Message,

}
