#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::Element::Element;
use crate::model::SubstanceSpecification_Official::SubstanceSpecification_Official;


/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSpecification_Name {
  /// Name type.
  #[serde(rename = "type")]
  fhir_type: Option<CodeableConcept>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Supporting literature.
  source: Option<Vec<Box<Reference>>>,

  /// Language of the name.
  language: Option<Vec<CodeableConcept>>,

  /// The actual name.
  name: Option<String>,

  /// A synonym of this name.
  synonym: Option<Vec<SubstanceSpecification_Name>>,

  /// Extensions for preferred
  #[serde(rename = "_preferred")]
  _preferred: Option<Element>,

  /// Details of the official nature of this name.
  official: Option<Vec<SubstanceSpecification_Official>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// If this is the preferred name for this substance.
  preferred: Option<bool>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// A translation for this name.
  translation: Option<Vec<SubstanceSpecification_Name>>,

  /// The use context of this name for example if there is a different name a drug
  /// active ingredient as opposed to a food colour additive.
  domain: Option<Vec<CodeableConcept>>,

  /// The status of the name.
  status: Option<CodeableConcept>,

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

  /// The jurisdiction where this name applies.
  jurisdiction: Option<Vec<CodeableConcept>>,

}
