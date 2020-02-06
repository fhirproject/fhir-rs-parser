#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Coding::Coding;
use crate::model::Extension::Extension;
use crate::model::Element::Element;


/// The CodeSystem resource is used to declare the existence of and describe a code
/// system or code system supplement and its key properties, and optionally define a
/// part or all of its content.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystem_Property1 {
  /// The value of this property.
  #[serde(rename = "valueCoding")]
  value_coding: Option<Coding>,

  /// The value of this property.
  #[serde(rename = "valueInteger")]
  value_integer: Option<i32>,

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
  modifier_extension: Option<Vec<Extension>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for valueDateTime
  #[serde(rename = "_valueDateTime")]
  _value_date_time: Option<Element>,

  /// Extensions for valueBoolean
  #[serde(rename = "_valueBoolean")]
  _value_boolean: Option<Element>,

  /// Extensions for valueString
  #[serde(rename = "_valueString")]
  _value_string: Option<Element>,

  /// Extensions for valueDecimal
  #[serde(rename = "_valueDecimal")]
  _value_decimal: Option<Element>,

  /// Extensions for valueCode
  #[serde(rename = "_valueCode")]
  _value_code: Option<Element>,

  /// Extensions for valueInteger
  #[serde(rename = "_valueInteger")]
  _value_integer: Option<Element>,

  /// Extensions for code
  #[serde(rename = "_code")]
  _code: Option<Element>,

  /// The value of this property.
  #[serde(rename = "valueCode")]
  value_code: Option<String>,

  /// The value of this property.
  #[serde(rename = "valueDateTime")]
  value_date_time: Option<String>,

  /// The value of this property.
  #[serde(rename = "valueDecimal")]
  value_decimal: Option<i32>,

  /// The value of this property.
  #[serde(rename = "valueString")]
  value_string: Option<String>,

  /// The value of this property.
  #[serde(rename = "valueBoolean")]
  value_boolean: Option<bool>,

  /// A code that is a reference to CodeSystem.property.code.
  code: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

}
