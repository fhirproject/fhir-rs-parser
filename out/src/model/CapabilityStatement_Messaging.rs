#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::CapabilityStatement_Endpoint::CapabilityStatement_Endpoint;
use crate::model::Extension::Extension;
use crate::model::CapabilityStatement_SupportedMessage::CapabilityStatement_SupportedMessage;


/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR
/// Server for a particular version of FHIR that may be used as a statement of
/// actual server functionality or a statement of required or desired server
/// implementation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityStatement_Messaging {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Documentation about the system's messaging capabilities for this endpoint not
  /// otherwise documented by the capability statement.  For example, the process for
  /// becoming an authorized messaging exchange partner.
  documentation: Option<String>,

  /// Length if the receiver's reliable messaging cache in minutes (if a receiver) or
  /// how long the cache length on the receiver should be (if a sender).
  #[serde(rename = "reliableCache")]
  reliable_cache: Option<u32>,

  /// Extensions for reliableCache
  #[serde(rename = "_reliableCache")]
  _reliable_cache: Option<Element>,

  /// Extensions for documentation
  #[serde(rename = "_documentation")]
  _documentation: Option<Element>,

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
  modifier_extension: Option<Vec<Extension>>,

  /// References to message definitions for messages this system can send or receive.
  #[serde(rename = "supportedMessage")]
  supported_message: Option<Vec<CapabilityStatement_SupportedMessage>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// An endpoint (network accessible address) to which messages and/or replies are to
  /// be sent.
  endpoint: Option<Vec<CapabilityStatement_Endpoint>>,

}
