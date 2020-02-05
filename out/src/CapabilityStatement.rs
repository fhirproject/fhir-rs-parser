use serde::{Deserialize, Serialize};

/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR
/// Server for a particular version of FHIR that may be used as a statement of
/// actual server functionality or a statement of required or desired server
/// implementation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CapabilityStatement {
  /// Extensions for kind
  _kind: Element,

  /// Identifies a specific implementation instance that is described by the
  /// capability statement - i.e. a particular installation, rather than the
  /// capabilities of a software program.
  implementation: CapabilityStatement_Implementation,

  /// Extensions for url
  _url: Element,

  /// Extensions for format
  _format: Vec<Element>,

  /// A list of implementation guides that the server does (or should) support in
  /// their entirety.
  #[serde(rename = "implementationGuide")]
  implementation_guide: Vec<canonical>,

  /// A free text natural language description of the capability statement from a
  /// consumer's perspective. Typically, this is used when the capability statement
  /// describes a desired rather than an actual solution, for example as a formal
  /// expression of requirements as part of an RFP.
  description: markdown,

  /// Reference to a canonical URL of another CapabilityStatement that this software
  /// adds to. The capability statement automatically includes everything in the other
  /// statement, and it is not duplicated, though the server may repeat the same
  /// resources, interactions and operations to add additional details to them.
  imports: Vec<canonical>,

  /// Extensions for language
  _language: Element,

  /// The base language in which the resource is written.
  language: String,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// A legal or geographic region in which the capability statement is intended to be
  /// used.
  jurisdiction: Vec<CodeableConcept>,

  /// The version of the FHIR specification that this CapabilityStatement describes
  /// (which SHALL be the same as the FHIR version of the CapabilityStatement itself).
  /// There is no default value.
  #[serde(rename = "fhirVersion")]
  fhir_version: CapabilityStatementFhirVersion,

  /// A list of the patch formats supported by this implementation using their content
  /// types.
  #[serde(rename = "patchFormat")]
  patch_format: Vec<String>,

  /// The name of the organization or individual that published the capability
  /// statement.
  publisher: String,

  /// A document definition.
  document: Vec<CapabilityStatement_Document>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate capability
  /// statement instances.
  #[serde(rename = "useContext")]
  use_context: Vec<UsageContext>,

  /// Extensions for purpose
  _purpose: Element,

  /// Extensions for version
  _version: Element,

  /// An absolute URI that is used to identify this capability statement when it is
  /// referenced in a specification, model, design or an instance; also called its
  /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
  /// address at which at which an authoritative instance of this capability statement
  /// is (or will be) published. This URL can be the target of a canonical reference.
  /// It SHALL remain the same when the capability statement is stored on different
  /// servers.
  url: String,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// A natural language name identifying the capability statement. This name should
  /// be usable as an identifier for the module by machine processing applications
  /// such as code generation.
  name: String,

  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  contact: Vec<ContactDetail>,

  /// Extensions for name
  _name: Element,

  /// Extensions for status
  _status: Element,

  /// The way that this statement is intended to be used, to describe an actual
  /// running instance of software, a particular product (kind, not instance of
  /// software) or a class of implementation (e.g. a desired purchase).
  kind: CapabilityStatementKind,

  /// The identifier that is used to identify this version of the capability statement
  /// when it is referenced in a specification, model, design or instance. This is an
  /// arbitrary value managed by the capability statement author and is not expected
  /// to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
  /// managed version is not available. There is also no expectation that versions can
  /// be placed in a lexicographical sequence.
  version: String,

  /// A Boolean value to indicate that this capability statement is authored for
  /// testing purposes (or education/evaluation/marketing) and is not intended to be
  /// used for genuine usage.
  experimental: bool,

  /// The status of this capability statement. Enables tracking the life-cycle of the
  /// content.
  status: CapabilityStatementStatus,

  /// Extensions for experimental
  _experimental: Element,

  /// Extensions for description
  _description: Element,

  /// Extensions for copyright
  _copyright: Element,

  /// Software that is covered by this capability statement.  It is used when the
  /// capability statement describes the capabilities of a particular software
  /// version, independent of an installation.
  software: CapabilityStatement_Software,

  /// A copyright statement relating to the capability statement and/or its contents.
  /// Copyright statements are generally legal restrictions on the use and publishing
  /// of the capability statement.
  copyright: markdown,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for publisher
  _publisher: Element,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Extensions for fhirVersion
  #[serde(rename = "_fhirVersion")]
  _fhir_version: Element,

  /// Extensions for patchFormat
  #[serde(rename = "_patchFormat")]
  _patch_format: Vec<Element>,

  /// Extensions for title
  _title: Element,

  /// A description of the messaging capabilities of the solution.
  messaging: Vec<CapabilityStatement_Messaging>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// A list of the formats supported by this implementation using their content
  /// types.
  format: Vec<String>,

  /// A short, descriptive, user-friendly title for the capability statement.
  title: String,

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

  /// Reference to a canonical URL of another CapabilityStatement that this software
  /// implements. This capability statement is a published API description that
  /// corresponds to a business service. The server may actually implement a subset of
  /// the capability statement it claims to implement, so the capability statement
  /// must specify the full capability details.
  instantiates: Vec<canonical>,

  /// Explanation of why this capability statement is needed and why it has been
  /// designed as it has.
  purpose: markdown,

  /// A definition of the restful capabilities of the solution, if any.
  rest: Vec<CapabilityStatement_Rest>,

  /// Extensions for date
  _date: Element,

  /// The date  (and optionally time) when the capability statement was published. The
  /// date must change when the business version changes and it must change if the
  /// status code changes. In addition, it should change when the substantive content
  /// of the capability statement changes.
  date: dateTime,

}

#[derive(Debug, Serialize, Deserialize)]
enum CapabilityStatementFhirVersion {
  #[serde(rename = "0.01")]
  0.01,

  #[serde(rename = "0.05")]
  0.05,

  #[serde(rename = "0.06")]
  0.06,

  #[serde(rename = "0.11")]
  0.11,

  #[serde(rename = "0.0.80")]
  0.0.80,

  #[serde(rename = "0.0.81")]
  0.0.81,

  #[serde(rename = "0.0.82")]
  0.0.82,

  #[serde(rename = "0.4.0")]
  0.4.0,

  #[serde(rename = "0.5.0")]
  0.5.0,

  #[serde(rename = "1.0.0")]
  1.0.0,

  #[serde(rename = "1.0.1")]
  1.0.1,

  #[serde(rename = "1.0.2")]
  1.0.2,

  #[serde(rename = "1.1.0")]
  1.1.0,

  #[serde(rename = "1.4.0")]
  1.4.0,

  #[serde(rename = "1.6.0")]
  1.6.0,

  #[serde(rename = "1.8.0")]
  1.8.0,

  #[serde(rename = "3.0.0")]
  3.0.0,

  #[serde(rename = "3.0.1")]
  3.0.1,

  #[serde(rename = "3.3.0")]
  3.3.0,

  #[serde(rename = "3.5.0")]
  3.5.0,

  #[serde(rename = "4.0.0")]
  4.0.0,

  #[serde(rename = "4.0.1")]
  4.0.1,

}

#[derive(Debug, Serialize, Deserialize)]
enum CapabilityStatementKind {
  #[serde(rename = "instance")]
  Instance,

  #[serde(rename = "capability")]
  Capability,

  #[serde(rename = "requirements")]
  Requirements,

}

#[derive(Debug, Serialize, Deserialize)]
enum CapabilityStatementStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "unknown")]
  Unknown,

}
