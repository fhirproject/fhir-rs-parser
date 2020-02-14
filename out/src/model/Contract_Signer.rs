#![allow(unused_imports, non_camel_case_types)]

use crate::model::Coding::Coding;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::Signature::Signature;
use serde_json::value::Value;

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.

#[derive(Debug)]
pub struct Contract_Signer<'a> {
    pub value: &'a Value,
}

impl Contract_Signer<'_> {
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Party which is a signator to this Contract.
    pub fn party(&self) -> Reference {
        Reference {
            value: &self.value["party"],
        }
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

    /// Legally binding Contract DSIG signature contents in Base64.
    pub fn signature(&self) -> Vec<Signature> {
        self.value
            .get("signature")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| Signature { value: e })
            .collect::<Vec<_>>()
    }

    /// Role of this Contract signer, e.g. notary, grantee.
    pub fn fhir_type(&self) -> Coding {
        Coding {
            value: &self.value["type"],
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.party().validate();
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.signature().into_iter().for_each(|e| {
            e.validate();
        });
        let _ = self.fhir_type().validate();
        return true;
    }
}
