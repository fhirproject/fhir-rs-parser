#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;


/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [[[CodeSystem]]] definitions and their use in [coded
/// elements](terminologies.html).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSet_Filter {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The match value may be either a code defined by the system, or a string value,
  /// which is a regex match on the literal string of the property value  (if the
  /// filter represents a property defined in CodeSystem) or of the system filter
  /// value (if the filter represents a filter defined in CodeSystem) when the
  /// operation is 'regex', or one of the values (true and false), when the operation
  /// is 'exists'.
  value: Option<String>,

  /// The kind of operation to perform as a part of the filter criteria.
  op: Option<ValueSet_FilterOp>,

  /// Extensions for property
  #[serde(rename = "_property")]
  _property: Option<Element>,

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

  /// A code that identifies a property or a filter defined in the code system.
  property: Option<String>,

  /// Extensions for op
  #[serde(rename = "_op")]
  _op: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Extensions for value
  #[serde(rename = "_value")]
  _value: Option<Element>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ValueSet_FilterOp {
  #[serde(rename = "=")]
  Equal,

  #[serde(rename = "is-a")]
  IsA,

  #[serde(rename = "descendent-of")]
  DescendentOf,

  #[serde(rename = "is-not-a")]
  IsNotA,

  #[serde(rename = "regex")]
  Regex,

  #[serde(rename = "in")]
  In,

  #[serde(rename = "not-in")]
  NotIn,

  #[serde(rename = "generalizes")]
  Generalizes,

  #[serde(rename = "exists")]
  Exists,

}
