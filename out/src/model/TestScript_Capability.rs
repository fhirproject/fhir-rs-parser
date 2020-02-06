#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestScript_Capability {
  /// Extensions for link
  #[serde(rename = "_link")]
  _link: Option<Vec<Element>>,

  /// Whether or not the test execution will validate the given capabilities of the
  /// server in order for this test script to execute.
  validated: Option<bool>,

  /// Extensions for destination
  #[serde(rename = "_destination")]
  _destination: Option<Element>,

  /// Extensions for validated
  #[serde(rename = "_validated")]
  _validated: Option<Element>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// Which origin server these requirements apply to.
  origin: Option<Vec<i32>>,

  /// Extensions for origin
  #[serde(rename = "_origin")]
  _origin: Option<Vec<Element>>,

  /// Which server these requirements apply to.
  destination: Option<i32>,

  /// Links to the FHIR specification that describes this interaction and the
  /// resources involved in more detail.
  link: Option<Vec<String>>,

  /// Description of the capabilities that this test script is requiring the server to
  /// support.
  description: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Minimum capabilities required of server for test script to execute successfully.
  /// If server does not meet at a minimum the referenced capability statement, then
  /// all tests in this script are skipped.
  capabilities: String,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Whether or not the test execution will require the given capabilities of the
  /// server in order for this test script to execute.
  required: Option<bool>,

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

  /// Extensions for required
  #[serde(rename = "_required")]
  _required: Option<Element>,

}
