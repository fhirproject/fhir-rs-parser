#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Coverage_Exception::Coverage_Exception;
use crate::model::Money::Money;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Quantity::Quantity;


/// Financial instrument which may be used to reimburse or pay for health care
/// products and services. Includes both insurance and self-payment.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coverage_CostToBeneficiary {
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

  /// The category of patient centric costs associated with treatment.
  #[serde(rename = "type")]
  fhir_type: Option<CodeableConcept>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// The amount due from the patient for the cost category.
  #[serde(rename = "valueQuantity")]
  value_quantity: Option<Quantity>,

  /// The amount due from the patient for the cost category.
  #[serde(rename = "valueMoney")]
  value_money: Option<Money>,

  /// A suite of codes indicating exceptions or reductions to patient costs and their
  /// effective periods.
  exception: Option<Vec<Coverage_Exception>>,

}
