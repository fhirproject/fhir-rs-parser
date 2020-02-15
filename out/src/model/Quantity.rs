#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// A measured amount (or an amount that can potentially be measured). Note that
/// measured amounts include amounts that are not precisely quantified, including
/// amounts involving arbitrary units and floating currencies.

#[derive(Debug)]
pub struct Quantity<'a> {
    pub value: &'a Value,
}

impl Quantity<'_> {
    /// Extensions for code
    pub fn _code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_code") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for comparator
    pub fn _comparator(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_comparator") {
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

    /// Extensions for unit
    pub fn _unit(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_unit") {
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

    /// A computer processable form of the unit in some unit representation system.
    pub fn code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("code") {
            return Some(string);
        }
        return None;
    }

    /// How the value should be understood and represented - whether the actual value is
    /// greater or less than the stated value due to measurement issues; e.g. if the
    /// comparator is "<" , then the real value is < stated value.
    pub fn comparator(&self) -> Option<QuantityComparator> {
        if let Some(Value::String(val)) = self.value.get("comparator") {
            return Some(QuantityComparator::from_string(&val).unwrap());
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

    /// The identification of the system that provides the coded form of the unit.
    pub fn system(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("system") {
            return Some(string);
        }
        return None;
    }

    /// A human-readable form of the unit.
    pub fn unit(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("unit") {
            return Some(string);
        }
        return None;
    }

    /// The value of the measured amount. The value includes an implicit precision in
    /// the presentation of the value.
    pub fn value(&self) -> Option<f64> {
        if let Some(val) = self.value.get("value") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._comparator() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._system() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._unit() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.code() {}
        if let Some(_val) = self.comparator() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.system() {}
        if let Some(_val) = self.unit() {}
        if let Some(_val) = self.value() {}
        return true;
    }
}

#[derive(Debug)]
pub enum QuantityComparator {
    LessThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    GreaterThan,
}

impl QuantityComparator {
    pub fn from_string(string: &str) -> Option<QuantityComparator> {
        match string {
            "<" => Some(QuantityComparator::LessThan),
            "<=" => Some(QuantityComparator::LessThanOrEqual),
            ">=" => Some(QuantityComparator::GreaterThanOrEqual),
            ">" => Some(QuantityComparator::GreaterThan),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            QuantityComparator::LessThan => "<".to_string(),
            QuantityComparator::LessThanOrEqual => "<=".to_string(),
            QuantityComparator::GreaterThanOrEqual => ">=".to_string(),
            QuantityComparator::GreaterThan => ">".to_string(),
        }
    }
}
