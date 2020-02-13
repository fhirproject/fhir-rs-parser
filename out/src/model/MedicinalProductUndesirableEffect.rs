#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::Population::Population;
use serde_json::value::Value;



/// Describe the undesirable effects of the medicinal product.

#[derive(Debug)]
pub struct MedicinalProductUndesirableEffect<'a> {
  pub value: &'a Value,
}

impl MedicinalProductUndesirableEffect<'_> {
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

  /// The frequency of occurrence of the effect.
  pub fn frequency_of_occurrence(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("frequencyOfOccurrence") {
      return Some(CodeableConcept { value: val });
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

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  pub fn contained(&self) -> Option<Vec<ResourceList>> {
    if let Some(Value::Array(val)) = self.value.get("contained") {
      return Some(val.into_iter().map(|e| ResourceList { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Classification of the effect.
  pub fn classification(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("classification") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// The population group to which this applies.
  pub fn population(&self) -> Option<Vec<Population>> {
    if let Some(Value::Array(val)) = self.value.get("population") {
      return Some(val.into_iter().map(|e| Population { value: e }).collect::<Vec<_>>());
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

  /// The medication for which this is an indication.
  pub fn subject(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("subject") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The symptom, condition or undesirable effect.
  pub fn symptom_condition_effect(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("symptomConditionEffect") {
      return Some(CodeableConcept { value: val });
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

  /// The base language in which the resource is written.
  pub fn language(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("language") {
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

  pub fn validate(&self) -> bool {
    if let Some(_val) = self.text() {
      _val.validate();
    }
    if let Some(_val) = self.frequency_of_occurrence() {
      _val.validate();
    }
    if let Some(_val) = self.meta() {
      _val.validate();
    }
    if let Some(_val) = self.contained() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.classification() {
      _val.validate();
    }
    if let Some(_val) = self.population() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.id() {
    }
    if let Some(_val) = self.implicit_rules() {
    }
    if let Some(_val) = self.modifier_extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.subject() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.symptom_condition_effect() {
      _val.validate();
    }
    if let Some(_val) = self._implicit_rules() {
      _val.validate();
    }
    if let Some(_val) = self.language() {
    }
    if let Some(_val) = self._language() {
      _val.validate();
    }
    if let Some(_val) = self.extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    return true;
  }

}
