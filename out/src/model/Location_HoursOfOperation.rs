#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Element::Element;
use serde_json::value::Value;



/// Details and position information for a physical place where services are
/// provided and resources and participants may be stored, found, contained, or
/// accommodated.

#[derive(Debug)]
pub struct Location_HoursOfOperation<'a> {
  pub value: &'a Value,
}

impl Location_HoursOfOperation<'_> {
  /// Indicates which days of the week are available between the start and end Times.
  pub fn days_of_week(&self) -> Option<Vec<&str>> {
    if let Some(Value::Array(val)) = self.value.get("daysOfWeek") {
      return Some(val.into_iter().map(|e| e.as_str().unwrap()).collect::<Vec<_>>());
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

  /// Extensions for daysOfWeek
  pub fn _days_of_week(&self) -> Option<Vec<Element>> {
    if let Some(Value::Array(val)) = self.value.get("_daysOfWeek") {
      return Some(val.into_iter().map(|e| Element { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for closingTime
  pub fn _closing_time(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_closingTime") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Time that the Location closes.
  pub fn closing_time(&self) -> Option<&str> {
    if let Some(Value::String(string)) = self.value.get("closingTime") {
      return Some(string);
    }
    return None;
  }

  /// Extensions for allDay
  pub fn _all_day(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_allDay") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Time that the Location opens.
  pub fn opening_time(&self) -> Option<&str> {
    if let Some(Value::String(string)) = self.value.get("openingTime") {
      return Some(string);
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

  /// Extensions for openingTime
  pub fn _opening_time(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_openingTime") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The Location is open all day.
  pub fn all_day(&self) -> Option<bool> {
    if let Some(val) = self.value.get("allDay") {
      return Some(val.as_bool().unwrap());
    }
    return None;
  }

  pub fn validate(&self) -> bool {
    if let Some(_val) = self.days_of_week() {
      _val.into_iter().for_each(|_e| {});
    }
    if let Some(_val) = self.id() {
    }
    if let Some(_val) = self.modifier_extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self._days_of_week() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self._closing_time() {
      _val.validate();
    }
    if let Some(_val) = self.closing_time() {
    }
    if let Some(_val) = self._all_day() {
      _val.validate();
    }
    if let Some(_val) = self.opening_time() {
    }
    if let Some(_val) = self.extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self._opening_time() {
      _val.validate();
    }
    if let Some(_val) = self.all_day() {
    }
    return true;
  }

}
