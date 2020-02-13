#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::Timing::Timing;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Annotation::Annotation;
use crate::model::Contract_Subject::Contract_Subject;
use serde_json::value::Value;



/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.

#[derive(Debug)]
pub struct Contract_Action<'a> {
  pub value: &'a Value,
}

impl Contract_Action<'_> {
  /// Current state of the term action.
  pub fn status(&self) -> CodeableConcept {
    CodeableConcept {
      value: &self.value["status"],
    }
  }

  /// Extensions for occurrenceDateTime
  pub fn _occurrence_date_time(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_occurrenceDateTime") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Id [identifier??] of the clause or question text related to this action in the
  /// referenced form or QuestionnaireResponse.
  pub fn link_id(&self) -> Option<Vec<String>> {
    if let Some(Value::Array(val)) = self.value.get("linkId") {
      return Some(val.into_iter().map(|e| e.as_str().unwrap().to_string()).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for reasonLinkId
  pub fn _reason_link_id(&self) -> Option<Vec<Element>> {
    if let Some(Value::Array(val)) = self.value.get("_reasonLinkId") {
      return Some(val.into_iter().map(|e| Element { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Activity or service obligation to be done or not done, performed or not
  /// performed, effectuated or not by this Contract term.
  pub fn fhir_type(&self) -> CodeableConcept {
    CodeableConcept {
      value: &self.value["type"],
    }
  }

  /// Extensions for linkId
  pub fn _link_id(&self) -> Option<Vec<Element>> {
    if let Some(Value::Array(val)) = self.value.get("_linkId") {
      return Some(val.into_iter().map(|e| Element { value: e }).collect::<Vec<_>>());
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

  /// Encounter or Episode with primary association to specified term activity.
  pub fn context(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("context") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Id [identifier??] of the clause or question text related to the requester of
  /// this action in the referenced form or QuestionnaireResponse.
  pub fn requester_link_id(&self) -> Option<Vec<String>> {
    if let Some(Value::Array(val)) = self.value.get("requesterLinkId") {
      return Some(val.into_iter().map(|e| e.as_str().unwrap().to_string()).collect::<Vec<_>>());
    }
    return None;
  }

  /// Id [identifier??] of the clause or question text related to the requester of
  /// this action in the referenced form or QuestionnaireResponse.
  pub fn context_link_id(&self) -> Option<Vec<String>> {
    if let Some(Value::Array(val)) = self.value.get("contextLinkId") {
      return Some(val.into_iter().map(|e| e.as_str().unwrap().to_string()).collect::<Vec<_>>());
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

  /// Indicates who or what is being asked to perform (or not perform) the ction.
  pub fn performer(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("performer") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// When action happens.
  pub fn occurrence_timing(&self) -> Option<Timing> {
    if let Some(val) = self.value.get("occurrenceTiming") {
      return Some(Timing { value: val });
    }
    return None;
  }

  /// True if the term prohibits the  action.
  pub fn do_not_perform(&self) -> Option<bool> {
    if let Some(val) = self.value.get("doNotPerform") {
      return Some(val.as_bool().unwrap());
    }
    return None;
  }

  /// Extensions for requesterLinkId
  pub fn _requester_link_id(&self) -> Option<Vec<Element>> {
    if let Some(Value::Array(val)) = self.value.get("_requesterLinkId") {
      return Some(val.into_iter().map(|e| Element { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The type of individual that is desired or required to perform or not perform the
  /// action.
  pub fn performer_type(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("performerType") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Describes why the action is to be performed or not performed in textual form.
  pub fn reason(&self) -> Option<Vec<String>> {
    if let Some(Value::Array(val)) = self.value.get("reason") {
      return Some(val.into_iter().map(|e| e.as_str().unwrap().to_string()).collect::<Vec<_>>());
    }
    return None;
  }

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Id [identifier??] of the clause or question text related to the reason type or
  /// reference of this  action in the referenced form or QuestionnaireResponse.
  pub fn performer_link_id(&self) -> Option<Vec<String>> {
    if let Some(Value::Array(val)) = self.value.get("performerLinkId") {
      return Some(val.into_iter().map(|e| e.as_str().unwrap().to_string()).collect::<Vec<_>>());
    }
    return None;
  }

  /// Id [identifier??] of the clause or question text related to the reason type or
  /// reference of this  action in the referenced form or QuestionnaireResponse.
  pub fn reason_link_id(&self) -> Option<Vec<String>> {
    if let Some(Value::Array(val)) = self.value.get("reasonLinkId") {
      return Some(val.into_iter().map(|e| e.as_str().unwrap().to_string()).collect::<Vec<_>>());
    }
    return None;
  }

  /// When action happens.
  pub fn occurrence_date_time(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("occurrenceDateTime") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Entity of the action.
  pub fn subject(&self) -> Option<Vec<Contract_Subject>> {
    if let Some(Value::Array(val)) = self.value.get("subject") {
      return Some(val.into_iter().map(|e| Contract_Subject { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Comments made about the term action made by the requester, performer, subject or
  /// other participants.
  pub fn note(&self) -> Option<Vec<Annotation>> {
    if let Some(Value::Array(val)) = self.value.get("note") {
      return Some(val.into_iter().map(|e| Annotation { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Security labels that protects the action.
  pub fn security_label_number(&self) -> Option<Vec<u64>> {
    if let Some(Value::Array(val)) = self.value.get("securityLabelNumber") {
      return Some(val.into_iter().map(|e| e.as_u64().unwrap()).collect::<Vec<_>>());
    }
    return None;
  }

  /// The type of role or competency of an individual desired or required to perform
  /// or not perform the action.
  pub fn performer_role(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("performerRole") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Who or what initiated the action and has responsibility for its activation.
  pub fn requester(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("requester") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for contextLinkId
  pub fn _context_link_id(&self) -> Option<Vec<Element>> {
    if let Some(Value::Array(val)) = self.value.get("_contextLinkId") {
      return Some(val.into_iter().map(|e| Element { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for performerLinkId
  pub fn _performer_link_id(&self) -> Option<Vec<Element>> {
    if let Some(Value::Array(val)) = self.value.get("_performerLinkId") {
      return Some(val.into_iter().map(|e| Element { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for reason
  pub fn _reason(&self) -> Option<Vec<Element>> {
    if let Some(Value::Array(val)) = self.value.get("_reason") {
      return Some(val.into_iter().map(|e| Element { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Indicates another resource whose existence justifies permitting or not
  /// permitting this action.
  pub fn reason_reference(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("reasonReference") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Reason or purpose for the action stipulated by this Contract Provision.
  pub fn intent(&self) -> CodeableConcept {
    CodeableConcept {
      value: &self.value["intent"],
    }
  }

  /// Extensions for securityLabelNumber
  pub fn _security_label_number(&self) -> Option<Vec<Element>> {
    if let Some(Value::Array(val)) = self.value.get("_securityLabelNumber") {
      return Some(val.into_iter().map(|e| Element { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for doNotPerform
  pub fn _do_not_perform(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_doNotPerform") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// When action happens.
  pub fn occurrence_period(&self) -> Option<Period> {
    if let Some(val) = self.value.get("occurrencePeriod") {
      return Some(Period { value: val });
    }
    return None;
  }

  /// Rationale for the action to be performed or not performed. Describes why the
  /// action is permitted or prohibited.
  pub fn reason_code(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("reasonCode") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

}
