#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Reference::Reference;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Identifier::Identifier;
use crate::model::SubstanceSpecification_MolecularWeight::SubstanceSpecification_MolecularWeight;
use crate::model::SubstanceSpecification_Code::SubstanceSpecification_Code;
use crate::model::SubstanceSpecification_Name::SubstanceSpecification_Name;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::SubstanceSpecification_Moiety::SubstanceSpecification_Moiety;
use crate::model::SubstanceSpecification_Structure::SubstanceSpecification_Structure;
use crate::model::Narrative::Narrative;
use crate::model::SubstanceSpecification_Property::SubstanceSpecification_Property;
use crate::model::ResourceList::ResourceList;
use crate::model::SubstanceSpecification_Relationship::SubstanceSpecification_Relationship;


/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSpecification {
  /// Names applicable to this substance.
  name: Option<Vec<SubstanceSpecification_Name>>,

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

  /// Supporting literature.
  source: Option<Vec<Box<Reference>>>,

  /// The molecular weight or weight range (for proteins, polymers or nucleic acids).
  #[serde(rename = "molecularWeight")]
  molecular_weight: Option<Vec<SubstanceSpecification_MolecularWeight>>,

  /// Status of substance within the catalogue e.g. approved.
  status: Option<CodeableConcept>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Identifier by which this substance is known.
  identifier: Option<Identifier>,

  /// Codes associated with the substance.
  code: Option<Vec<SubstanceSpecification_Code>>,

  /// Extensions for comment
  #[serde(rename = "_comment")]
  _comment: Option<Element>,

  /// General specifications for this substance, including how it is related to other
  /// substances.
  property: Option<Vec<SubstanceSpecification_Property>>,

  /// High level categorization, e.g. polymer or nucleic acid.
  #[serde(rename = "type")]
  fhir_type: Option<CodeableConcept>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Structural information.
  structure: Option<SubstanceSpecification_Structure>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Moiety, for structural modifications.
  moiety: Option<Vec<SubstanceSpecification_Moiety>>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Data items specific to nucleic acids.
  #[serde(rename = "nucleicAcid")]
  nucleic_acid: Option<Box<Reference>>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Textual comment about this record of a substance.
  comment: Option<String>,

  /// If the substance applies to only human or veterinary use.
  domain: Option<CodeableConcept>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// Data items specific to proteins.
  protein: Option<Box<Reference>>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Textual description of the substance.
  description: Option<String>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// General information detailing this substance.
  #[serde(rename = "referenceInformation")]
  reference_information: Option<Box<Reference>>,

  /// Data items specific to polymers.
  polymer: Option<Box<Reference>>,

  /// A link between this substance and another, with details of the relationship.
  relationship: Option<Vec<SubstanceSpecification_Relationship>>,

  /// Material or taxonomic/anatomical source for the substance.
  #[serde(rename = "sourceMaterial")]
  source_material: Option<Box<Reference>>,

}
