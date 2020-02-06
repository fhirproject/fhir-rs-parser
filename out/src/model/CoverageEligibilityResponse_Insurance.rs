#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Reference::Reference;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::CoverageEligibilityResponse_Item::CoverageEligibilityResponse_Item;
use crate::model::Period::Period;


/// This resource provides eligibility and plan details from the processing of an
/// CoverageEligibilityRequest resource.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityResponse_Insurance {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

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

  /// Extensions for inforce
  #[serde(rename = "_inforce")]
  _inforce: Option<Element>,

  /// Reference to the insurance card level information contained in the Coverage
  /// resource. The coverage issuing insurer will use these details to locate the
  /// patient's actual coverage within the insurer's information system.
  coverage: Box<Reference>,

  /// Benefits and optionally current balances, and authorization details by category
  /// or service.
  item: Option<Vec<CoverageEligibilityResponse_Item>>,

  /// The term of the benefits documented in this response.
  #[serde(rename = "benefitPeriod")]
  benefit_period: Option<Period>,

  /// Flag indicating if the coverage provided is inforce currently if no service
  /// date(s) specified or for the whole duration of the service dates.
  inforce: Option<bool>,

}
