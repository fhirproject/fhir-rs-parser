#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CatalogEntry_RelatedEntry::CatalogEntry_RelatedEntry;
use crate::model::ResourceList::ResourceList;
use crate::model::Reference::Reference;
use crate::model::Element::Element;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Identifier::Identifier;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Meta::Meta;
use crate::model::Extension::Extension;


/// Catalog entries are wrappers that contextualize items included in a catalog.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogEntry {
  /// Used to support catalog exchange even for unsupported products, e.g. getting
  /// list of medications even if not prescribable.
  status: CatalogEntryStatus,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The date until which this catalog entry is expected to be active.
  #[serde(rename = "validTo")]
  valid_to: String,

  /// Typically date of issue is different from the beginning of the validity. This
  /// can be used to see when an item was last updated.
  #[serde(rename = "lastUpdated")]
  last_updated: String,

  /// The type of item - medication, device, service, protocol or other.
  #[serde(rename = "type")]
  fhir_type: CodeableConcept,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Extensions for orderable
  _orderable: Element,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Used in supporting different identifiers for the same product, e.g. manufacturer
  /// code and retailer code.
  identifier: Vec<Identifier>,

  /// The item in a catalog or definition.
  #[serde(rename = "referencedItem")]
  referenced_item: Box<Reference>,

  /// Extensions for lastUpdated
  #[serde(rename = "_lastUpdated")]
  _last_updated: Element,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Extensions for validTo
  #[serde(rename = "_validTo")]
  _valid_to: Element,

  /// The base language in which the resource is written.
  language: String,

  /// The time period in which this catalog entry is expected to be active.
  #[serde(rename = "validityPeriod")]
  validity_period: Period,

  /// Whether the entry represents an orderable item.
  orderable: bool,

  /// Used for examplefor Out of Formulary, or any specifics.
  #[serde(rename = "additionalCharacteristic")]
  additional_characteristic: Vec<CodeableConcept>,

  /// Extensions for status
  _status: Element,

  /// User for example for ATC classification, or.
  #[serde(rename = "additionalClassification")]
  additional_classification: Vec<CodeableConcept>,

  /// Used for example, to point to a substance, or to a device used to administer a
  /// medication.
  #[serde(rename = "relatedEntry")]
  related_entry: Vec<CatalogEntry_RelatedEntry>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// Classes of devices, or ATC for medication.
  classification: Vec<CodeableConcept>,

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

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Extensions for language
  _language: Element,

  /// Used in supporting related concepts, e.g. NDC to RxNorm.
  #[serde(rename = "additionalIdentifier")]
  additional_identifier: Vec<Identifier>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum CatalogEntryStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "unknown")]
  Unknown,

}
