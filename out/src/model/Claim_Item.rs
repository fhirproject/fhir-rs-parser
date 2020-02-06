#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Quantity::Quantity;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::Money::Money;
use crate::model::Period::Period;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Claim_Detail::Claim_Detail;
use crate::model::Address::Address;


/// A provider issued list of professional services and products which have been
/// provided, or are to be provided, to a patient which is sent to an insurer for
/// reimbursement.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claim_Item {
  /// The Encounters during which this Claim was created or to which the creation of
  /// this record is tightly associated.
  encounter: Vec<Box<Reference>>,

  /// The date or dates when the service or product was supplied, performed or
  /// completed.
  #[serde(rename = "servicedPeriod")]
  serviced_period: Period,

  /// Physical service site on the patient (limb, tooth, etc.).
  #[serde(rename = "bodySite")]
  body_site: CodeableConcept,

  /// Extensions for factor
  _factor: Element,

  /// Procedures applicable for this service or product.
  #[serde(rename = "procedureSequence")]
  procedure_sequence: Vec<i32>,

  /// Identifies the program under which this may be recovered.
  #[serde(rename = "programCode")]
  program_code: Vec<CodeableConcept>,

  /// CareTeam members related to this service or product.
  #[serde(rename = "careTeamSequence")]
  care_team_sequence: Vec<i32>,

  /// The quantity times the unit price for an additional service or product or
  /// charge.
  net: Money,

  /// Code to identify the general type of benefits under which products and services
  /// are provided.
  category: CodeableConcept,

  /// The type of revenue or cost center providing the product and/or service.
  revenue: CodeableConcept,

  /// Unique Device Identifiers associated with this line item.
  udi: Vec<Box<Reference>>,

  /// A claim detail line. Either a simple (a product or service) or a 'group' of sub-
  /// details which are simple items.
  detail: Vec<Claim_Detail>,

  /// A real number that represents a multiplier used in determining the overall value
  /// of services delivered and/or goods received. The concept of a Factor allows for
  /// a discount or surcharge multiplier to be applied to a monetary amount.
  factor: f32,

  /// Extensions for diagnosisSequence
  #[serde(rename = "_diagnosisSequence")]
  _diagnosis_sequence: Vec<Element>,

  /// Extensions for sequence
  _sequence: Element,

  /// When the value is a group code then this item collects a set of related claim
  /// details, otherwise this contains the product, service, drug or other billing
  /// code for the item.
  #[serde(rename = "productOrService")]
  product_or_service: CodeableConcept,

  /// If the item is not a group then this is the fee for the product or service,
  /// otherwise this is the total of the fees for the details of the group.
  #[serde(rename = "unitPrice")]
  unit_price: Money,

  /// A number to uniquely identify item entries.
  sequence: i32,

  /// Extensions for informationSequence
  #[serde(rename = "_informationSequence")]
  _information_sequence: Vec<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for servicedDate
  #[serde(rename = "_servicedDate")]
  _serviced_date: Element,

  /// Extensions for careTeamSequence
  #[serde(rename = "_careTeamSequence")]
  _care_team_sequence: Vec<Element>,

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

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Diagnosis applicable for this service or product.
  #[serde(rename = "diagnosisSequence")]
  diagnosis_sequence: Vec<i32>,

  /// Item typification or modifiers codes to convey additional context for the
  /// product or service.
  modifier: Vec<CodeableConcept>,

  /// Extensions for procedureSequence
  #[serde(rename = "_procedureSequence")]
  _procedure_sequence: Vec<Element>,

  /// Exceptions, special conditions and supporting information applicable for this
  /// service or product.
  #[serde(rename = "informationSequence")]
  information_sequence: Vec<i32>,

  /// The date or dates when the service or product was supplied, performed or
  /// completed.
  #[serde(rename = "servicedDate")]
  serviced_date: String,

  /// Where the product or service was provided.
  #[serde(rename = "locationCodeableConcept")]
  location_codeable_concept: CodeableConcept,

  /// Where the product or service was provided.
  #[serde(rename = "locationAddress")]
  location_address: Address,

  /// The number of repetitions of a service or product.
  quantity: Quantity,

  /// Where the product or service was provided.
  #[serde(rename = "locationReference")]
  location_reference: Box<Reference>,

  /// A region or surface of the bodySite, e.g. limb region or tooth surface(s).
  #[serde(rename = "subSite")]
  sub_site: Vec<CodeableConcept>,

}
