use serde::{Deserialize, Serialize};

/// A definition of a FHIR structure. This resource is used to describe the
/// underlying resources, data types defined in FHIR, and also for describing
/// extensions and constraints on resources and data types.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StructureDefinition {
  /// Extensions for status
  _status: Element,

  /// The name of the organization or individual that published the structure
  /// definition.
  publisher: String,

  /// Extensions for contextInvariant
  #[serde(rename = "_contextInvariant")]
  _context_invariant: Vec<Element>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

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

  /// The identifier that is used to identify this version of the structure definition
  /// when it is referenced in a specification, model, design or instance. This is an
  /// arbitrary value managed by the structure definition author and is not expected
  /// to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
  /// managed version is not available. There is also no expectation that versions can
  /// be placed in a lexicographical sequence.
  version: String,

  /// The status of this structure definition. Enables tracking the life-cycle of the
  /// content.
  status: StructureDefinitionStatus,

  /// The date  (and optionally time) when the structure definition was published. The
  /// date must change when the business version changes and it must change if the
  /// status code changes. In addition, it should change when the substantive content
  /// of the structure definition changes.
  date: dateTime,

  /// A free text natural language description of the structure definition from a
  /// consumer's perspective.
  description: markdown,

  /// A set of key words or terms from external terminologies that may be used to
  /// assist with indexing and searching of templates nby describing the use of this
  /// structure definition, or the content it describes.
  keyword: Vec<Coding>,

  /// Extensions for experimental
  _experimental: Element,

  /// Defines the kind of structure that this definition is describing.
  kind: StructureDefinitionKind,

  /// Whether structure this definition describes is abstract or not  - that is,
  /// whether the structure is not intended to be instantiated. For Resources and Data
  /// types, abstract types will never be exchanged  between systems.
  abstract: bool,

  /// An absolute URI that is the base structure from which this type is derived,
  /// either by specialization or constraint.
  #[serde(rename = "baseDefinition")]
  base_definition: canonical,

  /// Extensions for copyright
  _copyright: Element,

  /// A copyright statement relating to the structure definition and/or its contents.
  /// Copyright statements are generally legal restrictions on the use and publishing
  /// of the structure definition.
  copyright: markdown,

  /// How the type relates to the baseDefinition.
  derivation: StructureDefinitionDerivation,

  /// A snapshot view is expressed in a standalone form that can be used and
  /// interpreted without considering the base StructureDefinition.
  snapshot: StructureDefinition_Snapshot,

  /// The version of the FHIR specification on which this StructureDefinition is based
  /// - this is the formal version of the specification, without the revision number,
  /// e.g. [publication].[major].[minor], which is 4.0.1. for this version.
  #[serde(rename = "fhirVersion")]
  fhir_version: StructureDefinitionFhirVersion,

  /// Extensions for title
  _title: Element,

  /// Extensions for fhirVersion
  #[serde(rename = "_fhirVersion")]
  _fhir_version: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// A formal identifier that is used to identify this structure definition when it
  /// is represented in other formats, or referenced in a specification, model, design
  /// or an instance.
  identifier: Vec<Identifier>,

  /// A set of rules as FHIRPath Invariants about when the extension can be used (e.g.
  /// co-occurrence variants for the extension). All the rules must be true.
  #[serde(rename = "contextInvariant")]
  context_invariant: Vec<String>,

  /// Extensions for derivation
  _derivation: Element,

  /// A differential view is expressed relative to the base StructureDefinition - a
  /// statement of differences that it applies.
  differential: StructureDefinition_Differential,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// An external specification that the content is mapped to.
  mapping: Vec<StructureDefinition_Mapping>,

  /// A short, descriptive, user-friendly title for the structure definition.
  title: String,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Extensions for url
  _url: Element,

  /// A legal or geographic region in which the structure definition is intended to be
  /// used.
  jurisdiction: Vec<CodeableConcept>,

  /// Identifies the types of resource or data type elements to which the extension
  /// can be applied.
  context: Vec<StructureDefinition_Context>,

  /// Extensions for type
  _type: Element,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Extensions for version
  _version: Element,

  /// Extensions for name
  _name: Element,

  /// Extensions for language
  _language: Element,

  /// A Boolean value to indicate that this structure definition is authored for
  /// testing purposes (or education/evaluation/marketing) and is not intended to be
  /// used for genuine usage.
  experimental: bool,

  /// A natural language name identifying the structure definition. This name should
  /// be usable as an identifier for the module by machine processing applications
  /// such as code generation.
  name: String,

  /// Extensions for publisher
  _publisher: Element,

  /// Extensions for description
  _description: Element,

  /// Explanation of why this structure definition is needed and why it has been
  /// designed as it has.
  purpose: markdown,

  /// Extensions for kind
  _kind: Element,

  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  contact: Vec<ContactDetail>,

  /// Extensions for abstract
  _abstract: Element,

  /// An absolute URI that is used to identify this structure definition when it is
  /// referenced in a specification, model, design or an instance; also called its
  /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
  /// address at which at which an authoritative instance of this structure definition
  /// is (or will be) published. This URL can be the target of a canonical reference.
  /// It SHALL remain the same when the structure definition is stored on different
  /// servers.
  url: String,

  /// Extensions for purpose
  _purpose: Element,

  /// The type this structure describes. If the derivation kind is 'specialization'
  /// then this is the master definition for a type, and there is always one of these
  /// (a data type, an extension, a resource, including abstract ones). Otherwise the
  /// structure definition is a constraint on the stated type (and in this case, the
  /// type cannot be an abstract type).  References are URLs that are relative to
  /// http://hl7.org/fhir/StructureDefinition e.g. "string" is a reference to
  /// http://hl7.org/fhir/StructureDefinition/string. Absolute URLs are only allowed
  /// in logical models.
  type: String,

  /// The base language in which the resource is written.
  language: String,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate structure
  /// definition instances.
  #[serde(rename = "useContext")]
  use_context: Vec<UsageContext>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

}

#[derive(Debug, Serialize, Deserialize)]
enum StructureDefinitionStatus {
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
enum StructureDefinitionKind {
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
enum StructureDefinitionDerivation {
  #[serde(rename = "specialization")]
  Specialization,

  #[serde(rename = "constraint")]
  Constraint,

}

#[derive(Debug, Serialize, Deserialize)]
enum StructureDefinitionFhirVersion {
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
