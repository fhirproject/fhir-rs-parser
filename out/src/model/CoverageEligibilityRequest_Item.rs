#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Money::Money;
use crate::model::CoverageEligibilityRequest_Diagnosis::CoverageEligibilityRequest_Diagnosis;
use crate::model::Reference::Reference;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Quantity::Quantity;


/// The CoverageEligibilityRequest provides patient and insurance coverage
/// information to an insurer for them to respond, in the form of an
/// CoverageEligibilityResponse, with information regarding whether the stated
/// coverage is valid and in-force and optionally to provide the insurance details
/// of the policy.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityRequest_Item {
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

  /// Extensions for supportingInfoSequence
  #[serde(rename = "_supportingInfoSequence")]
  _supporting_info_sequence: Option<Vec<Element>>,

  /// The amount charged to the patient by the provider for a single unit.
  #[serde(rename = "unitPrice")]
  unit_price: Option<Money>,

  /// The plan/proposal/order describing the proposed service in detail.
  detail: Option<Vec<Box<Reference>>>,

  /// Exceptions, special conditions and supporting information applicable for this
  /// service or product line.
  #[serde(rename = "supportingInfoSequence")]
  supporting_info_sequence: Option<Vec<i32>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The practitioner who is responsible for the product or service to be rendered to
  /// the patient.
  provider: Option<Box<Reference>>,

  /// Facility where the services will be provided.
  facility: Option<Box<Reference>>,

  /// Patient diagnosis for which care is sought.
  diagnosis: Option<Vec<CoverageEligibilityRequest_Diagnosis>>,

  /// The number of repetitions of a service or product.
  quantity: Option<Quantity>,

  /// This contains the product, service, drug or other billing code for the item.
  #[serde(rename = "productOrService")]
  product_or_service: Option<CodeableConcept>,

  /// Code to identify the general type of benefits under which products and services
  /// are provided.
  category: Option<CodeableConcept>,

  /// Item typification or modifiers codes to convey additional context for the
  /// product or service.
  modifier: Option<Vec<CodeableConcept>>,

}
