#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A specific set of Roles/Locations/specialties/services that a practitioner may
/// perform at an organization for a period of time.

#[derive(Debug)]
pub struct PractitionerRole_AvailableTime<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl PractitionerRole_AvailableTime<'_> {
    /// Extensions for allDay
    pub fn _all_day(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_allDay") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for availableEndTime
    pub fn _available_end_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_availableEndTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for availableStartTime
    pub fn _available_start_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_availableStartTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for daysOfWeek
    pub fn _days_of_week(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_daysOfWeek") {
            return Some(
                val.into_iter()
                    .map(|e| Element {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Is this always available? (hence times are irrelevant) e.g. 24 hour service.
    pub fn all_day(&self) -> Option<bool> {
        if let Some(val) = self.value.get("allDay") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The closing time of day. Note: If the AllDay flag is set, then this time is
    /// ignored.
    pub fn available_end_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("availableEndTime") {
            return Some(string);
        }
        return None;
    }

    /// The opening time of day. Note: If the AllDay flag is set, then this time is
    /// ignored.
    pub fn available_start_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("availableStartTime") {
            return Some(string);
        }
        return None;
    }

    /// Indicates which days of the week are available between the start and end Times.
    pub fn days_of_week(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("daysOfWeek") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
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
            return Some(
                val.into_iter()
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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
            return Some(
                val.into_iter()
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._all_day() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._available_end_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._available_start_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._days_of_week() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.all_day() {}
        if let Some(_val) = self.available_end_time() {}
        if let Some(_val) = self.available_start_time() {}
        if let Some(_val) = self.days_of_week() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct PractitionerRole_AvailableTimeBuilder {
    pub value: Value,
}

impl PractitionerRole_AvailableTimeBuilder {
    pub fn build(&self) -> PractitionerRole_AvailableTime {
        PractitionerRole_AvailableTime {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn new() -> PractitionerRole_AvailableTimeBuilder {
        let mut __value: Value = json!({});
        return PractitionerRole_AvailableTimeBuilder { value: __value };
    }
}
