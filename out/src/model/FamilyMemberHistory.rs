#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Narrative::Narrative;
use crate::model::Annotation::Annotation;
use crate::model::Period::Period;
use crate::model::ResourceList::ResourceList;
use crate::model::Reference::Reference;
use crate::model::Meta::Meta;
use crate::model::Range::Range;
use crate::model::Age::Age;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::FamilyMemberHistory_Condition::FamilyMemberHistory_Condition;


/// Significant health conditions for a person related to the patient relevant in
/// the context of care for the patient.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyMemberHistory {
  /// The base language in which the resource is written.
  language: Option<String>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// The birth sex of the family member.
  sex: Option<CodeableConcept>,

  /// Describes why the family member's history is not available.
  #[serde(rename = "dataAbsentReason")]
  data_absent_reason: Option<CodeableConcept>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// The person who this history concerns.
  patient: Box<Reference>,

  /// The type of relationship this person has to the patient (father, mother, brother
  /// etc.).
  relationship: CodeableConcept,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// The age of the relative at the time the family member history is recorded.
  #[serde(rename = "ageRange")]
  age_range: Option<Range>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// If true, indicates that the age value specified is an estimated value.
  #[serde(rename = "estimatedAge")]
  estimated_age: Option<bool>,

  /// Extensions for estimatedAge
  #[serde(rename = "_estimatedAge")]
  _estimated_age: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// The actual or approximate date of birth of the relative.
  #[serde(rename = "bornString")]
  born_string: Option<String>,

  /// Deceased flag or the actual or approximate age of the relative at the time of
  /// death for the family member history record.
  #[serde(rename = "deceasedBoolean")]
  deceased_boolean: Option<bool>,

  /// Extensions for deceasedBoolean
  #[serde(rename = "_deceasedBoolean")]
  _deceased_boolean: Option<Element>,

  /// Deceased flag or the actual or approximate age of the relative at the time of
  /// death for the family member history record.
  #[serde(rename = "deceasedAge")]
  deceased_age: Option<Age>,

  /// Extensions for bornDate
  #[serde(rename = "_bornDate")]
  _born_date: Option<Element>,

  /// Deceased flag or the actual or approximate age of the relative at the time of
  /// death for the family member history record.
  #[serde(rename = "deceasedRange")]
  deceased_range: Option<Range>,

  /// Deceased flag or the actual or approximate age of the relative at the time of
  /// death for the family member history record.
  #[serde(rename = "deceasedString")]
  deceased_string: Option<String>,

  /// Indicates a Condition, Observation, AllergyIntolerance, or QuestionnaireResponse
  /// that justifies this family member history event.
  #[serde(rename = "reasonReference")]
  reason_reference: Option<Vec<Box<Reference>>>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// The actual or approximate date of birth of the relative.
  #[serde(rename = "bornPeriod")]
  born_period: Option<Period>,

  /// Extensions for deceasedString
  #[serde(rename = "_deceasedString")]
  _deceased_string: Option<Element>,

  /// The date (and possibly time) when the family member history was recorded or last
  /// updated.
  date: Option<String>,

  /// The URL pointing to an externally maintained protocol, guideline, orderset or
  /// other definition that is adhered to in whole or in part by this
  /// FamilyMemberHistory.
  #[serde(rename = "instantiatesUri")]
  instantiates_uri: Option<Vec<String>>,

  /// Extensions for deceasedDate
  #[serde(rename = "_deceasedDate")]
  _deceased_date: Option<Element>,

  /// Extensions for instantiatesUri
  #[serde(rename = "_instantiatesUri")]
  _instantiates_uri: Option<Vec<Element>>,

  /// Describes why the family member history occurred in coded or textual form.
  #[serde(rename = "reasonCode")]
  reason_code: Option<Vec<CodeableConcept>>,

  /// This property allows a non condition-specific note to the made about the related
  /// person. Ideally, the note would be in the condition property, but this is not
  /// always possible.
  note: Option<Vec<Annotation>>,

  /// Business identifiers assigned to this family member history by the performer or
  /// other systems which remain constant as the resource is updated and propagates
  /// from server to server.
  identifier: Option<Vec<Identifier>>,

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
  modifier_extension: Option<Vec<Box<Extension>>>,

  /// Deceased flag or the actual or approximate age of the relative at the time of
  /// death for the family member history record.
  #[serde(rename = "deceasedDate")]
  deceased_date: Option<String>,

  /// The age of the relative at the time the family member history is recorded.
  #[serde(rename = "ageAge")]
  age_age: Option<Age>,

  /// A code specifying the status of the record of the family history of a specific
  /// family member.
  status: Option<FamilyMemberHistoryStatus>,

  /// Extensions for date
  #[serde(rename = "_date")]
  _date: Option<Element>,

  /// The actual or approximate date of birth of the relative.
  #[serde(rename = "bornDate")]
  born_date: Option<String>,

  /// The age of the relative at the time the family member history is recorded.
  #[serde(rename = "ageString")]
  age_string: Option<String>,

  /// The significant Conditions (or condition) that the family member had. This is a
  /// repeating section to allow a system to represent more than one condition per
  /// resource, though there is nothing stopping multiple resources - one per
  /// condition.
  condition: Option<Vec<FamilyMemberHistory_Condition>>,

  /// Extensions for bornString
  #[serde(rename = "_bornString")]
  _born_string: Option<Element>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// Extensions for ageString
  #[serde(rename = "_ageString")]
  _age_string: Option<Element>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// This will either be a name or a description; e.g. "Aunt Susan", "my cousin with
  /// the red hair".
  name: Option<String>,

  /// The URL pointing to a FHIR-defined protocol, guideline, orderset or other
  /// definition that is adhered to in whole or in part by this FamilyMemberHistory.
  #[serde(rename = "instantiatesCanonical")]
  instantiates_canonical: Option<Vec<String>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum FamilyMemberHistoryStatus {
  #[serde(rename = "partial")]
  Partial,

  #[serde(rename = "completed")]
  Completed,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

  #[serde(rename = "health-unknown")]
  HealthUnknown,

}
