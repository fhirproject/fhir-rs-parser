#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Meta::Meta;
use crate::model::Extension::Extension;
use crate::model::Narrative::Narrative;
use crate::model::BiologicallyDerivedProduct_Collection::BiologicallyDerivedProduct_Collection;
use crate::model::ResourceList::ResourceList;
use crate::model::Reference::Reference;
use crate::model::BiologicallyDerivedProduct_Storage::BiologicallyDerivedProduct_Storage;
use crate::model::Element::Element;
use crate::model::BiologicallyDerivedProduct_Manipulation::BiologicallyDerivedProduct_Manipulation;
use crate::model::Identifier::Identifier;
use crate::model::BiologicallyDerivedProduct_Processing::BiologicallyDerivedProduct_Processing;
use crate::model::CodeableConcept::CodeableConcept;


/// A material substance originating from a biological entity intended to be
/// transplanted or infused
/// into another (possibly the same) biological entity.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BiologicallyDerivedProduct {
  /// Number of discrete units within this product.
  quantity: Option<i32>,

  /// How this product was collected.
  collection: Option<BiologicallyDerivedProduct_Collection>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Extensions for productCategory
  #[serde(rename = "_productCategory")]
  _product_category: Option<Element>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Broad category of this product.
  #[serde(rename = "productCategory")]
  product_category: Option<BiologicallyDerivedProductProductCategory>,

  /// Any manipulation of product post-collection that is intended to alter the
  /// product.  For example a buffy-coat enrichment or CD8 reduction of Peripheral
  /// Blood Stem Cells to make it more suitable for infusion.
  manipulation: Option<BiologicallyDerivedProduct_Manipulation>,

  /// This records identifiers associated with this biologically derived product
  /// instance that are defined by business processes and/or used to refer to it when
  /// a direct URL reference to the resource itself is not appropriate (e.g. in CDA
  /// documents, or in written / printed documentation).
  identifier: Option<Vec<Identifier>>,

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

  /// Extensions for quantity
  #[serde(rename = "_quantity")]
  _quantity: Option<Element>,

  /// Parent product (if any).
  parent: Option<Vec<Box<Reference>>>,

  /// Any processing of the product during collection that does not change the
  /// fundamental nature of the product. For example adding anti-coagulants during the
  /// collection of Peripheral Blood Stem Cells.
  processing: Option<Vec<BiologicallyDerivedProduct_Processing>>,

  /// Procedure request to obtain this biologically derived product.
  request: Option<Vec<Box<Reference>>>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Whether the product is currently available.
  status: Option<BiologicallyDerivedProductStatus>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Product storage.
  storage: Option<Vec<BiologicallyDerivedProduct_Storage>>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// A code that identifies the kind of this biologically derived product (SNOMED
  /// Ctcode).
  #[serde(rename = "productCode")]
  product_code: Option<CodeableConcept>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum BiologicallyDerivedProductProductCategory {
  #[serde(rename = "organ")]
  Organ,

  #[serde(rename = "tissue")]
  Tissue,

  #[serde(rename = "fluid")]
  Fluid,

  #[serde(rename = "cells")]
  Cells,

  #[serde(rename = "biologicalAgent")]
  BiologicalAgent,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum BiologicallyDerivedProductStatus {
  #[serde(rename = "available")]
  Available,

  #[serde(rename = "unavailable")]
  Unavailable,

}
