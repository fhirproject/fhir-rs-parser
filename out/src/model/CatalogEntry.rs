#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Narrative::Narrative;
use crate::model::CatalogEntry_RelatedEntry::CatalogEntry_RelatedEntry;
use crate::model::Meta::Meta;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::Reference::Reference;
use crate::model::Period::Period;
use crate::model::ResourceList::ResourceList;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Identifier::Identifier;


/// Catalog entries are wrappers that contextualize items included in a catalog.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogEntry {
  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// Used to support catalog exchange even for unsupported products, e.g. getting
  /// list of medications even if not prescribable.
  status: Option<CatalogEntryStatus>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// User for example for ATC classification, or.
  #[serde(rename = "additionalClassification")]
  additional_classification: Option<Vec<CodeableConcept>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// The type of item - medication, device, service, protocol or other.
  #[serde(rename = "type")]
  fhir_type: Option<CodeableConcept>,

  /// Extensions for orderable
  #[serde(rename = "_orderable")]
  _orderable: Option<Element>,

  /// The date until which this catalog entry is expected to be active.
  #[serde(rename = "validTo")]
  valid_to: Option<String>,

  /// Used in supporting related concepts, e.g. NDC to RxNorm.
  #[serde(rename = "additionalIdentifier")]
  additional_identifier: Option<Vec<Identifier>>,

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
  modifier_extension: Option<Vec<Box<Extension>>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// The time period in which this catalog entry is expected to be active.
  #[serde(rename = "validityPeriod")]
  validity_period: Option<Period>,

  /// Extensions for validTo
  #[serde(rename = "_validTo")]
  _valid_to: Option<Element>,

  /// Extensions for lastUpdated
  #[serde(rename = "_lastUpdated")]
  _last_updated: Option<Element>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Used in supporting different identifiers for the same product, e.g. manufacturer
  /// code and retailer code.
  identifier: Option<Vec<Identifier>>,

  /// The item in a catalog or definition.
  #[serde(rename = "referencedItem")]
  referenced_item: Box<Reference>,

  /// Typically date of issue is different from the beginning of the validity. This
  /// can be used to see when an item was last updated.
  #[serde(rename = "lastUpdated")]
  last_updated: Option<String>,

  /// Used for example, to point to a substance, or to a device used to administer a
  /// medication.
  #[serde(rename = "relatedEntry")]
  related_entry: Option<Vec<CatalogEntry_RelatedEntry>>,

  /// Used for examplefor Out of Formulary, or any specifics.
  #[serde(rename = "additionalCharacteristic")]
  additional_characteristic: Option<Vec<CodeableConcept>>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Whether the entry represents an orderable item.
  orderable: Option<bool>,

  /// Classes of devices, or ATC for medication.
  classification: Option<Vec<CodeableConcept>>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

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
