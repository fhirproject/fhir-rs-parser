#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::InsurancePlan_SpecificCost::InsurancePlan_SpecificCost;
use crate::model::Reference::Reference;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Identifier::Identifier;
use crate::model::InsurancePlan_GeneralCost::InsurancePlan_GeneralCost;
use crate::model::Extension::Extension;


/// Details of a Health Insurance product/plan provided by an organization.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlan_Plan {
  /// Business identifiers assigned to this health insurance plan which remain
  /// constant as the resource is updated and propagates from server to server.
  identifier: Vec<Identifier>,

  /// Type of plan. For example, "Platinum" or "High Deductable".
  #[serde(rename = "type")]
  fhir_type: CodeableConcept,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

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
  modifier_extension: Vec<Extension>,

  /// Overall costs associated with the plan.
  #[serde(rename = "generalCost")]
  general_cost: Vec<InsurancePlan_GeneralCost>,

  /// Costs associated with the coverage provided by the product.
  #[serde(rename = "specificCost")]
  specific_cost: Vec<InsurancePlan_SpecificCost>,

  /// The geographic region in which a health insurance plan's benefits apply.
  #[serde(rename = "coverageArea")]
  coverage_area: Vec<Box<Reference>>,

  /// Reference to the network that providing the type of coverage.
  network: Vec<Box<Reference>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

}
