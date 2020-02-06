#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Reference::Reference;
use crate::model::Period::Period;
use crate::model::Timing::Timing;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::Element::Element;


/// Describes the intention of how one or more practitioners intend to deliver care
/// for a particular patient, group or community for a period of time, possibly
/// limited to care for a specific condition or set of conditions.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarePlan_Detail {
  /// Detailed description of the type of planned activity; e.g. what lab test, what
  /// procedure, what kind of encounter.
  code: CodeableConcept,

  /// Identifies the quantity expected to be supplied, administered or consumed by the
  /// subject.
  quantity: Quantity,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// The period, timing or frequency upon which the described activity is to occur.
  #[serde(rename = "scheduledTiming")]
  scheduled_timing: Timing,

  /// The URL pointing to an externally maintained protocol, guideline, questionnaire
  /// or other definition that is adhered to in whole or in part by this CarePlan
  /// activity.
  #[serde(rename = "instantiatesUri")]
  instantiates_uri: Vec<String>,

  /// Identifies who's expected to be involved in the activity.
  performer: Vec<Box<Reference>>,

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
  modifier_extension: Vec<Extension>,

  /// Identifies what progress is being made for the specific activity.
  status: CarePlan_DetailStatus,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for kind
  _kind: Element,

  /// The period, timing or frequency upon which the described activity is to occur.
  #[serde(rename = "scheduledString")]
  scheduled_string: String,

  /// Extensions for status
  _status: Element,

  /// Identifies the facility where the activity will occur; e.g. home, hospital,
  /// specific clinic, etc.
  location: Box<Reference>,

  /// The URL pointing to a FHIR-defined protocol, guideline, questionnaire or other
  /// definition that is adhered to in whole or in part by this CarePlan activity.
  #[serde(rename = "instantiatesCanonical")]
  instantiates_canonical: Vec<String>,

  /// Extensions for description
  _description: Element,

  /// Identifies the quantity expected to be consumed in a given day.
  #[serde(rename = "dailyAmount")]
  daily_amount: Quantity,

  /// If true, indicates that the described activity is one that must NOT be engaged
  /// in when following the plan.  If false, or missing, indicates that the described
  /// activity is one that should be engaged in when following the plan.
  #[serde(rename = "doNotPerform")]
  do_not_perform: bool,

  /// Extensions for scheduledString
  #[serde(rename = "_scheduledString")]
  _scheduled_string: Element,

  /// Internal reference that identifies the goals that this activity is intended to
  /// contribute towards meeting.
  goal: Vec<Box<Reference>>,

  /// Provides reason why the activity isn't yet started, is on hold, was cancelled,
  /// etc.
  #[serde(rename = "statusReason")]
  status_reason: CodeableConcept,

  /// A description of the kind of resource the in-line definition of a care plan
  /// activity is representing.  The CarePlan.activity.detail is an in-line definition
  /// when a resource is not referenced using CarePlan.activity.reference.  For
  /// example, a MedicationRequest, a ServiceRequest, or a CommunicationRequest.
  kind: String,

  /// Identifies the food, drug or other product to be consumed or supplied in the
  /// activity.
  #[serde(rename = "productCodeableConcept")]
  product_codeable_concept: CodeableConcept,

  /// Extensions for instantiatesUri
  #[serde(rename = "_instantiatesUri")]
  _instantiates_uri: Vec<Element>,

  /// Identifies the food, drug or other product to be consumed or supplied in the
  /// activity.
  #[serde(rename = "productReference")]
  product_reference: Box<Reference>,

  /// Extensions for doNotPerform
  #[serde(rename = "_doNotPerform")]
  _do_not_perform: Element,

  /// This provides a textual description of constraints on the intended activity
  /// occurrence, including relation to other activities.  It may also include
  /// objectives, pre-conditions and end-conditions.  Finally, it may convey specifics
  /// about the activity such as body site, method, route, etc.
  description: String,

  /// Indicates another resource, such as the health condition(s), whose existence
  /// justifies this request and drove the inclusion of this particular activity as
  /// part of the plan.
  #[serde(rename = "reasonReference")]
  reason_reference: Vec<Box<Reference>>,

  /// Provides the rationale that drove the inclusion of this particular activity as
  /// part of the plan or the reason why the activity was prohibited.
  #[serde(rename = "reasonCode")]
  reason_code: Vec<CodeableConcept>,

  /// The period, timing or frequency upon which the described activity is to occur.
  #[serde(rename = "scheduledPeriod")]
  scheduled_period: Period,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum CarePlan_DetailStatus {
  #[serde(rename = "not-started")]
  NotStarted,

  #[serde(rename = "scheduled")]
  Scheduled,

  #[serde(rename = "in-progress")]
  InProgress,

  #[serde(rename = "on-hold")]
  OnHold,

  #[serde(rename = "completed")]
  Completed,

  #[serde(rename = "cancelled")]
  Cancelled,

  #[serde(rename = "stopped")]
  Stopped,

  #[serde(rename = "unknown")]
  Unknown,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

}
