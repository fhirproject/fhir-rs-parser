#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ResourceList::ResourceList;
use crate::model::Reference::Reference;
use crate::model::ParameterDefinition::ParameterDefinition;
use crate::model::Extension::Extension;
use crate::model::ContactDetail::ContactDetail;
use crate::model::Identifier::Identifier;
use crate::model::RelatedArtifact::RelatedArtifact;
use crate::model::Attachment::Attachment;
use crate::model::Period::Period;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::DataRequirement::DataRequirement;
use crate::model::Element::Element;
use crate::model::UsageContext::UsageContext;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;


/// The Library resource is a general-purpose container for knowledge asset
/// definitions. It can be used to describe and expose existing knowledge assets
/// such as logic libraries and information model descriptions, as well as to
/// describe a collection of knowledge assets.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Library {
  /// Extensions for subtitle
  #[serde(rename = "_subtitle")]
  _subtitle: Option<Element>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Extensions for approvalDate
  #[serde(rename = "_approvalDate")]
  _approval_date: Option<Element>,

  /// The parameter element defines parameters used by the library.
  parameter: Option<Vec<ParameterDefinition>>,

  /// Explanation of why this library is needed and why it has been designed as it
  /// has.
  purpose: Option<String>,

  /// The content of the library as an Attachment. The content may be a reference to a
  /// url, or may be directly embedded as a base-64 string. Either way, the
  /// contentType of the attachment determines how to interpret the content.
  content: Option<Vec<Attachment>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// Extensions for version
  #[serde(rename = "_version")]
  _version: Option<Element>,

  /// A free text natural language description of the library from a consumer's
  /// perspective.
  description: Option<String>,

  /// Extensions for copyright
  #[serde(rename = "_copyright")]
  _copyright: Option<Element>,

  /// Extensions for usage
  #[serde(rename = "_usage")]
  _usage: Option<Element>,

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
  modifier_extension: Option<Vec<Box<Extension>>>,

  /// The date on which the resource content was last reviewed. Review happens
  /// periodically after approval but does not change the original approval date.
  #[serde(rename = "lastReviewDate")]
  last_review_date: Option<i32>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// Extensions for publisher
  #[serde(rename = "_publisher")]
  _publisher: Option<Element>,

  /// The period during which the library content was or is planned to be in active
  /// use.
  #[serde(rename = "effectivePeriod")]
  effective_period: Option<Period>,

  /// An individiual or organization primarily involved in the creation and
  /// maintenance of the content.
  author: Option<Vec<ContactDetail>>,

  /// Extensions for url
  #[serde(rename = "_url")]
  _url: Option<Element>,

  /// Describes a set of data that must be provided in order to be able to
  /// successfully perform the computations defined by the library.
  #[serde(rename = "dataRequirement")]
  data_requirement: Option<Vec<DataRequirement>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// An explanatory or alternate title for the library giving additional information
  /// about its content.
  subtitle: Option<String>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  contact: Option<Vec<ContactDetail>>,

  /// An absolute URI that is used to identify this library when it is referenced in a
  /// specification, model, design or an instance; also called its canonical
  /// identifier. This SHOULD be globally unique and SHOULD be a literal address at
  /// which at which an authoritative instance of this library is (or will be)
  /// published. This URL can be the target of a canonical reference. It SHALL remain
  /// the same when the library is stored on different servers.
  url: Option<String>,

  /// Related artifacts such as additional documentation, justification, or
  /// bibliographic references.
  #[serde(rename = "relatedArtifact")]
  related_artifact: Option<Vec<RelatedArtifact>>,

  /// Extensions for experimental
  #[serde(rename = "_experimental")]
  _experimental: Option<Element>,

  /// Extensions for lastReviewDate
  #[serde(rename = "_lastReviewDate")]
  _last_review_date: Option<Element>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// An individual or organization primarily responsible for internal coherence of
  /// the content.
  editor: Option<Vec<ContactDetail>>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// A natural language name identifying the library. This name should be usable as
  /// an identifier for the module by machine processing applications such as code
  /// generation.
  name: Option<String>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// The status of this library. Enables tracking the life-cycle of the content.
  status: Option<LibraryStatus>,

  /// The name of the organization or individual that published the library.
  publisher: Option<String>,

  /// A copyright statement relating to the library and/or its contents. Copyright
  /// statements are generally legal restrictions on the use and publishing of the
  /// library.
  copyright: Option<String>,

  /// A legal or geographic region in which the library is intended to be used.
  jurisdiction: Option<Vec<CodeableConcept>>,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate library instances.
  #[serde(rename = "useContext")]
  use_context: Option<Vec<UsageContext>>,

  /// The date on which the resource content was approved by the publisher. Approval
  /// happens once when the content is officially approved for usage.
  #[serde(rename = "approvalDate")]
  approval_date: Option<i32>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// A code or group definition that describes the intended subject of the contents
  /// of the library.
  #[serde(rename = "subjectCodeableConcept")]
  subject_codeable_concept: Option<CodeableConcept>,

  /// A short, descriptive, user-friendly title for the library.
  title: Option<String>,

  /// A code or group definition that describes the intended subject of the contents
  /// of the library.
  #[serde(rename = "subjectReference")]
  subject_reference: Option<Box<Reference>>,

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
  version: Option<String>,

  /// A formal identifier that is used to identify this library when it is represented
  /// in other formats, or referenced in a specification, model, design or an
  /// instance. e.g. CMS or NQF identifiers for a measure artifact. Note that at least
  /// one identifier is required for non-experimental active artifacts.
  identifier: Option<Vec<Identifier>>,

  /// A detailed description of how the library is used from a clinical perspective.
  usage: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Identifies the type of library such as a Logic Library, Model Definition, Asset
  /// Collection, or Module Definition.
  #[serde(rename = "type")]
  fhir_type: CodeableConcept,

  /// Extensions for purpose
  #[serde(rename = "_purpose")]
  _purpose: Option<Element>,

  /// An individual or organization responsible for officially endorsing the content
  /// for use in some setting.
  endorser: Option<Vec<ContactDetail>>,

  /// An individual or organization primarily responsible for review of some aspect of
  /// the content.
  reviewer: Option<Vec<ContactDetail>>,

  /// A Boolean value to indicate that this library is authored for testing purposes
  /// (or education/evaluation/marketing) and is not intended to be used for genuine
  /// usage.
  experimental: Option<bool>,

  /// The date  (and optionally time) when the library was published. The date must
  /// change when the business version changes and it must change if the status code
  /// changes. In addition, it should change when the substantive content of the
  /// library changes.
  date: Option<String>,

  /// Extensions for date
  #[serde(rename = "_date")]
  _date: Option<Element>,

  /// Extensions for title
  #[serde(rename = "_title")]
  _title: Option<Element>,

  /// Descriptive topics related to the content of the library. Topics provide a high-
  /// level categorization of the library that can be useful for filtering and
  /// searching.
  topic: Option<Vec<CodeableConcept>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum LibraryStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "unknown")]
  Unknown,

}
