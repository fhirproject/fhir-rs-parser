#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Meta::Meta;
use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::MarketingStatus::MarketingStatus;
use crate::model::ResourceList::ResourceList;
use crate::model::Reference::Reference;
use crate::model::Narrative::Narrative;
use crate::model::MedicinalProductPackaged_PackageItem::MedicinalProductPackaged_PackageItem;
use crate::model::Identifier::Identifier;
use crate::model::Element::Element;
use crate::model::MedicinalProductPackaged_BatchIdentifier::MedicinalProductPackaged_BatchIdentifier;


/// A medicinal product in a container or package.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductPackaged {
  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Textual description.
  description: String,

  /// Manufacturer of this Package Item.
  manufacturer: Vec<Box<Reference>>,

  /// The legal status of supply of the medicinal product as classified by the
  /// regulator.
  #[serde(rename = "legalStatusOfSupply")]
  legal_status_of_supply: CodeableConcept,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Marketing information.
  #[serde(rename = "marketingStatus")]
  marketing_status: Vec<MarketingStatus>,

  /// The product with this is a pack for.
  subject: Vec<Box<Reference>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The base language in which the resource is written.
  language: String,

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

  /// Extensions for description
  _description: Element,

  /// Manufacturer of this Package Item.
  #[serde(rename = "marketingAuthorization")]
  marketing_authorization: Box<Reference>,

  /// Extensions for language
  _language: Element,

  /// A packaging item, as a contained for medicine, possibly with other packaging
  /// items within.
  #[serde(rename = "packageItem")]
  package_item: Vec<MedicinalProductPackaged_PackageItem>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// Unique identifier.
  identifier: Vec<Identifier>,

  /// Batch numbering.
  #[serde(rename = "batchIdentifier")]
  batch_identifier: Vec<MedicinalProductPackaged_BatchIdentifier>,

}
