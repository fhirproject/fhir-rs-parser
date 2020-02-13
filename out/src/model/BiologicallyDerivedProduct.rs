#![allow(unused_imports, non_camel_case_types)]

use crate::model::Narrative::Narrative;
use crate::model::BiologicallyDerivedProduct_Storage::BiologicallyDerivedProduct_Storage;
use crate::model::Element::Element;
use crate::model::ResourceList::ResourceList;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::BiologicallyDerivedProduct_Collection::BiologicallyDerivedProduct_Collection;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::BiologicallyDerivedProduct_Processing::BiologicallyDerivedProduct_Processing;
use crate::model::Meta::Meta;
use crate::model::Reference::Reference;
use crate::model::BiologicallyDerivedProduct_Manipulation::BiologicallyDerivedProduct_Manipulation;
use serde_json::value::Value;



/// A material substance originating from a biological entity intended to be
/// transplanted or infused
/// into another (possibly the same) biological entity.

#[derive(Debug)]
pub struct BiologicallyDerivedProduct<'a> {
  pub value: &'a Value,
}

impl BiologicallyDerivedProduct<'_> {
  /// Whether the product is currently available.
  pub fn status(&self) -> Option<BiologicallyDerivedProductStatus> {
    if let Some(Value::String(val)) = self.value.get("status") {
      return Some(BiologicallyDerivedProductStatus::from_string(&val).unwrap());
    }
    return None;
  }

  /// Extensions for implicitRules
  pub fn _implicit_rules(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_implicitRules") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Any manipulation of product post-collection that is intended to alter the
  /// product.  For example a buffy-coat enrichment or CD8 reduction of Peripheral
  /// Blood Stem Cells to make it more suitable for infusion.
  pub fn manipulation(&self) -> Option<BiologicallyDerivedProduct_Manipulation> {
    if let Some(val) = self.value.get("manipulation") {
      return Some(BiologicallyDerivedProduct_Manipulation { value: val });
    }
    return None;
  }

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  pub fn meta(&self) -> Option<Meta> {
    if let Some(val) = self.value.get("meta") {
      return Some(Meta { value: val });
    }
    return None;
  }

  /// How this product was collected.
  pub fn collection(&self) -> Option<BiologicallyDerivedProduct_Collection> {
    if let Some(val) = self.value.get("collection") {
      return Some(BiologicallyDerivedProduct_Collection { value: val });
    }
    return None;
  }

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  pub fn extension(&self) -> Option<Vec<Extension>> {
    if let Some(Value::Array(val)) = self.value.get("extension") {
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for language
  pub fn _language(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_language") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Product storage.
  pub fn storage(&self) -> Option<Vec<BiologicallyDerivedProduct_Storage>> {
    if let Some(Value::Array(val)) = self.value.get("storage") {
      return Some(val.into_iter().map(|e| BiologicallyDerivedProduct_Storage { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Broad category of this product.
  pub fn product_category(&self) -> Option<BiologicallyDerivedProductProductCategory> {
    if let Some(Value::String(val)) = self.value.get("productCategory") {
      return Some(BiologicallyDerivedProductProductCategory::from_string(&val).unwrap());
    }
    return None;
  }

  /// Number of discrete units within this product.
  pub fn quantity(&self) -> Option<i64> {
    if let Some(val) = self.value.get("quantity") {
      return Some(val.as_i64().unwrap());
    }
    return None;
  }

  /// Any processing of the product during collection that does not change the
  /// fundamental nature of the product. For example adding anti-coagulants during the
  /// collection of Peripheral Blood Stem Cells.
  pub fn processing(&self) -> Option<Vec<BiologicallyDerivedProduct_Processing>> {
    if let Some(Value::Array(val)) = self.value.get("processing") {
      return Some(val.into_iter().map(|e| BiologicallyDerivedProduct_Processing { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  pub fn contained(&self) -> Option<Vec<ResourceList>> {
    if let Some(Value::Array(val)) = self.value.get("contained") {
      return Some(val.into_iter().map(|e| ResourceList { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The base language in which the resource is written.
  pub fn language(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("language") {
      return Some(string.to_string());
    }
    return None;
  }

  /// This records identifiers associated with this biologically derived product
  /// instance that are defined by business processes and/or used to refer to it when
  /// a direct URL reference to the resource itself is not appropriate (e.g. in CDA
  /// documents, or in written / printed documentation).
  pub fn identifier(&self) -> Option<Vec<Identifier>> {
    if let Some(Value::Array(val)) = self.value.get("identifier") {
      return Some(val.into_iter().map(|e| Identifier { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A code that identifies the kind of this biologically derived product (SNOMED
  /// Ctcode).
  pub fn product_code(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("productCode") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Extensions for quantity
  pub fn _quantity(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_quantity") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Parent product (if any).
  pub fn parent(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("parent") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for productCategory
  pub fn _product_category(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_productCategory") {
      return Some(Element { value: val });
    }
    return None;
  }

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
  pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
    if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Procedure request to obtain this biologically derived product.
  pub fn request(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("request") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  pub fn text(&self) -> Option<Narrative> {
    if let Some(val) = self.value.get("text") {
      return Some(Narrative { value: val });
    }
    return None;
  }

  /// Extensions for status
  pub fn _status(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_status") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  pub fn implicit_rules(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("implicitRules") {
      return Some(string.to_string());
    }
    return None;
  }

}

#[derive(Debug)]
pub enum BiologicallyDerivedProductStatus {
  Available,
  Unavailable,
}

impl BiologicallyDerivedProductStatus {
    pub fn from_string(string: &str) -> Option<BiologicallyDerivedProductStatus> {
      match string {
        "available" => Some(BiologicallyDerivedProductStatus::Available),
        "unavailable" => Some(BiologicallyDerivedProductStatus::Unavailable),
        _ => None,
    }
  }
}


#[derive(Debug)]
pub enum BiologicallyDerivedProductProductCategory {
  Organ,
  Tissue,
  Fluid,
  Cells,
  BiologicalAgent,
}

impl BiologicallyDerivedProductProductCategory {
    pub fn from_string(string: &str) -> Option<BiologicallyDerivedProductProductCategory> {
      match string {
        "organ" => Some(BiologicallyDerivedProductProductCategory::Organ),
        "tissue" => Some(BiologicallyDerivedProductProductCategory::Tissue),
        "fluid" => Some(BiologicallyDerivedProductProductCategory::Fluid),
        "cells" => Some(BiologicallyDerivedProductProductCategory::Cells),
        "biologicalAgent" => Some(BiologicallyDerivedProductProductCategory::BiologicalAgent),
        _ => None,
    }
  }
}

