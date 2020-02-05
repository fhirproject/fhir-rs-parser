use serde::{Deserialize, Serialize};

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Contract_Offer {
  /// Security labels that protects the offer.
  #[serde(rename = "securityLabelNumber")]
  security_label_number: Vec<unsignedInt>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Type of Contract Provision such as specific requirements, purposes for actions,
  /// obligations, prohibitions, e.g. life time maximum benefit.
  type: CodeableConcept,

  /// Extensions for text
  _text: Element,

  /// Unique identifier for this particular Contract Provision.
  identifier: Vec<Identifier>,

  /// The owner of an asset has the residual control rights over the asset: the right
  /// to decide all usages of the asset in any way not inconsistent with a prior
  /// contract, custom, or law (Hart, 1995, p. 30).
  topic: Reference,

  /// How the decision about a Contract was conveyed.
  #[serde(rename = "decisionMode")]
  decision_mode: Vec<CodeableConcept>,

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

  /// Response to offer text.
  answer: Vec<Contract_Answer>,

  /// Offer Recipient.
  party: Vec<Contract_Party>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for securityLabelNumber
  #[serde(rename = "_securityLabelNumber")]
  _security_label_number: Vec<Element>,

  /// The id of the clause or question text of the offer in the referenced
  /// questionnaire/response.
  #[serde(rename = "linkId")]
  link_id: Vec<String>,

  /// Human readable form of this Contract Offer.
  text: String,

  /// Type of choice made by accepting party with respect to an offer made by an
  /// offeror/ grantee.
  decision: CodeableConcept,

  /// Extensions for linkId
  #[serde(rename = "_linkId")]
  _link_id: Vec<Element>,

}
