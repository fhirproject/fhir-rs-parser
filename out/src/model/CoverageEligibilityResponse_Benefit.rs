#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Money::Money;
use crate::model::Extension::Extension;
use crate::model::Element::Element;


/// This resource provides eligibility and plan details from the processing of an
/// CoverageEligibilityRequest resource.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityResponse_Benefit {
  /// The quantity of the benefit which have been consumed to date.
  #[serde(rename = "usedUnsignedInt")]
  used_unsigned_int: Option<i32>,

  /// Classification of benefit being provided.
  #[serde(rename = "type")]
  fhir_type: CodeableConcept,

  /// The quantity of the benefit which is permitted under the coverage.
  #[serde(rename = "allowedMoney")]
  allowed_money: Option<Money>,

  /// The quantity of the benefit which is permitted under the coverage.
  #[serde(rename = "allowedUnsignedInt")]
  allowed_unsigned_int: Option<i32>,

  /// Extensions for allowedString
  #[serde(rename = "_allowedString")]
  _allowed_string: Option<Element>,

  /// Extensions for usedUnsignedInt
  #[serde(rename = "_usedUnsignedInt")]
  _used_unsigned_int: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The quantity of the benefit which have been consumed to date.
  #[serde(rename = "usedString")]
  used_string: Option<String>,

  /// Extensions for usedString
  #[serde(rename = "_usedString")]
  _used_string: Option<Element>,

  /// The quantity of the benefit which have been consumed to date.
  #[serde(rename = "usedMoney")]
  used_money: Option<Money>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

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

  /// The quantity of the benefit which is permitted under the coverage.
  #[serde(rename = "allowedString")]
  allowed_string: Option<String>,

  /// Extensions for allowedUnsignedInt
  #[serde(rename = "_allowedUnsignedInt")]
  _allowed_unsigned_int: Option<Element>,

}
