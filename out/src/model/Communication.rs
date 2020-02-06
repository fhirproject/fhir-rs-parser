#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ResourceList::ResourceList;
use crate::model::Narrative::Narrative;
use crate::model::Element::Element;
use crate::model::Meta::Meta;
use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Annotation::Annotation;
use crate::model::Reference::Reference;
use crate::model::Identifier::Identifier;
use crate::model::Communication_Payload::Communication_Payload;


/// An occurrence of information being transmitted; e.g. an alert that was sent to a
/// responsible provider, a public health agency that was notified about a
/// reportable condition.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Communication {
  /// The patient or group that was the focus of this communication.
  subject: Option<Box<Reference>>,

  /// Characterizes how quickly the planned or in progress communication must be
  /// addressed. Includes concepts such as stat, urgent, routine.
  priority: Option<String>,

  /// Prior communication that this communication is in response to.
  #[serde(rename = "inResponseTo")]
  in_response_to: Option<Vec<Box<Reference>>>,

  /// Extensions for priority
  #[serde(rename = "_priority")]
  _priority: Option<Element>,

  /// The entity (e.g. person, organization, clinical information system, care team or
  /// device) which was the target of the communication. If receipts need to be
  /// tracked by an individual, a separate resource instance will need to be created
  /// for each recipient.  Multiple recipient communications are intended where either
  /// receipts are not tracked (e.g. a mass mail-out) or a receipt is captured in
  /// aggregate (all emails confirmed received by a particular time).
  recipient: Option<Vec<Box<Reference>>>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Indicates another resource whose existence justifies this communication.
  #[serde(rename = "reasonReference")]
  reason_reference: Option<Vec<Box<Reference>>>,

  /// Business identifiers assigned to this communication by the performer or other
  /// systems which remain constant as the resource is updated and propagates from
  /// server to server.
  identifier: Option<Vec<Identifier>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The Encounter during which this Communication was created or to which the
  /// creation of this record is tightly associated.
  encounter: Option<Box<Reference>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Extensions for sent
  #[serde(rename = "_sent")]
  _sent: Option<Element>,

  /// The time when this communication was sent.
  sent: Option<String>,

  /// The URL pointing to an externally maintained protocol, guideline, orderset or
  /// other definition that is adhered to in whole or in part by this Communication.
  #[serde(rename = "instantiatesUri")]
  instantiates_uri: Option<Vec<String>>,

  /// An order, proposal or plan fulfilled in whole or in part by this Communication.
  #[serde(rename = "basedOn")]
  based_on: Option<Vec<Box<Reference>>>,

  /// Extensions for received
  #[serde(rename = "_received")]
  _received: Option<Element>,

  /// Extensions for instantiatesUri
  #[serde(rename = "_instantiatesUri")]
  _instantiates_uri: Option<Vec<Element>>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// A channel that was used for this communication (e.g. email, fax).
  medium: Option<Vec<CodeableConcept>>,

  /// Other resources that pertain to this communication and to which this
  /// communication should be associated.
  about: Option<Vec<Box<Reference>>>,

  /// Description of the purpose/content, similar to a subject line in an email.
  topic: Option<CodeableConcept>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// The status of the transmission.
  status: Option<String>,

  /// The type of message conveyed such as alert, notification, reminder, instruction,
  /// etc.
  category: Option<Vec<CodeableConcept>>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

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

  /// The entity (e.g. person, organization, clinical information system, or device)
  /// which was the source of the communication.
  sender: Option<Box<Reference>>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// The URL pointing to a FHIR-defined protocol, guideline, orderset or other
  /// definition that is adhered to in whole or in part by this Communication.
  #[serde(rename = "instantiatesCanonical")]
  instantiates_canonical: Option<Vec<String>>,

  /// Part of this action.
  #[serde(rename = "partOf")]
  part_of: Option<Vec<Box<Reference>>>,

  /// The reason or justification for the communication.
  #[serde(rename = "reasonCode")]
  reason_code: Option<Vec<CodeableConcept>>,

  /// Text, attachment(s), or resource(s) that was communicated to the recipient.
  payload: Option<Vec<Communication_Payload>>,

  /// Additional notes or commentary about the communication by the sender, receiver
  /// or other interested parties.
  note: Option<Vec<Annotation>>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// The time when this communication arrived at the destination.
  received: Option<String>,

  /// Captures the reason for the current state of the Communication.
  #[serde(rename = "statusReason")]
  status_reason: Option<CodeableConcept>,

}
