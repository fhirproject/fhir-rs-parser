use serde::{Deserialize, Serialize};

/// This resource provides eligibility and plan details from the processing of an
/// CoverageEligibilityRequest resource.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CoverageEligibilityResponse_Item {
  /// A short name or tag for the benefit.
  name: String,

  /// Extensions for authorizationUrl
  #[serde(rename = "_authorizationUrl")]
  _authorization_url: Element,

  /// True if the indicated class of service is excluded from the plan, missing or
  /// False indicates the product or service is included in the coverage.
  excluded: bool,

  /// A boolean flag indicating whether a preauthorization is required prior to actual
  /// service delivery.
  #[serde(rename = "authorizationRequired")]
  authorization_required: bool,

  /// Is a flag to indicate whether the benefits refer to in-network providers or out-
  /// of-network providers.
  network: CodeableConcept,

  /// Indicates if the benefits apply to an individual or to the family.
  unit: CodeableConcept,

  /// Codes or comments regarding information or actions associated with the
  /// preauthorization.
  #[serde(rename = "authorizationSupporting")]
  authorization_supporting: Vec<CodeableConcept>,

  /// Extensions for authorizationRequired
  #[serde(rename = "_authorizationRequired")]
  _authorization_required: Element,

  /// Extensions for excluded
  _excluded: Element,

  /// The practitioner who is eligible for the provision of the product or service.
  provider: Reference,

  /// The term or period of the values such as 'maximum lifetime benefit' or 'maximum
  /// annual visits'.
  term: CodeableConcept,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// A richer description of the benefit or services covered.
  description: String,

  /// Code to identify the general type of benefits under which products and services
  /// are provided.
  category: CodeableConcept,

  /// Item typification or modifiers codes to convey additional context for the
  /// product or service.
  modifier: Vec<CodeableConcept>,

  /// Extensions for name
  _name: Element,

  /// This contains the product, service, drug or other billing code for the item.
  #[serde(rename = "productOrService")]
  product_or_service: CodeableConcept,

  /// Extensions for description
  _description: Element,

  /// A web location for obtaining requirements or descriptive information regarding
  /// the preauthorization.
  #[serde(rename = "authorizationUrl")]
  authorization_url: String,

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

  /// Benefits used to date.
  benefit: Vec<CoverageEligibilityResponse_Benefit>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

}
