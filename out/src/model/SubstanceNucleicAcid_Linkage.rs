#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Element::Element;


/// Nucleic acids are defined by three distinct elements: the base, sugar and
/// linkage. Individual substance/moiety IDs will be created for each of these
/// elements. The nucleotide sequence will be always entered in the 5’-3’ direction.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceNucleicAcid_Linkage {
  /// The entity that links the sugar residues together should also be captured for
  /// nearly all naturally occurring nucleic acid the linkage is a phosphate group.
  /// For many synthetic oligonucleotides phosphorothioate linkages are often seen.
  /// Linkage connectivity is assumed to be 3’-5’. If the linkage is either 3’-3’ or
  /// 5’-5’ this should be specified.
  connectivity: Option<String>,

  /// Residues shall be captured as described in 5.3.6.8.3.
  #[serde(rename = "residueSite")]
  residue_site: Option<String>,

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

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for residueSite
  #[serde(rename = "_residueSite")]
  _residue_site: Option<Element>,

  /// Extensions for connectivity
  #[serde(rename = "_connectivity")]
  _connectivity: Option<Element>,

  /// Each linkage will be registered as a fragment and have at least one name. A
  /// single name shall be assigned to each linkage.
  name: Option<String>,

  /// Each linkage will be registered as a fragment and have an ID.
  identifier: Option<Identifier>,

}
