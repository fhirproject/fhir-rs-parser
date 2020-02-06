#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::Identifier::Identifier;
use crate::model::Reference::Reference;


/// A provider issued list of professional services and products which have been
/// provided, or are to be provided, to a patient which is sent to an insurer for
/// reimbursement.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claim_Insurance {
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

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for focal
  #[serde(rename = "_focal")]
  _focal: Option<Element>,

  /// A number to uniquely identify insurance entries and provide a sequence of
  /// coverages to convey coordination of benefit order.
  sequence: Option<i32>,

  /// A business agreement number established between the provider and the insurer for
  /// special business processing purposes.
  #[serde(rename = "businessArrangement")]
  business_arrangement: Option<String>,

  /// Reference numbers previously provided by the insurer to the provider to be
  /// quoted on subsequent claims containing services or products related to the prior
  /// authorization.
  #[serde(rename = "preAuthRef")]
  pre_auth_ref: Option<Vec<String>>,

  /// Reference to the insurance card level information contained in the Coverage
  /// resource. The coverage issuing insurer will use these details to locate the
  /// patient's actual coverage within the insurer's information system.
  coverage: Box<Reference>,

  /// Extensions for businessArrangement
  #[serde(rename = "_businessArrangement")]
  _business_arrangement: Option<Element>,

  /// The result of the adjudication of the line items for the Coverage specified in
  /// this insurance.
  #[serde(rename = "claimResponse")]
  claim_response: Option<Box<Reference>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for sequence
  #[serde(rename = "_sequence")]
  _sequence: Option<Element>,

  /// The business identifier to be used when the claim is sent for adjudication
  /// against this insurance policy.
  identifier: Option<Identifier>,

  /// Extensions for preAuthRef
  #[serde(rename = "_preAuthRef")]
  _pre_auth_ref: Option<Vec<Element>>,

  /// A flag to indicate that this Coverage is to be used for adjudication of this
  /// claim when set to true.
  focal: Option<bool>,

}
