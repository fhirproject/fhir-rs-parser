#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use serde_json::value::Value;

/// Details for all kinds of technology mediated contact points for a person or
/// organization, including telephone, email, etc.

#[derive(Debug)]
pub struct ContactPoint<'a> {
    pub value: &'a Value,
}

impl ContactPoint<'_> {
    /// Extensions for rank
    pub fn _rank(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_rank") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for system
    pub fn _system(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_system") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for use
    pub fn _use(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_use") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for value
    pub fn _value(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_value") {
            return Some(Element { value: val });
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
                    .map(|e| Extension { value: e })
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

    /// Time period when the contact point was/is in use.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// Specifies a preferred order in which to use a set of contacts. ContactPoints
    /// with lower rank values are more preferred than those with higher rank values.
    pub fn rank(&self) -> Option<i64> {
        if let Some(val) = self.value.get("rank") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// Telecommunications form for contact point - what communications system is
    /// required to make use of the contact.
    pub fn system(&self) -> Option<ContactPointSystem> {
        if let Some(Value::String(val)) = self.value.get("system") {
            return Some(ContactPointSystem::from_string(&val).unwrap());
        }
        return None;
    }

    /// Identifies the purpose for the contact point.
    pub fn fhir_use(&self) -> Option<ContactPointUse> {
        if let Some(Value::String(val)) = self.value.get("use") {
            return Some(ContactPointUse::from_string(&val).unwrap());
        }
        return None;
    }

    /// The actual contact point details, in a form that is meaningful to the designated
    /// communication system (i.e. phone number or email address).
    pub fn value(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("value") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._rank() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._system() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._use() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.rank() {}
        if let Some(_val) = self.system() {}
        if let Some(_val) = self.fhir_use() {}
        if let Some(_val) = self.value() {}
        return true;
    }
}

#[derive(Debug)]
pub enum ContactPointSystem {
    Phone,
    Fax,
    Email,
    Pager,
    Url,
    Sms,
    Other,
}

impl ContactPointSystem {
    pub fn from_string(string: &str) -> Option<ContactPointSystem> {
        match string {
            "phone" => Some(ContactPointSystem::Phone),
            "fax" => Some(ContactPointSystem::Fax),
            "email" => Some(ContactPointSystem::Email),
            "pager" => Some(ContactPointSystem::Pager),
            "url" => Some(ContactPointSystem::Url),
            "sms" => Some(ContactPointSystem::Sms),
            "other" => Some(ContactPointSystem::Other),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ContactPointSystem::Phone => "phone".to_string(),
            ContactPointSystem::Fax => "fax".to_string(),
            ContactPointSystem::Email => "email".to_string(),
            ContactPointSystem::Pager => "pager".to_string(),
            ContactPointSystem::Url => "url".to_string(),
            ContactPointSystem::Sms => "sms".to_string(),
            ContactPointSystem::Other => "other".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum ContactPointUse {
    Home,
    Work,
    Temp,
    Old,
    Mobile,
}

impl ContactPointUse {
    pub fn from_string(string: &str) -> Option<ContactPointUse> {
        match string {
            "home" => Some(ContactPointUse::Home),
            "work" => Some(ContactPointUse::Work),
            "temp" => Some(ContactPointUse::Temp),
            "old" => Some(ContactPointUse::Old),
            "mobile" => Some(ContactPointUse::Mobile),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ContactPointUse::Home => "home".to_string(),
            ContactPointUse::Work => "work".to_string(),
            ContactPointUse::Temp => "temp".to_string(),
            ContactPointUse::Old => "old".to_string(),
            ContactPointUse::Mobile => "mobile".to_string(),
        }
    }
}
