#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::DetectedIssue_Evidence::DetectedIssue_Evidence;
use crate::model::Extension::Extension;
use crate::model::ResourceList::ResourceList;
use crate::model::Narrative::Narrative;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Reference::Reference;
use crate::model::Period::Period;
use crate::model::DetectedIssue_Mitigation::DetectedIssue_Mitigation;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;


/// Indicates an actual or potential clinical issue with or between one or more
/// active or proposed clinical actions for a patient; e.g. Drug-drug interaction,
/// Ineffective treatment frequency, Procedure-condition conflict, etc.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectedIssue {
  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Extensions for detail
  #[serde(rename = "_detail")]
  _detail: Option<Element>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Individual or device responsible for the issue being raised.  For example, a
  /// decision support application or a pharmacist conducting a medication review.
  author: Option<Box<Reference>>,

  /// Business identifier associated with the detected issue record.
  identifier: Option<Vec<Identifier>>,

  /// A textual explanation of the detected issue.
  detail: Option<String>,

  /// Indicates the degree of importance associated with the identified issue based on
  /// the potential impact on the patient.
  severity: Option<DetectedIssueSeverity>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

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

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Identifies the general type of issue identified.
  code: Option<CodeableConcept>,

  /// Extensions for reference
  #[serde(rename = "_reference")]
  _reference: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Indicates an action that has been taken or is committed to reduce or eliminate
  /// the likelihood of the risk identified by the detected issue from manifesting.
  /// Can also reflect an observation of known mitigating factors that may
  /// reduce/eliminate the need for any action.
  mitigation: Option<Vec<DetectedIssue_Mitigation>>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// Indicates the status of the detected issue.
  status: Option<String>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// Indicates the resource representing the current activity or proposed activity
  /// that is potentially problematic.
  implicated: Option<Vec<Box<Reference>>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// Indicates the patient whose record the detected issue is associated with.
  patient: Option<Box<Reference>>,

  /// The date or period when the detected issue was initially identified.
  #[serde(rename = "identifiedDateTime")]
  identified_date_time: Option<String>,

  /// The literature, knowledge-base or similar reference that describes the
  /// propensity for the detected issue identified.
  reference: Option<String>,

  /// Supporting evidence or manifestations that provide the basis for identifying the
  /// detected issue such as a GuidanceResponse or MeasureReport.
  evidence: Option<Vec<DetectedIssue_Evidence>>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// The date or period when the detected issue was initially identified.
  #[serde(rename = "identifiedPeriod")]
  identified_period: Option<Period>,

  /// Extensions for identifiedDateTime
  #[serde(rename = "_identifiedDateTime")]
  _identified_date_time: Option<Element>,

  /// Extensions for severity
  #[serde(rename = "_severity")]
  _severity: Option<Element>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum DetectedIssueSeverity {
  #[serde(rename = "high")]
  High,

  #[serde(rename = "moderate")]
  Moderate,

  #[serde(rename = "low")]
  Low,

}
