use serde::{Deserialize, Serialize};

/// Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ElementDefinition_Discriminator {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

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

  /// How the element value is interpreted when discrimination is evaluated.
  type: ElementDefinition_DiscriminatorType,

  /// Extensions for type
  _type: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// A FHIRPath expression, using [the simple subset of
  /// FHIRPath](fhirpath.html#simple), that is used to identify the element on which
  /// discrimination is based.
  path: String,

  /// Extensions for path
  _path: Element,

}

#[derive(Debug, Serialize, Deserialize)]
enum ElementDefinition_DiscriminatorType {
  #[serde(rename = "value")]
  Value,

  #[serde(rename = "exists")]
  Exists,

  #[serde(rename = "pattern")]
  Pattern,

  #[serde(rename = "type")]
  Type,

  #[serde(rename = "profile")]
  Profile,

}
