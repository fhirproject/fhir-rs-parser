#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::ExampleScenario_Step::ExampleScenario_Step;


/// Example of workflow instance.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenario_Process {
  /// The diagram title of the group of operations.
  title: String,

  /// A longer description of the group of operations.
  description: String,

  /// Extensions for postConditions
  #[serde(rename = "_postConditions")]
  _post_conditions: Element,

  /// Extensions for title
  _title: Element,

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

  /// Extensions for description
  _description: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Each step of the process.
  step: Vec<ExampleScenario_Step>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Description of initial status before the process starts.
  #[serde(rename = "preConditions")]
  pre_conditions: String,

  /// Extensions for preConditions
  #[serde(rename = "_preConditions")]
  _pre_conditions: Element,

  /// Description of final status after the process ends.
  #[serde(rename = "postConditions")]
  post_conditions: String,

}
