use serde::{Deserialize, Serialize};

/// Risk of harmful or undesirable, physiological response which is unique to an
/// individual and associated with exposure to a substance.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AllergyIntolerance {
  /// Additional narrative about the propensity for the Adverse Reaction, not captured
  /// in other fields.
  note: Vec<Annotation>,

  /// Estimated or actual date,  date-time, or age when allergy or intolerance was
  /// identified.
  #[serde(rename = "onsetAge")]
  onset_age: Age,

  /// Extensions for lastOccurrence
  #[serde(rename = "_lastOccurrence")]
  _last_occurrence: Element,

  /// Estimate of the potential clinical harm, or seriousness, of the reaction to the
  /// identified substance.
  criticality: AllergyIntoleranceCriticality,

  /// The source of the information about the allergy that is recorded.
  asserter: Reference,

  /// Extensions for type
  _type: Element,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// The clinical status of the allergy or intolerance.
  #[serde(rename = "clinicalStatus")]
  clinical_status: CodeableConcept,

  /// Estimated or actual date,  date-time, or age when allergy or intolerance was
  /// identified.
  #[serde(rename = "onsetDateTime")]
  onset_date_time: String,

  /// The patient who has the allergy or intolerance.
  patient: Reference,

  /// Extensions for language
  _language: Element,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Extensions for criticality
  _criticality: Element,

  /// The encounter when the allergy or intolerance was asserted.
  encounter: Reference,

  /// Details about each adverse reaction event linked to exposure to the identified
  /// substance.
  reaction: Vec<AllergyIntolerance_Reaction>,

  /// Extensions for onsetDateTime
  #[serde(rename = "_onsetDateTime")]
  _onset_date_time: Element,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// Estimated or actual date,  date-time, or age when allergy or intolerance was
  /// identified.
  #[serde(rename = "onsetPeriod")]
  onset_period: Period,

  /// Individual who recorded the record and takes responsibility for its content.
  recorder: Reference,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Identification of the underlying physiological mechanism for the reaction risk.
  type: AllergyIntoleranceType,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Assertion about certainty associated with the propensity, or potential risk, of
  /// a reaction to the identified substance (including pharmaceutical product).
  #[serde(rename = "verificationStatus")]
  verification_status: CodeableConcept,

  /// Code for an allergy or intolerance statement (either a positive or a
  /// negated/excluded statement).  This may be a code for a substance or
  /// pharmaceutical product that is considered to be responsible for the adverse
  /// reaction risk (e.g., "Latex"), an allergy or intolerance condition (e.g., "Latex
  /// allergy"), or a negated/excluded code for a specific substance or class (e.g.,
  /// "No latex allergy") or a general or categorical negated statement (e.g.,  "No
  /// known allergy", "No known drug allergies").  Note: the substance for a specific
  /// reaction may be different from the substance identified as the cause of the
  /// risk, but it must be consistent with it. For instance, it may be a more specific
  /// substance (e.g. a brand medication) or a composite product that includes the
  /// identified substance. It must be clinically safe to only process the 'code' and
  /// ignore the 'reaction.substance'.  If a receiving system is unable to confirm
  /// that AllergyIntolerance.reaction.substance falls within the semantic scope of
  /// AllergyIntolerance.code, then the receiving system should ignore
  /// AllergyIntolerance.reaction.substance.
  code: CodeableConcept,

  /// Estimated or actual date,  date-time, or age when allergy or intolerance was
  /// identified.
  #[serde(rename = "onsetString")]
  onset_string: String,

  /// Represents the date and/or time of the last known occurrence of a reaction
  /// event.
  #[serde(rename = "lastOccurrence")]
  last_occurrence: dateTime,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Extensions for onsetString
  #[serde(rename = "_onsetString")]
  _onset_string: Element,

  /// Business identifiers assigned to this AllergyIntolerance by the performer or
  /// other systems which remain constant as the resource is updated and propagates
  /// from server to server.
  identifier: Vec<Identifier>,

  /// The base language in which the resource is written.
  language: String,

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

  /// The recordedDate represents when this particular AllergyIntolerance record was
  /// created in the system, which is often a system-generated date.
  #[serde(rename = "recordedDate")]
  recorded_date: dateTime,

  /// Extensions for recordedDate
  #[serde(rename = "_recordedDate")]
  _recorded_date: Element,

  /// Extensions for category
  _category: Vec<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Estimated or actual date,  date-time, or age when allergy or intolerance was
  /// identified.
  #[serde(rename = "onsetRange")]
  onset_range: Range,

}

#[derive(Debug, Serialize, Deserialize)]
enum AllergyIntoleranceCriticality {
  #[serde(rename = "low")]
  Low,

  #[serde(rename = "high")]
  High,

  #[serde(rename = "unable-to-assess")]
  UnableToAssess,

}

#[derive(Debug, Serialize, Deserialize)]
enum AllergyIntoleranceType {
  #[serde(rename = "allergy")]
  Allergy,

  #[serde(rename = "intolerance")]
  Intolerance,

}
