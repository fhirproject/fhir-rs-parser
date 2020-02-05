use serde::{Deserialize, Serialize};

/// Represents a request for a patient to employ a medical device. The device may be
/// an implantable device, or an external assistive device, such as a walker.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeviceRequest_Parameter {
  /// The value of the device detail.
  #[serde(rename = "valueQuantity")]
  value_quantity: Quantity,

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

  /// The value of the device detail.
  #[serde(rename = "valueCodeableConcept")]
  value_codeable_concept: CodeableConcept,

  /// A code or string that identifies the device detail being asserted.
  code: CodeableConcept,

  /// The value of the device detail.
  #[serde(rename = "valueBoolean")]
  value_boolean: bool,

  /// The value of the device detail.
  #[serde(rename = "valueRange")]
  value_range: Range,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for valueBoolean
  #[serde(rename = "_valueBoolean")]
  _value_boolean: Element,

}
