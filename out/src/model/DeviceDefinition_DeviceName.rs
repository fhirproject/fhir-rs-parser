#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.

#[derive(Debug)]
pub struct DeviceDefinition_DeviceName<'a> {
    pub value: &'a Value,
}

impl DeviceDefinition_DeviceName<'_> {
    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
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

    /// The name of the device.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// The type of deviceName.  UDILabelName | UserFriendlyName | PatientReportedName |
    /// ManufactureDeviceName | ModelName.
    pub fn fhir_type(&self) -> Option<DeviceDefinition_DeviceNameType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(DeviceDefinition_DeviceNameType::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._name() {
            _val.validate();
        }
        if let Some(_val) = self._type() {
            _val.validate();
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
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.fhir_type() {}
        return true;
    }
}

#[derive(Debug)]
pub enum DeviceDefinition_DeviceNameType {
    UdiLabelName,
    UserFriendlyName,
    PatientReportedName,
    ManufacturerName,
    ModelName,
    Other,
}

impl DeviceDefinition_DeviceNameType {
    pub fn from_string(string: &str) -> Option<DeviceDefinition_DeviceNameType> {
        match string {
            "udi-label-name" => Some(DeviceDefinition_DeviceNameType::UdiLabelName),
            "user-friendly-name" => Some(DeviceDefinition_DeviceNameType::UserFriendlyName),
            "patient-reported-name" => Some(DeviceDefinition_DeviceNameType::PatientReportedName),
            "manufacturer-name" => Some(DeviceDefinition_DeviceNameType::ManufacturerName),
            "model-name" => Some(DeviceDefinition_DeviceNameType::ModelName),
            "other" => Some(DeviceDefinition_DeviceNameType::Other),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            DeviceDefinition_DeviceNameType::UdiLabelName => "udi-label-name".to_string(),
            DeviceDefinition_DeviceNameType::UserFriendlyName => "user-friendly-name".to_string(),
            DeviceDefinition_DeviceNameType::PatientReportedName => {
                "patient-reported-name".to_string()
            }
            DeviceDefinition_DeviceNameType::ManufacturerName => "manufacturer-name".to_string(),
            DeviceDefinition_DeviceNameType::ModelName => "model-name".to_string(),
            DeviceDefinition_DeviceNameType::Other => "other".to_string(),
        }
    }
}
