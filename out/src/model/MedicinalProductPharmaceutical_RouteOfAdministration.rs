#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::MedicinalProductPharmaceutical_TargetSpecies::MedicinalProductPharmaceutical_TargetSpecies;
use crate::model::Quantity::Quantity;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Duration::Duration;
use crate::model::Ratio::Ratio;
use crate::model::Extension::Extension;


/// A pharmaceutical product described in terms of its composition and dose form.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductPharmaceutical_RouteOfAdministration {
  /// The first dose (dose quantity) administered in humans can be specified, for a
  /// product under investigation, using a numerical value and its unit of
  /// measurement.
  #[serde(rename = "firstDose")]
  first_dose: Option<Quantity>,

  /// The maximum single dose that can be administered as per the protocol of a
  /// clinical trial can be specified using a numerical value and its unit of
  /// measurement.
  #[serde(rename = "maxSingleDose")]
  max_single_dose: Option<Quantity>,

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
  modifier_extension: Option<Vec<Extension>>,

  /// A species for which this route applies.
  #[serde(rename = "targetSpecies")]
  target_species: Option<Vec<MedicinalProductPharmaceutical_TargetSpecies>>,

  /// Coded expression for the route.
  code: CodeableConcept,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The maximum treatment period during which an Investigational Medicinal Product
  /// can be administered as per the protocol referenced in the clinical trial
  /// authorisation.
  #[serde(rename = "maxTreatmentPeriod")]
  max_treatment_period: Option<Duration>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The maximum dose per day (maximum dose quantity to be administered in any one
  /// 24-h period) that can be administered as per the protocol referenced in the
  /// clinical trial authorisation.
  #[serde(rename = "maxDosePerDay")]
  max_dose_per_day: Option<Quantity>,

  /// The maximum dose per treatment period that can be administered as per the
  /// protocol referenced in the clinical trial authorisation.
  #[serde(rename = "maxDosePerTreatmentPeriod")]
  max_dose_per_treatment_period: Option<Ratio>,

}
