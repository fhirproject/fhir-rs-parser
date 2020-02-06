#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// Example of workflow instance.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenario_Actor {
  /// The type of actor - person or system.
  #[serde(rename = "type")]
  fhir_type: ExampleScenario_ActorType,

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

  /// ID or acronym of actor.
  #[serde(rename = "actorId")]
  actor_id: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for actorId
  #[serde(rename = "_actorId")]
  _actor_id: Element,

  /// The description of the actor.
  description: String,

  /// Extensions for description
  _description: Element,

  /// Extensions for type
  _type: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for name
  _name: Element,

  /// The name of the actor as shown in the page.
  name: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExampleScenario_ActorType {
  #[serde(rename = "person")]
  Person,

  #[serde(rename = "entity")]
  Entity,

}
