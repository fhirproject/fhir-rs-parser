#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Money::Money;
use crate::model::Element::Element;
use crate::model::Address::Address;
use crate::model::ExplanationOfBenefit_Adjudication::ExplanationOfBenefit_Adjudication;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Reference::Reference;
use crate::model::ExplanationOfBenefit_Detail::ExplanationOfBenefit_Detail;
use crate::model::Quantity::Quantity;


/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefit_Item {
  /// If this item is a group then the values here are a summary of the adjudication
  /// of the detail items. If this item is a simple product or service then this is
  /// the result of the adjudication of this item.
  adjudication: Option<Vec<ExplanationOfBenefit_Adjudication>>,

  /// Physical service site on the patient (limb, tooth, etc.).
  #[serde(rename = "bodySite")]
  body_site: Option<CodeableConcept>,

  /// Care team members related to this service or product.
  #[serde(rename = "careTeamSequence")]
  care_team_sequence: Option<Vec<i32>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for procedureSequence
  #[serde(rename = "_procedureSequence")]
  _procedure_sequence: Option<Vec<Element>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// When the value is a group code then this item collects a set of related claim
  /// details, otherwise this contains the product, service, drug or other billing
  /// code for the item.
  #[serde(rename = "productOrService")]
  product_or_service: CodeableConcept,

  /// Item typification or modifiers codes to convey additional context for the
  /// product or service.
  modifier: Option<Vec<CodeableConcept>>,

  /// Where the product or service was provided.
  #[serde(rename = "locationReference")]
  location_reference: Option<Box<Reference>>,

  /// Unique Device Identifiers associated with this line item.
  udi: Option<Vec<Box<Reference>>>,

  /// Exceptions, special conditions and supporting information applicable for this
  /// service or product.
  #[serde(rename = "informationSequence")]
  information_sequence: Option<Vec<i32>>,

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

  /// Code to identify the general type of benefits under which products and services
  /// are provided.
  category: Option<CodeableConcept>,

  /// The date or dates when the service or product was supplied, performed or
  /// completed.
  #[serde(rename = "servicedPeriod")]
  serviced_period: Option<Period>,

  /// The date or dates when the service or product was supplied, performed or
  /// completed.
  #[serde(rename = "servicedDate")]
  serviced_date: Option<String>,

  /// The quantity times the unit price for an additional service or product or
  /// charge.
  net: Option<Money>,

  /// A number to uniquely identify item entries.
  sequence: Option<i32>,

  /// Diagnoses applicable for this service or product.
  #[serde(rename = "diagnosisSequence")]
  diagnosis_sequence: Option<Vec<i32>>,

  /// Extensions for sequence
  #[serde(rename = "_sequence")]
  _sequence: Option<Element>,

  /// Extensions for diagnosisSequence
  #[serde(rename = "_diagnosisSequence")]
  _diagnosis_sequence: Option<Vec<Element>>,

  /// The type of revenue or cost center providing the product and/or service.
  revenue: Option<CodeableConcept>,

  /// Identifies the program under which this may be recovered.
  #[serde(rename = "programCode")]
  program_code: Option<Vec<CodeableConcept>>,

  /// Where the product or service was provided.
  #[serde(rename = "locationCodeableConcept")]
  location_codeable_concept: Option<CodeableConcept>,

  /// A real number that represents a multiplier used in determining the overall value
  /// of services delivered and/or goods received. The concept of a Factor allows for
  /// a discount or surcharge multiplier to be applied to a monetary amount.
  factor: Option<f32>,

  /// Second-tier of goods and services.
  detail: Option<Vec<ExplanationOfBenefit_Detail>>,

  /// The number of repetitions of a service or product.
  quantity: Option<Quantity>,

  /// Where the product or service was provided.
  #[serde(rename = "locationAddress")]
  location_address: Option<Address>,

  /// A billed item may include goods or services provided in multiple encounters.
  encounter: Option<Vec<Box<Reference>>>,

  /// Extensions for servicedDate
  #[serde(rename = "_servicedDate")]
  _serviced_date: Option<Element>,

  /// Procedures applicable for this service or product.
  #[serde(rename = "procedureSequence")]
  procedure_sequence: Option<Vec<i32>>,

  /// Extensions for noteNumber
  #[serde(rename = "_noteNumber")]
  _note_number: Option<Vec<Element>>,

  /// The numbers associated with notes below which apply to the adjudication of this
  /// item.
  #[serde(rename = "noteNumber")]
  note_number: Option<Vec<i32>>,

  /// Extensions for informationSequence
  #[serde(rename = "_informationSequence")]
  _information_sequence: Option<Vec<Element>>,

  /// A region or surface of the bodySite, e.g. limb region or tooth surface(s).
  #[serde(rename = "subSite")]
  sub_site: Option<Vec<CodeableConcept>>,

  /// Extensions for factor
  #[serde(rename = "_factor")]
  _factor: Option<Element>,

  /// If the item is not a group then this is the fee for the product or service,
  /// otherwise this is the total of the fees for the details of the group.
  #[serde(rename = "unitPrice")]
  unit_price: Option<Money>,

  /// Extensions for careTeamSequence
  #[serde(rename = "_careTeamSequence")]
  _care_team_sequence: Option<Vec<Element>>,

}
