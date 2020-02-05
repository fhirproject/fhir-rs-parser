use serde::{Deserialize, Serialize};

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Contract_Asset {
  /// Specifies the applicability of the term to an asset resource instance, and
  /// instances it refers to orinstances that refer to it, and/or are owned by the
  /// offeree.
  relationship: Coding,

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

  /// Target entity type about which the term may be concerned.
  type: Vec<CodeableConcept>,

  /// Description of the quality and completeness of the asset that imay be a factor
  /// in its valuation.
  condition: String,

  /// Clause or question text (Prose Object) concerning the asset in a linked form,
  /// such as a QuestionnaireResponse used in the formation of the contract.
  text: String,

  /// Extensions for linkId
  #[serde(rename = "_linkId")]
  _link_id: Vec<Element>,

  /// Contract Valued Item List.
  #[serde(rename = "valuedItem")]
  valued_item: Vec<Contract_ValuedItem>,

  /// May be a subtype or part of an offered asset.
  subtype: Vec<CodeableConcept>,

  /// Type of Asset availability for use or ownership.
  #[serde(rename = "periodType")]
  period_type: Vec<CodeableConcept>,

  /// Id [identifier??] of the clause or question text about the asset in the
  /// referenced form or QuestionnaireResponse.
  #[serde(rename = "linkId")]
  link_id: Vec<String>,

  /// Time period of asset use.
  #[serde(rename = "usePeriod")]
  use_period: Vec<Period>,

  /// Extensions for securityLabelNumber
  #[serde(rename = "_securityLabelNumber")]
  _security_label_number: Vec<Element>,

  /// Asset relevant contractual time period.
  period: Vec<Period>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for text
  _text: Element,

  /// Response to assets.
  answer: Vec<Contract_Answer>,

  /// Circumstance of the asset.
  context: Vec<Contract_Context>,

  /// Associated entities.
  #[serde(rename = "typeReference")]
  type_reference: Vec<Reference>,

  /// Differentiates the kind of the asset .
  scope: CodeableConcept,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for condition
  _condition: Element,

  /// Security labels that protects the asset.
  #[serde(rename = "securityLabelNumber")]
  security_label_number: Vec<unsignedInt>,

}
