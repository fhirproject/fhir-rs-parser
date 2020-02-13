#![allow(unused_imports, non_camel_case_types)]

use crate::model::MedicationRequest_InitialFill::MedicationRequest_InitialFill;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::Duration::Duration;
use crate::model::Quantity::Quantity;
use crate::model::Period::Period;
use serde_json::value::Value;



/// An order or request for both supply of the medication and the instructions for
/// administration of the medication to a patient. The resource is called
/// "MedicationRequest" rather than "MedicationPrescription" or "MedicationOrder" to
/// generalize the use across inpatient and outpatient settings, including care
/// plans, etc., and to harmonize with workflow patterns.

#[derive(Debug)]
pub struct MedicationRequest_DispenseRequest<'a> {
  pub value: &'a Value,
}

impl MedicationRequest_DispenseRequest<'_> {
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
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
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
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

  /// This indicates the validity period of a prescription (stale dating the
  /// Prescription).
  pub fn validity_period(&self) -> Option<Period> {
    if let Some(val) = self.value.get("validityPeriod") {
      return Some(Period { value: val });
    }
    return None;
  }

  /// Extensions for numberOfRepeatsAllowed
  pub fn _number_of_repeats_allowed(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_numberOfRepeatsAllowed") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Indicates the intended dispensing Organization specified by the prescriber.
  pub fn performer(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("performer") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Indicates the quantity or duration for the first dispense of the medication.
  pub fn initial_fill(&self) -> Option<MedicationRequest_InitialFill> {
    if let Some(val) = self.value.get("initialFill") {
      return Some(MedicationRequest_InitialFill { value: val });
    }
    return None;
  }

  /// Identifies the period time over which the supplied product is expected to be
  /// used, or the length of time the dispense is expected to last.
  pub fn expected_supply_duration(&self) -> Option<Duration> {
    if let Some(val) = self.value.get("expectedSupplyDuration") {
      return Some(Duration { value: val });
    }
    return None;
  }

  /// An integer indicating the number of times, in addition to the original dispense,
  /// (aka refills or repeats) that the patient can receive the prescribed medication.
  /// Usage Notes: This integer does not include the original order dispense. This
  /// means that if an order indicates dispense 30 tablets plus "3 repeats", then the
  /// order can be dispensed a total of 4 times and the patient can receive a total of
  /// 120 tablets.  A prescriber may explicitly say that zero refills are permitted
  /// after the initial dispense.
  pub fn number_of_repeats_allowed(&self) -> Option<u64> {
    if let Some(val) = self.value.get("numberOfRepeatsAllowed") {
      return Some(val.as_u64().unwrap());
    }
    return None;
  }

  /// The minimum period of time that must occur between dispenses of the medication.
  pub fn dispense_interval(&self) -> Option<Duration> {
    if let Some(val) = self.value.get("dispenseInterval") {
      return Some(Duration { value: val });
    }
    return None;
  }

  /// The amount that is to be dispensed for one fill.
  pub fn quantity(&self) -> Option<Quantity> {
    if let Some(val) = self.value.get("quantity") {
      return Some(Quantity { value: val });
    }
    return None;
  }

}
