#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Contract_Answer::Contract_Answer;
use crate::model::Reference::Reference;
use crate::model::Coding::Coding;
use crate::model::Element::Element;
use crate::model::Period::Period;
use crate::model::Contract_ValuedItem::Contract_ValuedItem;
use crate::model::Contract_Context::Contract_Context;


/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract_Asset {
  /// Time period of asset use.
  #[serde(rename = "usePeriod")]
  use_period: Option<Vec<Period>>,

  /// May be a subtype or part of an offered asset.
  subtype: Option<Vec<CodeableConcept>>,

  /// Target entity type about which the term may be concerned.
  #[serde(rename = "type")]
  fhir_type: Option<Vec<CodeableConcept>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for condition
  #[serde(rename = "_condition")]
  _condition: Option<Element>,

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

  /// Extensions for securityLabelNumber
  #[serde(rename = "_securityLabelNumber")]
  _security_label_number: Option<Vec<Element>>,

  /// Contract Valued Item List.
  #[serde(rename = "valuedItem")]
  valued_item: Option<Vec<Contract_ValuedItem>>,

  /// Security labels that protects the asset.
  #[serde(rename = "securityLabelNumber")]
  security_label_number: Option<Vec<u32>>,

  /// Asset relevant contractual time period.
  period: Option<Vec<Period>>,

  /// Circumstance of the asset.
  context: Option<Vec<Contract_Context>>,

  /// Description of the quality and completeness of the asset that imay be a factor
  /// in its valuation.
  condition: Option<String>,

  /// Extensions for text
  #[serde(rename = "_text")]
  _text: Option<Element>,

  /// Response to assets.
  answer: Option<Vec<Contract_Answer>>,

  /// Associated entities.
  #[serde(rename = "typeReference")]
  type_reference: Option<Vec<Box<Reference>>>,

  /// Specifies the applicability of the term to an asset resource instance, and
  /// instances it refers to orinstances that refer to it, and/or are owned by the
  /// offeree.
  relationship: Option<Coding>,

  /// Differentiates the kind of the asset .
  scope: Option<CodeableConcept>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for linkId
  #[serde(rename = "_linkId")]
  _link_id: Option<Vec<Element>>,

  /// Id [identifier??] of the clause or question text about the asset in the
  /// referenced form or QuestionnaireResponse.
  #[serde(rename = "linkId")]
  link_id: Option<Vec<String>>,

  /// Clause or question text (Prose Object) concerning the asset in a linked form,
  /// such as a QuestionnaireResponse used in the formation of the contract.
  text: Option<String>,

  /// Type of Asset availability for use or ownership.
  #[serde(rename = "periodType")]
  period_type: Option<Vec<CodeableConcept>>,

}
