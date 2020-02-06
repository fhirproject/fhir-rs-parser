#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Reference::Reference;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Attachment::Attachment;


/// A request to convey information; e.g. the CDS system proposes that an alert be
/// sent to a responsible provider, the CDS system proposes that the public health
/// agency be notified about a reportable condition.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunicationRequest_Payload {
  /// Extensions for contentString
  #[serde(rename = "_contentString")]
  _content_string: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// The communicated content (or for multi-part communications, one portion of the
  /// communication).
  #[serde(rename = "contentAttachment")]
  content_attachment: Attachment,

  /// The communicated content (or for multi-part communications, one portion of the
  /// communication).
  #[serde(rename = "contentReference")]
  content_reference: Box<Reference>,

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
  /// resource are required to check for modifier extensions.    Modifier extensions
  /// SHALL NOT change the meaning of any elements on Resource or DomainResource
  /// (including cannot change the meaning of modifierExtension itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// The communicated content (or for multi-part communications, one portion of the
  /// communication).
  #[serde(rename = "contentString")]
  content_string: String,

}
