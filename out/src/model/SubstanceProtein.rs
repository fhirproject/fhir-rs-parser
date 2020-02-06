#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Meta::Meta;
use crate::model::ResourceList::ResourceList;
use crate::model::SubstanceProtein_Subunit::SubstanceProtein_Subunit;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Narrative::Narrative;


/// A SubstanceProtein is defined as a single unit of a linear amino acid sequence,
/// or a combination of subunits that are either covalently linked or have a defined
/// invariant stoichiometric relationship. This includes all synthetic, recombinant
/// and purified SubstanceProteins of defined sequence, whether the use is
/// therapeutic or prophylactic. This set of elements will be used to describe
/// albumins, coagulation factors, cytokines, growth factors,
/// peptide/SubstanceProtein hormones, enzymes, toxins, toxoids, recombinant
/// vaccines, and immunomodulators.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceProtein {
  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The base language in which the resource is written.
  language: String,

  /// This subclause refers to the description of each subunit constituting the
  /// SubstanceProtein. A subunit is a linear sequence of amino acids linked through
  /// peptide bonds. The Subunit information shall be provided when the finished
  /// SubstanceProtein is a complex of multiple sequences; subunits are not used to
  /// delineate domains within a single sequence. Subunits are listed in order of
  /// decreasing length; sequences of the same length will be ordered by decreasing
  /// molecular weight; subunits that have identical sequences will be repeated
  /// multiple times.
  subunit: Vec<SubstanceProtein_Subunit>,

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

  /// Number of linear sequences of amino acids linked through peptide bonds. The
  /// number of subunits constituting the SubstanceProtein shall be described. It is
  /// possible that the number of subunits can be variable.
  #[serde(rename = "numberOfSubunits")]
  number_of_subunits: i32,

  /// Extensions for disulfideLinkage
  #[serde(rename = "_disulfideLinkage")]
  _disulfide_linkage: Vec<Element>,

  /// The disulphide bond between two cysteine residues either on the same subunit or
  /// on two different subunits shall be described. The position of the disulfide
  /// bonds in the SubstanceProtein shall be listed in increasing order of subunit
  /// number and position within subunit followed by the abbreviation of the amino
  /// acids involved. The disulfide linkage positions shall actually contain the amino
  /// acid Cysteine at the respective positions.
  #[serde(rename = "disulfideLinkage")]
  disulfide_linkage: Vec<String>,

  /// Extensions for language
  _language: Element,

  /// The SubstanceProtein descriptive elements will only be used when a complete or
  /// partial amino acid sequence is available or derivable from a nucleic acid
  /// sequence.
  #[serde(rename = "sequenceType")]
  sequence_type: CodeableConcept,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// Extensions for numberOfSubunits
  #[serde(rename = "_numberOfSubunits")]
  _number_of_subunits: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

}
