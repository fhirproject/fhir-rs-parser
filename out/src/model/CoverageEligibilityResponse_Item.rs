#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::CoverageEligibilityResponse_Benefit::CoverageEligibilityResponse_Benefit;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::CodeableConcept::CodeableConcept;


/// This resource provides eligibility and plan details from the processing of an
/// CoverageEligibilityRequest resource.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityResponse_Item {
  /// True if the indicated class of service is excluded from the plan, missing or
  /// False indicates the product or service is included in the coverage.
  excluded: Option<bool>,

  /// Indicates if the benefits apply to an individual or to the family.
  unit: Option<CodeableConcept>,

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

  /// The term or period of the values such as 'maximum lifetime benefit' or 'maximum
  /// annual visits'.
  term: Option<CodeableConcept>,

  /// Benefits used to date.
  benefit: Option<Vec<CoverageEligibilityResponse_Benefit>>,

  /// Item typification or modifiers codes to convey additional context for the
  /// product or service.
  modifier: Option<Vec<CodeableConcept>>,

  /// A short name or tag for the benefit.
  name: Option<String>,

  /// A boolean flag indicating whether a preauthorization is required prior to actual
  /// service delivery.
  #[serde(rename = "authorizationRequired")]
  authorization_required: Option<bool>,

  /// Extensions for authorizationRequired
  #[serde(rename = "_authorizationRequired")]
  _authorization_required: Option<Element>,

  /// A web location for obtaining requirements or descriptive information regarding
  /// the preauthorization.
  #[serde(rename = "authorizationUrl")]
  authorization_url: Option<String>,

  /// Extensions for authorizationUrl
  #[serde(rename = "_authorizationUrl")]
  _authorization_url: Option<Element>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// Code to identify the general type of benefits under which products and services
  /// are provided.
  category: Option<CodeableConcept>,

  /// Extensions for excluded
  #[serde(rename = "_excluded")]
  _excluded: Option<Element>,

  /// This contains the product, service, drug or other billing code for the item.
  #[serde(rename = "productOrService")]
  product_or_service: Option<CodeableConcept>,

  /// A richer description of the benefit or services covered.
  description: Option<String>,

  /// Codes or comments regarding information or actions associated with the
  /// preauthorization.
  #[serde(rename = "authorizationSupporting")]
  authorization_supporting: Option<Vec<CodeableConcept>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The practitioner who is eligible for the provision of the product or service.
  provider: Option<Box<Reference>>,

  /// Is a flag to indicate whether the benefits refer to in-network providers or out-
  /// of-network providers.
  network: Option<CodeableConcept>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

}
