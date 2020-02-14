#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Coding::Coding;
use crate::model::Element::Element;
use crate::model::Reference::Reference;
use crate::model::AuditEvent_Network::AuditEvent_Network;
use serde_json::value::Value;



/// A record of an event made for purposes of maintaining a security log. Typical
/// uses include detection of intrusion attempts and monitoring for inappropriate
/// usage.

#[derive(Debug)]
pub struct AuditEvent_Agent<'a> {
  pub value: &'a Value,
}

impl AuditEvent_Agent<'_> {
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

  /// The policy or plan that authorized the activity being recorded. Typically, a
  /// single activity may have multiple applicable policies, such as patient consent,
  /// guarantor funding, etc. The policy would also indicate the security token used.
  pub fn policy(&self) -> Option<Vec<&str>> {
    if let Some(Value::Array(val)) = self.value.get("policy") {
      return Some(val.into_iter().map(|e| e.as_str().unwrap()).collect::<Vec<_>>());
    }
    return None;
  }

  /// Alternative agent Identifier. For a human, this should be a user identifier text
  /// string from authentication system. This identifier would be one known to a
  /// common authentication system (e.g. single sign-on), if available.
  pub fn alt_id(&self) -> Option<&str> {
    if let Some(Value::String(string)) = self.value.get("altId") {
      return Some(string);
    }
    return None;
  }

  /// The reason (purpose of use), specific to this agent, that was used during the
  /// event being recorded.
  pub fn purpose_of_use(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("purposeOfUse") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for requestor
  pub fn _requestor(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_requestor") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Type of media involved. Used when the event is about exporting/importing onto
  /// media.
  pub fn media(&self) -> Option<Coding> {
    if let Some(val) = self.value.get("media") {
      return Some(Coding { value: val });
    }
    return None;
  }

  /// Extensions for policy
  pub fn _policy(&self) -> Option<Vec<Element>> {
    if let Some(Value::Array(val)) = self.value.get("_policy") {
      return Some(val.into_iter().map(|e| Element { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Reference to who this agent is that was involved in the event.
  pub fn who(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("who") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Logical network location for application activity, if the activity has a network
  /// location.
  pub fn network(&self) -> Option<AuditEvent_Network> {
    if let Some(val) = self.value.get("network") {
      return Some(AuditEvent_Network { value: val });
    }
    return None;
  }

  /// Specification of the participation type the user plays when performing the
  /// event.
  pub fn fhir_type(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("type") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<&str> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string);
    }
    return None;
  }

  /// Extensions for altId
  pub fn _alt_id(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_altId") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Indicator that the user is or is not the requestor, or initiator, for the event
  /// being audited.
  pub fn requestor(&self) -> Option<bool> {
    if let Some(val) = self.value.get("requestor") {
      return Some(val.as_bool().unwrap());
    }
    return None;
  }

  /// Where the event occurred.
  pub fn location(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("location") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Extensions for name
  pub fn _name(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_name") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The security role that the user was acting under, that come from local codes
  /// defined by the access control security system (e.g. RBAC, ABAC) used in the
  /// local context.
  pub fn role(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("role") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Human-meaningful name for the agent.
  pub fn name(&self) -> Option<&str> {
    if let Some(Value::String(string)) = self.value.get("name") {
      return Some(string);
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

  pub fn validate(&self) -> bool {
    if let Some(_val) = self.extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.policy() {
      _val.into_iter().for_each(|_e| {});
    }
    if let Some(_val) = self.alt_id() {
    }
    if let Some(_val) = self.purpose_of_use() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self._requestor() {
      _val.validate();
    }
    if let Some(_val) = self.media() {
      _val.validate();
    }
    if let Some(_val) = self._policy() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.who() {
      _val.validate();
    }
    if let Some(_val) = self.network() {
      _val.validate();
    }
    if let Some(_val) = self.fhir_type() {
      _val.validate();
    }
    if let Some(_val) = self.id() {
    }
    if let Some(_val) = self._alt_id() {
      _val.validate();
    }
    if let Some(_val) = self.requestor() {
    }
    if let Some(_val) = self.location() {
      _val.validate();
    }
    if let Some(_val) = self._name() {
      _val.validate();
    }
    if let Some(_val) = self.role() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.name() {
    }
    if let Some(_val) = self.modifier_extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    return true;
  }

}
