#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Reference::Reference;
use crate::model::Extension::Extension;
use crate::model::InsurancePlan_SpecificCost::InsurancePlan_SpecificCost;
use crate::model::InsurancePlan_GeneralCost::InsurancePlan_GeneralCost;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Identifier::Identifier;


/// Details of a Health Insurance product/plan provided by an organization.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlan_Plan {
  /// Overall costs associated with the plan.
  #[serde(rename = "generalCost")]
  general_cost: Option<Vec<InsurancePlan_GeneralCost>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Business identifiers assigned to this health insurance plan which remain
  /// constant as the resource is updated and propagates from server to server.
  identifier: Option<Vec<Identifier>>,

  /// The geographic region in which a health insurance plan's benefits apply.
  #[serde(rename = "coverageArea")]
  coverage_area: Option<Vec<Box<Reference>>>,

  /// Costs associated with the coverage provided by the product.
  #[serde(rename = "specificCost")]
  specific_cost: Option<Vec<InsurancePlan_SpecificCost>>,

  /// Type of plan. For example, "Platinum" or "High Deductable".
  #[serde(rename = "type")]
  fhir_type: Option<CodeableConcept>,

  /// Reference to the network that providing the type of coverage.
  network: Option<Vec<Box<Reference>>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

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
