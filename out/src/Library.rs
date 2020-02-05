use serde::{Deserialize, Serialize};

/// The Library resource is a general-purpose container for knowledge asset
/// definitions. It can be used to describe and expose existing knowledge assets
/// such as logic libraries and information model descriptions, as well as to
/// describe a collection of knowledge assets.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Library {
  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  contact: Vec<ContactDetail>,

  /// The parameter element defines parameters used by the library.
  parameter: Vec<ParameterDefinition>,

  /// Extensions for status
  _status: Element,

  /// A code or group definition that describes the intended subject of the contents
  /// of the library.
  #[serde(rename = "subjectReference")]
  subject_reference: Reference,

  /// An individual or organization primarily responsible for internal coherence of
  /// the content.
  editor: Vec<ContactDetail>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Extensions for url
  _url: Element,

  /// Extensions for name
  _name: Element,

  /// An explanatory or alternate title for the library giving additional information
  /// about its content.
  subtitle: String,

  /// Extensions for usage
  _usage: Element,

  /// The date on which the resource content was approved by the publisher. Approval
  /// happens once when the content is officially approved for usage.
  #[serde(rename = "approvalDate")]
  approval_date: date,

  /// Describes a set of data that must be provided in order to be able to
  /// successfully perform the computations defined by the library.
  #[serde(rename = "dataRequirement")]
  data_requirement: Vec<DataRequirement>,

  /// The identifier that is used to identify this version of the library when it is
  /// referenced in a specification, model, design or instance. This is an arbitrary
  /// value managed by the library author and is not expected to be globally unique.
  /// For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not
  /// available. There is also no expectation that versions can be placed in a
  /// lexicographical sequence. To provide a version consistent with the Decision
  /// Support Service specification, use the format Major.Minor.Revision (e.g. 1.0.0).
  /// For more information on versioning knowledge assets, refer to the Decision
  /// Support Service specification. Note that a version is required for non-
  /// experimental active artifacts.
  version: String,

  /// Extensions for experimental
  _experimental: Element,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Explanation of why this library is needed and why it has been designed as it
  /// has.
  purpose: markdown,

  /// The status of this library. Enables tracking the life-cycle of the content.
  status: LibraryStatus,

  /// Extensions for copyright
  _copyright: Element,

  /// A copyright statement relating to the library and/or its contents. Copyright
  /// statements are generally legal restrictions on the use and publishing of the
  /// library.
  copyright: markdown,

  /// A detailed description of how the library is used from a clinical perspective.
  usage: String,

  /// An individual or organization primarily responsible for review of some aspect of
  /// the content.
  reviewer: Vec<ContactDetail>,

  /// Extensions for version
  _version: Element,

  /// A Boolean value to indicate that this library is authored for testing purposes
  /// (or education/evaluation/marketing) and is not intended to be used for genuine
  /// usage.
  experimental: bool,

  /// A legal or geographic region in which the library is intended to be used.
  jurisdiction: Vec<CodeableConcept>,

  /// Extensions for approvalDate
  #[serde(rename = "_approvalDate")]
  _approval_date: Element,

  /// Extensions for title
  _title: Element,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate library instances.
  #[serde(rename = "useContext")]
  use_context: Vec<UsageContext>,

  /// Extensions for lastReviewDate
  #[serde(rename = "_lastReviewDate")]
  _last_review_date: Element,

  /// The base language in which the resource is written.
  language: String,

  /// Extensions for language
  _language: Element,

  /// The date  (and optionally time) when the library was published. The date must
  /// change when the business version changes and it must change if the status code
  /// changes. In addition, it should change when the substantive content of the
  /// library changes.
  date: dateTime,

  /// A code or group definition that describes the intended subject of the contents
  /// of the library.
  #[serde(rename = "subjectCodeableConcept")]
  subject_codeable_concept: CodeableConcept,

  /// Extensions for date
  _date: Element,

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

  /// An absolute URI that is used to identify this library when it is referenced in a
  /// specification, model, design or an instance; also called its canonical
  /// identifier. This SHOULD be globally unique and SHOULD be a literal address at
  /// which at which an authoritative instance of this library is (or will be)
  /// published. This URL can be the target of a canonical reference. It SHALL remain
  /// the same when the library is stored on different servers.
  url: String,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Descriptive topics related to the content of the library. Topics provide a high-
  /// level categorization of the library that can be useful for filtering and
  /// searching.
  topic: Vec<CodeableConcept>,

  /// The name of the organization or individual that published the library.
  publisher: String,

  /// A short, descriptive, user-friendly title for the library.
  title: String,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The period during which the library content was or is planned to be in active
  /// use.
  #[serde(rename = "effectivePeriod")]
  effective_period: Period,

  /// The content of the library as an Attachment. The content may be a reference to a
  /// url, or may be directly embedded as a base-64 string. Either way, the
  /// contentType of the attachment determines how to interpret the content.
  content: Vec<Attachment>,

  /// Extensions for subtitle
  _subtitle: Element,

  /// A formal identifier that is used to identify this library when it is represented
  /// in other formats, or referenced in a specification, model, design or an
  /// instance. e.g. CMS or NQF identifiers for a measure artifact. Note that at least
  /// one identifier is required for non-experimental active artifacts.
  identifier: Vec<Identifier>,

  /// An individual or organization responsible for officially endorsing the content
  /// for use in some setting.
  endorser: Vec<ContactDetail>,

  /// The date on which the resource content was last reviewed. Review happens
  /// periodically after approval but does not change the original approval date.
  #[serde(rename = "lastReviewDate")]
  last_review_date: date,

  /// Extensions for publisher
  _publisher: Element,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// Extensions for description
  _description: Element,

  /// A natural language name identifying the library. This name should be usable as
  /// an identifier for the module by machine processing applications such as code
  /// generation.
  name: String,

  /// Extensions for purpose
  _purpose: Element,

  /// Related artifacts such as additional documentation, justification, or
  /// bibliographic references.
  #[serde(rename = "relatedArtifact")]
  related_artifact: Vec<RelatedArtifact>,

  /// Identifies the type of library such as a Logic Library, Model Definition, Asset
  /// Collection, or Module Definition.
  type: CodeableConcept,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// An individiual or organization primarily involved in the creation and
  /// maintenance of the content.
  author: Vec<ContactDetail>,

  /// A free text natural language description of the library from a consumer's
  /// perspective.
  description: markdown,

}

#[derive(Debug, Serialize, Deserialize)]
enum LibraryStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "unknown")]
  Unknown,

}
