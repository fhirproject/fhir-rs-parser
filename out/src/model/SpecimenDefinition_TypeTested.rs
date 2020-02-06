#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Duration::Duration;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::SpecimenDefinition_Container::SpecimenDefinition_Container;
use crate::model::Element::Element;
use crate::model::SpecimenDefinition_Handling::SpecimenDefinition_Handling;


/// A kind of specimen with associated set of requirements.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecimenDefinition_TypeTested {
  /// The usual time that a specimen of this kind is retained after the ordered tests
  /// are completed, for the purpose of additional testing.
  #[serde(rename = "retentionTime")]
  retention_time: Option<Duration>,

  /// Primary of secondary specimen.
  #[serde(rename = "isDerived")]
  is_derived: Option<bool>,

  /// Extensions for isDerived
  #[serde(rename = "_isDerived")]
  _is_derived: Option<Element>,

  /// The preference for this type of conditioned specimen.
  preference: Option<SpecimenDefinition_TypeTestedPreference>,

  /// The kind of specimen conditioned for testing expected by lab.
  #[serde(rename = "type")]
  fhir_type: Option<CodeableConcept>,

  /// Set of instructions for preservation/transport of the specimen at a defined
  /// temperature interval, prior the testing process.
  handling: Option<Vec<SpecimenDefinition_Handling>>,

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

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for preference
  #[serde(rename = "_preference")]
  _preference: Option<Element>,

  /// Requirements for delivery and special handling of this kind of conditioned
  /// specimen.
  requirement: Option<String>,

  /// Criterion for rejection of the specimen in its container by the laboratory.
  #[serde(rename = "rejectionCriterion")]
  rejection_criterion: Option<Vec<CodeableConcept>>,

  /// Extensions for requirement
  #[serde(rename = "_requirement")]
  _requirement: Option<Element>,

  /// The specimen's container.
  container: Option<SpecimenDefinition_Container>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum SpecimenDefinition_TypeTestedPreference {
  #[serde(rename = "preferred")]
  Preferred,

  #[serde(rename = "alternate")]
  Alternate,

}
