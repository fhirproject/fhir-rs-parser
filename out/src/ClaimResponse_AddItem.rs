use serde::{Deserialize, Serialize};

/// This resource provides the adjudication details from the processing of a Claim
/// resource.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClaimResponse_AddItem {
  /// A region or surface of the bodySite, e.g. limb region or tooth surface(s).
  #[serde(rename = "subSite")]
  sub_site: Vec<CodeableConcept>,

  /// The numbers associated with notes below which apply to the adjudication of this
  /// item.
  #[serde(rename = "noteNumber")]
  note_number: Vec<positiveInt>,

  /// Extensions for detailSequence
  #[serde(rename = "_detailSequence")]
  _detail_sequence: Vec<Element>,

  /// Item typification or modifiers codes to convey additional context for the
  /// product or service.
  modifier: Vec<CodeableConcept>,

  /// Where the product or service was provided.
  #[serde(rename = "locationReference")]
  location_reference: Reference,

  /// When the value is a group code then this item collects a set of related claim
  /// details, otherwise this contains the product, service, drug or other billing
  /// code for the item.
  #[serde(rename = "productOrService")]
  product_or_service: CodeableConcept,

  /// The sequence number of the details within the claim item which this line is
  /// intended to replace.
  #[serde(rename = "detailSequence")]
  detail_sequence: Vec<positiveInt>,

  /// If the item is not a group then this is the fee for the product or service,
  /// otherwise this is the total of the fees for the details of the group.
  #[serde(rename = "unitPrice")]
  unit_price: Money,

  /// Where the product or service was provided.
  #[serde(rename = "locationAddress")]
  location_address: Address,

  /// Identifies the program under which this may be recovered.
  #[serde(rename = "programCode")]
  program_code: Vec<CodeableConcept>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The quantity times the unit price for an additional service or product or
  /// charge.
  net: Money,

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

  /// Extensions for itemSequence
  #[serde(rename = "_itemSequence")]
  _item_sequence: Vec<Element>,

  /// The providers who are authorized for the services rendered to the patient.
  provider: Vec<Reference>,

  /// Extensions for subdetailSequence
  #[serde(rename = "_subdetailSequence")]
  _subdetail_sequence: Vec<Element>,

  /// The number of repetitions of a service or product.
  quantity: Quantity,

  /// Extensions for factor
  _factor: Element,

  /// The second-tier service adjudications for payor added services.
  detail: Vec<ClaimResponse_Detail1>,

  /// Where the product or service was provided.
  #[serde(rename = "locationCodeableConcept")]
  location_codeable_concept: CodeableConcept,

  /// The date or dates when the service or product was supplied, performed or
  /// completed.
  #[serde(rename = "servicedDate")]
  serviced_date: String,

  /// Physical service site on the patient (limb, tooth, etc.).
  #[serde(rename = "bodySite")]
  body_site: CodeableConcept,

  /// Claim items which this service line is intended to replace.
  #[serde(rename = "itemSequence")]
  item_sequence: Vec<positiveInt>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// The date or dates when the service or product was supplied, performed or
  /// completed.
  #[serde(rename = "servicedPeriod")]
  serviced_period: Period,

  /// Extensions for noteNumber
  #[serde(rename = "_noteNumber")]
  _note_number: Vec<Element>,

  /// Extensions for servicedDate
  #[serde(rename = "_servicedDate")]
  _serviced_date: Element,

  /// The adjudication results.
  adjudication: Vec<ClaimResponse_Adjudication>,

  /// A real number that represents a multiplier used in determining the overall value
  /// of services delivered and/or goods received. The concept of a Factor allows for
  /// a discount or surcharge multiplier to be applied to a monetary amount.
  factor: decimal,

  /// The sequence number of the sub-details within the details within the claim item
  /// which this line is intended to replace.
  #[serde(rename = "subdetailSequence")]
  subdetail_sequence: Vec<positiveInt>,

}
