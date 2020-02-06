#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Reference::Reference;
use crate::model::RelatedArtifact::RelatedArtifact;
use crate::model::Duration::Duration;
use crate::model::Timing::Timing;
use crate::model::Range::Range;
use crate::model::Age::Age;
use crate::model::RequestGroup_Condition::RequestGroup_Condition;
use crate::model::Period::Period;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::RequestGroup_RelatedAction::RequestGroup_RelatedAction;
use crate::model::Extension::Extension;


/// A group of related requests that can be used to capture intended activities that
/// have inter-dependencies such as "give this medication after that one".
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestGroup_Action {
  /// An optional value describing when the action should be performed.
  #[serde(rename = "timingTiming")]
  timing_timing: Option<Timing>,

  /// An optional value describing when the action should be performed.
  #[serde(rename = "timingDateTime")]
  timing_date_time: Option<String>,

  /// An expression that describes applicability criteria, or start/stop conditions
  /// for the action.
  condition: Option<Vec<RequestGroup_Condition>>,

  /// Defines whether the action should usually be preselected.
  #[serde(rename = "precheckBehavior")]
  precheck_behavior: Option<String>,

  /// A user-visible prefix for the action.
  prefix: Option<String>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for prefix
  #[serde(rename = "_prefix")]
  _prefix: Option<Element>,

  /// Extensions for priority
  #[serde(rename = "_priority")]
  _priority: Option<Element>,

  /// A code that provides meaning for the action or action group. For example, a
  /// section may have a LOINC code for a section of a documentation template.
  code: Option<Vec<CodeableConcept>>,

  /// A short description of the action used to provide a summary to display to the
  /// user.
  description: Option<String>,

  /// Defines the selection behavior for the action and its children.
  #[serde(rename = "selectionBehavior")]
  selection_behavior: Option<String>,

  /// Defines whether the action can be selected multiple times.
  #[serde(rename = "cardinalityBehavior")]
  cardinality_behavior: Option<String>,

  /// Extensions for cardinalityBehavior
  #[serde(rename = "_cardinalityBehavior")]
  _cardinality_behavior: Option<Element>,

  /// An optional value describing when the action should be performed.
  #[serde(rename = "timingRange")]
  timing_range: Option<Range>,

  /// Extensions for textEquivalent
  #[serde(rename = "_textEquivalent")]
  _text_equivalent: Option<Element>,

  /// Defines expectations around whether an action is required.
  #[serde(rename = "requiredBehavior")]
  required_behavior: Option<String>,

  /// A relationship to another action such as "before" or "30-60 minutes after start
  /// of".
  #[serde(rename = "relatedAction")]
  related_action: Option<Vec<RequestGroup_RelatedAction>>,

  /// An optional value describing when the action should be performed.
  #[serde(rename = "timingAge")]
  timing_age: Option<Age>,

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
  modifier_extension: Option<Vec<Box<Extension>>>,

  /// A text equivalent of the action to be performed. This provides a human-
  /// interpretable description of the action when the definition is consumed by
  /// a system that might not be capable of interpreting it dynamically.
  #[serde(rename = "textEquivalent")]
  text_equivalent: Option<String>,

  /// Indicates how quickly the action should be addressed with respect to other
  /// actions.
  priority: Option<String>,

  /// Extensions for timingDateTime
  #[serde(rename = "_timingDateTime")]
  _timing_date_time: Option<Element>,

  /// An optional value describing when the action should be performed.
  #[serde(rename = "timingPeriod")]
  timing_period: Option<Period>,

  /// The type of action to perform (create, update, remove).
  #[serde(rename = "type")]
  fhir_type: Option<CodeableConcept>,

  /// Extensions for groupingBehavior
  #[serde(rename = "_groupingBehavior")]
  _grouping_behavior: Option<Element>,

  /// Extensions for precheckBehavior
  #[serde(rename = "_precheckBehavior")]
  _precheck_behavior: Option<Element>,

  /// Defines the grouping behavior for the action and its children.
  #[serde(rename = "groupingBehavior")]
  grouping_behavior: Option<String>,

  /// Extensions for selectionBehavior
  #[serde(rename = "_selectionBehavior")]
  _selection_behavior: Option<Element>,

  /// The resource that is the target of the action (e.g. CommunicationRequest).
  resource: Option<Box<Reference>>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// The title of the action displayed to a user.
  title: Option<String>,

  /// The participant that should perform or be responsible for this action.
  participant: Option<Vec<Box<Reference>>>,

  /// An optional value describing when the action should be performed.
  #[serde(rename = "timingDuration")]
  timing_duration: Option<Duration>,

  /// Sub actions.
  action: Option<Vec<RequestGroup_Action>>,

  /// Extensions for requiredBehavior
  #[serde(rename = "_requiredBehavior")]
  _required_behavior: Option<Element>,

  /// Extensions for title
  #[serde(rename = "_title")]
  _title: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Didactic or other informational resources associated with the action that can be
  /// provided to the CDS recipient. Information resources can include inline text
  /// commentary and links to web resources.
  documentation: Option<Vec<RelatedArtifact>>,

}
