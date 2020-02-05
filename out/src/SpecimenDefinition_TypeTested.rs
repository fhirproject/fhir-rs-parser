use serde::{Deserialize, Serialize};

/// A kind of specimen with associated set of requirements.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpecimenDefinition_TypeTested {
  /// The preference for this type of conditioned specimen.
  preference: SpecimenDefinition_TypeTestedPreference,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for requirement
  _requirement: Element,

  /// The usual time that a specimen of this kind is retained after the ordered tests
  /// are completed, for the purpose of additional testing.
  #[serde(rename = "retentionTime")]
  retention_time: Duration,

  /// Criterion for rejection of the specimen in its container by the laboratory.
  #[serde(rename = "rejectionCriterion")]
  rejection_criterion: Vec<CodeableConcept>,

  /// Set of instructions for preservation/transport of the specimen at a defined
  /// temperature interval, prior the testing process.
  handling: Vec<SpecimenDefinition_Handling>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// Extensions for isDerived
  #[serde(rename = "_isDerived")]
  _is_derived: Element,

  /// The specimen's container.
  container: SpecimenDefinition_Container,

  /// Extensions for preference
  _preference: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// The kind of specimen conditioned for testing expected by lab.
  type: CodeableConcept,

  /// Requirements for delivery and special handling of this kind of conditioned
  /// specimen.
  requirement: String,

  /// Primary of secondary specimen.
  #[serde(rename = "isDerived")]
  is_derived: bool,

}

#[derive(Debug, Serialize, Deserialize)]
enum SpecimenDefinition_TypeTestedPreference {
  #[serde(rename = "preferred")]
  Preferred,

  #[serde(rename = "alternate")]
  Alternate,

}
