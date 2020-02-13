#![allow(unused_imports, non_camel_case_types)]

use crate::model::Reference::Reference;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Element::Element;
use serde_json::value::Value;



/// A financial tool for tracking value accrued for a particular purpose.  In the
/// healthcare field, used to track charges for a patient, cost centers, etc.

#[derive(Debug)]
pub struct Account_Guarantor<'a> {
  pub value: &'a Value,
}

impl Account_Guarantor<'_> {
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

  /// The entity who is responsible.
  pub fn party(&self) -> Reference {
    Reference {
      value: &self.value["party"],
    }
  }

  /// Extensions for onHold
  pub fn _on_hold(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_onHold") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A guarantor may be placed on credit hold or otherwise have their role
  /// temporarily suspended.
  pub fn on_hold(&self) -> Option<bool> {
    if let Some(val) = self.value.get("onHold") {
      return Some(val.as_bool().unwrap());
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

  /// The timeframe during which the guarantor accepts responsibility for the account.
  pub fn period(&self) -> Option<Period> {
    if let Some(val) = self.value.get("period") {
      return Some(Period { value: val });
    }
    return None;
  }

  pub fn validate(&self) -> bool {
    if let Some(_val) = self.extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.modifier_extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    let _ = self.party().validate();
    if let Some(_val) = self._on_hold() {
      _val.validate();
    }
    if let Some(_val) = self.on_hold() {
    }
    if let Some(_val) = self.id() {
    }
    if let Some(_val) = self.period() {
      _val.validate();
    }
    return true;
  }

}
