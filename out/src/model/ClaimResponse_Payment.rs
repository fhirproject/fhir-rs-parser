#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Identifier::Identifier;
use crate::model::Extension::Extension;
use crate::model::Money::Money;


/// This resource provides the adjudication details from the processing of a Claim
/// resource.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponse_Payment {
  /// Whether this represents partial or complete payment of the benefits payable.
  #[serde(rename = "type")]
  fhir_type: CodeableConcept,

  /// Estimated date the payment will be issued or the actual issue date of payment.
  date: Option<i32>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Reason for the payment adjustment.
  #[serde(rename = "adjustmentReason")]
  adjustment_reason: Option<CodeableConcept>,

  /// Issuer's unique identifier for the payment instrument.
  identifier: Option<Identifier>,

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

  /// Benefits payable less any payment adjustment.
  amount: Money,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Total amount of all adjustments to this payment included in this transaction
  /// which are not related to this claim's adjudication.
  adjustment: Option<Money>,

  /// Extensions for date
  #[serde(rename = "_date")]
  _date: Option<Element>,

}
