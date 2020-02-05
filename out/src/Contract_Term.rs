use serde::{Deserialize, Serialize};

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Contract_Term {
  /// The entity that the term applies to.
  #[serde(rename = "topicCodeableConcept")]
  topic_codeable_concept: CodeableConcept,

  /// Unique identifier for this particular Contract Provision.
  identifier: Identifier,

  /// When this Contract Provision was issued.
  issued: dateTime,

  /// Statement of a provision in a policy or a contract.
  text: String,

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

  /// The entity that the term applies to.
  #[serde(rename = "topicReference")]
  topic_reference: Reference,

  /// Extensions for issued
  _issued: Element,

  /// A legal clause or condition contained within a contract that requires one or
  /// both parties to perform a particular requirement by some specified time or
  /// prevents one or both parties from performing a particular requirement by some
  /// specified time.
  type: CodeableConcept,

  /// A specialized legal clause or condition based on overarching contract type.
  #[serde(rename = "subType")]
  sub_type: CodeableConcept,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Contract Term Asset List.
  asset: Vec<Contract_Asset>,

  /// Security labels that protect the handling of information about the term and its
  /// elements, which may be specifically identified..
  #[serde(rename = "securityLabel")]
  security_label: Vec<Contract_SecurityLabel>,

  /// The matter of concern in the context of this provision of the agrement.
  offer: Contract_Offer,

  /// Relevant time or time-period when this Contract Provision is applicable.
  applies: Period,

  /// Nested group of Contract Provisions.
  group: Vec<Contract_Term>,

  /// Extensions for text
  _text: Element,

  /// An actor taking a role in an activity for which it can be assigned some degree
  /// of responsibility for the activity taking place.
  action: Vec<Contract_Action>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

}
