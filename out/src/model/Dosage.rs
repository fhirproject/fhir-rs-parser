#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Dosage_DoseAndRate::Dosage_DoseAndRate;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::Element::Element;
use crate::model::Ratio::Ratio;
use crate::model::Timing::Timing;


/// Indicates how the medication is/was taken or should be taken by the patient.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dosage {
  /// Extensions for asNeededBoolean
  #[serde(rename = "_asNeededBoolean")]
  _as_needed_boolean: Option<Element>,

  /// The amount of medication administered.
  #[serde(rename = "doseAndRate")]
  dose_and_rate: Option<Vec<Dosage_DoseAndRate>>,

  /// How drug should enter body.
  route: Option<CodeableConcept>,

  /// Extensions for text
  #[serde(rename = "_text")]
  _text: Option<Element>,

  /// Upper limit on medication per unit of time.
  #[serde(rename = "maxDosePerPeriod")]
  max_dose_per_period: Option<Ratio>,

  /// Extensions for patientInstruction
  #[serde(rename = "_patientInstruction")]
  _patient_instruction: Option<Element>,

  /// Indicates the order in which the dosage instructions should be applied or
  /// interpreted.
  sequence: Option<i32>,

  /// Extensions for sequence
  #[serde(rename = "_sequence")]
  _sequence: Option<Element>,

  /// Upper limit on medication per administration.
  #[serde(rename = "maxDosePerAdministration")]
  max_dose_per_administration: Option<Quantity>,

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

  /// Supplemental instructions to the patient on how to take the medication  (e.g.
  /// "with meals" or"take half to one hour before food") or warnings for the patient
  /// about the medication (e.g. "may cause drowsiness" or "avoid exposure of skin to
  /// direct sunlight or sunlamps").
  #[serde(rename = "additionalInstruction")]
  additional_instruction: Option<Vec<CodeableConcept>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Upper limit on medication per lifetime of the patient.
  #[serde(rename = "maxDosePerLifetime")]
  max_dose_per_lifetime: Option<Quantity>,

  /// Indicates whether the Medication is only taken when needed within a specific
  /// dosing schedule (Boolean option), or it indicates the precondition for taking
  /// the Medication (CodeableConcept).
  #[serde(rename = "asNeededCodeableConcept")]
  as_needed_codeable_concept: Option<CodeableConcept>,

  /// When medication should be administered.
  timing: Option<Timing>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Indicates whether the Medication is only taken when needed within a specific
  /// dosing schedule (Boolean option), or it indicates the precondition for taking
  /// the Medication (CodeableConcept).
  #[serde(rename = "asNeededBoolean")]
  as_needed_boolean: Option<bool>,

  /// Body site to administer to.
  site: Option<CodeableConcept>,

  /// Free text dosage instructions e.g. SIG.
  text: Option<String>,

  /// Technique for administering medication.
  method: Option<CodeableConcept>,

  /// Instructions in terms that are understood by the patient or consumer.
  #[serde(rename = "patientInstruction")]
  patient_instruction: Option<String>,

}
