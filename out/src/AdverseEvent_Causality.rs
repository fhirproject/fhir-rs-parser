use serde::{Deserialize, Serialize};

/// Actual or  potential/avoided event causing unintended physical injury resulting
/// from or contributed to by medical care, a research study or other healthcare
/// setting factors that requires additional monitoring, treatment, or
/// hospitalization, or that results in death.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AdverseEvent_Causality {
  /// AdverseEvent.suspectEntity.causalityProductRelatedness.
  #[serde(rename = "productRelatedness")]
  product_relatedness: String,

  /// Extensions for productRelatedness
  #[serde(rename = "_productRelatedness")]
  _product_relatedness: Element,

  /// AdverseEvent.suspectEntity.causalityAuthor.
  author: Reference,

  /// Assessment of if the entity caused the event.
  assessment: CodeableConcept,

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

  /// ProbabilityScale | Bayesian | Checklist.
  method: CodeableConcept,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

}
