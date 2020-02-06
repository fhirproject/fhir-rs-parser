#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ResourceList::ResourceList;
use crate::model::Narrative::Narrative;
use crate::model::Annotation::Annotation;
use crate::model::Reference::Reference;
use crate::model::Identifier::Identifier;
use crate::model::Age::Age;
use crate::model::Period::Period;
use crate::model::Range::Range;
use crate::model::Meta::Meta;
use crate::model::AllergyIntolerance_Reaction::AllergyIntolerance_Reaction;


/// Risk of harmful or undesirable, physiological response which is unique to an
/// individual and associated with exposure to a substance.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllergyIntolerance {
  /// The encounter when the allergy or intolerance was asserted.
  encounter: Option<Box<Reference>>,

  /// Estimated or actual date,  date-time, or age when allergy or intolerance was
  /// identified.
  #[serde(rename = "onsetRange")]
  onset_range: Option<Range>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Estimated or actual date,  date-time, or age when allergy or intolerance was
  /// identified.
  #[serde(rename = "onsetDateTime")]
  onset_date_time: Option<String>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.    Modifier
  /// extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Option<Vec<Extension>>,

  /// Extensions for onsetString
  #[serde(rename = "_onsetString")]
  _onset_string: Option<Element>,

  /// Details about each adverse reaction event linked to exposure to the identified
  /// substance.
  reaction: Option<Vec<AllergyIntolerance_Reaction>>,

  /// Assertion about certainty associated with the propensity, or potential risk, of
  /// a reaction to the identified substance (including pharmaceutical product).
  #[serde(rename = "verificationStatus")]
  verification_status: Option<CodeableConcept>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Extensions for recordedDate
  #[serde(rename = "_recordedDate")]
  _recorded_date: Option<Element>,

  /// Estimate of the potential clinical harm, or seriousness, of the reaction to the
  /// identified substance.
  criticality: Option<AllergyIntoleranceCriticality>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// The patient who has the allergy or intolerance.
  patient: Box<Reference>,

  /// The recordedDate represents when this particular AllergyIntolerance record was
  /// created in the system, which is often a system-generated date.
  #[serde(rename = "recordedDate")]
  recorded_date: Option<String>,

  /// Additional narrative about the propensity for the Adverse Reaction, not captured
  /// in other fields.
  note: Option<Vec<Annotation>>,

  /// Represents the date and/or time of the last known occurrence of a reaction
  /// event.
  #[serde(rename = "lastOccurrence")]
  last_occurrence: Option<String>,

  /// The clinical status of the allergy or intolerance.
  #[serde(rename = "clinicalStatus")]
  clinical_status: Option<CodeableConcept>,

  /// Extensions for type
  #[serde(rename = "_type")]
  _type: Option<Element>,

  /// Business identifiers assigned to this AllergyIntolerance by the performer or
  /// other systems which remain constant as the resource is updated and propagates
  /// from server to server.
  identifier: Option<Vec<Identifier>>,

  /// Estimated or actual date,  date-time, or age when allergy or intolerance was
  /// identified.
  #[serde(rename = "onsetString")]
  onset_string: Option<String>,

  /// Estimated or actual date,  date-time, or age when allergy or intolerance was
  /// identified.
  #[serde(rename = "onsetAge")]
  onset_age: Option<Age>,

  /// Extensions for onsetDateTime
  #[serde(rename = "_onsetDateTime")]
  _onset_date_time: Option<Element>,

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
  code: Option<CodeableConcept>,

  /// Extensions for category
  #[serde(rename = "_category")]
  _category: Option<Vec<Element>>,

  /// Individual who recorded the record and takes responsibility for its content.
  recorder: Option<Box<Reference>>,

  /// Extensions for lastOccurrence
  #[serde(rename = "_lastOccurrence")]
  _last_occurrence: Option<Element>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Identification of the underlying physiological mechanism for the reaction risk.
  #[serde(rename = "type")]
  fhir_type: Option<AllergyIntoleranceType>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// The source of the information about the allergy that is recorded.
  asserter: Option<Box<Reference>>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// Extensions for criticality
  #[serde(rename = "_criticality")]
  _criticality: Option<Element>,

  /// Estimated or actual date,  date-time, or age when allergy or intolerance was
  /// identified.
  #[serde(rename = "onsetPeriod")]
  onset_period: Option<Period>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum AllergyIntoleranceCriticality {
  #[serde(rename = "low")]
  Low,

  #[serde(rename = "high")]
  High,

  #[serde(rename = "unable-to-assess")]
  UnableToAssess,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum AllergyIntoleranceType {
  #[serde(rename = "allergy")]
  Allergy,

  #[serde(rename = "intolerance")]
  Intolerance,

}
