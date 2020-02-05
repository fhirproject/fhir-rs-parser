use serde::{Deserialize, Serialize};

/// Describes a comparison of an immunization event against published
/// recommendations to determine if the administration is "valid" in relation to
/// those  recommendations.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImmunizationEvaluation {
  /// One possible path to achieve presumed immunity against a disease - within the
  /// context of an authority.
  series: String,

  /// A unique identifier assigned to this immunization evaluation record.
  identifier: Vec<Identifier>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The date the evaluation of the vaccine administration event was performed.
  date: dateTime,

  /// Indicates if the dose is valid or not valid with respect to the published
  /// recommendations.
  #[serde(rename = "doseStatus")]
  dose_status: CodeableConcept,

  /// Extensions for seriesDosesString
  #[serde(rename = "_seriesDosesString")]
  _series_doses_string: Element,

  /// Nominal position in a series.
  #[serde(rename = "doseNumberString")]
  dose_number_string: String,

  /// The recommended number of doses to achieve immunity.
  #[serde(rename = "seriesDosesString")]
  series_doses_string: String,

  /// The individual for whom the evaluation is being done.
  patient: Reference,

  /// The vaccine administration event being evaluated.
  #[serde(rename = "immunizationEvent")]
  immunization_event: Reference,

  /// Provides an explanation as to why the vaccine administration event is valid or
  /// not relative to the published recommendations.
  #[serde(rename = "doseStatusReason")]
  dose_status_reason: Vec<CodeableConcept>,

  /// Extensions for description
  _description: Element,

  /// Extensions for status
  _status: Element,

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

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// Extensions for doseNumberPositiveInt
  #[serde(rename = "_doseNumberPositiveInt")]
  _dose_number_positive_int: Element,

  /// Extensions for seriesDosesPositiveInt
  #[serde(rename = "_seriesDosesPositiveInt")]
  _series_doses_positive_int: Element,

  /// Extensions for date
  _date: Element,

  /// Extensions for doseNumberString
  #[serde(rename = "_doseNumberString")]
  _dose_number_string: Element,

  /// Indicates the authority who published the protocol (e.g. ACIP).
  authority: Reference,

  /// Additional information about the evaluation.
  description: String,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Extensions for series
  _series: Element,

  /// Nominal position in a series.
  #[serde(rename = "doseNumberPositiveInt")]
  dose_number_positive_int: number,

  /// Extensions for language
  _language: Element,

  /// The vaccine preventable disease the dose is being evaluated against.
  #[serde(rename = "targetDisease")]
  target_disease: CodeableConcept,

  /// The recommended number of doses to achieve immunity.
  #[serde(rename = "seriesDosesPositiveInt")]
  series_doses_positive_int: number,

  /// The base language in which the resource is written.
  language: String,

  /// Indicates the current status of the evaluation of the vaccination administration
  /// event.
  status: String,

}
