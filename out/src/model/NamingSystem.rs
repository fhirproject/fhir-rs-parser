#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::Narrative::Narrative;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ResourceList::ResourceList;
use crate::model::Meta::Meta;
use crate::model::ContactDetail::ContactDetail;
use crate::model::UsageContext::UsageContext;
use crate::model::NamingSystem_UniqueId::NamingSystem_UniqueId;


/// A curated namespace that issues unique symbols within that namespace for the
/// identification of concepts, people, devices, etc.  Represents a "System" used
/// within the Identifier and Coding data types.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamingSystem {
  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// The name of the organization or individual that published the naming system.
  publisher: String,

  /// Extensions for publisher
  _publisher: Element,

  /// The base language in which the resource is written.
  language: String,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// The name of the organization that is responsible for issuing identifiers or
  /// codes for this namespace and ensuring their non-collision.
  responsible: String,

  /// Extensions for responsible
  _responsible: Element,

  /// Extensions for usage
  _usage: Element,

  /// Extensions for language
  _language: Element,

  /// Extensions for description
  _description: Element,

  /// Provides guidance on the use of the namespace, including the handling of
  /// formatting characters, use of upper vs. lower case, etc.
  usage: String,

  /// Extensions for kind
  _kind: Element,

  /// Indicates the purpose for the naming system - what kinds of things does it make
  /// unique?
  kind: NamingSystemKind,

  /// Extensions for name
  _name: Element,

  /// The status of this naming system. Enables tracking the life-cycle of the
  /// content.
  status: NamingSystemStatus,

  /// Categorizes a naming system for easier search by grouping related naming
  /// systems.
  #[serde(rename = "type")]
  fhir_type: CodeableConcept,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// A legal or geographic region in which the naming system is intended to be used.
  jurisdiction: Vec<CodeableConcept>,

  /// Indicates how the system may be identified when referenced in electronic
  /// exchange.
  #[serde(rename = "uniqueId")]
  unique_id: Vec<NamingSystem_UniqueId>,

  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  contact: Vec<ContactDetail>,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate naming system
  /// instances.
  #[serde(rename = "useContext")]
  use_context: Vec<UsageContext>,

  /// The date  (and optionally time) when the naming system was published. The date
  /// must change when the business version changes and it must change if the status
  /// code changes. In addition, it should change when the substantive content of the
  /// naming system changes.
  date: String,

  /// Extensions for date
  _date: Element,

  /// A natural language name identifying the naming system. This name should be
  /// usable as an identifier for the module by machine processing applications such
  /// as code generation.
  name: String,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for status
  _status: Element,

  /// A free text natural language description of the naming system from a consumer's
  /// perspective. Details about what the namespace identifies including scope,
  /// granularity, version labeling, etc.
  description: String,

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

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum NamingSystemKind {
  #[serde(rename = "codesystem")]
  Codesystem,

  #[serde(rename = "identifier")]
  Identifier,

  #[serde(rename = "root")]
  Root,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum NamingSystemStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "unknown")]
  Unknown,

}
