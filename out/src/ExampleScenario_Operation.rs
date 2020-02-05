use serde::{Deserialize, Serialize};

/// Example of workflow instance.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExampleScenario_Operation {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Whether the receiver is deactivated right after the transaction.
  #[serde(rename = "receiverActive")]
  receiver_active: bool,

  /// Extensions for name
  _name: Element,

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

  /// Extensions for receiver
  _receiver: Element,

  /// Whether the initiator is deactivated right after the transaction.
  #[serde(rename = "initiatorActive")]
  initiator_active: bool,

  /// The type of operation - CRUD.
  type: String,

  /// Extensions for initiatorActive
  #[serde(rename = "_initiatorActive")]
  _initiator_active: Element,

  /// Extensions for receiverActive
  #[serde(rename = "_receiverActive")]
  _receiver_active: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Each resource instance used by the initiator.
  request: ExampleScenario_ContainedInstance,

  /// Extensions for initiator
  _initiator: Element,

  /// Each resource instance used by the responder.
  response: ExampleScenario_ContainedInstance,

  /// Extensions for description
  _description: Element,

  /// Extensions for number
  _number: Element,

  /// The human-friendly name of the interaction.
  name: String,

  /// Who starts the transaction.
  initiator: String,

  /// Who receives the transaction.
  receiver: String,

  /// A comment to be inserted in the diagram.
  description: markdown,

  /// The sequential number of the interaction, e.g. 1.2.5.
  number: String,

  /// Extensions for type
  _type: Element,

}
