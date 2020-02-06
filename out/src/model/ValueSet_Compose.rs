#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::ValueSet_Include::ValueSet_Include;


/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [[[CodeSystem]]] definitions and their use in [coded
/// elements](terminologies.html).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSet_Compose {
  /// Whether inactive codes - codes that are not approved for current use - are in
  /// the value set. If inactive = true, inactive codes are to be included in the
  /// expansion, if inactive = false, the inactive codes will not be included in the
  /// expansion. If absent, the behavior is determined by the implementation, or by
  /// the applicable $expand parameters (but generally, inactive codes would be
  /// expected to be included).
  inactive: Option<bool>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The Locked Date is  the effective date that is used to determine the version of
  /// all referenced Code Systems and Value Set Definitions included in the compose
  /// that are not already tied to a specific version.
  #[serde(rename = "lockedDate")]
  locked_date: Option<i32>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for lockedDate
  #[serde(rename = "_lockedDate")]
  _locked_date: Option<Element>,

  /// Exclude one or more codes from the value set based on code system filters and/or
  /// other value sets.
  exclude: Option<Vec<ValueSet_Include>>,

  /// Include one or more codes from a code system or other value set(s).
  include: Vec<ValueSet_Include>,

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

  /// Extensions for inactive
  #[serde(rename = "_inactive")]
  _inactive: Option<Element>,

}
