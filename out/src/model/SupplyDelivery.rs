#![allow(unused_imports, non_camel_case_types)]

use crate::model::ResourceList::ResourceList;
use crate::model::SupplyDelivery_SuppliedItem::SupplyDelivery_SuppliedItem;
use crate::model::Reference::Reference;
use crate::model::Identifier::Identifier;
use crate::model::Narrative::Narrative;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Period::Period;
use crate::model::Element::Element;
use crate::model::Meta::Meta;
use crate::model::Timing::Timing;
use crate::model::Extension::Extension;
use serde_json::value::Value;



/// Record of delivery of what is supplied.

#[derive(Debug)]
pub struct SupplyDelivery<'a> {
  pub value: &'a Value,
}

impl SupplyDelivery<'_> {
  /// A plan, proposal or order that is fulfilled in whole or in part by this event.
  pub fn based_on(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("basedOn") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The date or time(s) the activity occurred.
  pub fn occurrence_period(&self) -> Option<Period> {
    if let Some(val) = self.value.get("occurrencePeriod") {
      return Some(Period { value: val });
    }
    return None;
  }

  /// The date or time(s) the activity occurred.
  pub fn occurrence_timing(&self) -> Option<Timing> {
    if let Some(val) = self.value.get("occurrenceTiming") {
      return Some(Timing { value: val });
    }
    return None;
  }

  /// A larger event of which this particular event is a component or step.
  pub fn part_of(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("partOf") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
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

  /// Extensions for language
  pub fn _language(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_language") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Identifies the person who picked up the Supply.
  pub fn receiver(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("receiver") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Identifier for the supply delivery event that is used to identify it across
  /// multiple disparate systems.
  pub fn identifier(&self) -> Option<Vec<Identifier>> {
    if let Some(Value::Array(val)) = self.value.get("identifier") {
      return Some(val.into_iter().map(|e| Identifier { value: e }).collect::<Vec<_>>());
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

  /// The item that is being delivered or has been supplied.
  pub fn supplied_item(&self) -> Option<SupplyDelivery_SuppliedItem> {
    if let Some(val) = self.value.get("suppliedItem") {
      return Some(SupplyDelivery_SuppliedItem { value: val });
    }
    return None;
  }

  /// The individual responsible for dispensing the medication, supplier or device.
  pub fn supplier(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("supplier") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Identification of the facility/location where the Supply was shipped to, as part
  /// of the dispense event.
  pub fn destination(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("destination") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Indicates the type of dispensing event that is performed. Examples include:
  /// Trial Fill, Completion of Trial, Partial Fill, Emergency Fill, Samples, etc.
  pub fn fhir_type(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("type") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// The date or time(s) the activity occurred.
  pub fn occurrence_date_time(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("occurrenceDateTime") {
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

  /// Extensions for occurrenceDateTime
  pub fn _occurrence_date_time(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_occurrenceDateTime") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A link to a resource representing the person whom the delivered item is for.
  pub fn patient(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("patient") {
      return Some(Reference { value: val });
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

  /// A code specifying the state of the dispense event.
  pub fn status(&self) -> Option<SupplyDeliveryStatus> {
    if let Some(Value::String(val)) = self.value.get("status") {
      return Some(SupplyDeliveryStatus::from_string(&val).unwrap());
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

  /// The base language in which the resource is written.
  pub fn language(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("language") {
      return Some(string.to_string());
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

}

#[derive(Debug)]
pub enum SupplyDeliveryStatus {
  InProgress,
  Completed,
  Abandoned,
  EnteredInError,
}

impl SupplyDeliveryStatus {
    pub fn from_string(string: &str) -> Option<SupplyDeliveryStatus> {
      match string {
        "in-progress" => Some(SupplyDeliveryStatus::InProgress),
        "completed" => Some(SupplyDeliveryStatus::Completed),
        "abandoned" => Some(SupplyDeliveryStatus::Abandoned),
        "entered-in-error" => Some(SupplyDeliveryStatus::EnteredInError),
        _ => None,
    }
  }
}

