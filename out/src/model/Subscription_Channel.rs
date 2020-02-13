#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Element::Element;
use serde_json::value::Value;



/// The subscription resource is used to define a push-based subscription from a
/// server to another system. Once a subscription is registered with the server, the
/// server checks every resource that is created or updated, and if the resource
/// matches the given criteria, it sends a message on the defined "channel" so that
/// another system can take an appropriate action.

#[derive(Debug)]
pub struct Subscription_Channel<'a> {
  pub value: &'a Value,
}

impl Subscription_Channel<'_> {
  /// Additional headers / information to send as part of the notification.
  pub fn header(&self) -> Option<Vec<String>> {
    if let Some(Value::Array(val)) = self.value.get("header") {
      return Some(val.into_iter().map(|e| e.as_str().unwrap().to_string()).collect::<Vec<_>>());
    }
    return None;
  }

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  pub fn extension(&self) -> Option<Vec<Extension>> {
    if let Some(Value::Array(val)) = self.value.get("extension") {
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for header
  pub fn _header(&self) -> Option<Vec<Element>> {
    if let Some(Value::Array(val)) = self.value.get("_header") {
      return Some(val.into_iter().map(|e| Element { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

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
  pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
    if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The url that describes the actual end-point to send messages to.
  pub fn endpoint(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("endpoint") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for payload
  pub fn _payload(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_payload") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The type of channel to send notifications on.
  pub fn fhir_type(&self) -> Option<Subscription_ChannelType> {
    if let Some(Value::String(val)) = self.value.get("type") {
      return Some(Subscription_ChannelType::from_string(&val).unwrap());
    }
    return None;
  }

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for endpoint
  pub fn _endpoint(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_endpoint") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for type
  pub fn _type(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_type") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The mime type to send the payload in - either application/fhir+xml, or
  /// application/fhir+json. If the payload is not present, then there is no payload
  /// in the notification, just a notification. The mime type "text/plain" may also be
  /// used for Email and SMS subscriptions.
  pub fn payload(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("payload") {
      return Some(string.to_string());
    }
    return None;
  }

}

#[derive(Debug)]
pub enum Subscription_ChannelType {
  RestHook,
  Websocket,
  Email,
  Sms,
  Message,
}

impl Subscription_ChannelType {
    pub fn from_string(string: &str) -> Option<Subscription_ChannelType> {
      match string {
        "rest-hook" => Some(Subscription_ChannelType::RestHook),
        "websocket" => Some(Subscription_ChannelType::Websocket),
        "email" => Some(Subscription_ChannelType::Email),
        "sms" => Some(Subscription_ChannelType::Sms),
        "message" => Some(Subscription_ChannelType::Message),
        _ => None,
    }
  }
}

