#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Narrative::Narrative;
use crate::model::Extension::Extension;
use crate::model::ResourceList::ResourceList;
use crate::model::StructureDefinition_Differential::StructureDefinition_Differential;
use crate::model::Coding::Coding;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::StructureDefinition_Context::StructureDefinition_Context;
use crate::model::StructureDefinition_Snapshot::StructureDefinition_Snapshot;
use crate::model::Meta::Meta;
use crate::model::StructureDefinition_Mapping::StructureDefinition_Mapping;
use crate::model::Element::Element;
use crate::model::Identifier::Identifier;
use crate::model::UsageContext::UsageContext;
use crate::model::ContactDetail::ContactDetail;


/// A definition of a FHIR structure. This resource is used to describe the
/// underlying resources, data types defined in FHIR, and also for describing
/// extensions and constraints on resources and data types.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureDefinition {
  /// A short, descriptive, user-friendly title for the structure definition.
  title: Option<String>,

  /// The name of the organization or individual that published the structure
  /// definition.
  publisher: Option<String>,

  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  contact: Option<Vec<ContactDetail>>,

  /// A free text natural language description of the structure definition from a
  /// consumer's perspective.
  description: Option<String>,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate structure
  /// definition instances.
  #[serde(rename = "useContext")]
  use_context: Option<Vec<UsageContext>>,

  /// A legal or geographic region in which the structure definition is intended to be
  /// used.
  jurisdiction: Option<Vec<CodeableConcept>>,

  /// Extensions for fhirVersion
  #[serde(rename = "_fhirVersion")]
  _fhir_version: Option<Element>,

  /// A Boolean value to indicate that this structure definition is authored for
  /// testing purposes (or education/evaluation/marketing) and is not intended to be
  /// used for genuine usage.
  experimental: Option<bool>,

  /// Extensions for title
  #[serde(rename = "_title")]
  _title: Option<Element>,

  /// Explanation of why this structure definition is needed and why it has been
  /// designed as it has.
  purpose: Option<String>,

  /// Extensions for abstract
  #[serde(rename = "_abstract")]
  _abstract: Option<Element>,

  /// Identifies the types of resource or data type elements to which the extension
  /// can be applied.
  context: Option<Vec<StructureDefinition_Context>>,

  /// Extensions for derivation
  #[serde(rename = "_derivation")]
  _derivation: Option<Element>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// A snapshot view is expressed in a standalone form that can be used and
  /// interpreted without considering the base StructureDefinition.
  snapshot: Option<StructureDefinition_Snapshot>,

  /// How the type relates to the baseDefinition.
  derivation: Option<StructureDefinitionDerivation>,

  /// A set of key words or terms from external terminologies that may be used to
  /// assist with indexing and searching of templates nby describing the use of this
  /// structure definition, or the content it describes.
  keyword: Option<Vec<Coding>>,

  /// Extensions for version
  #[serde(rename = "_version")]
  _version: Option<Element>,

  /// Extensions for url
  #[serde(rename = "_url")]
  _url: Option<Element>,

  /// A natural language name identifying the structure definition. This name should
  /// be usable as an identifier for the module by machine processing applications
  /// such as code generation.
  name: Option<String>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// Extensions for experimental
  #[serde(rename = "_experimental")]
  _experimental: Option<Element>,

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

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

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

  /// Extensions for copyright
  #[serde(rename = "_copyright")]
  _copyright: Option<Element>,

  /// Defines the kind of structure that this definition is describing.
  kind: Option<StructureDefinitionKind>,

  /// The type this structure describes. If the derivation kind is 'specialization'
  /// then this is the master definition for a type, and there is always one of these
  /// (a data type, an extension, a resource, including abstract ones). Otherwise the
  /// structure definition is a constraint on the stated type (and in this case, the
  /// type cannot be an abstract type).  References are URLs that are relative to
  /// http://hl7.org/fhir/StructureDefinition e.g. "string" is a reference to
  /// http://hl7.org/fhir/StructureDefinition/string. Absolute URLs are only allowed
  /// in logical models.
  #[serde(rename = "type")]
  fhir_type: Option<String>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// An absolute URI that is used to identify this structure definition when it is
  /// referenced in a specification, model, design or an instance; also called its
  /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
  /// address at which at which an authoritative instance of this structure definition
  /// is (or will be) published. This URL can be the target of a canonical reference.
  /// It SHALL remain the same when the structure definition is stored on different
  /// servers.
  url: Option<String>,

  /// Whether structure this definition describes is abstract or not  - that is,
  /// whether the structure is not intended to be instantiated. For Resources and Data
  /// types, abstract types will never be exchanged  between systems.
  #[serde(rename = "abstract")]
  fhir_abstract: Option<bool>,

  /// A set of rules as FHIRPath Invariants about when the extension can be used (e.g.
  /// co-occurrence variants for the extension). All the rules must be true.
  #[serde(rename = "contextInvariant")]
  context_invariant: Option<Vec<String>>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// An absolute URI that is the base structure from which this type is derived,
  /// either by specialization or constraint.
  #[serde(rename = "baseDefinition")]
  base_definition: Option<String>,

  /// Extensions for date
  #[serde(rename = "_date")]
  _date: Option<Element>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// A copyright statement relating to the structure definition and/or its contents.
  /// Copyright statements are generally legal restrictions on the use and publishing
  /// of the structure definition.
  copyright: Option<String>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// Extensions for publisher
  #[serde(rename = "_publisher")]
  _publisher: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// An external specification that the content is mapped to.
  mapping: Option<Vec<StructureDefinition_Mapping>>,

  /// The status of this structure definition. Enables tracking the life-cycle of the
  /// content.
  status: Option<StructureDefinitionStatus>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Extensions for kind
  #[serde(rename = "_kind")]
  _kind: Option<Element>,

  /// Extensions for type
  #[serde(rename = "_type")]
  _type: Option<Element>,

  /// Extensions for purpose
  #[serde(rename = "_purpose")]
  _purpose: Option<Element>,

  /// The date  (and optionally time) when the structure definition was published. The
  /// date must change when the business version changes and it must change if the
  /// status code changes. In addition, it should change when the substantive content
  /// of the structure definition changes.
  date: Option<String>,

  /// Extensions for contextInvariant
  #[serde(rename = "_contextInvariant")]
  _context_invariant: Option<Vec<Element>>,

  /// The version of the FHIR specification on which this StructureDefinition is based
  /// - this is the formal version of the specification, without the revision number,
  /// e.g. [publication].[major].[minor], which is 4.0.1. for this version.
  #[serde(rename = "fhirVersion")]
  fhir_version: Option<StructureDefinitionFhirVersion>,

  /// A differential view is expressed relative to the base StructureDefinition - a
  /// statement of differences that it applies.
  differential: Option<StructureDefinition_Differential>,

  /// A formal identifier that is used to identify this structure definition when it
  /// is represented in other formats, or referenced in a specification, model, design
  /// or an instance.
  identifier: Option<Vec<Identifier>>,

  /// The identifier that is used to identify this version of the structure definition
  /// when it is referenced in a specification, model, design or instance. This is an
  /// arbitrary value managed by the structure definition author and is not expected
  /// to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
  /// managed version is not available. There is also no expectation that versions can
  /// be placed in a lexicographical sequence.
  version: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum StructureDefinitionDerivation {
  #[serde(rename = "specialization")]
  Specialization,

  #[serde(rename = "constraint")]
  Constraint,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum StructureDefinitionKind {
  #[serde(rename = "primitive-type")]
  PrimitiveType,

  #[serde(rename = "complex-type")]
  ComplexType,

  #[serde(rename = "resource")]
  Resource,

  #[serde(rename = "logical")]
  Logical,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum StructureDefinitionStatus {
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
pub enum StructureDefinitionFhirVersion {
  #[serde(rename = "0.01")]
  Fhir001,

  #[serde(rename = "0.05")]
  Fhir005,

  #[serde(rename = "0.06")]
  Fhir006,

  #[serde(rename = "0.11")]
  Fhir011,

  #[serde(rename = "0.0.80")]
  Fhir0080,

  #[serde(rename = "0.0.81")]
  Fhir0081,

  #[serde(rename = "0.0.82")]
  Fhir0082,

  #[serde(rename = "0.4.0")]
  Fhir040,

  #[serde(rename = "0.5.0")]
  Fhir050,

  #[serde(rename = "1.0.0")]
  Fhir100,

  #[serde(rename = "1.0.1")]
  Fhir101,

  #[serde(rename = "1.0.2")]
  Fhir102,

  #[serde(rename = "1.1.0")]
  Fhir110,

  #[serde(rename = "1.4.0")]
  Fhir140,

  #[serde(rename = "1.6.0")]
  Fhir160,

  #[serde(rename = "1.8.0")]
  Fhir180,

  #[serde(rename = "3.0.0")]
  Fhir300,

  #[serde(rename = "3.0.1")]
  Fhir301,

  #[serde(rename = "3.3.0")]
  Fhir330,

  #[serde(rename = "3.5.0")]
  Fhir350,

  #[serde(rename = "4.0.0")]
  Fhir400,

  #[serde(rename = "4.0.1")]
  Fhir401,

}
