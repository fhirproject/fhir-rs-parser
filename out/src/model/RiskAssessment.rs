#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Period::Period;
use crate::model::Element::Element;
use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Narrative::Narrative;
use crate::model::Meta::Meta;
use crate::model::RiskAssessment_Prediction::RiskAssessment_Prediction;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;


/// An assessment of the likely outcome(s) for a patient or other subject as well as
/// the likelihood of each outcome.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskAssessment {
  /// The algorithm, process or mechanism used to evaluate the risk.
  method: Option<CodeableConcept>,

  /// The date (and possibly time) the risk assessment was performed.
  #[serde(rename = "occurrencePeriod")]
  occurrence_period: Option<Period>,

  /// Additional comments about the risk assessment.
  note: Option<Vec<Annotation>>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// A reference to a resource that this risk assessment is part of, such as a
  /// Procedure.
  parent: Option<Box<Reference>>,

  /// The encounter where the assessment was performed.
  encounter: Option<Box<Reference>>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// The provider or software application that performed the assessment.
  performer: Option<Box<Reference>>,

  /// A reference to the request that is fulfilled by this risk assessment.
  #[serde(rename = "basedOn")]
  based_on: Option<Box<Reference>>,

  /// Extensions for mitigation
  #[serde(rename = "_mitigation")]
  _mitigation: Option<Element>,

  /// Extensions for occurrenceDateTime
  #[serde(rename = "_occurrenceDateTime")]
  _occurrence_date_time: Option<Element>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// The reason the risk assessment was performed.
  #[serde(rename = "reasonCode")]
  reason_code: Option<Vec<CodeableConcept>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// The status of the RiskAssessment, using the same statuses as an Observation.
  status: Option<String>,

  /// The date (and possibly time) the risk assessment was performed.
  #[serde(rename = "occurrenceDateTime")]
  occurrence_date_time: Option<String>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// The patient or group the risk assessment applies to.
  subject: Box<Reference>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// The type of the risk assessment performed.
  code: Option<CodeableConcept>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Resources supporting the reason the risk assessment was performed.
  #[serde(rename = "reasonReference")]
  reason_reference: Option<Vec<Box<Reference>>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// A description of the steps that might be taken to reduce the identified risk(s).
  mitigation: Option<String>,

  /// Business identifier assigned to the risk assessment.
  identifier: Option<Vec<Identifier>>,

  /// Describes the expected outcome for the subject.
  prediction: Option<Vec<RiskAssessment_Prediction>>,

  /// For assessments or prognosis specific to a particular condition, indicates the
  /// condition being assessed.
  condition: Option<Box<Reference>>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Indicates the source data considered as part of the assessment (for example,
  /// FamilyHistory, Observations, Procedures, Conditions, etc.).
  basis: Option<Vec<Box<Reference>>>,

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

}
