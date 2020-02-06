#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ResourceList::ResourceList;
use crate::model::SubstanceSourceMaterial_Organism::SubstanceSourceMaterial_Organism;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Identifier::Identifier;
use crate::model::SubstanceSourceMaterial_PartDescription::SubstanceSourceMaterial_PartDescription;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::SubstanceSourceMaterial_FractionDescription::SubstanceSourceMaterial_FractionDescription;
use crate::model::Narrative::Narrative;


/// Source material shall capture information on the taxonomic and anatomical
/// origins as well as the fraction of a material that can result in or can be
/// modified to form a substance. This set of data elements shall be used to define
/// polymer substances isolated from biological matrices. Taxonomic and anatomical
/// origins shall be described using a controlled vocabulary as required. This
/// information is captured for naturally derived polymers ( . starch) and
/// structurally diverse substances. For Organisms belonging to the Kingdom Plantae
/// the Substance level defines the fresh material of a single species or
/// infraspecies, the Herbal Drug and the Herbal preparation. For Herbal
/// preparations, the fraction information will be captured at the Substance
/// information level and additional information for herbal extracts will be
/// captured at the Specified Substance Group 1 information level. See for further
/// explanation the Substance Class: Structurally Diverse and the herbal annex.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSourceMaterial {
  /// To do.
  #[serde(rename = "partDescription")]
  part_description: Option<Vec<SubstanceSourceMaterial_PartDescription>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// The parent of the herbal drug Ginkgo biloba, Leaf is the substance ID of the
  /// substance (fresh) of Ginkgo biloba L. or Ginkgo biloba L. (Whole plant).
  #[serde(rename = "parentSubstanceId")]
  parent_substance_id: Option<Vec<Identifier>>,

  /// The state of the source material when extracted.
  #[serde(rename = "sourceMaterialState")]
  source_material_state: Option<CodeableConcept>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// The type of the source material shall be specified based on a controlled
  /// vocabulary. For vaccines, this subclause refers to the class of infectious
  /// agent.
  #[serde(rename = "sourceMaterialType")]
  source_material_type: Option<CodeableConcept>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// The parent substance of the Herbal Drug, or Herbal preparation.
  #[serde(rename = "parentSubstanceName")]
  parent_substance_name: Option<Vec<String>>,

  /// Extensions for organismName
  #[serde(rename = "_organismName")]
  _organism_name: Option<Element>,

  /// Stage of life for animals, plants, insects and microorganisms. This information
  /// shall be provided only when the substance is significantly different in these
  /// stages (e.g. foetal bovine serum).
  #[serde(rename = "developmentStage")]
  development_stage: Option<CodeableConcept>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// The unique identifier associated with the source material parent organism shall
  /// be specified.
  #[serde(rename = "organismId")]
  organism_id: Option<Identifier>,

  /// The country where the plant material is harvested or the countries where the
  /// plasma is sourced from as laid down in accordance with the Plasma Master File.
  /// For “Plasma-derived substances” the attribute country of origin provides
  /// information about the countries used for the manufacturing of the Cryopoor plama
  /// or Crioprecipitate.
  #[serde(rename = "countryOfOrigin")]
  country_of_origin: Option<Vec<CodeableConcept>>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Extensions for parentSubstanceName
  #[serde(rename = "_parentSubstanceName")]
  _parent_substance_name: Option<Vec<Element>>,

  /// The organism accepted Scientific name shall be provided based on the organism
  /// taxonomy.
  #[serde(rename = "organismName")]
  organism_name: Option<String>,

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

  /// Extensions for geographicalLocation
  #[serde(rename = "_geographicalLocation")]
  _geographical_location: Option<Vec<Element>>,

  /// General high level classification of the source material specific to the origin
  /// of the material.
  #[serde(rename = "sourceMaterialClass")]
  source_material_class: Option<CodeableConcept>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Many complex materials are fractions of parts of plants, animals, or minerals.
  /// Fraction elements are often necessary to define both Substances and Specified
  /// Group 1 Substances. For substances derived from Plants, fraction information
  /// will be captured at the Substance information level ( . Oils, Juices and
  /// Exudates). Additional information for Extracts, such as extraction solvent
  /// composition, will be captured at the Specified Substance Group 1 information
  /// level. For plasma-derived products fraction information will be captured at the
  /// Substance and the Specified Substance Group 1 levels.
  #[serde(rename = "fractionDescription")]
  fraction_description: Option<Vec<SubstanceSourceMaterial_FractionDescription>>,

  /// The place/region where the plant is harvested or the places/regions where the
  /// animal source material has its habitat.
  #[serde(rename = "geographicalLocation")]
  geographical_location: Option<Vec<String>>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// This subclause describes the organism which the substance is derived from. For
  /// vaccines, the parent organism shall be specified based on these subclause
  /// elements. As an example, full taxonomy will be described for the Substance Name:
  /// ., Leaf.
  organism: Option<SubstanceSourceMaterial_Organism>,

}
