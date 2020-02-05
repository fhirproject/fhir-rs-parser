use serde::{Deserialize, Serialize};

/// A provider issued list of professional services and products which have been
/// provided, or are to be provided, to a patient which is sent to an insurer for
/// reimbursement.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Claim_Diagnosis {
  /// Extensions for sequence
  _sequence: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// When the condition was observed or the relative ranking.
  type: Vec<CodeableConcept>,

  /// The nature of illness or problem in a coded form or as a reference to an
  /// external defined Condition.
  #[serde(rename = "diagnosisReference")]
  diagnosis_reference: Reference,

  /// The nature of illness or problem in a coded form or as a reference to an
  /// external defined Condition.
  #[serde(rename = "diagnosisCodeableConcept")]
  diagnosis_codeable_concept: CodeableConcept,

  /// Indication of whether the diagnosis was present on admission to a facility.
  #[serde(rename = "onAdmission")]
  on_admission: CodeableConcept,

  /// A number to uniquely identify diagnosis entries.
  sequence: positiveInt,

  /// A package billing code or bundle code used to group products and services to a
  /// particular health condition (such as heart attack) which is based on a
  /// predetermined grouping code system.
  #[serde(rename = "packageCode")]
  package_code: CodeableConcept,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

}
