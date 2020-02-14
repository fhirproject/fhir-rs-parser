#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::MedicinalProduct_CountryLanguage::MedicinalProduct_CountryLanguage;
use crate::model::MedicinalProduct_NamePart::MedicinalProduct_NamePart;
use serde_json::value::Value;

/// Detailed definition of a medicinal product, typically for uses other than direct
/// patient care (e.g. regulatory use).

#[derive(Debug)]
pub struct MedicinalProduct_Name<'a> {
    pub value: &'a Value,
}

impl MedicinalProduct_Name<'_> {
    /// Extensions for productName
    pub fn _product_name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_productName") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Country where the name applies.
    pub fn country_language(&self) -> Option<Vec<MedicinalProduct_CountryLanguage>> {
        if let Some(Value::Array(val)) = self.value.get("countryLanguage") {
            return Some(
                val.into_iter()
                    .map(|e| MedicinalProduct_CountryLanguage { value: e })
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

    /// Coding words or phrases of the name.
    pub fn name_part(&self) -> Option<Vec<MedicinalProduct_NamePart>> {
        if let Some(Value::Array(val)) = self.value.get("namePart") {
            return Some(
                val.into_iter()
                    .map(|e| MedicinalProduct_NamePart { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The full product name.
    pub fn product_name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("productName") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._product_name() {
            _val.validate();
        }
        if let Some(_val) = self.country_language() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.name_part() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.product_name() {}
        return true;
    }
}
