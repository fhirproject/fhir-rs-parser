use serde::{Deserialize, Serialize};

/// A group of related requests that can be used to capture intended activities that
/// have inter-dependencies such as "give this medication after that one".
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RequestGroup {
  /// Completed or terminated request(s) whose function is taken by this new request.
  replaces: Vec<Reference>,

  /// Extensions for instantiatesUri
  #[serde(rename = "_instantiatesUri")]
  _instantiates_uri: Vec<Element>,

  /// Describes the context of the request group, if any.
  encounter: Reference,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for status
  _status: Element,

  /// Indicates another resource whose existence justifies this request group.
  #[serde(rename = "reasonReference")]
  reason_reference: Vec<Reference>,

  /// A shared identifier common to all requests that were authorized more or less
  /// simultaneously by a single author, representing the identifier of the
  /// requisition, prescription or similar form.
  #[serde(rename = "groupIdentifier")]
  group_identifier: Identifier,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Provides a reference to the author of the request group.
  author: Reference,

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

  /// The actions, if any, produced by the evaluation of the artifact.
  action: Vec<RequestGroup_Action>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The current state of the request. For request groups, the status reflects the
  /// status of all the requests in the group.
  status: String,

  /// A canonical URL referencing a FHIR-defined protocol, guideline, orderset or
  /// other definition that is adhered to in whole or in part by this request.
  #[serde(rename = "instantiatesCanonical")]
  instantiates_canonical: Vec<canonical>,

  /// A plan, proposal or order that is fulfilled in whole or in part by this request.
  #[serde(rename = "basedOn")]
  based_on: Vec<Reference>,

  /// The base language in which the resource is written.
  language: String,

  /// Allows a service to provide a unique, business identifier for the request.
  identifier: Vec<Identifier>,

  /// A URL referencing an externally defined protocol, guideline, orderset or other
  /// definition that is adhered to in whole or in part by this request.
  #[serde(rename = "instantiatesUri")]
  instantiates_uri: Vec<String>,

  /// Indicates the level of authority/intentionality associated with the request and
  /// where the request fits into the workflow chain.
  intent: String,

  /// The subject for which the request group was created.
  subject: Reference,

  /// Extensions for priority
  _priority: Element,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Provides a mechanism to communicate additional information about the response.
  note: Vec<Annotation>,

  /// Extensions for language
  _language: Element,

  /// Describes the reason for the request group in coded or textual form.
  #[serde(rename = "reasonCode")]
  reason_code: Vec<CodeableConcept>,

  /// Indicates how quickly the request should be addressed with respect to other
  /// requests.
  priority: String,

  /// A code that identifies what the overall request group is.
  code: CodeableConcept,

  /// Extensions for authoredOn
  #[serde(rename = "_authoredOn")]
  _authored_on: Element,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// Extensions for intent
  _intent: Element,

  /// Extensions for instantiatesCanonical
  #[serde(rename = "_instantiatesCanonical")]
  _instantiates_canonical: Vec<Element>,

  /// Indicates when the request group was created.
  #[serde(rename = "authoredOn")]
  authored_on: dateTime,

}
