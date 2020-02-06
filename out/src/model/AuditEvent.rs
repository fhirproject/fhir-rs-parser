#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::AuditEvent_Entity::AuditEvent_Entity;
use crate::model::Meta::Meta;
use crate::model::Period::Period;
use crate::model::Element::Element;
use crate::model::AuditEvent_Agent::AuditEvent_Agent;
use crate::model::Narrative::Narrative;
use crate::model::Extension::Extension;
use crate::model::AuditEvent_Source::AuditEvent_Source;
use crate::model::ResourceList::ResourceList;
use crate::model::Coding::Coding;
use crate::model::CodeableConcept::CodeableConcept;


/// A record of an event made for purposes of maintaining a security log. Typical
/// uses include detection of intrusion attempts and monitoring for inappropriate
/// usage.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditEvent {
  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// Extensions for action
  #[serde(rename = "_action")]
  _action: Option<Element>,

  /// The period during which the activity occurred.
  period: Option<Period>,

  /// Indicates whether the event succeeded or failed.
  outcome: Option<AuditEventOutcome>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Specific instances of data or objects that have been accessed.
  entity: Option<Vec<AuditEvent_Entity>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// The system that is reporting the event.
  source: AuditEvent_Source,

  /// The time when the event was recorded.
  recorded: Option<String>,

  /// Extensions for recorded
  #[serde(rename = "_recorded")]
  _recorded: Option<Element>,

  /// A free text description of the outcome of the event.
  #[serde(rename = "outcomeDesc")]
  outcome_desc: Option<String>,

  /// Extensions for outcomeDesc
  #[serde(rename = "_outcomeDesc")]
  _outcome_desc: Option<Element>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Identifier for a family of the event.  For example, a menu item, program, rule,
  /// policy, function code, application name or URL. It identifies the performed
  /// function.
  #[serde(rename = "type")]
  fhir_type: Coding,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

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

  /// The purposeOfUse (reason) that was used during the event being recorded.
  #[serde(rename = "purposeOfEvent")]
  purpose_of_event: Option<Vec<CodeableConcept>>,

  /// An actor taking an active role in the event or activity that is logged.
  agent: Vec<AuditEvent_Agent>,

  /// Extensions for outcome
  #[serde(rename = "_outcome")]
  _outcome: Option<Element>,

  /// Identifier for the category of event.
  subtype: Option<Vec<Coding>>,

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

  /// Indicator for type of action performed during the event that generated the
  /// audit.
  action: Option<AuditEventAction>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuditEventOutcome {
  #[serde(rename = "0")]
  Zero,

  #[serde(rename = "4")]
  Four,

  #[serde(rename = "8")]
  Eight,

  #[serde(rename = "12")]
  Twelve,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuditEventAction {
  #[serde(rename = "C")]
  C,

  #[serde(rename = "R")]
  R,

  #[serde(rename = "U")]
  U,

  #[serde(rename = "D")]
  D,

  #[serde(rename = "E")]
  E,

}
