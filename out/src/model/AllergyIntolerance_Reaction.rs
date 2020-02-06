#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Annotation::Annotation;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;


/// Risk of harmful or undesirable, physiological response which is unique to an
/// individual and associated with exposure to a substance.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllergyIntolerance_Reaction {
  /// Extensions for severity
  #[serde(rename = "_severity")]
  _severity: Option<Element>,

  /// Clinical symptoms and/or signs that are observed or associated with the adverse
  /// reaction event.
  manifestation: Vec<CodeableConcept>,

  /// Identification of the route by which the subject was exposed to the substance.
  #[serde(rename = "exposureRoute")]
  exposure_route: Option<CodeableConcept>,

  /// Text description about the reaction as a whole, including details of the
  /// manifestation if required.
  description: Option<String>,

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
  modifier_extension: Option<Vec<Extension>>,

  /// Clinical assessment of the severity of the reaction event as a whole,
  /// potentially considering multiple different manifestations.
  severity: Option<AllergyIntolerance_ReactionSeverity>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// Identification of the specific substance (or pharmaceutical product) considered
  /// to be responsible for the Adverse Reaction event. Note: the substance for a
  /// specific reaction may be different from the substance identified as the cause of
  /// the risk, but it must be consistent with it. For instance, it may be a more
  /// specific substance (e.g. a brand medication) or a composite product that
  /// includes the identified substance. It must be clinically safe to only process
  /// the 'code' and ignore the 'reaction.substance'.  If a receiving system is unable
  /// to confirm that AllergyIntolerance.reaction.substance falls within the semantic
  /// scope of AllergyIntolerance.code, then the receiving system should ignore
  /// AllergyIntolerance.reaction.substance.
  substance: Option<CodeableConcept>,

  /// Extensions for onset
  #[serde(rename = "_onset")]
  _onset: Option<Element>,

  /// Additional text about the adverse reaction event not captured in other fields.
  note: Option<Vec<Annotation>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Record of the date and/or time of the onset of the Reaction.
  onset: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum AllergyIntolerance_ReactionSeverity {
  #[serde(rename = "mild")]
  Mild,

  #[serde(rename = "moderate")]
  Moderate,

  #[serde(rename = "severe")]
  Severe,

}
