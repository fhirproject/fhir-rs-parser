use serde::{Deserialize, Serialize};

/// Represents a defined collection of entities that may be discussed or acted upon
/// collectively but which are not expected to act collectively, and are not
/// formally or legally recognized; i.e. a collection of entities that isn't an
/// Organization.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Group {
  /// Identifies the broad classification of the kind of resources the group includes.
  type: GroupType,

  /// Extensions for type
  _type: Element,

  /// A unique business identifier for this group.
  identifier: Vec<Identifier>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// A count of the number of resource instances that are part of the group.
  quantity: unsignedInt,

  /// The base language in which the resource is written.
  language: String,

  /// Extensions for language
  _language: Element,

  /// Extensions for quantity
  _quantity: Element,

  /// Entity responsible for defining and maintaining Group characteristics and/or
  /// registered members.
  #[serde(rename = "managingEntity")]
  managing_entity: Reference,

  /// Identifies traits whose presence r absence is shared by members of the group.
  characteristic: Vec<Group_Characteristic>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Indicates whether the record for the group is available for use or is merely
  /// being retained for historical purposes.
  active: bool,

  /// Extensions for name
  _name: Element,

  /// A label assigned to the group for human identification and communication.
  name: String,

  /// If true, indicates that the resource refers to a specific group of real
  /// individuals.  If false, the group defines a set of intended individuals.
  actual: bool,

  /// Extensions for actual
  _actual: Element,

  /// Provides a specific type of resource the group includes; e.g. "cow", "syringe",
  /// etc.
  code: CodeableConcept,

  /// Identifies the resource instances that are members of the group.
  member: Vec<Group_Member>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for active
  _active: Element,

}

#[derive(Debug, Serialize, Deserialize)]
enum GroupType {
  #[serde(rename = "person")]
  Person,

  #[serde(rename = "animal")]
  Animal,

  #[serde(rename = "practitioner")]
  Practitioner,

  #[serde(rename = "device")]
  Device,

  #[serde(rename = "medication")]
  Medication,

  #[serde(rename = "substance")]
  Substance,

}
