#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::SearchParameter_Component::SearchParameter_Component;
use crate::model::ContactDetail::ContactDetail;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Narrative::Narrative;
use crate::model::ResourceList::ResourceList;
use crate::model::Meta::Meta;
use crate::model::UsageContext::UsageContext;


/// A search parameter that defines a named search item that can be used to
/// search/filter on a resource.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchParameter {
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

  /// A natural language name identifying the search parameter. This name should be
  /// usable as an identifier for the module by machine processing applications such
  /// as code generation.
  name: String,

  /// Where this search parameter is originally defined. If a derivedFrom is provided,
  /// then the details in the search parameter must be consistent with the definition
  /// from which it is defined. i.e. the parameter should have the same meaning, and
  /// (usually) the functionality should be a proper subset of the underlying search
  /// parameter.
  #[serde(rename = "derivedFrom")]
  derived_from: String,

  /// How the search parameter relates to the set of elements returned by evaluating
  /// the xpath query.
  #[serde(rename = "xpathUsage")]
  xpath_usage: SearchParameterXpathUsage,

  /// Extensions for modifier
  _modifier: Vec<Element>,

  /// Extensions for version
  _version: Element,

  /// Contains the names of any search parameters which may be chained to the
  /// containing search parameter. Chained parameters may be added to search
  /// parameters of type reference and specify that resources will only be returned if
  /// they contain a reference to a resource which matches the chained parameter
  /// value. Values for this field should be drawn from SearchParameter.code for a
  /// parameter on the target resource type.
  chain: Vec<String>,

  /// The name of the organization or individual that published the search parameter.
  publisher: String,

  /// Extensions for status
  _status: Element,

  /// A legal or geographic region in which the search parameter is intended to be
  /// used.
  jurisdiction: Vec<CodeableConcept>,

  /// Explanation of why this search parameter is needed and why it has been designed
  /// as it has.
  purpose: String,

  /// The code used in the URL or the parameter name in a parameters resource for this
  /// search parameter.
  code: String,

  /// Extensions for name
  _name: Element,

  /// Extensions for base
  _base: Vec<Element>,

  /// Whether multiple parameters are allowed - e.g. more than one parameter with the
  /// same name. The search matches if all the parameters match.
  #[serde(rename = "multipleAnd")]
  multiple_and: bool,

  /// Extensions for publisher
  _publisher: Element,

  /// Extensions for language
  _language: Element,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Extensions for xpath
  _xpath: Element,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Extensions for multipleOr
  #[serde(rename = "_multipleOr")]
  _multiple_or: Element,

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

  /// The base language in which the resource is written.
  language: String,

  /// Extensions for experimental
  _experimental: Element,

  /// Extensions for date
  _date: Element,

  /// A FHIRPath expression that returns a set of elements for the search parameter.
  expression: String,

  /// The base resource type(s) that this search parameter can be used against.
  base: Vec<String>,

  /// Extensions for expression
  _expression: Element,

  /// Extensions for type
  _type: Element,

  /// The identifier that is used to identify this version of the search parameter
  /// when it is referenced in a specification, model, design or instance. This is an
  /// arbitrary value managed by the search parameter author and is not expected to be
  /// globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
  /// managed version is not available. There is also no expectation that versions can
  /// be placed in a lexicographical sequence.
  version: String,

  /// Extensions for code
  _code: Element,

  /// A Boolean value to indicate that this search parameter is authored for testing
  /// purposes (or education/evaluation/marketing) and is not intended to be used for
  /// genuine usage.
  experimental: bool,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// An absolute URI that is used to identify this search parameter when it is
  /// referenced in a specification, model, design or an instance; also called its
  /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
  /// address at which at which an authoritative instance of this search parameter is
  /// (or will be) published. This URL can be the target of a canonical reference. It
  /// SHALL remain the same when the search parameter is stored on different servers.
  url: String,

  /// Used to define the parts of a composite search parameter.
  component: Vec<SearchParameter_Component>,

  /// Extensions for xpathUsage
  #[serde(rename = "_xpathUsage")]
  _xpath_usage: Element,

  /// The date  (and optionally time) when the search parameter was published. The
  /// date must change when the business version changes and it must change if the
  /// status code changes. In addition, it should change when the substantive content
  /// of the search parameter changes.
  date: String,

  /// And how it used.
  description: String,

  /// Extensions for multipleAnd
  #[serde(rename = "_multipleAnd")]
  _multiple_and: Element,

  /// Extensions for chain
  _chain: Vec<Element>,

  /// Extensions for purpose
  _purpose: Element,

  /// Extensions for description
  _description: Element,

  /// Extensions for comparator
  _comparator: Vec<Element>,

  /// The type of value that a search parameter may contain, and how the content is
  /// interpreted.
  #[serde(rename = "type")]
  fhir_type: SearchParameterType,

  /// The status of this search parameter. Enables tracking the life-cycle of the
  /// content.
  status: SearchParameterStatus,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Whether multiple values are allowed for each time the parameter exists. Values
  /// are separated by commas, and the parameter matches if any of the values match.
  #[serde(rename = "multipleOr")]
  multiple_or: bool,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate search parameter
  /// instances.
  #[serde(rename = "useContext")]
  use_context: Vec<UsageContext>,

  /// Extensions for url
  _url: Element,

  /// Types of resource (if a resource is referenced).
  target: Vec<String>,

  /// Extensions for target
  _target: Vec<Element>,

  /// An XPath expression that returns a set of elements for the search parameter.
  xpath: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum SearchParameterXpathUsage {
  #[serde(rename = "normal")]
  Normal,

  #[serde(rename = "phonetic")]
  Phonetic,

  #[serde(rename = "nearby")]
  Nearby,

  #[serde(rename = "distance")]
  Distance,

  #[serde(rename = "other")]
  Other,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum SearchParameterType {
  #[serde(rename = "number")]
  Number,

  #[serde(rename = "date")]
  Date,

  #[serde(rename = "string")]
  String,

  #[serde(rename = "token")]
  Token,

  #[serde(rename = "reference")]
  Reference,

  #[serde(rename = "composite")]
  Composite,

  #[serde(rename = "quantity")]
  Quantity,

  #[serde(rename = "uri")]
  Uri,

  #[serde(rename = "special")]
  Special,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum SearchParameterStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "unknown")]
  Unknown,

}
