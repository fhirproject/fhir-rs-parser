use serde::{Deserialize, Serialize};

/// A formally or informally recognized grouping of people or organizations formed
/// for the purpose of achieving some form of collective action.  Includes
/// companies, institutions, corporations, departments, community groups, healthcare
/// practice groups, payer/insurer, etc.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Organization_Contact {
  /// Indicates a purpose for which the contact can be reached.
  purpose: CodeableConcept,

  /// A name associated with the contact.
  name: HumanName,

  /// A contact detail (e.g. a telephone number or an email address) by which the
  /// party may be contacted.
  telecom: Vec<ContactPoint>,

  /// Visiting or postal addresses for the contact.
  address: Address,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

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

}
