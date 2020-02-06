#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Address::Address;
use crate::model::Period::Period;
use crate::model::Claim_Detail::Claim_Detail;
use crate::model::Quantity::Quantity;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Money::Money;
use crate::model::Reference::Reference;


/// A provider issued list of professional services and products which have been
/// provided, or are to be provided, to a patient which is sent to an insurer for
/// reimbursement.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claim_Item {
  /// If the item is not a group then this is the fee for the product or service,
  /// otherwise this is the total of the fees for the details of the group.
  #[serde(rename = "unitPrice")]
  unit_price: Option<Money>,

  /// Exceptions, special conditions and supporting information applicable for this
  /// service or product.
  #[serde(rename = "informationSequence")]
  information_sequence: Option<Vec<i32>>,

  /// The date or dates when the service or product was supplied, performed or
  /// completed.
  #[serde(rename = "servicedDate")]
  serviced_date: Option<String>,

  /// The number of repetitions of a service or product.
  quantity: Option<Quantity>,

  /// Extensions for informationSequence
  #[serde(rename = "_informationSequence")]
  _information_sequence: Option<Vec<Element>>,

  /// Extensions for diagnosisSequence
  #[serde(rename = "_diagnosisSequence")]
  _diagnosis_sequence: Option<Vec<Element>>,

  /// Where the product or service was provided.
  #[serde(rename = "locationReference")]
  location_reference: Option<Box<Reference>>,

  /// The date or dates when the service or product was supplied, performed or
  /// completed.
  #[serde(rename = "servicedPeriod")]
  serviced_period: Option<Period>,

  /// When the value is a group code then this item collects a set of related claim
  /// details, otherwise this contains the product, service, drug or other billing
  /// code for the item.
  #[serde(rename = "productOrService")]
  product_or_service: CodeableConcept,

  /// Procedures applicable for this service or product.
  #[serde(rename = "procedureSequence")]
  procedure_sequence: Option<Vec<i32>>,

  /// The type of revenue or cost center providing the product and/or service.
  revenue: Option<CodeableConcept>,

  /// The Encounters during which this Claim was created or to which the creation of
  /// this record is tightly associated.
  encounter: Option<Vec<Box<Reference>>>,

  /// Extensions for procedureSequence
  #[serde(rename = "_procedureSequence")]
  _procedure_sequence: Option<Vec<Element>>,

  /// A number to uniquely identify item entries.
  sequence: Option<i32>,

  /// The quantity times the unit price for an additional service or product or
  /// charge.
  net: Option<Money>,

  /// Diagnosis applicable for this service or product.
  #[serde(rename = "diagnosisSequence")]
  diagnosis_sequence: Option<Vec<i32>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Unique Device Identifiers associated with this line item.
  udi: Option<Vec<Box<Reference>>>,

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

  /// A real number that represents a multiplier used in determining the overall value
  /// of services delivered and/or goods received. The concept of a Factor allows for
  /// a discount or surcharge multiplier to be applied to a monetary amount.
  factor: Option<f32>,

  /// A region or surface of the bodySite, e.g. limb region or tooth surface(s).
  #[serde(rename = "subSite")]
  sub_site: Option<Vec<CodeableConcept>>,

  /// Where the product or service was provided.
  #[serde(rename = "locationCodeableConcept")]
  location_codeable_concept: Option<CodeableConcept>,

  /// Physical service site on the patient (limb, tooth, etc.).
  #[serde(rename = "bodySite")]
  body_site: Option<CodeableConcept>,

  /// A claim detail line. Either a simple (a product or service) or a 'group' of sub-
  /// details which are simple items.
  detail: Option<Vec<Claim_Detail>>,

  /// Extensions for sequence
  #[serde(rename = "_sequence")]
  _sequence: Option<Element>,

  /// Extensions for careTeamSequence
  #[serde(rename = "_careTeamSequence")]
  _care_team_sequence: Option<Vec<Element>>,

  /// Where the product or service was provided.
  #[serde(rename = "locationAddress")]
  location_address: Option<Address>,

  /// CareTeam members related to this service or product.
  #[serde(rename = "careTeamSequence")]
  care_team_sequence: Option<Vec<i32>>,

  /// Identifies the program under which this may be recovered.
  #[serde(rename = "programCode")]
  program_code: Option<Vec<CodeableConcept>>,

  /// Extensions for factor
  #[serde(rename = "_factor")]
  _factor: Option<Element>,

  /// Code to identify the general type of benefits under which products and services
  /// are provided.
  category: Option<CodeableConcept>,

  /// Item typification or modifiers codes to convey additional context for the
  /// product or service.
  modifier: Option<Vec<CodeableConcept>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for servicedDate
  #[serde(rename = "_servicedDate")]
  _serviced_date: Option<Element>,

}
