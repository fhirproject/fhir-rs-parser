#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Questionnaire_Initial::Questionnaire_Initial;
use crate::model::Element::Element;
use crate::model::Coding::Coding;
use crate::model::Extension::Extension;
use crate::model::Questionnaire_EnableWhen::Questionnaire_EnableWhen;
use crate::model::Questionnaire_AnswerOption::Questionnaire_AnswerOption;


/// A structured set of questions intended to guide the collection of answers from
/// end-users. Questionnaires provide detailed control over order, presentation,
/// phraseology and grouping to allow coherent, consistent data collection.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Questionnaire_Item {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Controls how multiple enableWhen values are interpreted -  whether all or any
  /// must be true.
  #[serde(rename = "enableBehavior")]
  enable_behavior: Option<Questionnaire_ItemEnableBehavior>,

  /// A short label for a particular group, question or set of display text within the
  /// questionnaire used for reference by the individual completing the questionnaire.
  prefix: Option<String>,

  /// Extensions for definition
  #[serde(rename = "_definition")]
  _definition: Option<Element>,

  /// An indication, if true, that the item may occur multiple times in the response,
  /// collecting multiple answers for questions or multiple sets of answers for
  /// groups.
  repeats: Option<bool>,

  /// Extensions for readOnly
  #[serde(rename = "_readOnly")]
  _read_only: Option<Element>,

  /// An identifier that is unique within the Questionnaire allowing linkage to the
  /// equivalent item in a QuestionnaireResponse resource.
  #[serde(rename = "linkId")]
  link_id: Option<String>,

  /// One of the permitted answers for a "choice" or "open-choice" question.
  #[serde(rename = "answerOption")]
  answer_option: Option<Vec<Questionnaire_AnswerOption>>,

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
  modifier_extension: Option<Vec<Extension>>,

  /// Text, questions and other groups to be nested beneath a question or group.
  item: Option<Vec<Questionnaire_Item>>,

  /// Extensions for text
  #[serde(rename = "_text")]
  _text: Option<Element>,

  /// A reference to a value set containing a list of codes representing permitted
  /// answers for a "choice" or "open-choice" question.
  #[serde(rename = "answerValueSet")]
  answer_value_set: Option<String>,

  /// An indication, if true, that the item must be present in a "completed"
  /// QuestionnaireResponse.  If false, the item may be skipped when answering the
  /// questionnaire.
  required: Option<bool>,

  /// Extensions for type
  #[serde(rename = "_type")]
  _type: Option<Element>,

  /// The name of a section, the text of a question or text content for a display
  /// item.
  text: Option<String>,

  /// Extensions for prefix
  #[serde(rename = "_prefix")]
  _prefix: Option<Element>,

  /// The type of questionnaire item this is - whether text for display, a grouping of
  /// other items or a particular type of data to be captured (string, integer, coded
  /// choice, etc.).
  #[serde(rename = "type")]
  fhir_type: Option<Questionnaire_ItemType>,

  /// The maximum number of characters that are permitted in the answer to be
  /// considered a "valid" QuestionnaireResponse.
  #[serde(rename = "maxLength")]
  max_length: Option<i32>,

  /// An indication, when true, that the value cannot be changed by a human respondent
  /// to the Questionnaire.
  #[serde(rename = "readOnly")]
  read_only: Option<bool>,

  /// This element is a URI that refers to an [[[ElementDefinition]]] that provides
  /// information about this item, including information that might otherwise be
  /// included in the instance of the Questionnaire resource. A detailed description
  /// of the construction of the URI is shown in Comments, below. If this element is
  /// present then the following element values MAY be derived from the Element
  /// Definition if the corresponding elements of this Questionnaire resource instance
  /// have no value:    * code (ElementDefinition.code)   * type
  /// (ElementDefinition.type)   * required (ElementDefinition.min)   * repeats
  /// (ElementDefinition.max)   * maxLength (ElementDefinition.maxLength)   *
  /// answerValueSet (ElementDefinition.binding)  * options
  /// (ElementDefinition.binding).
  definition: Option<String>,

  /// Extensions for enableBehavior
  #[serde(rename = "_enableBehavior")]
  _enable_behavior: Option<Element>,

  /// Extensions for linkId
  #[serde(rename = "_linkId")]
  _link_id: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// A constraint indicating that this item should only be enabled (displayed/allow
  /// answers to be captured) when the specified condition is true.
  #[serde(rename = "enableWhen")]
  enable_when: Option<Vec<Questionnaire_EnableWhen>>,

  /// Extensions for required
  #[serde(rename = "_required")]
  _required: Option<Element>,

  /// Extensions for maxLength
  #[serde(rename = "_maxLength")]
  _max_length: Option<Element>,

  /// One or more values that should be pre-populated in the answer when initially
  /// rendering the questionnaire for user input.
  initial: Option<Vec<Questionnaire_Initial>>,

  /// Extensions for repeats
  #[serde(rename = "_repeats")]
  _repeats: Option<Element>,

  /// A terminology code that corresponds to this group or question (e.g. a code from
  /// LOINC, which defines many questions and answers).
  code: Option<Vec<Coding>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum Questionnaire_ItemEnableBehavior {
  #[serde(rename = "all")]
  All,

  #[serde(rename = "any")]
  Any,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum Questionnaire_ItemType {
  #[serde(rename = "group")]
  Group,

  #[serde(rename = "display")]
  Display,

  #[serde(rename = "boolean")]
  Boolean,

  #[serde(rename = "decimal")]
  Decimal,

  #[serde(rename = "integer")]
  Integer,

  #[serde(rename = "date")]
  Date,

  #[serde(rename = "dateTime")]
  DateTime,

  #[serde(rename = "time")]
  Time,

  #[serde(rename = "string")]
  String,

  #[serde(rename = "text")]
  Text,

  #[serde(rename = "url")]
  Url,

  #[serde(rename = "choice")]
  Choice,

  #[serde(rename = "open-choice")]
  OpenChoice,

  #[serde(rename = "attachment")]
  Attachment,

  #[serde(rename = "reference")]
  Reference,

  #[serde(rename = "quantity")]
  Quantity,

}
