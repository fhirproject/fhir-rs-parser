use serde::{Deserialize, Serialize};

/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StructureMap_Group {
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

  /// Extensions for typeMode
  #[serde(rename = "_typeMode")]
  _type_mode: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for name
  _name: Element,

  /// Additional supporting documentation that explains the purpose of the group and
  /// the types of mappings within it.
  documentation: String,

  /// Another group that this group adds rules to.
  extends: id,

  /// A name assigned to an instance of data. The instance must be provided when the
  /// mapping is invoked.
  input: Vec<StructureMap_Input>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Transform Rule from source to target.
  rule: Vec<StructureMap_Rule>,

  /// Extensions for extends
  _extends: Element,

  /// Extensions for documentation
  _documentation: Element,

  /// If this is the default rule set to apply for the source type or this combination
  /// of types.
  #[serde(rename = "typeMode")]
  type_mode: StructureMap_GroupTypeMode,

  /// A unique name for the group for the convenience of human readers.
  name: id,

}

#[derive(Debug, Serialize, Deserialize)]
enum StructureMap_GroupTypeMode {
  #[serde(rename = "none")]
  None,

  #[serde(rename = "types")]
  Types,

  #[serde(rename = "type-and-types")]
  TypeAndTypes,

}
