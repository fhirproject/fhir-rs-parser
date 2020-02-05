use serde::{Deserialize, Serialize};

/// Todo.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SubstanceReferenceInformation_Target {
  /// Todo.
  #[serde(rename = "amountString")]
  amount_string: String,

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

  /// Todo.
  interaction: CodeableConcept,

  /// Todo.
  target: Identifier,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Todo.
  #[serde(rename = "organismType")]
  organism_type: CodeableConcept,

  /// Todo.
  #[serde(rename = "amountRange")]
  amount_range: Range,

  /// Todo.
  #[serde(rename = "amountQuantity")]
  amount_quantity: Quantity,

  /// Todo.
  #[serde(rename = "amountType")]
  amount_type: CodeableConcept,

  /// Extensions for amountString
  #[serde(rename = "_amountString")]
  _amount_string: Element,

  /// Todo.
  organism: CodeableConcept,

  /// Todo.
  type: CodeableConcept,

  /// Todo.
  source: Vec<Reference>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

}
