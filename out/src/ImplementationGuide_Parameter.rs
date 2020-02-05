use serde::{Deserialize, Serialize};

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used to
/// gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImplementationGuide_Parameter {
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

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// apply | path-resource | path-pages | path-tx-cache | expansion-parameter | rule-
  /// broken-links | generate-xml | generate-json | generate-turtle | html-
  /// template.
  code: ImplementationGuide_ParameterCode,

  /// Extensions for code
  _code: Element,

  /// Extensions for value
  _value: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Value for named type.
  value: String,

}

#[derive(Debug, Serialize, Deserialize)]
enum ImplementationGuide_ParameterCode {
  #[serde(rename = "apply")]
  Apply,

  #[serde(rename = "path-resource")]
  PathResource,

  #[serde(rename = "path-pages")]
  PathPages,

  #[serde(rename = "path-tx-cache")]
  PathTxCache,

  #[serde(rename = "expansion-parameter")]
  ExpansionParameter,

  #[serde(rename = "rule-broken-links")]
  RuleBrokenLinks,

  #[serde(rename = "generate-xml")]
  GenerateXml,

  #[serde(rename = "generate-json")]
  GenerateJson,

  #[serde(rename = "generate-turtle")]
  GenerateTurtle,

  #[serde(rename = "html-template")]
  HtmlTemplate,

}
