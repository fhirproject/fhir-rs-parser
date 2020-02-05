use serde::{Deserialize, Serialize};

/// A type of a manufactured item that is used in the provision of healthcare
/// without being substantially changed through that activity. The device may be a
/// medical or non-medical device.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Device_DeviceName {
  /// Extensions for type
  _type: Element,

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

  /// Extensions for name
  _name: Element,

  /// The name of the device.
  name: String,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// The type of deviceName.
  /// UDILabelName | UserFriendlyName | PatientReportedName | ManufactureDeviceName |
  /// ModelName.
  type: Device_DeviceNameType,

}

#[derive(Debug, Serialize, Deserialize)]
enum Device_DeviceNameType {
  #[serde(rename = "udi-label-name")]
  UdiLabelName,

  #[serde(rename = "user-friendly-name")]
  UserFriendlyName,

  #[serde(rename = "patient-reported-name")]
  PatientReportedName,

  #[serde(rename = "manufacturer-name")]
  ManufacturerName,

  #[serde(rename = "model-name")]
  ModelName,

  #[serde(rename = "other")]
  Other,

}
