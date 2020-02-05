use serde::{Deserialize, Serialize};

/// Details of a Health Insurance product/plan provided by an organization.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InsurancePlan_GeneralCost {
  /// Additional information about the general costs associated with this plan.
  comment: String,

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

  /// Extensions for groupSize
  #[serde(rename = "_groupSize")]
  _group_size: Element,

  /// Type of cost.
  type: CodeableConcept,

  /// Extensions for comment
  _comment: Element,

  /// Value of the cost.
  cost: Money,

  /// Number of participants enrolled in the plan.
  #[serde(rename = "groupSize")]
  group_size: positiveInt,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

}
