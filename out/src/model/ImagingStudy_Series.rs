#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::ImagingStudy_Performer::ImagingStudy_Performer;
use crate::model::ImagingStudy_Instance::ImagingStudy_Instance;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::Coding::Coding;


/// Representation of the content produced in a DICOM imaging study. A study
/// comprises a set of series, each of which includes a set of Service-Object Pair
/// Instances (SOP Instances - images or other data) acquired or produced in a
/// common context.  A series is of only one modality (e.g. X-ray, CT, MR,
/// ultrasound), but a study may have multiple series of different modalities.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagingStudy_Series {
  /// The date and time the series was started.
  started: Option<String>,

  /// Indicates who or what performed the series and how they were involved.
  performer: Option<Vec<ImagingStudy_Performer>>,

  /// A single SOP instance within the series, e.g. an image, or presentation state.
  instance: Option<Vec<ImagingStudy_Instance>>,

  /// A description of the series.
  description: Option<String>,

  /// The laterality of the (possibly paired) anatomic structures examined. E.g., the
  /// left knee, both lungs, or unpaired abdomen. If present, shall be consistent with
  /// any laterality information indicated in ImagingStudy.series.bodySite.
  laterality: Option<Coding>,

  /// The anatomic structures examined. See DICOM Part 16 Annex L
  /// (http://dicom.nema.org/medical/dicom/current/output/chtml/part16/chapter_L.html)
  /// for DICOM to SNOMED-CT mappings. The bodySite may indicate the laterality of
  /// body part imaged; if so, it shall be consistent with any content of
  /// ImagingStudy.series.laterality.
  #[serde(rename = "bodySite")]
  body_site: Option<Coding>,

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

  /// The network service providing access (e.g., query, view, or retrieval) for this
  /// series. See implementation notes for information about using DICOM endpoints. A
  /// series-level endpoint, if present, has precedence over a study-level endpoint
  /// with the same Endpoint.connectionType.
  endpoint: Option<Vec<Box<Reference>>>,

  /// The specimen imaged, e.g., for whole slide imaging of a biopsy.
  specimen: Option<Vec<Box<Reference>>>,

  /// Extensions for started
  #[serde(rename = "_started")]
  _started: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Extensions for number
  #[serde(rename = "_number")]
  _number: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The DICOM Series Instance UID for the series.
  uid: Option<String>,

  /// Extensions for numberOfInstances
  #[serde(rename = "_numberOfInstances")]
  _number_of_instances: Option<Element>,

  /// The numeric identifier of this series in the study.
  number: Option<u32>,

  /// Number of SOP Instances in the Study. The value given may be larger than the
  /// number of instance elements this resource contains due to resource availability,
  /// security, or other factors. This element should be present if any instance
  /// elements are present.
  #[serde(rename = "numberOfInstances")]
  number_of_instances: Option<u32>,

  /// The modality of this series sequence.
  modality: Coding,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// Extensions for uid
  #[serde(rename = "_uid")]
  _uid: Option<Element>,

}
