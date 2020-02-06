#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ProdCharacteristic::ProdCharacteristic;
use crate::model::DeviceDefinition_Material::DeviceDefinition_Material;
use crate::model::Identifier::Identifier;
use crate::model::DeviceDefinition_Property::DeviceDefinition_Property;
use crate::model::Element::Element;
use crate::model::DeviceDefinition_DeviceName::DeviceDefinition_DeviceName;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Narrative::Narrative;
use crate::model::DeviceDefinition_Specialization::DeviceDefinition_Specialization;
use crate::model::Quantity::Quantity;
use crate::model::Extension::Extension;
use crate::model::ResourceList::ResourceList;
use crate::model::DeviceDefinition_Capability::DeviceDefinition_Capability;
use crate::model::DeviceDefinition_UdiDeviceIdentifier::DeviceDefinition_UdiDeviceIdentifier;
use crate::model::Reference::Reference;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Meta::Meta;
use crate::model::ProductShelfLife::ProductShelfLife;
use crate::model::Annotation::Annotation;


/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinition {
  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Contact details for an organization or a particular human that is responsible
  /// for the device.
  contact: Option<Vec<ContactPoint>>,

  /// A network address on which the device may be contacted directly.
  url: Option<String>,

  /// Descriptive information, usage information or implantation information that is
  /// not captured in an existing element.
  note: Option<Vec<Annotation>>,

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

  /// Extensions for manufacturerString
  #[serde(rename = "_manufacturerString")]
  _manufacturer_string: Option<Element>,

  /// The available versions of the device, e.g., software versions.
  version: Option<Vec<String>>,

  /// The quantity of the device present in the packaging (e.g. the number of devices
  /// present in a pack, or the number of devices in the same package of the medicinal
  /// product).
  quantity: Option<Quantity>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Safety characteristics of the device.
  safety: Option<Vec<CodeableConcept>>,

  /// Extensions for url
  #[serde(rename = "_url")]
  _url: Option<Element>,

  /// The capabilities supported on a  device, the standards to which the device
  /// conforms for a particular purpose, and used for the communication.
  specialization: Option<Vec<DeviceDefinition_Specialization>>,

  /// A name of the manufacturer.
  #[serde(rename = "manufacturerString")]
  manufacturer_string: Option<String>,

  /// Device capabilities.
  capability: Option<Vec<DeviceDefinition_Capability>>,

  /// Language code for the human-readable text strings produced by the device (all
  /// supported).
  #[serde(rename = "languageCode")]
  language_code: Option<Vec<CodeableConcept>>,

  /// Shelf Life and storage information.
  #[serde(rename = "shelfLifeStorage")]
  shelf_life_storage: Option<Vec<ProductShelfLife>>,

  /// Extensions for onlineInformation
  #[serde(rename = "_onlineInformation")]
  _online_information: Option<Element>,

  /// A name of the manufacturer.
  #[serde(rename = "manufacturerReference")]
  manufacturer_reference: Option<Box<Reference>>,

  /// The parent device it can be part of.
  #[serde(rename = "parentDevice")]
  parent_device: Option<Box<Reference>>,

  /// A substance used to create the material(s) of which the device is made.
  material: Option<Vec<DeviceDefinition_Material>>,

  /// Extensions for version
  #[serde(rename = "_version")]
  _version: Option<Vec<Element>>,

  /// Unique instance identifiers assigned to a device by the software, manufacturers,
  /// other organizations or owners. For example: handle ID.
  identifier: Option<Vec<Identifier>>,

  /// Access to on-line information about the device.
  #[serde(rename = "onlineInformation")]
  online_information: Option<String>,

  /// Extensions for modelNumber
  #[serde(rename = "_modelNumber")]
  _model_number: Option<Element>,

  /// Unique device identifier (UDI) assigned to device label or package.  Note that
  /// the Device may include multiple udiCarriers as it either may include just the
  /// udiCarrier for the jurisdiction it is sold, or for multiple jurisdictions it
  /// could have been sold.
  #[serde(rename = "udiDeviceIdentifier")]
  udi_device_identifier: Option<Vec<DeviceDefinition_UdiDeviceIdentifier>>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Dimensions, color etc.
  #[serde(rename = "physicalCharacteristics")]
  physical_characteristics: Option<ProdCharacteristic>,

  /// A name given to the device to identify it.
  #[serde(rename = "deviceName")]
  device_name: Option<Vec<DeviceDefinition_DeviceName>>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// The model number for the device.
  #[serde(rename = "modelNumber")]
  model_number: Option<String>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// The actual configuration settings of a device as it actually operates, e.g.,
  /// regulation status, time properties.
  property: Option<Vec<DeviceDefinition_Property>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// An organization that is responsible for the provision and ongoing maintenance of
  /// the device.
  owner: Option<Box<Reference>>,

  /// What kind of device or device system this is.
  #[serde(rename = "type")]
  fhir_type: Option<CodeableConcept>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

}
