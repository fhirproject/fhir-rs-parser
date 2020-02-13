#![allow(unused_imports, non_camel_case_types)]

use crate::model::Identifier::Identifier;
use crate::model::Extension::Extension;
use crate::model::ResourceList::ResourceList;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::Meta::Meta;
use serde_json::value::Value;



/// Describes a comparison of an immunization event against published
/// recommendations to determine if the administration is "valid" in relation to
/// those  recommendations.

#[derive(Debug)]
pub struct ImmunizationEvaluation<'a> {
  pub value: &'a Value,
}

impl ImmunizationEvaluation<'_> {
  /// Extensions for status
  pub fn _status(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_status") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for date
  pub fn _date(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_date") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for doseNumberString
  pub fn _dose_number_string(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_doseNumberString") {
      return Some(Element { value: val });
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

  /// The vaccine administration event being evaluated.
  pub fn immunization_event(&self) -> Reference {
    Reference {
      value: &self.value["immunizationEvent"],
    }
  }

  /// Additional information about the evaluation.
  pub fn description(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("description") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The recommended number of doses to achieve immunity.
  pub fn series_doses_string(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("seriesDosesString") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for seriesDosesString
  pub fn _series_doses_string(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_seriesDosesString") {
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

  /// Extensions for seriesDosesPositiveInt
  pub fn _series_doses_positive_int(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_seriesDosesPositiveInt") {
      return Some(Element { value: val });
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

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The date the evaluation of the vaccine administration event was performed.
  pub fn date(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("date") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for series
  pub fn _series(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_series") {
      return Some(Element { value: val });
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

  /// Extensions for doseNumberPositiveInt
  pub fn _dose_number_positive_int(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_doseNumberPositiveInt") {
      return Some(Element { value: val });
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

  /// Provides an explanation as to why the vaccine administration event is valid or
  /// not relative to the published recommendations.
  pub fn dose_status_reason(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("doseStatusReason") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A unique identifier assigned to this immunization evaluation record.
  pub fn identifier(&self) -> Option<Vec<Identifier>> {
    if let Some(Value::Array(val)) = self.value.get("identifier") {
      return Some(val.into_iter().map(|e| Identifier { value: e }).collect::<Vec<_>>());
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

  /// Indicates the authority who published the protocol (e.g. ACIP).
  pub fn authority(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("authority") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Indicates the current status of the evaluation of the vaccination administration
  /// event.
  pub fn status(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("status") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Indicates if the dose is valid or not valid with respect to the published
  /// recommendations.
  pub fn dose_status(&self) -> CodeableConcept {
    CodeableConcept {
      value: &self.value["doseStatus"],
    }
  }

  /// Extensions for implicitRules
  pub fn _implicit_rules(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_implicitRules") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The individual for whom the evaluation is being done.
  pub fn patient(&self) -> Reference {
    Reference {
      value: &self.value["patient"],
    }
  }

  /// Extensions for description
  pub fn _description(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_description") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// One possible path to achieve presumed immunity against a disease - within the
  /// context of an authority.
  pub fn series(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("series") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Nominal position in a series.
  pub fn dose_number_string(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("doseNumberString") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The vaccine preventable disease the dose is being evaluated against.
  pub fn target_disease(&self) -> CodeableConcept {
    CodeableConcept {
      value: &self.value["targetDisease"],
    }
  }

  /// Nominal position in a series.
  pub fn dose_number_positive_int(&self) -> Option<i64> {
    if let Some(val) = self.value.get("doseNumberPositiveInt") {
      return Some(val.as_i64().unwrap());
    }
    return None;
  }

  /// The recommended number of doses to achieve immunity.
  pub fn series_doses_positive_int(&self) -> Option<i64> {
    if let Some(val) = self.value.get("seriesDosesPositiveInt") {
      return Some(val.as_i64().unwrap());
    }
    return None;
  }

}
