use serde::{Deserialize, Serialize};

/// Specifies clinical/business/etc. metadata that can be used to retrieve, index
/// and/or categorize an artifact. This metadata can either be specific to the
/// applicable population (e.g., age category, DRG) or the specific context of care
/// (e.g., venue, care setting, provider of care).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UsageContext {
  /// A value that defines the context specified in this context of use. The
  /// interpretation of the value is defined by the code.
  #[serde(rename = "valueQuantity")]
  value_quantity: Quantity,

  /// A code that identifies the type of context being specified by this usage
  /// context.
  code: Coding,

  /// A value that defines the context specified in this context of use. The
  /// interpretation of the value is defined by the code.
  #[serde(rename = "valueCodeableConcept")]
  value_codeable_concept: CodeableConcept,

  /// A value that defines the context specified in this context of use. The
  /// interpretation of the value is defined by the code.
  #[serde(rename = "valueReference")]
  value_reference: Reference,

  /// A value that defines the context specified in this context of use. The
  /// interpretation of the value is defined by the code.
  #[serde(rename = "valueRange")]
  value_range: Range,

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
