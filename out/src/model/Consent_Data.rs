#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// A record of a healthcare consumer’s  choices, which permits or denies identified
/// recipient(s) or recipient role(s) to perform one or more actions within a given
/// policy context, for specific purposes and periods of time.

#[derive(Debug)]
pub struct Consent_Data<'a> {
    pub value: &'a Value,
}

impl Consent_Data<'_> {
    /// Extensions for meaning
    pub fn _meaning(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_meaning") {
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

    /// How the resource reference is interpreted when testing consent restrictions.
    pub fn meaning(&self) -> Option<Consent_DataMeaning> {
        if let Some(Value::String(val)) = self.value.get("meaning") {
            return Some(Consent_DataMeaning::from_string(&val).unwrap());
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A reference to a specific resource that defines which resources are covered by
    /// this consent.
    pub fn reference(&self) -> Reference {
        Reference {
            value: &self.value["reference"],
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._meaning() {
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
        if let Some(_val) = self.meaning() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.reference().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub enum Consent_DataMeaning {
    Instance,
    Related,
    Dependents,
    Authoredby,
}

impl Consent_DataMeaning {
    pub fn from_string(string: &str) -> Option<Consent_DataMeaning> {
        match string {
            "instance" => Some(Consent_DataMeaning::Instance),
            "related" => Some(Consent_DataMeaning::Related),
            "dependents" => Some(Consent_DataMeaning::Dependents),
            "authoredby" => Some(Consent_DataMeaning::Authoredby),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Consent_DataMeaning::Instance => "instance".to_string(),
            Consent_DataMeaning::Related => "related".to_string(),
            Consent_DataMeaning::Dependents => "dependents".to_string(),
            Consent_DataMeaning::Authoredby => "authoredby".to_string(),
        }
    }
}
