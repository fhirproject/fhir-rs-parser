#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// A summary of information based on the results of executing a TestScript.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestReport_Assert {
  /// A link to further details on the result.
  detail: Option<String>,

  /// An explanatory message associated with the result.
  message: Option<String>,

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

  /// Extensions for result
  #[serde(rename = "_result")]
  _result: Option<Element>,

  /// The result of this assertion.
  result: Option<TestReport_AssertResult>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for detail
  #[serde(rename = "_detail")]
  _detail: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for message
  #[serde(rename = "_message")]
  _message: Option<Element>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum TestReport_AssertResult {
  #[serde(rename = "pass")]
  Pass,

  #[serde(rename = "skip")]
  Skip,

  #[serde(rename = "fail")]
  Fail,

  #[serde(rename = "warning")]
  Warning,

  #[serde(rename = "error")]
  Error,

}
