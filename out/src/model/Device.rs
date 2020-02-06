#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Device_DeviceName::Device_DeviceName;
use crate::model::Device_Specialization::Device_Specialization;
use crate::model::Device_Version::Device_Version;
use crate::model::Identifier::Identifier;
use crate::model::ResourceList::ResourceList;
use crate::model::Narrative::Narrative;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Device_Property::Device_Property;
use crate::model::Meta::Meta;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Device_UdiCarrier::Device_UdiCarrier;
use crate::model::Annotation::Annotation;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;


/// A type of a manufactured item that is used in the provision of healthcare
/// without being substantially changed through that activity. The device may be a
/// medical or non-medical device.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
  /// A name of the manufacturer.
  manufacturer: Option<String>,

  /// This represents the manufacturer's name of the device as provided by the device,
  /// from a UDI label, or by a person describing the Device.  This typically would be
  /// used when a person provides the name(s) or when the device represents one of the
  /// names available from DeviceDefinition.
  #[serde(rename = "deviceName")]
  device_name: Option<Vec<Device_DeviceName>>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Unique instance identifiers assigned to a device by manufacturers other
  /// organizations or owners.
  identifier: Option<Vec<Identifier>>,

  /// The reference to the definition for the device.
  definition: Option<Box<Reference>>,

  /// Extensions for manufacturer
  #[serde(rename = "_manufacturer")]
  _manufacturer: Option<Element>,

  /// Extensions for lotNumber
  #[serde(rename = "_lotNumber")]
  _lot_number: Option<Element>,

  /// The serial number assigned by the organization when the device was manufactured.
  #[serde(rename = "serialNumber")]
  serial_number: Option<String>,

  /// Extensions for serialNumber
  #[serde(rename = "_serialNumber")]
  _serial_number: Option<Element>,

  /// Extensions for partNumber
  #[serde(rename = "_partNumber")]
  _part_number: Option<Element>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// The capabilities supported on a  device, the standards to which the device
  /// conforms for a particular purpose, and used for the communication.
  specialization: Option<Vec<Device_Specialization>>,

  /// Status of the Device availability.
  status: Option<DeviceStatus>,

  /// Contact details for an organization or a particular human that is responsible
  /// for the device.
  contact: Option<Vec<ContactPoint>>,

  /// The part number of the device.
  #[serde(rename = "partNumber")]
  part_number: Option<String>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Descriptive information, usage information or implantation information that is
  /// not captured in an existing element.
  note: Option<Vec<Annotation>>,

  /// Extensions for manufactureDate
  #[serde(rename = "_manufactureDate")]
  _manufacture_date: Option<Element>,

  /// Patient information, If the device is affixed to a person.
  patient: Option<Box<Reference>>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// The actual configuration settings of a device as it actually operates, e.g.,
  /// regulation status, time properties.
  property: Option<Vec<Device_Property>>,

  /// A network address on which the device may be contacted directly.
  url: Option<String>,

  /// Lot number assigned by the manufacturer.
  #[serde(rename = "lotNumber")]
  lot_number: Option<String>,

  /// Reason for the dtatus of the Device availability.
  #[serde(rename = "statusReason")]
  status_reason: Option<Vec<CodeableConcept>>,

  /// Unique device identifier (UDI) assigned to device label or package.  Note that
  /// the Device may include multiple udiCarriers as it either may include just the
  /// udiCarrier for the jurisdiction it is sold, or for multiple jurisdictions it
  /// could have been sold.
  #[serde(rename = "udiCarrier")]
  udi_carrier: Option<Vec<Device_UdiCarrier>>,

  /// Extensions for expirationDate
  #[serde(rename = "_expirationDate")]
  _expiration_date: Option<Element>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Extensions for distinctIdentifier
  #[serde(rename = "_distinctIdentifier")]
  _distinct_identifier: Option<Element>,

  /// The model number for the device.
  #[serde(rename = "modelNumber")]
  model_number: Option<String>,

  /// The kind or type of device.
  #[serde(rename = "type")]
  fhir_type: Option<CodeableConcept>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Extensions for modelNumber
  #[serde(rename = "_modelNumber")]
  _model_number: Option<Element>,

  /// The actual design of the device or software version running on the device.
  version: Option<Vec<Device_Version>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The place where the device can be found.
  location: Option<Box<Reference>>,

  /// The parent device.
  parent: Option<Box<Reference>>,

  /// The date and time beyond which this device is no longer valid or should not be
  /// used (if applicable).
  #[serde(rename = "expirationDate")]
  expiration_date: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.    Modifier
  /// extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Option<Vec<Extension>>,

  /// The distinct identification string as required by regulation for a human cell,
  /// tissue, or cellular and tissue-based product.
  #[serde(rename = "distinctIdentifier")]
  distinct_identifier: Option<String>,

  /// The date and time when the device was manufactured.
  #[serde(rename = "manufactureDate")]
  manufacture_date: Option<String>,

  /// An organization that is responsible for the provision and ongoing maintenance of
  /// the device.
  owner: Option<Box<Reference>>,

  /// Provides additional safety characteristics about a medical device.  For example
  /// devices containing latex.
  safety: Option<Vec<CodeableConcept>>,

  /// Extensions for url
  #[serde(rename = "_url")]
  _url: Option<Element>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum DeviceStatus {
  #[serde(rename = "active")]
  Active,

  #[serde(rename = "inactive")]
  Inactive,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

  #[serde(rename = "unknown")]
  Unknown,

}
