use serde::{Deserialize, Serialize};

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExplanationOfBenefit_SubDetail {
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
  /// resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// Code to identify the general type of benefits under which products and services
  /// are provided.
  category: CodeableConcept,

  /// When the value is a group code then this item collects a set of related claim
  /// details, otherwise this contains the product, service, drug or other billing
  /// code for the item.
  #[serde(rename = "productOrService")]
  product_or_service: CodeableConcept,

  /// Extensions for factor
  _factor: Element,

  /// Identifies the program under which this may be recovered.
  #[serde(rename = "programCode")]
  program_code: Vec<CodeableConcept>,

  /// The quantity times the unit price for an additional service or product or
  /// charge.
  net: Money,

  /// If the item is not a group then this is the fee for the product or service,
  /// otherwise this is the total of the fees for the details of the group.
  #[serde(rename = "unitPrice")]
  unit_price: Money,

  /// A real number that represents a multiplier used in determining the overall value
  /// of services delivered and/or goods received. The concept of a Factor allows for
  /// a discount or surcharge multiplier to be applied to a monetary amount.
  factor: decimal,

  /// The type of revenue or cost center providing the product and/or service.
  revenue: CodeableConcept,

  /// The numbers associated with notes below which apply to the adjudication of this
  /// item.
  #[serde(rename = "noteNumber")]
  note_number: Vec<positiveInt>,

  /// Extensions for sequence
  _sequence: Element,

  /// The adjudication results.
  adjudication: Vec<ExplanationOfBenefit_Adjudication>,

  /// Extensions for noteNumber
  #[serde(rename = "_noteNumber")]
  _note_number: Vec<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// A claim detail line. Either a simple (a product or service) or a 'group' of sub-
  /// details which are simple items.
  sequence: positiveInt,

  /// Item typification or modifiers codes to convey additional context for the
  /// product or service.
  modifier: Vec<CodeableConcept>,

  /// The number of repetitions of a service or product.
  quantity: Quantity,

  /// Unique Device Identifiers associated with this line item.
  udi: Vec<Reference>,

}
