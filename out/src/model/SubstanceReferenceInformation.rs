#![allow(unused_imports, non_camel_case_types)]

use crate::model::SubstanceReferenceInformation_Target::SubstanceReferenceInformation_Target;
use crate::model::Element::Element;
use crate::model::SubstanceReferenceInformation_GeneElement::SubstanceReferenceInformation_GeneElement;
use crate::model::ResourceList::ResourceList;
use crate::model::Extension::Extension;
use crate::model::SubstanceReferenceInformation_Gene::SubstanceReferenceInformation_Gene;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::SubstanceReferenceInformation_Classification::SubstanceReferenceInformation_Classification;
use serde_json::value::Value;



/// Todo.

#[derive(Debug)]
pub struct SubstanceReferenceInformation<'a> {
  pub value: &'a Value,
}

impl SubstanceReferenceInformation<'_> {
  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
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

  /// Todo.
  pub fn gene_element(&self) -> Option<Vec<SubstanceReferenceInformation_GeneElement>> {
    if let Some(Value::Array(val)) = self.value.get("geneElement") {
      return Some(val.into_iter().map(|e| SubstanceReferenceInformation_GeneElement { value: e }).collect::<Vec<_>>());
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

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  pub fn contained(&self) -> Option<Vec<ResourceList>> {
    if let Some(Value::Array(val)) = self.value.get("contained") {
      return Some(val.into_iter().map(|e| ResourceList { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Todo.
  pub fn comment(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("comment") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for comment
  pub fn _comment(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_comment") {
      return Some(Element { value: val });
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

  /// Todo.
  pub fn target(&self) -> Option<Vec<SubstanceReferenceInformation_Target>> {
    if let Some(Value::Array(val)) = self.value.get("target") {
      return Some(val.into_iter().map(|e| SubstanceReferenceInformation_Target { value: e }).collect::<Vec<_>>());
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

  /// Todo.
  pub fn gene(&self) -> Option<Vec<SubstanceReferenceInformation_Gene>> {
    if let Some(Value::Array(val)) = self.value.get("gene") {
      return Some(val.into_iter().map(|e| SubstanceReferenceInformation_Gene { value: e }).collect::<Vec<_>>());
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

  /// Todo.
  pub fn classification(&self) -> Option<Vec<SubstanceReferenceInformation_Classification>> {
    if let Some(Value::Array(val)) = self.value.get("classification") {
      return Some(val.into_iter().map(|e| SubstanceReferenceInformation_Classification { value: e }).collect::<Vec<_>>());
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

}
