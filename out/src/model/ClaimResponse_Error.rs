#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;


/// This resource provides the adjudication details from the processing of a Claim
/// resource.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponse_Error {
  /// Extensions for detailSequence
  #[serde(rename = "_detailSequence")]
  _detail_sequence: Option<Element>,

  /// The sequence number of the detail within the line item submitted which contains
  /// the error. This value is omitted when the error occurs outside of the item
  /// structure.
  #[serde(rename = "detailSequence")]
  detail_sequence: Option<i32>,

  /// The sequence number of the sub-detail within the detail within the line item
  /// submitted which contains the error. This value is omitted when the error occurs
  /// outside of the item structure.
  #[serde(rename = "subDetailSequence")]
  sub_detail_sequence: Option<i32>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for itemSequence
  #[serde(rename = "_itemSequence")]
  _item_sequence: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for subDetailSequence
  #[serde(rename = "_subDetailSequence")]
  _sub_detail_sequence: Option<Element>,

  /// The sequence number of the line item submitted which contains the error. This
  /// value is omitted when the error occurs outside of the item structure.
  #[serde(rename = "itemSequence")]
  item_sequence: Option<i32>,

  /// An error code, from a specified code system, which details why the claim could
  /// not be adjudicated.
  code: CodeableConcept,

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

}
