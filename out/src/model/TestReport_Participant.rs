#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;


/// A summary of information based on the results of executing a TestScript.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestReport_Participant {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

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

  /// Extensions for uri
  #[serde(rename = "_uri")]
  _uri: Option<Element>,

  /// The display name of the participant.
  display: Option<String>,

  /// Extensions for display
  #[serde(rename = "_display")]
  _display: Option<Element>,

  /// The type of participant.
  #[serde(rename = "type")]
  fhir_type: Option<TestReport_ParticipantType>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for type
  #[serde(rename = "_type")]
  _type: Option<Element>,

  /// The uri of the participant. An absolute URL is preferred.
  uri: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum TestReport_ParticipantType {
  #[serde(rename = "test-engine")]
  TestEngine,

  #[serde(rename = "client")]
  Client,

  #[serde(rename = "server")]
  Server,

}
