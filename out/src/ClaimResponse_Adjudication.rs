use serde::{Deserialize, Serialize};

/// This resource provides the adjudication details from the processing of a Claim
/// resource.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClaimResponse_Adjudication {
  /// A non-monetary value associated with the category. Mutually exclusive to the
  /// amount element above.
  value: decimal,

  /// Monetary amount associated with the category.
  amount: Money,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// A code supporting the understanding of the adjudication result and explaining
  /// variance from expected amount.
  reason: CodeableConcept,

  /// Extensions for value
  _value: Element,

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

  /// A code to indicate the information type of this adjudication record. Information
  /// types may include the value submitted, maximum values or percentages allowed or
  /// payable under the plan, amounts that: the patient is responsible for in
  /// aggregate or pertaining to this item; amounts paid by other coverages; and, the
  /// benefit payable for this item.
  category: CodeableConcept,

}
