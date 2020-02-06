#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::UsageContext::UsageContext;
use crate::model::ValueSet_Compose::ValueSet_Compose;
use crate::model::Meta::Meta;
use crate::model::ContactDetail::ContactDetail;
use crate::model::ValueSet_Expansion::ValueSet_Expansion;
use crate::model::Extension::Extension;
use crate::model::ResourceList::ResourceList;
use crate::model::Narrative::Narrative;
use crate::model::Identifier::Identifier;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;


/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [[[CodeSystem]]] definitions and their use in [coded
/// elements](terminologies.html).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSet {
  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// A legal or geographic region in which the value set is intended to be used.
  jurisdiction: Option<Vec<CodeableConcept>>,

  /// A formal identifier that is used to identify this value set when it is
  /// represented in other formats, or referenced in a specification, model, design or
  /// an instance.
  identifier: Option<Vec<Identifier>>,

  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  contact: Option<Vec<ContactDetail>>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Extensions for title
  #[serde(rename = "_title")]
  _title: Option<Element>,

  /// Extensions for date
  #[serde(rename = "_date")]
  _date: Option<Element>,

  /// Extensions for immutable
  #[serde(rename = "_immutable")]
  _immutable: Option<Element>,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate value set
  /// instances.
  #[serde(rename = "useContext")]
  use_context: Option<Vec<UsageContext>>,

  /// A value set can also be "expanded", where the value set is turned into a simple
  /// collection of enumerated codes. This element holds the expansion, if it has been
  /// performed.
  expansion: Option<ValueSet_Expansion>,

  /// A set of criteria that define the contents of the value set by including or
  /// excluding codes selected from the specified code system(s) that the value set
  /// draws from. This is also known as the Content Logical Definition (CLD).
  compose: Option<ValueSet_Compose>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Extensions for copyright
  #[serde(rename = "_copyright")]
  _copyright: Option<Element>,

  /// A copyright statement relating to the value set and/or its contents. Copyright
  /// statements are generally legal restrictions on the use and publishing of the
  /// value set.
  copyright: Option<String>,

  /// Extensions for experimental
  #[serde(rename = "_experimental")]
  _experimental: Option<Element>,

  /// The identifier that is used to identify this version of the value set when it is
  /// referenced in a specification, model, design or instance. This is an arbitrary
  /// value managed by the value set author and is not expected to be globally unique.
  /// For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not
  /// available. There is also no expectation that versions can be placed in a
  /// lexicographical sequence.
  version: Option<String>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// If this is set to 'true', then no new versions of the content logical definition
  /// can be created.  Note: Other metadata might still change.
  immutable: Option<bool>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// A short, descriptive, user-friendly title for the value set.
  title: Option<String>,

  /// Extensions for url
  #[serde(rename = "_url")]
  _url: Option<Element>,

  /// Extensions for version
  #[serde(rename = "_version")]
  _version: Option<Element>,

  /// A natural language name identifying the value set. This name should be usable as
  /// an identifier for the module by machine processing applications such as code
  /// generation.
  name: Option<String>,

  /// An absolute URI that is used to identify this value set when it is referenced in
  /// a specification, model, design or an instance; also called its canonical
  /// identifier. This SHOULD be globally unique and SHOULD be a literal address at
  /// which at which an authoritative instance of this value set is (or will be)
  /// published. This URL can be the target of a canonical reference. It SHALL remain
  /// the same when the value set is stored on different servers.
  url: Option<String>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// Extensions for purpose
  #[serde(rename = "_purpose")]
  _purpose: Option<Element>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// The date (and optionally time) when the value set was created or revised (e.g.
  /// the 'content logical definition').
  date: Option<String>,

  /// Explanation of why this value set is needed and why it has been designed as it
  /// has.
  purpose: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The name of the organization or individual that published the value set.
  publisher: Option<String>,

  /// Extensions for publisher
  #[serde(rename = "_publisher")]
  _publisher: Option<Element>,

  /// The status of this value set. Enables tracking the life-cycle of the content.
  /// The status of the value set applies to the value set definition
  /// (ValueSet.compose) and the associated ValueSet metadata. Expansions do not have
  /// a state.
  status: Option<ValueSetStatus>,

  /// A Boolean value to indicate that this value set is authored for testing purposes
  /// (or education/evaluation/marketing) and is not intended to be used for genuine
  /// usage.
  experimental: Option<bool>,

  /// A free text natural language description of the value set from a consumer's
  /// perspective. The textual description specifies the span of meanings for concepts
  /// to be included within the Value Set Expansion, and also may specify the intended
  /// use and limitations of the Value Set.
  description: Option<String>,

  /// The base language in which the resource is written.
  language: Option<String>,

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

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ValueSetStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "unknown")]
  Unknown,

}
