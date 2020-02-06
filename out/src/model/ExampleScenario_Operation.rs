#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::ExampleScenario_ContainedInstance::ExampleScenario_ContainedInstance;
use crate::model::Element::Element;


/// Example of workflow instance.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenario_Operation {
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

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Each resource instance used by the responder.
  response: ExampleScenario_ContainedInstance,

  /// Extensions for receiverActive
  #[serde(rename = "_receiverActive")]
  _receiver_active: Element,

  /// Extensions for type
  _type: Element,

  /// Extensions for name
  _name: Element,

  /// Each resource instance used by the initiator.
  request: ExampleScenario_ContainedInstance,

  /// The human-friendly name of the interaction.
  name: String,

  /// The type of operation - CRUD.
  #[serde(rename = "type")]
  fhir_type: String,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for initiator
  _initiator: Element,

  /// The sequential number of the interaction, e.g. 1.2.5.
  number: String,

  /// Extensions for description
  _description: Element,

  /// Who starts the transaction.
  initiator: String,

  /// Extensions for receiver
  _receiver: Element,

  /// Whether the initiator is deactivated right after the transaction.
  #[serde(rename = "initiatorActive")]
  initiator_active: bool,

  /// Extensions for number
  _number: Element,

  /// Extensions for initiatorActive
  #[serde(rename = "_initiatorActive")]
  _initiator_active: Element,

  /// Whether the receiver is deactivated right after the transaction.
  #[serde(rename = "receiverActive")]
  receiver_active: bool,

  /// A comment to be inserted in the diagram.
  description: String,

  /// Who receives the transaction.
  receiver: String,

}
