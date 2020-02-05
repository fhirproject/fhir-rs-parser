use serde::{Deserialize, Serialize};

/// Representation of the content produced in a DICOM imaging study. A study
/// comprises a set of series, each of which includes a set of Service-Object Pair
/// Instances (SOP Instances - images or other data) acquired or produced in a
/// common context.  A series is of only one modality (e.g. X-ray, CT, MR,
/// ultrasound), but a study may have multiple series of different modalities.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImagingStudy {
  /// The base language in which the resource is written.
  language: String,

  /// Number of SOP Instances in Study. This value given may be larger than the number
  /// of instance elements this resource contains due to resource availability,
  /// security, or other factors. This element should be present if any instance
  /// elements are present.
  #[serde(rename = "numberOfInstances")]
  number_of_instances: unsignedInt,

  /// Each study has one or more series of images or other content.
  series: Vec<ImagingStudy_Series>,

  /// The healthcare event (e.g. a patient and healthcare provider interaction) during
  /// which this ImagingStudy is made.
  encounter: Reference,

  /// A list of the diagnostic requests that resulted in this imaging study being
  /// performed.
  #[serde(rename = "basedOn")]
  based_on: Vec<Reference>,

  /// Who read the study and interpreted the images or other content.
  interpreter: Vec<Reference>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// The code for the performed procedure type.
  #[serde(rename = "procedureCode")]
  procedure_code: Vec<CodeableConcept>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Indicates another resource whose existence justifies this Study.
  #[serde(rename = "reasonReference")]
  reason_reference: Vec<Reference>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Extensions for numberOfInstances
  #[serde(rename = "_numberOfInstances")]
  _number_of_instances: Element,

  /// The procedure which this ImagingStudy was part of.
  #[serde(rename = "procedureReference")]
  procedure_reference: Reference,

  /// The Imaging Manager description of the study. Institution-generated description
  /// or classification of the Study (component) performed.
  description: String,

  /// Date and time the study started.
  started: dateTime,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// The network service providing access (e.g., query, view, or retrieval) for the
  /// study. See implementation notes for information about using DICOM endpoints. A
  /// study-level endpoint applies to each series in the study, unless overridden by a
  /// series-level endpoint with the same Endpoint.connectionType.
  endpoint: Vec<Reference>,

  /// The subject, typically a patient, of the imaging study.
  subject: Reference,

  /// Extensions for started
  _started: Element,

  /// Extensions for numberOfSeries
  #[serde(rename = "_numberOfSeries")]
  _number_of_series: Element,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// Description of clinical condition indicating why the ImagingStudy was requested.
  #[serde(rename = "reasonCode")]
  reason_code: Vec<CodeableConcept>,

  /// The requesting/referring physician.
  referrer: Reference,

  /// Identifiers for the ImagingStudy such as DICOM Study Instance UID, and Accession
  /// Number.
  identifier: Vec<Identifier>,

  /// A list of all the series.modality values that are actual acquisition modalities,
  /// i.e. those in the DICOM Context Group 29 (value set OID 1.2.840.10008.6.1.19).
  modality: Vec<Coding>,

  /// The current state of the ImagingStudy.
  status: ImagingStudyStatus,

  /// Extensions for status
  _status: Element,

  /// Number of Series in the Study. This value given may be larger than the number of
  /// series elements this Resource contains due to resource availability, security,
  /// or other factors. This element should be present if any series elements are
  /// present.
  #[serde(rename = "numberOfSeries")]
  number_of_series: unsignedInt,

  /// Extensions for description
  _description: Element,

  /// Per the recommended DICOM mapping, this element is derived from the Study
  /// Description attribute (0008,1030). Observations or findings about the imaging
  /// study should be recorded in another resource, e.g. Observation, and not in this
  /// element.
  note: Vec<Annotation>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Extensions for language
  _language: Element,

  /// The principal physical location where the ImagingStudy was performed.
  location: Reference,

}

#[derive(Debug, Serialize, Deserialize)]
enum ImagingStudyStatus {
  #[serde(rename = "registered")]
  Registered,

  #[serde(rename = "available")]
  Available,

  #[serde(rename = "cancelled")]
  Cancelled,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

  #[serde(rename = "unknown")]
  Unknown,

}
