use serde::{Deserialize, Serialize};

/// The findings and interpretation of diagnostic  tests performed on patients,
/// groups of patients, devices, and locations, and/or specimens derived from these.
/// The report includes clinical context such as requesting and provider
/// information, and some mix of atomic results, images, textual and coded
/// interpretations, and formatted representation of diagnostic reports.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DiagnosticReport {
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

  /// Details about the specimens on which this diagnostic report is based.
  specimen: Vec<Reference>,

  /// One or more links to full details of any imaging performed during the diagnostic
  /// investigation. Typically, this is imaging performed by DICOM enabled modalities,
  /// but this is not required. A fully enabled PACS viewer can use this information
  /// to provide views of the source images.
  #[serde(rename = "imagingStudy")]
  imaging_study: Vec<Reference>,

  /// The time or time-period the observed values are related to. When the subject of
  /// the report is a patient, this is usually either the time of the procedure or of
  /// specimen collection(s), but very often the source of the date/time is not known,
  /// only the date/time itself.
  #[serde(rename = "effectiveDateTime")]
  effective_date_time: String,

  /// Extensions for conclusion
  _conclusion: Element,

  /// A code that classifies the clinical discipline, department or diagnostic service
  /// that created the report (e.g. cardiology, biochemistry, hematology, MRI). This
  /// is used for searching, sorting and display purposes.
  category: Vec<CodeableConcept>,

  /// The time or time-period the observed values are related to. When the subject of
  /// the report is a patient, this is usually either the time of the procedure or of
  /// specimen collection(s), but very often the source of the date/time is not known,
  /// only the date/time itself.
  #[serde(rename = "effectivePeriod")]
  effective_period: Period,

  /// Rich text representation of the entire result as issued by the diagnostic
  /// service. Multiple formats are allowed but they SHALL be semantically equivalent.
  #[serde(rename = "presentedForm")]
  presented_form: Vec<Attachment>,

  /// Concise and clinically contextualized summary conclusion
  /// (interpretation/impression) of the diagnostic report.
  conclusion: String,

  /// One or more codes that represent the summary conclusion
  /// (interpretation/impression) of the diagnostic report.
  #[serde(rename = "conclusionCode")]
  conclusion_code: Vec<CodeableConcept>,

  /// The status of the diagnostic report.
  status: DiagnosticReportStatus,

  /// Extensions for language
  _language: Element,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The date and time that this version of the report was made available to
  /// providers, typically after the report was reviewed and verified.
  issued: instant,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// The subject of the report. Usually, but not always, this is a patient. However,
  /// diagnostic services also perform analyses on specimens collected from a variety
  /// of other sources.
  subject: Reference,

  /// A list of key images associated with this report. The images are generally
  /// created during the diagnostic process, and may be directly of the patient, or of
  /// treated specimens (i.e. slides of interest).
  media: Vec<DiagnosticReport_Media>,

  /// The practitioner or organization that is responsible for the report's
  /// conclusions and interpretations.
  #[serde(rename = "resultsInterpreter")]
  results_interpreter: Vec<Reference>,

  /// [Observations](observation.html)  that are part of this diagnostic report.
  result: Vec<Reference>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The healthcare event  (e.g. a patient and healthcare provider interaction) which
  /// this DiagnosticReport is about.
  encounter: Reference,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// The diagnostic service that is responsible for issuing the report.
  performer: Vec<Reference>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Extensions for status
  _status: Element,

  /// Extensions for issued
  _issued: Element,

  /// Details concerning a service requested.
  #[serde(rename = "basedOn")]
  based_on: Vec<Reference>,

  /// Identifiers assigned to this report by the performer or other systems.
  identifier: Vec<Identifier>,

  /// A code or name that describes this diagnostic report.
  code: CodeableConcept,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Extensions for effectiveDateTime
  #[serde(rename = "_effectiveDateTime")]
  _effective_date_time: Element,

  /// The base language in which the resource is written.
  language: String,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

}

#[derive(Debug, Serialize, Deserialize)]
enum DiagnosticReportStatus {
  #[serde(rename = "registered")]
  Registered,

  #[serde(rename = "partial")]
  Partial,

  #[serde(rename = "preliminary")]
  Preliminary,

  #[serde(rename = "final")]
  Final,

  #[serde(rename = "amended")]
  Amended,

  #[serde(rename = "corrected")]
  Corrected,

  #[serde(rename = "appended")]
  Appended,

  #[serde(rename = "cancelled")]
  Cancelled,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

  #[serde(rename = "unknown")]
  Unknown,

}
