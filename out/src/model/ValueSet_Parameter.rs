#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [[[CodeSystem]]] definitions and their use in [coded
/// elements](terminologies.html).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSet_Parameter {
  /// The value of the parameter.
  #[serde(rename = "valueBoolean")]
  value_boolean: Option<bool>,

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

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for valueDateTime
  #[serde(rename = "_valueDateTime")]
  _value_date_time: Option<Element>,

  /// Name of the input parameter to the $expand operation; may be a server-assigned
  /// name for additional default or other server-supplied parameters used to control
  /// the expansion process.
  name: Option<String>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// Extensions for valueUri
  #[serde(rename = "_valueUri")]
  _value_uri: Option<Element>,

  /// Extensions for valueCode
  #[serde(rename = "_valueCode")]
  _value_code: Option<Element>,

  /// The value of the parameter.
  #[serde(rename = "valueDecimal")]
  value_decimal: Option<i32>,

  /// The value of the parameter.
  #[serde(rename = "valueString")]
  value_string: Option<String>,

  /// Extensions for valueInteger
  #[serde(rename = "_valueInteger")]
  _value_integer: Option<Element>,

  /// Extensions for valueBoolean
  #[serde(rename = "_valueBoolean")]
  _value_boolean: Option<Element>,

  /// The value of the parameter.
  #[serde(rename = "valueDateTime")]
  value_date_time: Option<String>,

  /// Extensions for valueDecimal
  #[serde(rename = "_valueDecimal")]
  _value_decimal: Option<Element>,

  /// Extensions for valueString
  #[serde(rename = "_valueString")]
  _value_string: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// The value of the parameter.
  #[serde(rename = "valueCode")]
  value_code: Option<String>,

  /// The value of the parameter.
  #[serde(rename = "valueUri")]
  value_uri: Option<String>,

  /// The value of the parameter.
  #[serde(rename = "valueInteger")]
  value_integer: Option<i32>,

}
