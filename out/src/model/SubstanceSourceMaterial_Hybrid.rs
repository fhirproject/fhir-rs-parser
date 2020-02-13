#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use serde_json::value::Value;



/// Source material shall capture information on the taxonomic and anatomical
/// origins as well as the fraction of a material that can result in or can be
/// modified to form a substance. This set of data elements shall be used to define
/// polymer substances isolated from biological matrices. Taxonomic and anatomical
/// origins shall be described using a controlled vocabulary as required. This
/// information is captured for naturally derived polymers ( . starch) and
/// structurally diverse substances. For Organisms belonging to the Kingdom Plantae
/// the Substance level defines the fresh material of a single species or
/// infraspecies, the Herbal Drug and the Herbal preparation. For Herbal
/// preparations, the fraction information will be captured at the Substance
/// information level and additional information for herbal extracts will be
/// captured at the Specified Substance Group 1 information level. See for further
/// explanation the Substance Class: Structurally Diverse and the herbal annex.

#[derive(Debug)]
pub struct SubstanceSourceMaterial_Hybrid<'a> {
  pub value: &'a Value,
}

impl SubstanceSourceMaterial_Hybrid<'_> {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The hybrid type of an organism shall be specified.
  pub fn hybrid_type(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("hybridType") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  pub fn extension(&self) -> Option<Vec<Extension>> {
    if let Some(Value::Array(val)) = self.value.get("extension") {
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for maternalOrganismName
  pub fn _maternal_organism_name(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_maternalOrganismName") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The identifier of the maternal species constituting the hybrid organism shall be
  /// specified based on a controlled vocabulary. For plants, the parents aren’t
  /// always known, and it is unlikely that it will be known which is maternal and
  /// which is paternal.
  pub fn maternal_organism_id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("maternalOrganismId") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The name of the maternal species constituting the hybrid organism shall be
  /// specified. For plants, the parents aren’t always known, and it is unlikely that
  /// it will be known which is maternal and which is paternal.
  pub fn maternal_organism_name(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("maternalOrganismName") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The identifier of the paternal species constituting the hybrid organism shall be
  /// specified based on a controlled vocabulary.
  pub fn paternal_organism_id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("paternalOrganismId") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The name of the paternal species constituting the hybrid organism shall be
  /// specified.
  pub fn paternal_organism_name(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("paternalOrganismName") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for paternalOrganismId
  pub fn _paternal_organism_id(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_paternalOrganismId") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for paternalOrganismName
  pub fn _paternal_organism_name(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_paternalOrganismName") {
      return Some(Element { value: val });
    }
    return None;
  }

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
  pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
    if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for maternalOrganismId
  pub fn _maternal_organism_id(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_maternalOrganismId") {
      return Some(Element { value: val });
    }
    return None;
  }

}
