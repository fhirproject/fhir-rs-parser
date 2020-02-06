#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Contract_Asset::Contract_Asset;
use crate::model::Contract_Action::Contract_Action;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::Period::Period;
use crate::model::Element::Element;
use crate::model::Contract_Offer::Contract_Offer;
use crate::model::Contract_SecurityLabel::Contract_SecurityLabel;
use crate::model::Identifier::Identifier;


/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract_Term {
  /// When this Contract Provision was issued.
  issued: Option<String>,

  /// Extensions for issued
  #[serde(rename = "_issued")]
  _issued: Option<Element>,

  /// The matter of concern in the context of this provision of the agrement.
  offer: Contract_Offer,

  /// Relevant time or time-period when this Contract Provision is applicable.
  applies: Option<Period>,

  /// An actor taking a role in an activity for which it can be assigned some degree
  /// of responsibility for the activity taking place.
  action: Option<Vec<Contract_Action>>,

  /// A specialized legal clause or condition based on overarching contract type.
  #[serde(rename = "subType")]
  sub_type: Option<CodeableConcept>,

  /// Security labels that protect the handling of information about the term and its
  /// elements, which may be specifically identified..
  #[serde(rename = "securityLabel")]
  security_label: Option<Vec<Contract_SecurityLabel>>,

  /// Nested group of Contract Provisions.
  group: Option<Vec<Contract_Term>>,

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

  /// Contract Term Asset List.
  asset: Option<Vec<Contract_Asset>>,

  /// Extensions for text
  #[serde(rename = "_text")]
  _text: Option<Element>,

  /// The entity that the term applies to.
  #[serde(rename = "topicReference")]
  topic_reference: Option<Box<Reference>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// A legal clause or condition contained within a contract that requires one or
  /// both parties to perform a particular requirement by some specified time or
  /// prevents one or both parties from performing a particular requirement by some
  /// specified time.
  #[serde(rename = "type")]
  fhir_type: Option<CodeableConcept>,

  /// Unique identifier for this particular Contract Provision.
  identifier: Option<Identifier>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Statement of a provision in a policy or a contract.
  text: Option<String>,

  /// The entity that the term applies to.
  #[serde(rename = "topicCodeableConcept")]
  topic_codeable_concept: Option<CodeableConcept>,

}
