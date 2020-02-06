#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::ExampleScenario_ContainedInstance::ExampleScenario_ContainedInstance;


/// Example of workflow instance.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenario_Operation {
  /// Who starts the transaction.
  initiator: Option<String>,

  /// Who receives the transaction.
  receiver: Option<String>,

  /// Extensions for initiator
  #[serde(rename = "_initiator")]
  _initiator: Option<Element>,

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

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The sequential number of the interaction, e.g. 1.2.5.
  number: Option<String>,

  /// The type of operation - CRUD.
  #[serde(rename = "type")]
  fhir_type: Option<String>,

  /// The human-friendly name of the interaction.
  name: Option<String>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// Whether the receiver is deactivated right after the transaction.
  #[serde(rename = "receiverActive")]
  receiver_active: Option<bool>,

  /// Each resource instance used by the initiator.
  request: Option<ExampleScenario_ContainedInstance>,

  /// Each resource instance used by the responder.
  response: Option<ExampleScenario_ContainedInstance>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// Whether the initiator is deactivated right after the transaction.
  #[serde(rename = "initiatorActive")]
  initiator_active: Option<bool>,

  /// Extensions for receiverActive
  #[serde(rename = "_receiverActive")]
  _receiver_active: Option<Element>,

  /// Extensions for type
  #[serde(rename = "_type")]
  _type: Option<Element>,

  /// A comment to be inserted in the diagram.
  description: Option<String>,

  /// Extensions for initiatorActive
  #[serde(rename = "_initiatorActive")]
  _initiator_active: Option<Element>,

  /// Extensions for number
  #[serde(rename = "_number")]
  _number: Option<Element>,

  /// Extensions for receiver
  #[serde(rename = "_receiver")]
  _receiver: Option<Element>,

}
