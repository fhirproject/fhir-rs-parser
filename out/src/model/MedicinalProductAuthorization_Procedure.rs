#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Period::Period;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The regulatory authorization of a medicinal product.

#[derive(Debug)]
pub struct MedicinalProductAuthorization_Procedure<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicinalProductAuthorization_Procedure<'_> {
    /// Extensions for dateDateTime
    pub fn _date_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_dateDateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Applcations submitted to obtain a marketing authorization.
    pub fn application(&self) -> Option<Vec<MedicinalProductAuthorization_Procedure>> {
        if let Some(Value::Array(val)) = self.value.get("application") {
            return Some(
                val.into_iter()
                    .map(|e| MedicinalProductAuthorization_Procedure {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Date of procedure.
    pub fn date_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("dateDateTime") {
            return Some(string);
        }
        return None;
    }

    /// Date of procedure.
    pub fn date_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("datePeriod") {
            return Some(Period {
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

    /// Identifier for this procedure.
    pub fn identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifier") {
            return Some(Identifier {
                value: Cow::Borrowed(val),
            });
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

    /// Type of procedure.
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["type"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._date_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.application() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.date_date_time() {}
        if let Some(_val) = self.date_period() {
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
        if let Some(_val) = self.identifier() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.fhir_type().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicinalProductAuthorization_ProcedureBuilder {
    pub value: Value,
}

impl MedicinalProductAuthorization_ProcedureBuilder {
    pub fn build(&self) -> MedicinalProductAuthorization_Procedure {
        MedicinalProductAuthorization_Procedure {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn new(fhir_type: CodeableConcept) -> MedicinalProductAuthorization_ProcedureBuilder {
        let mut __value: Value = json!({});
        __value["type"] = json!(fhir_type.value);
        return MedicinalProductAuthorization_ProcedureBuilder { value: __value };
    }
}
