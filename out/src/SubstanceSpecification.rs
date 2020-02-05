use serde::{Deserialize, Serialize};

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SubstanceSpecification {
  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// Codes associated with the substance.
  code: Vec<SubstanceSpecification_Code>,

  /// Data items specific to proteins.
  protein: Reference,

  /// If the substance applies to only human or veterinary use.
  domain: CodeableConcept,

  /// Identifier by which this substance is known.
  identifier: Identifier,

  /// General specifications for this substance, including how it is related to other
  /// substances.
  property: Vec<SubstanceSpecification_Property>,

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

  /// Status of substance within the catalogue e.g. approved.
  status: CodeableConcept,

  /// Structural information.
  structure: SubstanceSpecification_Structure,

  /// Extensions for language
  _language: Element,

  /// The molecular weight or weight range (for proteins, polymers or nucleic acids).
  #[serde(rename = "molecularWeight")]
  molecular_weight: Vec<SubstanceSpecification_MolecularWeight>,

  /// The base language in which the resource is written.
  language: String,

  /// Extensions for comment
  _comment: Element,

  /// Extensions for description
  _description: Element,

  /// Supporting literature.
  source: Vec<Reference>,

  /// Names applicable to this substance.
  name: Vec<SubstanceSpecification_Name>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// High level categorization, e.g. polymer or nucleic acid.
  type: CodeableConcept,

  /// Data items specific to nucleic acids.
  #[serde(rename = "nucleicAcid")]
  nucleic_acid: Reference,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Textual description of the substance.
  description: String,

  /// Data items specific to polymers.
  polymer: Reference,

  /// General information detailing this substance.
  #[serde(rename = "referenceInformation")]
  reference_information: Reference,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Textual comment about this record of a substance.
  comment: String,

  /// Material or taxonomic/anatomical source for the substance.
  #[serde(rename = "sourceMaterial")]
  source_material: Reference,

  /// Moiety, for structural modifications.
  moiety: Vec<SubstanceSpecification_Moiety>,

  /// A link between this substance and another, with details of the relationship.
  relationship: Vec<SubstanceSpecification_Relationship>,

}
