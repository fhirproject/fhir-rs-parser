#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Annotation::Annotation;
use crate::model::Period::Period;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::Meta::Meta;
use crate::model::Identifier::Identifier;
use crate::model::Element::Element;
use crate::model::Narrative::Narrative;
use crate::model::CommunicationRequest_Payload::CommunicationRequest_Payload;
use crate::model::ResourceList::ResourceList;


/// A request to convey information; e.g. the CDS system proposes that an alert be
/// sent to a responsible provider, the CDS system proposes that the public health
/// agency be notified about a reportable condition.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunicationRequest {
  /// The status of the proposal or order.
  status: Option<String>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// The time when this communication is to occur.
  #[serde(rename = "occurrencePeriod")]
  occurrence_period: Option<Period>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Business identifiers assigned to this communication request by the performer or
  /// other systems which remain constant as the resource is updated and propagates
  /// from server to server.
  identifier: Option<Vec<Identifier>>,

  /// The device, individual, or organization who initiated the request and has
  /// responsibility for its activation.
  requester: Option<Box<Reference>>,

  /// For draft requests, indicates the date of initial creation.  For requests with
  /// other statuses, indicates the date of activation.
  #[serde(rename = "authoredOn")]
  authored_on: Option<String>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// Extensions for priority
  #[serde(rename = "_priority")]
  _priority: Option<Element>,

  /// Text, attachment(s), or resource(s) to be communicated to the recipient.
  payload: Option<Vec<CommunicationRequest_Payload>>,

  /// The entity (e.g. person, organization, clinical information system, or device)
  /// which is to be the source of the communication.
  sender: Option<Box<Reference>>,

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
  modifier_extension: Option<Vec<Extension>>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Captures the reason for the current state of the CommunicationRequest.
  #[serde(rename = "statusReason")]
  status_reason: Option<CodeableConcept>,

  /// Extensions for doNotPerform
  #[serde(rename = "_doNotPerform")]
  _do_not_perform: Option<Element>,

  /// Other resources that pertain to this communication request and to which this
  /// communication request should be associated.
  about: Option<Vec<Box<Reference>>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// A shared identifier common to all requests that were authorized more or less
  /// simultaneously by a single author, representing the identifier of the
  /// requisition, prescription or similar form.
  #[serde(rename = "groupIdentifier")]
  group_identifier: Option<Identifier>,

  /// If true indicates that the CommunicationRequest is asking for the specified
  /// action to *not* occur.
  #[serde(rename = "doNotPerform")]
  do_not_perform: Option<bool>,

  /// Indicates another resource whose existence justifies this request.
  #[serde(rename = "reasonReference")]
  reason_reference: Option<Vec<Box<Reference>>>,

  /// Characterizes how quickly the proposed act must be initiated. Includes concepts
  /// such as stat, urgent, routine.
  priority: Option<String>,

  /// Completed or terminated request(s) whose function is taken by this new request.
  replaces: Option<Vec<Box<Reference>>>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// A plan or proposal that is fulfilled in whole or in part by this request.
  #[serde(rename = "basedOn")]
  based_on: Option<Vec<Box<Reference>>>,

  /// Extensions for occurrenceDateTime
  #[serde(rename = "_occurrenceDateTime")]
  _occurrence_date_time: Option<Element>,

  /// Extensions for authoredOn
  #[serde(rename = "_authoredOn")]
  _authored_on: Option<Element>,

  /// A channel that was used for this communication (e.g. email, fax).
  medium: Option<Vec<CodeableConcept>>,

  /// The type of message to be sent such as alert, notification, reminder,
  /// instruction, etc.
  category: Option<Vec<CodeableConcept>>,

  /// Comments made about the request by the requester, sender, recipient, subject or
  /// other participants.
  note: Option<Vec<Annotation>>,

  /// The entity (e.g. person, organization, clinical information system, device,
  /// group, or care team) which is the intended target of the communication.
  recipient: Option<Vec<Box<Reference>>>,

  /// The patient or group that is the focus of this communication request.
  subject: Option<Box<Reference>>,

  /// The Encounter during which this CommunicationRequest was created or to which the
  /// creation of this record is tightly associated.
  encounter: Option<Box<Reference>>,

  /// The time when this communication is to occur.
  #[serde(rename = "occurrenceDateTime")]
  occurrence_date_time: Option<String>,

  /// Describes why the request is being made in coded or textual form.
  #[serde(rename = "reasonCode")]
  reason_code: Option<Vec<CodeableConcept>>,

}
