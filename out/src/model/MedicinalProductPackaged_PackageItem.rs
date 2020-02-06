#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Reference::Reference;
use crate::model::Quantity::Quantity;
use crate::model::ProductShelfLife::ProductShelfLife;
use crate::model::Identifier::Identifier;
use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ProdCharacteristic::ProdCharacteristic;


/// A medicinal product in a container or package.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductPackaged_PackageItem {
  /// Material type of the package item.
  material: Option<Vec<CodeableConcept>>,

  /// The quantity of this package in the medicinal product, at the current level of
  /// packaging. The outermost is always 1.
  quantity: Quantity,

  /// The manufactured item as contained in the packaged medicinal product.
  #[serde(rename = "manufacturedItem")]
  manufactured_item: Option<Vec<Box<Reference>>>,

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

  /// Other codeable characteristics.
  #[serde(rename = "otherCharacteristics")]
  other_characteristics: Option<Vec<CodeableConcept>>,

  /// Including possibly Data Carrier Identifier.
  identifier: Option<Vec<Identifier>>,

  /// A possible alternate material for the packaging.
  #[serde(rename = "alternateMaterial")]
  alternate_material: Option<Vec<CodeableConcept>>,

  /// Manufacturer of this Package Item.
  manufacturer: Option<Vec<Box<Reference>>>,

  /// Shelf Life and storage information.
  #[serde(rename = "shelfLifeStorage")]
  shelf_life_storage: Option<Vec<ProductShelfLife>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The physical type of the container of the medicine.
  #[serde(rename = "type")]
  fhir_type: CodeableConcept,

  /// Dimensions, color etc.
  #[serde(rename = "physicalCharacteristics")]
  physical_characteristics: Option<ProdCharacteristic>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Allows containers within containers.
  #[serde(rename = "packageItem")]
  package_item: Option<Vec<MedicinalProductPackaged_PackageItem>>,

  /// A device accompanying a medicinal product.
  device: Option<Vec<Box<Reference>>>,

}
