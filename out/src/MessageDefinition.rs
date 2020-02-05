use serde::{Deserialize, Serialize};

/// Defines the characteristics of a message that can be shared between systems,
/// including the type of event that initiates the message, the content to be
/// transmitted and what response(s), if any, are permitted.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessageDefinition {
  /// Extensions for name
  _name: Element,

  /// Declare at a message definition level whether a response is required or only
  /// upon error or success, or never.
  #[serde(rename = "responseRequired")]
  response_required: MessageDefinitionResponseRequired,

  /// Canonical reference to a GraphDefinition. If a URL is provided, it is the
  /// canonical reference to a [[[GraphDefinition]]] that it controls what resources
  /// are to be added to the bundle when building the document. The GraphDefinition
  /// can also specify profiles that apply to the various resources.
  graph: Vec<canonical>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// Extensions for status
  _status: Element,

  /// Identifies the resource (or resources) that are being addressed by the event.
  /// For example, the Encounter for an admit message or two Account records for a
  /// merge.
  focus: Vec<MessageDefinition_Focus>,

  /// The status of this message definition. Enables tracking the life-cycle of the
  /// content.
  status: MessageDefinitionStatus,

  /// The date  (and optionally time) when the message definition was published. The
  /// date must change when the business version changes and it must change if the
  /// status code changes. In addition, it should change when the substantive content
  /// of the message definition changes.
  date: dateTime,

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

  /// A formal identifier that is used to identify this message definition when it is
  /// represented in other formats, or referenced in a specification, model, design or
  /// an instance.
  identifier: Vec<Identifier>,

  /// Extensions for language
  _language: Element,

  /// A MessageDefinition that is superseded by this definition.
  replaces: Vec<canonical>,

  /// A Boolean value to indicate that this message definition is authored for testing
  /// purposes (or education/evaluation/marketing) and is not intended to be used for
  /// genuine usage.
  experimental: bool,

  /// A short, descriptive, user-friendly title for the message definition.
  title: String,

  /// Extensions for date
  _date: Element,

  /// Explanation of why this message definition is needed and why it has been
  /// designed as it has.
  purpose: markdown,

  /// Extensions for purpose
  _purpose: Element,

  /// Extensions for url
  _url: Element,

  /// A copyright statement relating to the message definition and/or its contents.
  /// Copyright statements are generally legal restrictions on the use and publishing
  /// of the message definition.
  copyright: markdown,

  /// Extensions for responseRequired
  #[serde(rename = "_responseRequired")]
  _response_required: Element,

  /// The MessageDefinition that is the basis for the contents of this resource.
  base: canonical,

  /// A natural language name identifying the message definition. This name should be
  /// usable as an identifier for the module by machine processing applications such
  /// as code generation.
  name: String,

  /// Extensions for description
  _description: Element,

  /// The base language in which the resource is written.
  language: String,

  /// A legal or geographic region in which the message definition is intended to be
  /// used.
  jurisdiction: Vec<CodeableConcept>,

  /// Extensions for experimental
  _experimental: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// The business identifier that is used to reference the MessageDefinition and *is*
  /// expected to be consistent from server to server.
  url: String,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// The name of the organization or individual that published the message
  /// definition.
  publisher: String,

  /// Extensions for version
  _version: Element,

  /// Identifies a protocol or workflow that this MessageDefinition represents a step
  /// in.
  parent: Vec<canonical>,

  /// Event code or link to the EventDefinition.
  #[serde(rename = "eventCoding")]
  event_coding: Coding,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Extensions for eventUri
  #[serde(rename = "_eventUri")]
  _event_uri: Element,

  /// The identifier that is used to identify this version of the message definition
  /// when it is referenced in a specification, model, design or instance. This is an
  /// arbitrary value managed by the message definition author and is not expected to
  /// be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
  /// managed version is not available. There is also no expectation that versions can
  /// be placed in a lexicographical sequence.
  version: String,

  /// Extensions for title
  _title: Element,

  /// The impact of the content of the message.
  category: MessageDefinitionCategory,

  /// Event code or link to the EventDefinition.
  #[serde(rename = "eventUri")]
  event_uri: String,

  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  contact: Vec<ContactDetail>,

  /// A free text natural language description of the message definition from a
  /// consumer's perspective.
  description: markdown,

  /// Indicates what types of messages may be sent as an application-level response to
  /// this message.
  #[serde(rename = "allowedResponse")]
  allowed_response: Vec<MessageDefinition_AllowedResponse>,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate message definition
  /// instances.
  #[serde(rename = "useContext")]
  use_context: Vec<UsageContext>,

  /// Extensions for copyright
  _copyright: Element,

  /// Extensions for category
  _category: Element,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Extensions for publisher
  _publisher: Element,

}

#[derive(Debug, Serialize, Deserialize)]
enum MessageDefinitionResponseRequired {
  #[serde(rename = "always")]
  Always,

  #[serde(rename = "on-error")]
  OnError,

  #[serde(rename = "never")]
  Never,

  #[serde(rename = "on-success")]
  OnSuccess,

}

#[derive(Debug, Serialize, Deserialize)]
enum MessageDefinitionStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "unknown")]
  Unknown,

}

#[derive(Debug, Serialize, Deserialize)]
enum MessageDefinitionCategory {
  #[serde(rename = "consequence")]
  Consequence,

  #[serde(rename = "currency")]
  Currency,

  #[serde(rename = "notification")]
  Notification,

}
