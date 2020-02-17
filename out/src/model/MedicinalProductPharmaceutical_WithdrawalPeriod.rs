#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A pharmaceutical product described in terms of its composition and dose form.

#[derive(Debug)]
pub struct MedicinalProductPharmaceutical_WithdrawalPeriod<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicinalProductPharmaceutical_WithdrawalPeriod<'_> {
    /// Extensions for supportingInformation
    pub fn _supporting_information(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_supportingInformation") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
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

    /// Extra information about the withdrawal period.
    pub fn supporting_information(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("supportingInformation") {
            return Some(string);
        }
        return None;
    }

    /// Coded expression for the type of tissue for which the withdrawal period applues,
    /// e.g. meat, milk.
    pub fn tissue(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["tissue"]),
        }
    }

    /// A value for the time.
    pub fn value(&self) -> Quantity {
        Quantity {
            value: Cow::Borrowed(&self.value["value"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._supporting_information() {
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
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.supporting_information() {}
        if !self.tissue().validate() {
            return false;
        }
        if !self.value().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicinalProductPharmaceutical_WithdrawalPeriodBuilder {
    pub value: Value,
}

impl MedicinalProductPharmaceutical_WithdrawalPeriodBuilder {
    pub fn build(&self) -> MedicinalProductPharmaceutical_WithdrawalPeriod {
        MedicinalProductPharmaceutical_WithdrawalPeriod {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn new(
        tissue: CodeableConcept,
        value: Quantity,
    ) -> MedicinalProductPharmaceutical_WithdrawalPeriodBuilder {
        let mut __value: Value = json!({});
        __value["tissue"] = json!(tissue.value);
        __value["value"] = json!(value.value);
        return MedicinalProductPharmaceutical_WithdrawalPeriodBuilder { value: __value };
    }
}
