use serde::{Deserialize, Serialize};

/// Representation of the content produced in a DICOM imaging study. A study
/// comprises a set of series, each of which includes a set of Service-Object Pair
/// Instances (SOP Instances - images or other data) acquired or produced in a
/// common context.  A series is of only one modality (e.g. X-ray, CT, MR,
/// ultrasound), but a study may have multiple series of different modalities.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImagingStudy_Series {
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

  /// The specimen imaged, e.g., for whole slide imaging of a biopsy.
  specimen: Vec<Reference>,

  /// A single SOP instance within the series, e.g. an image, or presentation state.
  instance: Vec<ImagingStudy_Instance>,

  /// The numeric identifier of this series in the study.
  number: unsignedInt,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// A description of the series.
  description: String,

  /// The modality of this series sequence.
  modality: Coding,

  /// The network service providing access (e.g., query, view, or retrieval) for this
  /// series. See implementation notes for information about using DICOM endpoints. A
  /// series-level endpoint, if present, has precedence over a study-level endpoint
  /// with the same Endpoint.connectionType.
  endpoint: Vec<Reference>,

  /// Extensions for description
  _description: Element,

  /// Extensions for started
  _started: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// The date and time the series was started.
  started: dateTime,

  /// Indicates who or what performed the series and how they were involved.
  performer: Vec<ImagingStudy_Performer>,

  /// The DICOM Series Instance UID for the series.
  uid: id,

  /// Extensions for number
  _number: Element,

  /// Extensions for uid
  _uid: Element,

  /// Extensions for numberOfInstances
  #[serde(rename = "_numberOfInstances")]
  _number_of_instances: Element,

  /// The anatomic structures examined. See DICOM Part 16 Annex L
  /// (http://dicom.nema.org/medical/dicom/current/output/chtml/part16/chapter_L.html)
  /// for DICOM to SNOMED-CT mappings. The bodySite may indicate the laterality of
  /// body part imaged; if so, it shall be consistent with any content of
  /// ImagingStudy.series.laterality.
  #[serde(rename = "bodySite")]
  body_site: Coding,

  /// Number of SOP Instances in the Study. The value given may be larger than the
  /// number of instance elements this resource contains due to resource availability,
  /// security, or other factors. This element should be present if any instance
  /// elements are present.
  #[serde(rename = "numberOfInstances")]
  number_of_instances: unsignedInt,

  /// The laterality of the (possibly paired) anatomic structures examined. E.g., the
  /// left knee, both lungs, or unpaired abdomen. If present, shall be consistent with
  /// any laterality information indicated in ImagingStudy.series.bodySite.
  laterality: Coding,

}
