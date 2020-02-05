use serde::{Deserialize, Serialize};

/// This resource provides enrollment and plan details from the processing of an
/// EnrollmentRequest resource.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EnrollmentResponse {
  /// Extensions for disposition
  _disposition: Element,

  /// The practitioner who is responsible for the services rendered to the patient.
  #[serde(rename = "requestProvider")]
  request_provider: Reference,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Processing status: error, complete.
  outcome: EnrollmentResponseOutcome,

  /// A description of the status of the adjudication.
  disposition: String,

  /// Extensions for status
  _status: Element,

  /// Original request resource reference.
  request: Reference,

  /// The date when the enclosed suite of services were performed or completed.
  created: dateTime,

  /// The Insurer who produced this adjudicated response.
  organization: Reference,

  /// Extensions for language
  _language: Element,

  /// The Response business identifier.
  identifier: Vec<Identifier>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// The status of the resource instance.
  status: String,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The base language in which the resource is written.
  language: String,

  /// Extensions for created
  _created: Element,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for outcome
  _outcome: Element,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

}

#[derive(Debug, Serialize, Deserialize)]
enum EnrollmentResponseOutcome {
  #[serde(rename = "queued")]
  Queued,

  #[serde(rename = "complete")]
  Complete,

  #[serde(rename = "error")]
  Error,

  #[serde(rename = "partial")]
  Partial,

}
