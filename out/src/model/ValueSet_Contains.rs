#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ValueSet_Designation::ValueSet_Designation;
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [[[CodeSystem]]] definitions and their use in [coded
/// elements](terminologies.html).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSet_Contains {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The code for this item in the expansion hierarchy. If this code is missing the
  /// entry in the hierarchy is a place holder (abstract) and does not represent a
  /// valid code in the value set.
  code: Option<String>,

  /// Extensions for code
  #[serde(rename = "_code")]
  _code: Option<Element>,

  /// An absolute URI which is the code system in which the code for this item in the
  /// expansion is defined.
  system: Option<String>,

  /// The recommended display for this item in the expansion.
  display: Option<String>,

  /// Extensions for version
  #[serde(rename = "_version")]
  _version: Option<Element>,

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

  /// The version of the code system from this code was taken. Note that a well-
  /// maintained code system does not need the version reported, because the
  /// meaning of codes is consistent across versions. However this cannot consistently
  /// be assured, and when the meaning is not guaranteed to be consistent, the version
  /// SHOULD be exchanged.
  version: Option<String>,

  /// Extensions for inactive
  #[serde(rename = "_inactive")]
  _inactive: Option<Element>,

  /// Additional representations for this item - other languages, aliases, specialized
  /// purposes, used for particular purposes, etc. These are relevant when the
  /// conditions of the expansion do not fix to a single correct representation.
  designation: Option<Vec<ValueSet_Designation>>,

  /// If true, this entry is included in the expansion for navigational purposes, and
  /// the user cannot select the code directly as a proper value.
  #[serde(rename = "abstract")]
  fhir_abstract: Option<bool>,

  /// Other codes and entries contained under this entry in the hierarchy.
  contains: Option<Vec<ValueSet_Contains>>,

  /// Extensions for system
  #[serde(rename = "_system")]
  _system: Option<Element>,

  /// Extensions for abstract
  #[serde(rename = "_abstract")]
  _abstract: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// If the concept is inactive in the code system that defines it. Inactive codes
  /// are those that are no longer to be used, but are maintained by the code system
  /// for understanding legacy data. It might not be known or specified whether an
  /// concept is inactive (and it may depend on the context of use).
  inactive: Option<bool>,

  /// Extensions for display
  #[serde(rename = "_display")]
  _display: Option<Element>,

}
