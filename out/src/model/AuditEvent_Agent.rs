#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::AuditEvent_Network::AuditEvent_Network;
use crate::model::Reference::Reference;
use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Coding::Coding;
use crate::model::Element::Element;


/// A record of an event made for purposes of maintaining a security log. Typical
/// uses include detection of intrusion attempts and monitoring for inappropriate
/// usage.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditEvent_Agent {
  /// Alternative agent Identifier. For a human, this should be a user identifier text
  /// string from authentication system. This identifier would be one known to a
  /// common authentication system (e.g. single sign-on), if available.
  #[serde(rename = "altId")]
  alt_id: String,

  /// Type of media involved. Used when the event is about exporting/importing onto
  /// media.
  media: Coding,

  /// Logical network location for application activity, if the activity has a network
  /// location.
  network: AuditEvent_Network,

  /// The reason (purpose of use), specific to this agent, that was used during the
  /// event being recorded.
  #[serde(rename = "purposeOfUse")]
  purpose_of_use: Vec<CodeableConcept>,

  /// Indicator that the user is or is not the requestor, or initiator, for the event
  /// being audited.
  requestor: bool,

  /// Specification of the participation type the user plays when performing the
  /// event.
  #[serde(rename = "type")]
  fhir_type: CodeableConcept,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Human-meaningful name for the agent.
  name: String,

  /// Extensions for policy
  _policy: Vec<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// The policy or plan that authorized the activity being recorded. Typically, a
  /// single activity may have multiple applicable policies, such as patient consent,
  /// guarantor funding, etc. The policy would also indicate the security token used.
  policy: Vec<String>,

  /// Extensions for altId
  #[serde(rename = "_altId")]
  _alt_id: Element,

  /// Extensions for requestor
  _requestor: Element,

  /// Reference to who this agent is that was involved in the event.
  who: Box<Reference>,

  /// Extensions for name
  _name: Element,

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

  /// The security role that the user was acting under, that come from local codes
  /// defined by the access control security system (e.g. RBAC, ABAC) used in the
  /// local context.
  role: Vec<CodeableConcept>,

  /// Where the event occurred.
  location: Box<Reference>,

}
