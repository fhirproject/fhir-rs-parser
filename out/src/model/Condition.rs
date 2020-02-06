#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ResourceList::ResourceList;
use crate::model::Extension::Extension;
use crate::model::Narrative::Narrative;
use crate::model::Range::Range;
use crate::model::Age::Age;
use crate::model::Identifier::Identifier;
use crate::model::Period::Period;
use crate::model::Element::Element;
use crate::model::Reference::Reference;
use crate::model::Condition_Stage::Condition_Stage;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Annotation::Annotation;
use crate::model::Condition_Evidence::Condition_Evidence;
use crate::model::Meta::Meta;


/// A clinical condition, problem, diagnosis, or other event, situation, issue, or
/// clinical concept that has risen to a level of concern.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
  /// Extensions for recordedDate
  #[serde(rename = "_recordedDate")]
  _recorded_date: Option<Element>,

  /// Individual who recorded the record and takes responsibility for its content.
  recorder: Option<Box<Reference>>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Estimated or actual date or date-time  the condition began, in the opinion of
  /// the clinician.
  #[serde(rename = "onsetAge")]
  onset_age: Option<Age>,

  /// Individual who is making the condition statement.
  asserter: Option<Box<Reference>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

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

  /// Clinical stage or grade of a condition. May include formal severity assessments.
  stage: Option<Vec<Condition_Stage>>,

  /// Extensions for onsetDateTime
  #[serde(rename = "_onsetDateTime")]
  _onset_date_time: Option<Element>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// A subjective assessment of the severity of the condition as evaluated by the
  /// clinician.
  severity: Option<CodeableConcept>,

  /// The recordedDate represents when this particular Condition record was created in
  /// the system, which is often a system-generated date.
  #[serde(rename = "recordedDate")]
  recorded_date: Option<String>,

  /// Supporting evidence / manifestations that are the basis of the Condition's
  /// verification status, such as evidence that confirmed or refuted the condition.
  evidence: Option<Vec<Condition_Evidence>>,

  /// Additional information about the Condition. This is a general notes/comments
  /// entry  for description of the Condition, its diagnosis and prognosis.
  note: Option<Vec<Annotation>>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// The date or estimated date that the condition resolved or went into remission.
  /// This is called "abatement" because of the many overloaded connotations
  /// associated with "remission" or "resolution" - Conditions are never really
  /// resolved, but they can abate.
  #[serde(rename = "abatementAge")]
  abatement_age: Option<Age>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// The clinical status of the condition.
  #[serde(rename = "clinicalStatus")]
  clinical_status: Option<CodeableConcept>,

  /// Extensions for abatementString
  #[serde(rename = "_abatementString")]
  _abatement_string: Option<Element>,

  /// The verification status to support the clinical status of the condition.
  #[serde(rename = "verificationStatus")]
  verification_status: Option<CodeableConcept>,

  /// The Encounter during which this Condition was created or to which the creation
  /// of this record is tightly associated.
  encounter: Option<Box<Reference>>,

  /// Estimated or actual date or date-time  the condition began, in the opinion of
  /// the clinician.
  #[serde(rename = "onsetDateTime")]
  onset_date_time: Option<String>,

  /// Extensions for onsetString
  #[serde(rename = "_onsetString")]
  _onset_string: Option<Element>,

  /// Business identifiers assigned to this condition by the performer or other
  /// systems which remain constant as the resource is updated and propagates from
  /// server to server.
  identifier: Option<Vec<Identifier>>,

  /// Estimated or actual date or date-time  the condition began, in the opinion of
  /// the clinician.
  #[serde(rename = "onsetRange")]
  onset_range: Option<Range>,

  /// The date or estimated date that the condition resolved or went into remission.
  /// This is called "abatement" because of the many overloaded connotations
  /// associated with "remission" or "resolution" - Conditions are never really
  /// resolved, but they can abate.
  #[serde(rename = "abatementRange")]
  abatement_range: Option<Range>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// The date or estimated date that the condition resolved or went into remission.
  /// This is called "abatement" because of the many overloaded connotations
  /// associated with "remission" or "resolution" - Conditions are never really
  /// resolved, but they can abate.
  #[serde(rename = "abatementPeriod")]
  abatement_period: Option<Period>,

  /// The date or estimated date that the condition resolved or went into remission.
  /// This is called "abatement" because of the many overloaded connotations
  /// associated with "remission" or "resolution" - Conditions are never really
  /// resolved, but they can abate.
  #[serde(rename = "abatementDateTime")]
  abatement_date_time: Option<String>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// A category assigned to the condition.
  category: Option<Vec<CodeableConcept>>,

  /// The anatomical location where this condition manifests itself.
  #[serde(rename = "bodySite")]
  body_site: Option<Vec<CodeableConcept>>,

  /// Indicates the patient or group who the condition record is associated with.
  subject: Box<Reference>,

  /// Identification of the condition, problem or diagnosis.
  code: Option<CodeableConcept>,

  /// Extensions for abatementDateTime
  #[serde(rename = "_abatementDateTime")]
  _abatement_date_time: Option<Element>,

  /// Estimated or actual date or date-time  the condition began, in the opinion of
  /// the clinician.
  #[serde(rename = "onsetString")]
  onset_string: Option<String>,

  /// Estimated or actual date or date-time  the condition began, in the opinion of
  /// the clinician.
  #[serde(rename = "onsetPeriod")]
  onset_period: Option<Period>,

  /// The date or estimated date that the condition resolved or went into remission.
  /// This is called "abatement" because of the many overloaded connotations
  /// associated with "remission" or "resolution" - Conditions are never really
  /// resolved, but they can abate.
  #[serde(rename = "abatementString")]
  abatement_string: Option<String>,

}
