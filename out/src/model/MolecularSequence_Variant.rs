#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;


/// Raw data describing a biological sequence.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequence_Variant {
  /// Start position of the variant on the  reference sequence. If the coordinate
  /// system is either 0-based or 1-based, then start position is inclusive.
  start: Option<i32>,

  /// End position of the variant on the reference sequence. If the coordinate system
  /// is 0-based then end is exclusive and does not include the last position. If the
  /// coordinate system is 1-base, then end is inclusive and includes the last
  /// position.
  end: Option<i32>,

  /// An allele is one of a set of coexisting sequence variants of a gene
  /// ([SO:0001023](http://www.sequenceontology.org/browser/current_svn/term/SO:000102
  /// 3)).  Nucleotide(s)/amino acids from start position of sequence to stop position
  /// of sequence on the positive (+) strand of the observed  sequence. When the
  /// sequence  type is DNA, it should be the sequence on the positive (+) strand.
  /// This will lay in the range between variant.start and variant.end.
  #[serde(rename = "observedAllele")]
  observed_allele: Option<String>,

  /// Extensions for referenceAllele
  #[serde(rename = "_referenceAllele")]
  _reference_allele: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for cigar
  #[serde(rename = "_cigar")]
  _cigar: Option<Element>,

  /// A pointer to an Observation containing variant information.
  #[serde(rename = "variantPointer")]
  variant_pointer: Option<Box<Reference>>,

  /// Extensions for observedAllele
  #[serde(rename = "_observedAllele")]
  _observed_allele: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// An allele is one of a set of coexisting sequence variants of a gene
  /// ([SO:0001023](http://www.sequenceontology.org/browser/current_svn/term/SO:000102
  /// 3)). Nucleotide(s)/amino acids from start position of sequence to stop position
  /// of sequence on the positive (+) strand of the reference sequence. When the
  /// sequence  type is DNA, it should be the sequence on the positive (+) strand.
  /// This will lay in the range between variant.start and variant.end.
  #[serde(rename = "referenceAllele")]
  reference_allele: Option<String>,

  /// Extensions for end
  #[serde(rename = "_end")]
  _end: Option<Element>,

  /// Extensions for start
  #[serde(rename = "_start")]
  _start: Option<Element>,

  /// Extended CIGAR string for aligning the sequence with reference bases. See
  /// detailed documentation
  /// [here](http://support.illumina.com/help/SequencingAnalysisWorkflow/Content/Vault
  /// /Informatics/Sequencing_Analysis/CASAVA/swSEQ_mCA_ExtendedCIGARFormat.htm).
  cigar: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.    Modifier extensions
  /// SHALL NOT change the meaning of any elements on Resource or DomainResource
  /// (including cannot change the meaning of modifierExtension itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Option<Vec<Box<Extension>>>,

}
