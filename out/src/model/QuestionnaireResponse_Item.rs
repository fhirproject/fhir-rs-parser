#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::QuestionnaireResponse_Answer::QuestionnaireResponse_Answer;
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// A structured set of questions and their answers. The questions are ordered and
/// grouped into coherent subsets, corresponding to the structure of the grouping of
/// the questionnaire being responded to.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionnaireResponse_Item {
  /// Extensions for linkId
  #[serde(rename = "_linkId")]
  _link_id: Option<Element>,

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

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// The item from the Questionnaire that corresponds to this item in the
  /// QuestionnaireResponse resource.
  #[serde(rename = "linkId")]
  link_id: Option<String>,

  /// Text that is displayed above the contents of the group or as the text of the
  /// question being answered.
  text: Option<String>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for text
  #[serde(rename = "_text")]
  _text: Option<Element>,

  /// Extensions for definition
  #[serde(rename = "_definition")]
  _definition: Option<Element>,

  /// Questions or sub-groups nested beneath a question or group.
  item: Option<Vec<QuestionnaireResponse_Item>>,

  /// The respondent's answer(s) to the question.
  answer: Option<Vec<QuestionnaireResponse_Answer>>,

  /// A reference to an [[[ElementDefinition]]] that provides the details for the
  /// item.
  definition: Option<String>,

}
