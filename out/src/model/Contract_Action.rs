#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Contract_Subject::Contract_Subject;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::Element::Element;
use crate::model::Annotation::Annotation;
use crate::model::Timing::Timing;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;


/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract_Action {
  /// Rationale for the action to be performed or not performed. Describes why the
  /// action is permitted or prohibited.
  #[serde(rename = "reasonCode")]
  reason_code: Option<Vec<CodeableConcept>>,

  /// Security labels that protects the action.
  #[serde(rename = "securityLabelNumber")]
  security_label_number: Option<Vec<u32>>,

  /// Id [identifier??] of the clause or question text related to the requester of
  /// this action in the referenced form or QuestionnaireResponse.
  #[serde(rename = "contextLinkId")]
  context_link_id: Option<Vec<String>>,

  /// The type of individual that is desired or required to perform or not perform the
  /// action.
  #[serde(rename = "performerType")]
  performer_type: Option<Vec<CodeableConcept>>,

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

  /// Extensions for reasonLinkId
  #[serde(rename = "_reasonLinkId")]
  _reason_link_id: Option<Vec<Element>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// When action happens.
  #[serde(rename = "occurrenceDateTime")]
  occurrence_date_time: Option<String>,

  /// Indicates who or what is being asked to perform (or not perform) the ction.
  performer: Option<Box<Reference>>,

  /// Extensions for performerLinkId
  #[serde(rename = "_performerLinkId")]
  _performer_link_id: Option<Vec<Element>>,

  /// True if the term prohibits the  action.
  #[serde(rename = "doNotPerform")]
  do_not_perform: Option<bool>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Indicates another resource whose existence justifies permitting or not
  /// permitting this action.
  #[serde(rename = "reasonReference")]
  reason_reference: Option<Vec<Box<Reference>>>,

  /// Describes why the action is to be performed or not performed in textual form.
  reason: Option<Vec<String>>,

  /// Id [identifier??] of the clause or question text related to the reason type or
  /// reference of this  action in the referenced form or QuestionnaireResponse.
  #[serde(rename = "performerLinkId")]
  performer_link_id: Option<Vec<String>>,

  /// Extensions for contextLinkId
  #[serde(rename = "_contextLinkId")]
  _context_link_id: Option<Vec<Element>>,

  /// Entity of the action.
  subject: Option<Vec<Contract_Subject>>,

  /// Extensions for requesterLinkId
  #[serde(rename = "_requesterLinkId")]
  _requester_link_id: Option<Vec<Element>>,

  /// When action happens.
  #[serde(rename = "occurrenceTiming")]
  occurrence_timing: Option<Timing>,

  /// Extensions for linkId
  #[serde(rename = "_linkId")]
  _link_id: Option<Vec<Element>>,

  /// Who or what initiated the action and has responsibility for its activation.
  requester: Option<Vec<Box<Reference>>>,

  /// Extensions for doNotPerform
  #[serde(rename = "_doNotPerform")]
  _do_not_perform: Option<Element>,

  /// Current state of the term action.
  status: CodeableConcept,

  /// Id [identifier??] of the clause or question text related to this action in the
  /// referenced form or QuestionnaireResponse.
  #[serde(rename = "linkId")]
  link_id: Option<Vec<String>>,

  /// Extensions for securityLabelNumber
  #[serde(rename = "_securityLabelNumber")]
  _security_label_number: Option<Vec<Element>>,

  /// Extensions for reason
  #[serde(rename = "_reason")]
  _reason: Option<Vec<Element>>,

  /// Id [identifier??] of the clause or question text related to the reason type or
  /// reference of this  action in the referenced form or QuestionnaireResponse.
  #[serde(rename = "reasonLinkId")]
  reason_link_id: Option<Vec<String>>,

  /// The type of role or competency of an individual desired or required to perform
  /// or not perform the action.
  #[serde(rename = "performerRole")]
  performer_role: Option<CodeableConcept>,

  /// Activity or service obligation to be done or not done, performed or not
  /// performed, effectuated or not by this Contract term.
  #[serde(rename = "type")]
  fhir_type: CodeableConcept,

  /// Comments made about the term action made by the requester, performer, subject or
  /// other participants.
  note: Option<Vec<Annotation>>,

  /// Reason or purpose for the action stipulated by this Contract Provision.
  intent: CodeableConcept,

  /// When action happens.
  #[serde(rename = "occurrencePeriod")]
  occurrence_period: Option<Period>,

  /// Encounter or Episode with primary association to specified term activity.
  context: Option<Box<Reference>>,

  /// Id [identifier??] of the clause or question text related to the requester of
  /// this action in the referenced form or QuestionnaireResponse.
  #[serde(rename = "requesterLinkId")]
  requester_link_id: Option<Vec<String>>,

  /// Extensions for occurrenceDateTime
  #[serde(rename = "_occurrenceDateTime")]
  _occurrence_date_time: Option<Element>,

}
