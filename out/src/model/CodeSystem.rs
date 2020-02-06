#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ContactDetail::ContactDetail;
use crate::model::Element::Element;
use crate::model::CodeSystem_Concept::CodeSystem_Concept;
use crate::model::Identifier::Identifier;
use crate::model::ResourceList::ResourceList;
use crate::model::Narrative::Narrative;
use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::UsageContext::UsageContext;
use crate::model::CodeSystem_Property::CodeSystem_Property;
use crate::model::Meta::Meta;
use crate::model::CodeSystem_Filter::CodeSystem_Filter;


/// The CodeSystem resource is used to declare the existence of and describe a code
/// system or code system supplement and its key properties, and optionally define a
/// part or all of its content.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystem {
  /// Explanation of why this code system is needed and why it has been designed as it
  /// has.
  purpose: String,

  /// Extensions for url
  _url: Element,

  /// Extensions for description
  _description: Element,

  /// The base language in which the resource is written.
  language: String,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// A property defines an additional slot through which additional information can
  /// be provided about a concept.
  property: Vec<CodeSystem_Property>,

  /// An absolute URI that is used to identify this code system when it is referenced
  /// in a specification, model, design or an instance; also called its canonical
  /// identifier. This SHOULD be globally unique and SHOULD be a literal address at
  /// which at which an authoritative instance of this code system is (or will be)
  /// published. This URL can be the target of a canonical reference. It SHALL remain
  /// the same when the code system is stored on different servers. This is used in
  /// [Coding](datatypes.html#Coding).system.
  url: String,

  /// The identifier that is used to identify this version of the code system when it
  /// is referenced in a specification, model, design or instance. This is an
  /// arbitrary value managed by the code system author and is not expected to be
  /// globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
  /// managed version is not available. There is also no expectation that versions can
  /// be placed in a lexicographical sequence. This is used in
  /// [Coding](datatypes.html#Coding).version.
  version: String,

  /// Extensions for purpose
  _purpose: Element,

  /// If code comparison is case sensitive when codes within this system are compared
  /// to each other.
  #[serde(rename = "caseSensitive")]
  case_sensitive: bool,

  /// Extensions for caseSensitive
  #[serde(rename = "_caseSensitive")]
  _case_sensitive: Element,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Extensions for title
  _title: Element,

  /// Extensions for language
  _language: Element,

  /// Extensions for experimental
  _experimental: Element,

  /// Extensions for date
  _date: Element,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  contact: Vec<ContactDetail>,

  /// Extensions for status
  _status: Element,

  /// Canonical reference to the value set that contains the entire code system.
  #[serde(rename = "valueSet")]
  value_set: String,

  /// Extensions for hierarchyMeaning
  #[serde(rename = "_hierarchyMeaning")]
  _hierarchy_meaning: Element,

  /// The total number of concepts defined by the code system. Where the code system
  /// has a compositional grammar, the basis of this count is defined by the system
  /// steward.
  count: u32,

  /// A filter that can be used in a value set compose statement when selecting
  /// concepts using a filter.
  filter: Vec<CodeSystem_Filter>,

  /// The meaning of the hierarchy of concepts as represented in this resource.
  #[serde(rename = "hierarchyMeaning")]
  hierarchy_meaning: CodeSystemHierarchyMeaning,

  /// The extent of the content of the code system (the concepts and codes it defines)
  /// are represented in this resource instance.
  content: CodeSystemContent,

  /// Extensions for compositional
  _compositional: Element,

  /// The name of the organization or individual that published the code system.
  publisher: String,

  /// A formal identifier that is used to identify this code system when it is
  /// represented in other formats, or referenced in a specification, model, design or
  /// an instance.
  identifier: Vec<Identifier>,

  /// A short, descriptive, user-friendly title for the code system.
  title: String,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Extensions for version
  _version: Element,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// The date  (and optionally time) when the code system was published. The date
  /// must change when the business version changes and it must change if the status
  /// code changes. In addition, it should change when the substantive content of the
  /// code system changes.
  date: String,

  /// Extensions for count
  _count: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// A copyright statement relating to the code system and/or its contents. Copyright
  /// statements are generally legal restrictions on the use and publishing of the
  /// code system.
  copyright: String,

  /// A Boolean value to indicate that this code system is authored for testing
  /// purposes (or education/evaluation/marketing) and is not intended to be used for
  /// genuine usage.
  experimental: bool,

  /// The canonical URL of the code system that this code system supplement is adding
  /// designations and properties to.
  supplements: String,

  /// The code system defines a compositional (post-coordination) grammar.
  compositional: bool,

  /// The date (and optionally time) when the code system resource was created or
  /// revised.
  status: CodeSystemStatus,

  /// Extensions for content
  _content: Element,

  /// This flag is used to signify that the code system does not commit to concept
  /// permanence across versions. If true, a version must be specified when
  /// referencing this code system.
  #[serde(rename = "versionNeeded")]
  version_needed: bool,

  /// Concepts that are in the code system. The concept definitions are inherently
  /// hierarchical, but the definitions must be consulted to determine what the
  /// meanings of the hierarchical relationships are.
  concept: Vec<CodeSystem_Concept>,

  /// Extensions for copyright
  _copyright: Element,

  /// Extensions for name
  _name: Element,

  /// A free text natural language description of the code system from a consumer's
  /// perspective.
  description: String,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate code system
  /// instances.
  #[serde(rename = "useContext")]
  use_context: Vec<UsageContext>,

  /// Extensions for publisher
  _publisher: Element,

  /// Extensions for versionNeeded
  #[serde(rename = "_versionNeeded")]
  _version_needed: Element,

  /// A legal or geographic region in which the code system is intended to be used.
  jurisdiction: Vec<CodeableConcept>,

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
  modifier_extension: Vec<Extension>,

  /// A natural language name identifying the code system. This name should be usable
  /// as an identifier for the module by machine processing applications such as code
  /// generation.
  name: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum CodeSystemHierarchyMeaning {
  #[serde(rename = "grouped-by")]
  GroupedBy,

  #[serde(rename = "is-a")]
  IsA,

  #[serde(rename = "part-of")]
  PartOf,

  #[serde(rename = "classified-with")]
  ClassifiedWith,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum CodeSystemContent {
  #[serde(rename = "not-present")]
  NotPresent,

  #[serde(rename = "example")]
  Example,

  #[serde(rename = "fragment")]
  Fragment,

  #[serde(rename = "complete")]
  Complete,

  #[serde(rename = "supplement")]
  Supplement,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum CodeSystemStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "unknown")]
  Unknown,

}
