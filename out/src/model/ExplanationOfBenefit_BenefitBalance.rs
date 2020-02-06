#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ExplanationOfBenefit_Financial::ExplanationOfBenefit_Financial;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;


/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefit_BenefitBalance {
  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// Indicates if the benefits apply to an individual or to the family.
  unit: Option<CodeableConcept>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// A richer description of the benefit or services covered.
  description: Option<String>,

  /// The term or period of the values such as 'maximum lifetime benefit' or 'maximum
  /// annual visits'.
  term: Option<CodeableConcept>,

  /// Is a flag to indicate whether the benefits refer to in-network providers or out-
  /// of-network providers.
  network: Option<CodeableConcept>,

  /// Benefits Used to date.
  financial: Option<Vec<ExplanationOfBenefit_Financial>>,

  /// A short name or tag for the benefit.
  name: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Code to identify the general type of benefits under which products and services
  /// are provided.
  category: CodeableConcept,

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

  /// Extensions for excluded
  #[serde(rename = "_excluded")]
  _excluded: Option<Element>,

  /// True if the indicated class of service is excluded from the plan, missing or
  /// False indicates the product or service is included in the coverage.
  excluded: Option<bool>,

}
