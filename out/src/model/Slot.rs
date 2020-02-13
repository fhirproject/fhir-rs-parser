#![allow(unused_imports, non_camel_case_types)]

use crate::model::Identifier::Identifier;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ResourceList::ResourceList;
use crate::model::Reference::Reference;
use crate::model::Narrative::Narrative;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::Element::Element;
use serde_json::value::Value;



/// A slot of time on a schedule that may be available for booking appointments.

#[derive(Debug)]
pub struct Slot<'a> {
  pub value: &'a Value,
}

impl Slot<'_> {
  /// Extensions for end
  pub fn _end(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_end") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for implicitRules
  pub fn _implicit_rules(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_implicitRules") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for language
  pub fn _language(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_language") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  pub fn meta(&self) -> Option<Meta> {
    if let Some(val) = self.value.get("meta") {
      return Some(Meta { value: val });
    }
    return None;
  }

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.    Modifier
  /// extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
    if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// External Ids for this item.
  pub fn identifier(&self) -> Option<Vec<Identifier>> {
    if let Some(Value::Array(val)) = self.value.get("identifier") {
      return Some(val.into_iter().map(|e| Identifier { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A broad categorization of the service that is to be performed during this
  /// appointment.
  pub fn service_category(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("serviceCategory") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The specialty of a practitioner that would be required to perform the service
  /// requested in this appointment.
  pub fn specialty(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("specialty") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The schedule resource that this slot defines an interval of status information.
  pub fn schedule(&self) -> Reference {
    Reference {
      value: &self.value["schedule"],
    }
  }

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

  /// busy | free | busy-unavailable | busy-tentative | entered-in-error.
  pub fn status(&self) -> Option<SlotStatus> {
    if let Some(Value::String(val)) = self.value.get("status") {
      return Some(SlotStatus::from_string(&val).unwrap());
    }
    return None;
  }

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  pub fn implicit_rules(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("implicitRules") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for status
  pub fn _status(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_status") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  pub fn extension(&self) -> Option<Vec<Extension>> {
    if let Some(Value::Array(val)) = self.value.get("extension") {
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Date/Time that the slot is to begin.
  pub fn start(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("start") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The type of appointments that can be booked into this slot (ideally this would
  /// be an identifiable service - which is at a location, rather than the location
  /// itself). If provided then this overrides the value provided on the availability
  /// resource.
  pub fn service_type(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("serviceType") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Date/Time that the slot is to conclude.
  pub fn end(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("end") {
      return Some(string.to_string());
    }
    return None;
  }

  /// This slot has already been overbooked, appointments are unlikely to be accepted
  /// for this time.
  pub fn overbooked(&self) -> Option<bool> {
    if let Some(val) = self.value.get("overbooked") {
      return Some(val.as_bool().unwrap());
    }
    return None;
  }

  /// Extensions for start
  pub fn _start(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_start") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Comments on the slot to describe any extended information. Such as custom
  /// constraints on the slot.
  pub fn comment(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("comment") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for comment
  pub fn _comment(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_comment") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  pub fn contained(&self) -> Option<Vec<ResourceList>> {
    if let Some(Value::Array(val)) = self.value.get("contained") {
      return Some(val.into_iter().map(|e| ResourceList { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The base language in which the resource is written.
  pub fn language(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("language") {
      return Some(string.to_string());
    }
    return None;
  }

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  pub fn text(&self) -> Option<Narrative> {
    if let Some(val) = self.value.get("text") {
      return Some(Narrative { value: val });
    }
    return None;
  }

  /// The style of appointment or patient that may be booked in the slot (not service
  /// type).
  pub fn appointment_type(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("appointmentType") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Extensions for overbooked
  pub fn _overbooked(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_overbooked") {
      return Some(Element { value: val });
    }
    return None;
  }

}

#[derive(Debug)]
pub enum SlotStatus {
  Busy,
  Free,
  BusyUnavailable,
  BusyTentative,
  EnteredInError,
}

impl SlotStatus {
    pub fn from_string(string: &str) -> Option<SlotStatus> {
      match string {
        "busy" => Some(SlotStatus::Busy),
        "free" => Some(SlotStatus::Free),
        "busy-unavailable" => Some(SlotStatus::BusyUnavailable),
        "busy-tentative" => Some(SlotStatus::BusyTentative),
        "entered-in-error" => Some(SlotStatus::EnteredInError),
        _ => None,
    }
  }
}

