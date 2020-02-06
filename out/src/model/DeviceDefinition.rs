#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::DeviceDefinition_DeviceName::DeviceDefinition_DeviceName;
use crate::model::Extension::Extension;
use crate::model::DeviceDefinition_UdiDeviceIdentifier::DeviceDefinition_UdiDeviceIdentifier;
use crate::model::Identifier::Identifier;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Reference::Reference;
use crate::model::DeviceDefinition_Specialization::DeviceDefinition_Specialization;
use crate::model::ProdCharacteristic::ProdCharacteristic;
use crate::model::DeviceDefinition_Capability::DeviceDefinition_Capability;
use crate::model::DeviceDefinition_Material::DeviceDefinition_Material;
use crate::model::DeviceDefinition_Property::DeviceDefinition_Property;
use crate::model::ResourceList::ResourceList;
use crate::model::Element::Element;
use crate::model::ProductShelfLife::ProductShelfLife;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Meta::Meta;
use crate::model::Annotation::Annotation;
use crate::model::Quantity::Quantity;
use crate::model::Narrative::Narrative;


/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinition {
  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// A network address on which the device may be contacted directly.
  url: String,

  /// Extensions for url
  _url: Element,

  /// Extensions for language
  _language: Element,

  /// Safety characteristics of the device.
  safety: Vec<CodeableConcept>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Unique device identifier (UDI) assigned to device label or package.  Note that
  /// the Device may include multiple udiCarriers as it either may include just the
  /// udiCarrier for the jurisdiction it is sold, or for multiple jurisdictions it
  /// could have been sold.
  #[serde(rename = "udiDeviceIdentifier")]
  udi_device_identifier: Vec<DeviceDefinition_UdiDeviceIdentifier>,

  /// Descriptive information, usage information or implantation information that is
  /// not captured in an existing element.
  note: Vec<Annotation>,

  /// Extensions for manufacturerString
  #[serde(rename = "_manufacturerString")]
  _manufacturer_string: Element,

  /// The model number for the device.
  #[serde(rename = "modelNumber")]
  model_number: String,

  /// Shelf Life and storage information.
  #[serde(rename = "shelfLifeStorage")]
  shelf_life_storage: Vec<ProductShelfLife>,

  /// The actual configuration settings of a device as it actually operates, e.g.,
  /// regulation status, time properties.
  property: Vec<DeviceDefinition_Property>,

  /// The base language in which the resource is written.
  language: String,

  /// Unique instance identifiers assigned to a device by the software, manufacturers,
  /// other organizations or owners. For example: handle ID.
  identifier: Vec<Identifier>,

  /// Contact details for an organization or a particular human that is responsible
  /// for the device.
  contact: Vec<ContactPoint>,

  /// A name of the manufacturer.
  #[serde(rename = "manufacturerReference")]
  manufacturer_reference: Box<Reference>,

  /// Access to on-line information about the device.
  #[serde(rename = "onlineInformation")]
  online_information: String,

  /// Extensions for version
  _version: Vec<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// A name given to the device to identify it.
  #[serde(rename = "deviceName")]
  device_name: Vec<DeviceDefinition_DeviceName>,

  /// The parent device it can be part of.
  #[serde(rename = "parentDevice")]
  parent_device: Box<Reference>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// The quantity of the device present in the packaging (e.g. the number of devices
  /// present in a pack, or the number of devices in the same package of the medicinal
  /// product).
  quantity: Quantity,

  /// Extensions for onlineInformation
  #[serde(rename = "_onlineInformation")]
  _online_information: Element,

  /// Language code for the human-readable text strings produced by the device (all
  /// supported).
  #[serde(rename = "languageCode")]
  language_code: Vec<CodeableConcept>,

  /// A name of the manufacturer.
  #[serde(rename = "manufacturerString")]
  manufacturer_string: String,

  /// What kind of device or device system this is.
  #[serde(rename = "type")]
  fhir_type: CodeableConcept,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Extensions for modelNumber
  #[serde(rename = "_modelNumber")]
  _model_number: Element,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

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
  /// processing a resource are required to check for modifier extensions.    Modifier
  /// extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// The available versions of the device, e.g., software versions.
  version: Vec<String>,

  /// Dimensions, color etc.
  #[serde(rename = "physicalCharacteristics")]
  physical_characteristics: ProdCharacteristic,

  /// Device capabilities.
  capability: Vec<DeviceDefinition_Capability>,

  /// An organization that is responsible for the provision and ongoing maintenance of
  /// the device.
  owner: Box<Reference>,

  /// The capabilities supported on a  device, the standards to which the device
  /// conforms for a particular purpose, and used for the communication.
  specialization: Vec<DeviceDefinition_Specialization>,

  /// A substance used to create the material(s) of which the device is made.
  material: Vec<DeviceDefinition_Material>,

}
