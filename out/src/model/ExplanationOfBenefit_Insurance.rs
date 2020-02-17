#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.

#[derive(Debug)]
pub struct ExplanationOfBenefit_Insurance<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ExplanationOfBenefit_Insurance<'_> {
    /// Extensions for focal
    pub fn _focal(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_focal") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for preAuthRef
    pub fn _pre_auth_ref(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_preAuthRef") {
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

    /// Reference to the insurance card level information contained in the Coverage
    /// resource. The coverage issuing insurer will use these details to locate the
    /// patient's actual coverage within the insurer's information system.
    pub fn coverage(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["coverage"]),
        }
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

    /// A flag to indicate that this Coverage is to be used for adjudication of this
    /// claim when set to true.
    pub fn focal(&self) -> Option<bool> {
        if let Some(val) = self.value.get("focal") {
            return Some(val.as_bool().unwrap());
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

    /// Reference numbers previously provided by the insurer to the provider to be
    /// quoted on subsequent claims containing services or products related to the prior
    /// authorization.
    pub fn pre_auth_ref(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("preAuthRef") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._focal() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._pre_auth_ref() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.coverage().validate() {
            return false;
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.focal() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.pre_auth_ref() {
            _val.into_iter().for_each(|_e| {});
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ExplanationOfBenefit_InsuranceBuilder {
    pub value: Value,
}

impl ExplanationOfBenefit_InsuranceBuilder {
    pub fn build(&self) -> ExplanationOfBenefit_Insurance {
        ExplanationOfBenefit_Insurance {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn new(coverage: Reference) -> ExplanationOfBenefit_InsuranceBuilder {
        let mut __value: Value = json!({});
        __value["coverage"] = json!(coverage.value);
        return ExplanationOfBenefit_InsuranceBuilder { value: __value };
    }
}
