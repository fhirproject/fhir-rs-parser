#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical artifacts such as clinical decision
/// support rules, order sets and protocols.

#[derive(Debug)]
pub struct PlanDefinition_Participant<'a> {
    pub value: &'a Value,
}

impl PlanDefinition_Participant<'_> {
    /// The role the participant should play in performing the described action.
    pub fn role(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("role") {
            return Some(CodeableConcept { value: val });
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

    /// The type of participant in the action.
    pub fn fhir_type(&self) -> Option<PlanDefinition_ParticipantType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(PlanDefinition_ParticipantType::from_string(&val).unwrap());
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.role() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._type() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum PlanDefinition_ParticipantType {
    Patient,
    Practitioner,
    RelatedPerson,
    Device,
}

impl PlanDefinition_ParticipantType {
    pub fn from_string(string: &str) -> Option<PlanDefinition_ParticipantType> {
        match string {
            "patient" => Some(PlanDefinition_ParticipantType::Patient),
            "practitioner" => Some(PlanDefinition_ParticipantType::Practitioner),
            "related-person" => Some(PlanDefinition_ParticipantType::RelatedPerson),
            "device" => Some(PlanDefinition_ParticipantType::Device),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            PlanDefinition_ParticipantType::Patient => "patient",
            PlanDefinition_ParticipantType::Practitioner => "practitioner",
            PlanDefinition_ParticipantType::RelatedPerson => "related-person",
            PlanDefinition_ParticipantType::Device => "device",
        }
    }
}
