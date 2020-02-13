#![allow(unused_imports, non_camel_case_types)]

use crate::model::ResourceList::ResourceList;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::Annotation::Annotation;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::Element::Element;
use crate::model::RiskAssessment_Prediction::RiskAssessment_Prediction;
use crate::model::Narrative::Narrative;
use crate::model::Identifier::Identifier;
use crate::model::CodeableConcept::CodeableConcept;
use serde_json::value::Value;



/// An assessment of the likely outcome(s) for a patient or other subject as well as
/// the likelihood of each outcome.

#[derive(Debug)]
pub struct RiskAssessment<'a> {
  pub value: &'a Value,
}

impl RiskAssessment<'_> {
  /// The base language in which the resource is written.
  pub fn language(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("language") {
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

  /// The provider or software application that performed the assessment.
  pub fn performer(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("performer") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// A reference to a resource that this risk assessment is part of, such as a
  /// Procedure.
  pub fn parent(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("parent") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// The type of the risk assessment performed.
  pub fn code(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("code") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// The date (and possibly time) the risk assessment was performed.
  pub fn occurrence_period(&self) -> Option<Period> {
    if let Some(val) = self.value.get("occurrencePeriod") {
      return Some(Period { value: val });
    }
    return None;
  }

  /// For assessments or prognosis specific to a particular condition, indicates the
  /// condition being assessed.
  pub fn condition(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("condition") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Extensions for occurrenceDateTime
  pub fn _occurrence_date_time(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_occurrenceDateTime") {
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

  /// The algorithm, process or mechanism used to evaluate the risk.
  pub fn method(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("method") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// A reference to the request that is fulfilled by this risk assessment.
  pub fn based_on(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("basedOn") {
      return Some(Reference { value: val });
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

  /// Resources supporting the reason the risk assessment was performed.
  pub fn reason_reference(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("reasonReference") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for mitigation
  pub fn _mitigation(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_mitigation") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Additional comments about the risk assessment.
  pub fn note(&self) -> Option<Vec<Annotation>> {
    if let Some(Value::Array(val)) = self.value.get("note") {
      return Some(val.into_iter().map(|e| Annotation { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The status of the RiskAssessment, using the same statuses as an Observation.
  pub fn status(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("status") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Indicates the source data considered as part of the assessment (for example,
  /// FamilyHistory, Observations, Procedures, Conditions, etc.).
  pub fn basis(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("basis") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The date (and possibly time) the risk assessment was performed.
  pub fn occurrence_date_time(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("occurrenceDateTime") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Describes the expected outcome for the subject.
  pub fn prediction(&self) -> Option<Vec<RiskAssessment_Prediction>> {
    if let Some(Value::Array(val)) = self.value.get("prediction") {
      return Some(val.into_iter().map(|e| RiskAssessment_Prediction { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The encounter where the assessment was performed.
  pub fn encounter(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("encounter") {
      return Some(Reference { value: val });
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

  /// Business identifier assigned to the risk assessment.
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

  /// Extensions for status
  pub fn _status(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_status") {
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

  /// The patient or group the risk assessment applies to.
  pub fn subject(&self) -> Reference {
    Reference {
      value: &self.value["subject"],
    }
  }

  /// A description of the steps that might be taken to reduce the identified risk(s).
  pub fn mitigation(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("mitigation") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The reason the risk assessment was performed.
  pub fn reason_code(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("reasonCode") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
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
