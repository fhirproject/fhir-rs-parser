use serde::{Deserialize, Serialize};

/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR
/// Server for a particular version of FHIR that may be used as a statement of
/// actual server functionality or a statement of required or desired server
/// implementation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CapabilityStatement_Interaction1 {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for code
  _code: Element,

  /// Guidance specific to the implementation of this operation, such as limitations
  /// on the kind of transactions allowed, or information about system wide search is
  /// implemented.
  documentation: markdown,

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

  /// Extensions for documentation
  _documentation: Element,

  /// A coded identifier of the operation, supported by the system.
  code: CapabilityStatement_Interaction1Code,

}

#[derive(Debug, Serialize, Deserialize)]
enum CapabilityStatement_Interaction1Code {
  #[serde(rename = "transaction")]
  Transaction,

  #[serde(rename = "batch")]
  Batch,

  #[serde(rename = "search-system")]
  SearchSystem,

  #[serde(rename = "history-system")]
  HistorySystem,

}
