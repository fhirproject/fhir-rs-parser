#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::MedicationRequest_InitialFill::MedicationRequest_InitialFill;
use crate::model::Duration::Duration;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use crate::model::Period::Period;
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// An order or request for both supply of the medication and the instructions for
/// administration of the medication to a patient. The resource is called
/// "MedicationRequest" rather than "MedicationPrescription" or "MedicationOrder" to
/// generalize the use across inpatient and outpatient settings, including care
/// plans, etc., and to harmonize with workflow patterns.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationRequest_DispenseRequest {
  /// The minimum period of time that must occur between dispenses of the medication.
  #[serde(rename = "dispenseInterval")]
  dispense_interval: Option<Duration>,

  /// An integer indicating the number of times, in addition to the original dispense,
  /// (aka refills or repeats) that the patient can receive the prescribed medication.
  /// Usage Notes: This integer does not include the original order dispense. This
  /// means that if an order indicates dispense 30 tablets plus "3 repeats", then the
  /// order can be dispensed a total of 4 times and the patient can receive a total of
  /// 120 tablets.  A prescriber may explicitly say that zero refills are permitted
  /// after the initial dispense.
  #[serde(rename = "numberOfRepeatsAllowed")]
  number_of_repeats_allowed: Option<u32>,

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
  #[serde(rename = "modifierExtension")]
  modifier_extension: Option<Vec<Box<Extension>>>,

  /// Indicates the quantity or duration for the first dispense of the medication.
  #[serde(rename = "initialFill")]
  initial_fill: Option<MedicationRequest_InitialFill>,

  /// The amount that is to be dispensed for one fill.
  quantity: Option<Quantity>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// This indicates the validity period of a prescription (stale dating the
  /// Prescription).
  #[serde(rename = "validityPeriod")]
  validity_period: Option<Period>,

  /// Indicates the intended dispensing Organization specified by the prescriber.
  performer: Option<Box<Reference>>,

  /// Identifies the period time over which the supplied product is expected to be
  /// used, or the length of time the dispense is expected to last.
  #[serde(rename = "expectedSupplyDuration")]
  expected_supply_duration: Option<Duration>,

  /// Extensions for numberOfRepeatsAllowed
  #[serde(rename = "_numberOfRepeatsAllowed")]
  _number_of_repeats_allowed: Option<Element>,

}
