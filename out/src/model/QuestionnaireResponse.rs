#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Narrative::Narrative;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::Identifier::Identifier;
use crate::model::ResourceList::ResourceList;
use crate::model::QuestionnaireResponse_Item::QuestionnaireResponse_Item;
use crate::model::Meta::Meta;


/// A structured set of questions and their answers. The questions are ordered and
/// grouped into coherent subsets, corresponding to the structure of the grouping of
/// the questionnaire being responded to.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionnaireResponse {
  /// The Questionnaire that defines and organizes the questions for which answers are
  /// being provided.
  questionnaire: String,

  /// The date and/or time that this set of answers were last changed.
  authored: String,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// A group or question item from the original questionnaire for which answers are
  /// provided.
  item: Vec<QuestionnaireResponse_Item>,

  /// Extensions for language
  _language: Element,

  /// The subject of the questionnaire response.  This could be a patient,
  /// organization, practitioner, device, etc.  This is who/what the answers apply to,
  /// but is not necessarily the source of information.
  subject: Box<Reference>,

  /// Extensions for authored
  _authored: Element,

  /// The order, proposal or plan that is fulfilled in whole or in part by this
  /// QuestionnaireResponse.  For example, a ServiceRequest seeking an intake
  /// assessment or a decision support recommendation to assess for post-partum
  /// depression.
  #[serde(rename = "basedOn")]
  based_on: Vec<Box<Reference>>,

  /// Person who received the answers to the questions in the QuestionnaireResponse
  /// and recorded them in the system.
  author: Box<Reference>,

  /// The person who answered the questions about the subject.
  source: Box<Reference>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The Encounter during which this questionnaire response was created or to which
  /// the creation of this record is tightly associated.
  encounter: Box<Reference>,

  /// The base language in which the resource is written.
  language: String,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Extensions for status
  _status: Element,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.    Modifier
  /// extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// The position of the questionnaire response within its overall lifecycle.
  status: QuestionnaireResponseStatus,

  /// A procedure or observation that this questionnaire was performed as part of the
  /// execution of.  For example, the surgery a checklist was executed as part of.
  #[serde(rename = "partOf")]
  part_of: Vec<Box<Reference>>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// A business identifier assigned to a particular completed (or partially
  /// completed) questionnaire.
  identifier: Identifier,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum QuestionnaireResponseStatus {
  #[serde(rename = "in-progress")]
  InProgress,

  #[serde(rename = "completed")]
  Completed,

  #[serde(rename = "amended")]
  Amended,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

  #[serde(rename = "stopped")]
  Stopped,

}
