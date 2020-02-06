#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Timing::Timing;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Contract_Subject::Contract_Subject;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Annotation::Annotation;
use crate::model::Reference::Reference;


/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract_Action {
  /// Extensions for linkId
  #[serde(rename = "_linkId")]
  _link_id: Vec<Element>,

  /// When action happens.
  #[serde(rename = "occurrenceTiming")]
  occurrence_timing: Timing,

  /// Id [identifier??] of the clause or question text related to the requester of
  /// this action in the referenced form or QuestionnaireResponse.
  #[serde(rename = "requesterLinkId")]
  requester_link_id: Vec<String>,

  /// Indicates another resource whose existence justifies permitting or not
  /// permitting this action.
  #[serde(rename = "reasonReference")]
  reason_reference: Vec<Box<Reference>>,

  /// Extensions for contextLinkId
  #[serde(rename = "_contextLinkId")]
  _context_link_id: Vec<Element>,

  /// Who or what initiated the action and has responsibility for its activation.
  requester: Vec<Box<Reference>>,

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
  modifier_extension: Vec<Extension>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Describes why the action is to be performed or not performed in textual form.
  reason: Vec<String>,

  /// When action happens.
  #[serde(rename = "occurrencePeriod")]
  occurrence_period: Period,

  /// Rationale for the action to be performed or not performed. Describes why the
  /// action is permitted or prohibited.
  #[serde(rename = "reasonCode")]
  reason_code: Vec<CodeableConcept>,

  /// Encounter or Episode with primary association to specified term activity.
  context: Box<Reference>,

  /// Extensions for reasonLinkId
  #[serde(rename = "_reasonLinkId")]
  _reason_link_id: Vec<Element>,

  /// Extensions for securityLabelNumber
  #[serde(rename = "_securityLabelNumber")]
  _security_label_number: Vec<Element>,

  /// Extensions for doNotPerform
  #[serde(rename = "_doNotPerform")]
  _do_not_perform: Element,

  /// The type of role or competency of an individual desired or required to perform
  /// or not perform the action.
  #[serde(rename = "performerRole")]
  performer_role: CodeableConcept,

  /// Activity or service obligation to be done or not done, performed or not
  /// performed, effectuated or not by this Contract term.
  #[serde(rename = "type")]
  fhir_type: CodeableConcept,

  /// Security labels that protects the action.
  #[serde(rename = "securityLabelNumber")]
  security_label_number: Vec<u32>,

  /// The type of individual that is desired or required to perform or not perform the
  /// action.
  #[serde(rename = "performerType")]
  performer_type: Vec<CodeableConcept>,

  /// Current state of the term action.
  status: CodeableConcept,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for performerLinkId
  #[serde(rename = "_performerLinkId")]
  _performer_link_id: Vec<Element>,

  /// Reason or purpose for the action stipulated by this Contract Provision.
  intent: CodeableConcept,

  /// When action happens.
  #[serde(rename = "occurrenceDateTime")]
  occurrence_date_time: String,

  /// Comments made about the term action made by the requester, performer, subject or
  /// other participants.
  note: Vec<Annotation>,

  /// Entity of the action.
  subject: Vec<Contract_Subject>,

  /// Id [identifier??] of the clause or question text related to the reason type or
  /// reference of this  action in the referenced form or QuestionnaireResponse.
  #[serde(rename = "reasonLinkId")]
  reason_link_id: Vec<String>,

  /// Id [identifier??] of the clause or question text related to the requester of
  /// this action in the referenced form or QuestionnaireResponse.
  #[serde(rename = "contextLinkId")]
  context_link_id: Vec<String>,

  /// Extensions for occurrenceDateTime
  #[serde(rename = "_occurrenceDateTime")]
  _occurrence_date_time: Element,

  /// True if the term prohibits the  action.
  #[serde(rename = "doNotPerform")]
  do_not_perform: bool,

  /// Extensions for requesterLinkId
  #[serde(rename = "_requesterLinkId")]
  _requester_link_id: Vec<Element>,

  /// Id [identifier??] of the clause or question text related to the reason type or
  /// reference of this  action in the referenced form or QuestionnaireResponse.
  #[serde(rename = "performerLinkId")]
  performer_link_id: Vec<String>,

  /// Extensions for reason
  _reason: Vec<Element>,

  /// Indicates who or what is being asked to perform (or not perform) the ction.
  performer: Box<Reference>,

  /// Id [identifier??] of the clause or question text related to this action in the
  /// referenced form or QuestionnaireResponse.
  #[serde(rename = "linkId")]
  link_id: Vec<String>,

}
