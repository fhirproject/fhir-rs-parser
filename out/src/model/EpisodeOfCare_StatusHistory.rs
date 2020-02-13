#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Period::Period;
use crate::model::Extension::Extension;
use serde_json::value::Value;



/// An association between a patient and an organization / healthcare provider(s)
/// during which time encounters may occur. The managing organization assumes a
/// level of responsibility for the patient during this time.

#[derive(Debug)]
pub struct EpisodeOfCare_StatusHistory<'a> {
  pub value: &'a Value,
}

impl EpisodeOfCare_StatusHistory<'_> {
  /// planned | waitlist | active | onhold | finished | cancelled.
  pub fn status(&self) -> Option<EpisodeOfCare_StatusHistoryStatus> {
    if let Some(Value::String(val)) = self.value.get("status") {
      return Some(EpisodeOfCare_StatusHistoryStatus::from_string(&val).unwrap());
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

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
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

  /// Extensions for status
  pub fn _status(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_status") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The period during this EpisodeOfCare that the specific status applied.
  pub fn period(&self) -> Period {
    Period {
      value: &self.value["period"],
    }
  }

  pub fn validate(&self) -> bool {
    if let Some(_val) = self.status() {
    }
    if let Some(_val) = self.modifier_extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.id() {
    }
    if let Some(_val) = self.extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self._status() {
      _val.validate();
    }
    let _ = self.period().validate();
    return true;
  }

}

#[derive(Debug)]
pub enum EpisodeOfCare_StatusHistoryStatus {
  Planned,
  Waitlist,
  Active,
  Onhold,
  Finished,
  Cancelled,
  EnteredInError,
}

impl EpisodeOfCare_StatusHistoryStatus {
    pub fn from_string(string: &str) -> Option<EpisodeOfCare_StatusHistoryStatus> {
      match string {
        "planned" => Some(EpisodeOfCare_StatusHistoryStatus::Planned),
        "waitlist" => Some(EpisodeOfCare_StatusHistoryStatus::Waitlist),
        "active" => Some(EpisodeOfCare_StatusHistoryStatus::Active),
        "onhold" => Some(EpisodeOfCare_StatusHistoryStatus::Onhold),
        "finished" => Some(EpisodeOfCare_StatusHistoryStatus::Finished),
        "cancelled" => Some(EpisodeOfCare_StatusHistoryStatus::Cancelled),
        "entered-in-error" => Some(EpisodeOfCare_StatusHistoryStatus::EnteredInError),
        _ => None,
    }
  }
}

