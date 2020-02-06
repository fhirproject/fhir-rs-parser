#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::TerminologyCapabilities_Implementation::TerminologyCapabilities_Implementation;
use crate::model::TerminologyCapabilities_CodeSystem::TerminologyCapabilities_CodeSystem;
use crate::model::Meta::Meta;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::TerminologyCapabilities_Software::TerminologyCapabilities_Software;
use crate::model::TerminologyCapabilities_Translation::TerminologyCapabilities_Translation;
use crate::model::Extension::Extension;
use crate::model::TerminologyCapabilities_ValidateCode::TerminologyCapabilities_ValidateCode;
use crate::model::TerminologyCapabilities_Closure::TerminologyCapabilities_Closure;
use crate::model::ResourceList::ResourceList;
use crate::model::Narrative::Narrative;
use crate::model::TerminologyCapabilities_Expansion::TerminologyCapabilities_Expansion;
use crate::model::Element::Element;
use crate::model::UsageContext::UsageContext;
use crate::model::ContactDetail::ContactDetail;


/// A TerminologyCapabilities resource documents a set of capabilities (behaviors)
/// of a FHIR Terminology Server that may be used as a statement of actual server
/// functionality or a statement of required or desired server implementation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilities {
  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// Extensions for copyright
  #[serde(rename = "_copyright")]
  _copyright: Option<Element>,

  /// Extensions for kind
  #[serde(rename = "_kind")]
  _kind: Option<Element>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Whether the server supports lockedDate.
  #[serde(rename = "lockedDate")]
  locked_date: Option<bool>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// The identifier that is used to identify this version of the terminology
  /// capabilities when it is referenced in a specification, model, design or
  /// instance. This is an arbitrary value managed by the terminology capabilities
  /// author and is not expected to be globally unique. For example, it might be a
  /// timestamp (e.g. yyyymmdd) if a managed version is not available. There is also
  /// no expectation that versions can be placed in a lexicographical sequence.
  version: Option<String>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// Extensions for date
  #[serde(rename = "_date")]
  _date: Option<Element>,

  /// Software that is covered by this terminology capability statement.  It is used
  /// when the statement describes the capabilities of a particular software version,
  /// independent of an installation.
  software: Option<TerminologyCapabilities_Software>,

  /// A free text natural language description of the terminology capabilities from a
  /// consumer's perspective. Typically, this is used when the capability statement
  /// describes a desired rather than an actual solution, for example as a formal
  /// expression of requirements as part of an RFP.
  description: Option<String>,

  /// Information about the [ConceptMap/$translate](conceptmap-operation-
  /// translate.html) operation.
  translation: Option<TerminologyCapabilities_Translation>,

  /// An absolute URI that is used to identify this terminology capabilities when it
  /// is referenced in a specification, model, design or an instance; also called its
  /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
  /// address at which at which an authoritative instance of this terminology
  /// capabilities is (or will be) published. This URL can be the target of a
  /// canonical reference. It SHALL remain the same when the terminology capabilities
  /// is stored on different servers.
  url: Option<String>,

  /// Extensions for purpose
  #[serde(rename = "_purpose")]
  _purpose: Option<Element>,

  /// A copyright statement relating to the terminology capabilities and/or its
  /// contents. Copyright statements are generally legal restrictions on the use and
  /// publishing of the terminology capabilities.
  copyright: Option<String>,

  /// The way that this statement is intended to be used, to describe an actual
  /// running instance of software, a particular product (kind, not instance of
  /// software) or a class of implementation (e.g. a desired purchase).
  kind: Option<String>,

  /// A Boolean value to indicate that this terminology capabilities is authored for
  /// testing purposes (or education/evaluation/marketing) and is not intended to be
  /// used for genuine usage.
  experimental: Option<bool>,

  /// Extensions for lockedDate
  #[serde(rename = "_lockedDate")]
  _locked_date: Option<Element>,

  /// Explanation of why this terminology capabilities is needed and why it has been
  /// designed as it has.
  purpose: Option<String>,

  /// The degree to which the server supports the code search parameter on ValueSet,
  /// if it is supported.
  #[serde(rename = "codeSearch")]
  code_search: Option<TerminologyCapabilitiesCodeSearch>,

  /// Information about the [ValueSet/$validate-code](valueset-operation-validate-
  /// code.html) operation.
  #[serde(rename = "validateCode")]
  validate_code: Option<TerminologyCapabilities_ValidateCode>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// Extensions for publisher
  #[serde(rename = "_publisher")]
  _publisher: Option<Element>,

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

  /// A natural language name identifying the terminology capabilities. This name
  /// should be usable as an identifier for the module by machine processing
  /// applications such as code generation.
  name: Option<String>,

  /// A legal or geographic region in which the terminology capabilities is intended
  /// to be used.
  jurisdiction: Option<Vec<CodeableConcept>>,

  /// Extensions for url
  #[serde(rename = "_url")]
  _url: Option<Element>,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate terminology
  /// capabilities instances.
  #[serde(rename = "useContext")]
  use_context: Option<Vec<UsageContext>>,

  /// Extensions for experimental
  #[serde(rename = "_experimental")]
  _experimental: Option<Element>,

  /// Extensions for version
  #[serde(rename = "_version")]
  _version: Option<Element>,

  /// Extensions for title
  #[serde(rename = "_title")]
  _title: Option<Element>,

  /// Identifies a specific implementation instance that is described by the
  /// terminology capability statement - i.e. a particular installation, rather than
  /// the capabilities of a software program.
  implementation: Option<TerminologyCapabilities_Implementation>,

  /// Whether the $closure operation is supported.
  closure: Option<TerminologyCapabilities_Closure>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Identifies a code system that is supported by the server. If there is a no code
  /// system URL, then this declares the general assumptions a client can make about
  /// support for any CodeSystem resource.
  #[serde(rename = "codeSystem")]
  code_system: Option<Vec<TerminologyCapabilities_CodeSystem>>,

  /// Extensions for codeSearch
  #[serde(rename = "_codeSearch")]
  _code_search: Option<Element>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

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

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// A short, descriptive, user-friendly title for the terminology capabilities.
  title: Option<String>,

  /// The date  (and optionally time) when the terminology capabilities was published.
  /// The date must change when the business version changes and it must change if the
  /// status code changes. In addition, it should change when the substantive content
  /// of the terminology capabilities changes.
  date: Option<String>,

  /// The status of this terminology capabilities. Enables tracking the life-cycle of
  /// the content.
  status: Option<TerminologyCapabilitiesStatus>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// Information about the [ValueSet/$expand](valueset-operation-expand.html)
  /// operation.
  expansion: Option<TerminologyCapabilities_Expansion>,

  /// The name of the organization or individual that published the terminology
  /// capabilities.
  publisher: Option<String>,

  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  contact: Option<Vec<ContactDetail>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum TerminologyCapabilitiesCodeSearch {
  #[serde(rename = "explicit")]
  Explicit,

  #[serde(rename = "all")]
  All,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum TerminologyCapabilitiesStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "unknown")]
  Unknown,

}
