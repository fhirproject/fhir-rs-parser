#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::ExampleScenario_ContainedInstance::ExampleScenario_ContainedInstance;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Example of workflow instance.

#[derive(Debug)]
pub struct ExampleScenario_Operation<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ExampleScenario_Operation<'_> {
    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for initiator
    pub fn _initiator(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_initiator") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for initiatorActive
    pub fn _initiator_active(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_initiatorActive") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for number
    pub fn _number(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_number") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for receiver
    pub fn _receiver(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_receiver") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for receiverActive
    pub fn _receiver_active(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_receiverActive") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A comment to be inserted in the diagram.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
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

    /// Who starts the transaction.
    pub fn initiator(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("initiator") {
            return Some(string);
        }
        return None;
    }

    /// Whether the initiator is deactivated right after the transaction.
    pub fn initiator_active(&self) -> Option<bool> {
        if let Some(val) = self.value.get("initiatorActive") {
            return Some(val.as_bool().unwrap());
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

    /// The human-friendly name of the interaction.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// The sequential number of the interaction, e.g. 1.2.5.
    pub fn number(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("number") {
            return Some(string);
        }
        return None;
    }

    /// Who receives the transaction.
    pub fn receiver(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("receiver") {
            return Some(string);
        }
        return None;
    }

    /// Whether the receiver is deactivated right after the transaction.
    pub fn receiver_active(&self) -> Option<bool> {
        if let Some(val) = self.value.get("receiverActive") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Each resource instance used by the initiator.
    pub fn request(&self) -> Option<ExampleScenario_ContainedInstance> {
        if let Some(val) = self.value.get("request") {
            return Some(ExampleScenario_ContainedInstance {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Each resource instance used by the responder.
    pub fn response(&self) -> Option<ExampleScenario_ContainedInstance> {
        if let Some(val) = self.value.get("response") {
            return Some(ExampleScenario_ContainedInstance {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The type of operation - CRUD.
    pub fn fhir_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("type") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._initiator() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._initiator_active() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._number() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._receiver() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._receiver_active() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.initiator() {}
        if let Some(_val) = self.initiator_active() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.number() {}
        if let Some(_val) = self.receiver() {}
        if let Some(_val) = self.receiver_active() {}
        if let Some(_val) = self.request() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.response() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ExampleScenario_OperationBuilder {
    pub value: Value,
}

impl ExampleScenario_OperationBuilder {
    pub fn build(&self) -> ExampleScenario_Operation {
        ExampleScenario_Operation {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn new() -> ExampleScenario_OperationBuilder {
        let mut __value: Value = json!({});
        return ExampleScenario_OperationBuilder { value: __value };
    }
}
