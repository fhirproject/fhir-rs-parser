use serde::{Deserialize, Serialize};

/// A type of a manufactured item that is used in the provision of healthcare
/// without being substantially changed through that activity. The device may be a
/// medical or non-medical device.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Device {
  /// An organization that is responsible for the provision and ongoing maintenance of
  /// the device.
  owner: Reference,

  /// The reference to the definition for the device.
  definition: Reference,

  /// Unique device identifier (UDI) assigned to device label or package.  Note that
  /// the Device may include multiple udiCarriers as it either may include just the
  /// udiCarrier for the jurisdiction it is sold, or for multiple jurisdictions it
  /// could have been sold.
  #[serde(rename = "udiCarrier")]
  udi_carrier: Vec<Device_UdiCarrier>,

  /// The date and time beyond which this device is no longer valid or should not be
  /// used (if applicable).
  #[serde(rename = "expirationDate")]
  expiration_date: dateTime,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Lot number assigned by the manufacturer.
  #[serde(rename = "lotNumber")]
  lot_number: String,

  /// This represents the manufacturer's name of the device as provided by the device,
  /// from a UDI label, or by a person describing the Device.  This typically would be
  /// used when a person provides the name(s) or when the device represents one of the
  /// names available from DeviceDefinition.
  #[serde(rename = "deviceName")]
  device_name: Vec<Device_DeviceName>,

  /// Extensions for manufactureDate
  #[serde(rename = "_manufactureDate")]
  _manufacture_date: Element,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Reason for the dtatus of the Device availability.
  #[serde(rename = "statusReason")]
  status_reason: Vec<CodeableConcept>,

  /// The place where the device can be found.
  location: Reference,

  /// The actual design of the device or software version running on the device.
  version: Vec<Device_Version>,

  /// Contact details for an organization or a particular human that is responsible
  /// for the device.
  contact: Vec<ContactPoint>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Extensions for partNumber
  #[serde(rename = "_partNumber")]
  _part_number: Element,

  /// Provides additional safety characteristics about a medical device.  For example
  /// devices containing latex.
  safety: Vec<CodeableConcept>,

  /// Extensions for distinctIdentifier
  #[serde(rename = "_distinctIdentifier")]
  _distinct_identifier: Element,

  /// The parent device.
  parent: Reference,

  /// The part number of the device.
  #[serde(rename = "partNumber")]
  part_number: String,

  /// Extensions for serialNumber
  #[serde(rename = "_serialNumber")]
  _serial_number: Element,

  /// The serial number assigned by the organization when the device was manufactured.
  #[serde(rename = "serialNumber")]
  serial_number: String,

  /// Status of the Device availability.
  status: DeviceStatus,

  /// The kind or type of device.
  type: CodeableConcept,

  /// A network address on which the device may be contacted directly.
  url: String,

  /// Unique instance identifiers assigned to a device by manufacturers other
  /// organizations or owners.
  identifier: Vec<Identifier>,

  /// A name of the manufacturer.
  manufacturer: String,

  /// The distinct identification string as required by regulation for a human cell,
  /// tissue, or cellular and tissue-based product.
  #[serde(rename = "distinctIdentifier")]
  distinct_identifier: String,

  /// Extensions for manufacturer
  _manufacturer: Element,

  /// The model number for the device.
  #[serde(rename = "modelNumber")]
  model_number: String,

  /// The capabilities supported on a  device, the standards to which the device
  /// conforms for a particular purpose, and used for the communication.
  specialization: Vec<Device_Specialization>,

  /// Descriptive information, usage information or implantation information that is
  /// not captured in an existing element.
  note: Vec<Annotation>,

  /// Extensions for modelNumber
  #[serde(rename = "_modelNumber")]
  _model_number: Element,

  /// Extensions for url
  _url: Element,

  /// Extensions for language
  _language: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The date and time when the device was manufactured.
  #[serde(rename = "manufactureDate")]
  manufacture_date: dateTime,

  /// The base language in which the resource is written.
  language: String,

  /// The actual configuration settings of a device as it actually operates, e.g.,
  /// regulation status, time properties.
  property: Vec<Device_Property>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// Extensions for expirationDate
  #[serde(rename = "_expirationDate")]
  _expiration_date: Element,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Extensions for status
  _status: Element,

  /// Extensions for lotNumber
  #[serde(rename = "_lotNumber")]
  _lot_number: Element,

  /// Patient information, If the device is affixed to a person.
  patient: Reference,

}

#[derive(Debug, Serialize, Deserialize)]
enum DeviceStatus {
  #[serde(rename = "active")]
  Active,

  #[serde(rename = "inactive")]
  Inactive,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

  #[serde(rename = "unknown")]
  Unknown,

}
