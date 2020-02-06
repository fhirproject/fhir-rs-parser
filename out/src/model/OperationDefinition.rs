#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Meta::Meta;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactDetail::ContactDetail;
use crate::model::UsageContext::UsageContext;
use crate::model::Narrative::Narrative;
use crate::model::ResourceList::ResourceList;
use crate::model::Element::Element;
use crate::model::OperationDefinition_Overload::OperationDefinition_Overload;
use crate::model::Extension::Extension;
use crate::model::OperationDefinition_Parameter::OperationDefinition_Parameter;


/// A formal computable definition of an operation (on the RESTful interface) or a
/// named query (using the search interaction).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationDefinition {
  /// Extensions for version
  #[serde(rename = "_version")]
  _version: Option<Element>,

  /// The name used to invoke the operation.
  code: Option<String>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Extensions for resource
  #[serde(rename = "_resource")]
  _resource: Option<Vec<Element>>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// Extensions for url
  #[serde(rename = "_url")]
  _url: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Extensions for date
  #[serde(rename = "_date")]
  _date: Option<Element>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// The status of this operation definition. Enables tracking the life-cycle of the
  /// content.
  status: Option<OperationDefinitionStatus>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// The identifier that is used to identify this version of the operation definition
  /// when it is referenced in a specification, model, design or instance. This is an
  /// arbitrary value managed by the operation definition author and is not expected
  /// to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
  /// managed version is not available. There is also no expectation that versions can
  /// be placed in a lexicographical sequence.
  version: Option<String>,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate operation
  /// definition instances.
  #[serde(rename = "useContext")]
  use_context: Option<Vec<UsageContext>>,

  /// An absolute URI that is used to identify this operation definition when it is
  /// referenced in a specification, model, design or an instance; also called its
  /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
  /// address at which at which an authoritative instance of this operation definition
  /// is (or will be) published. This URL can be the target of a canonical reference.
  /// It SHALL remain the same when the operation definition is stored on different
  /// servers.
  url: Option<String>,

  /// A free text natural language description of the operation definition from a
  /// consumer's perspective.
  description: Option<String>,

  /// Whether the operation affects state. Side effects such as producing audit trail
  /// entries do not count as 'affecting  state'.
  #[serde(rename = "affectsState")]
  affects_state: Option<bool>,

  /// Additional information about how to use this operation or named query.
  comment: Option<String>,

  /// Indicates whether this operation or named query can be invoked at the system
  /// level (e.g. without needing to choose a resource type for the context).
  system: Option<bool>,

  /// Extensions for title
  #[serde(rename = "_title")]
  _title: Option<Element>,

  /// Whether this is an operation or a named query.
  kind: Option<OperationDefinitionKind>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// Extensions for experimental
  #[serde(rename = "_experimental")]
  _experimental: Option<Element>,

  /// Extensions for system
  #[serde(rename = "_system")]
  _system: Option<Element>,

  /// Extensions for instance
  #[serde(rename = "_instance")]
  _instance: Option<Element>,

  /// Explanation of why this operation definition is needed and why it has been
  /// designed as it has.
  purpose: Option<String>,

  /// Extensions for kind
  #[serde(rename = "_kind")]
  _kind: Option<Element>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

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

  /// The name of the organization or individual that published the operation
  /// definition.
  publisher: Option<String>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// Extensions for type
  #[serde(rename = "_type")]
  _type: Option<Element>,

  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  contact: Option<Vec<ContactDetail>>,

  /// A legal or geographic region in which the operation definition is intended to be
  /// used.
  jurisdiction: Option<Vec<CodeableConcept>>,

  /// Indicates whether this operation can be invoked on a particular instance of one
  /// of the given types.
  instance: Option<bool>,

  /// The date  (and optionally time) when the operation definition was published. The
  /// date must change when the business version changes and it must change if the
  /// status code changes. In addition, it should change when the substantive content
  /// of the operation definition changes.
  date: Option<String>,

  /// A natural language name identifying the operation definition. This name should
  /// be usable as an identifier for the module by machine processing applications
  /// such as code generation.
  name: Option<String>,

  /// Extensions for affectsState
  #[serde(rename = "_affectsState")]
  _affects_state: Option<Element>,

  /// Extensions for code
  #[serde(rename = "_code")]
  _code: Option<Element>,

  /// Extensions for comment
  #[serde(rename = "_comment")]
  _comment: Option<Element>,

  /// The types on which this operation can be executed.
  resource: Option<Vec<String>>,

  /// Additional validation information for the out parameters - a single profile that
  /// covers all the parameters. The profile is a constraint on the parameters
  /// resource.
  #[serde(rename = "outputProfile")]
  output_profile: Option<String>,

  /// Extensions for purpose
  #[serde(rename = "_purpose")]
  _purpose: Option<Element>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// A short, descriptive, user-friendly title for the operation definition.
  title: Option<String>,

  /// A Boolean value to indicate that this operation definition is authored for
  /// testing purposes (or education/evaluation/marketing) and is not intended to be
  /// used for genuine usage.
  experimental: Option<bool>,

  /// Indicates that this operation definition is a constraining profile on the base.
  base: Option<String>,

  /// The parameters for the operation/query.
  parameter: Option<Vec<OperationDefinition_Parameter>>,

  /// Defines an appropriate combination of parameters to use when invoking this
  /// operation, to help code generators when generating overloaded parameter sets for
  /// this operation.
  overload: Option<Vec<OperationDefinition_Overload>>,

  /// Additional validation information for the in parameters - a single profile that
  /// covers all the parameters. The profile is a constraint on the parameters
  /// resource as a whole.
  #[serde(rename = "inputProfile")]
  input_profile: Option<String>,

  /// Extensions for publisher
  #[serde(rename = "_publisher")]
  _publisher: Option<Element>,

  /// Indicates whether this operation or named query can be invoked at the resource
  /// type level for any given resource type level (e.g. without needing to choose a
  /// specific resource id for the context).
  #[serde(rename = "type")]
  fhir_type: Option<bool>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum OperationDefinitionStatus {
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
pub enum OperationDefinitionKind {
  #[serde(rename = "operation")]
  Operation,

  #[serde(rename = "query")]
  Query,

}
