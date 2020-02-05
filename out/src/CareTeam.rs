use serde::{Deserialize, Serialize};

/// The Care Team includes all the people and organizations who plan to participate
/// in the coordination and delivery of care for a patient.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CareTeam {
  /// Comments made about the CareTeam.
  note: Vec<Annotation>,

  /// A label for human use intended to distinguish like teams.  E.g. the "red" vs.
  /// "green" trauma teams.
  name: String,

  /// The base language in which the resource is written.
  language: String,

  /// The Encounter during which this CareTeam was created or to which the creation of
  /// this record is tightly associated.
  encounter: Reference,

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

  /// Condition(s) that this care team addresses.
  #[serde(rename = "reasonReference")]
  reason_reference: Vec<Reference>,

  /// Describes why the care team exists.
  #[serde(rename = "reasonCode")]
  reason_code: Vec<CodeableConcept>,

  /// Extensions for status
  _status: Element,

  /// Extensions for language
  _language: Element,

  /// Identifies what kind of team.  This is to support differentiation between
  /// multiple co-existing teams, such as care plan team, episode of care team,
  /// longitudinal care team.
  category: Vec<CodeableConcept>,

  /// Identifies the patient or group whose intended care is handled by the team.
  subject: Reference,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Indicates the current state of the care team.
  status: CareTeamStatus,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for name
  _name: Element,

  /// Indicates when the team did (or is intended to) come into effect and end.
  period: Period,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// The organization responsible for the care team.
  #[serde(rename = "managingOrganization")]
  managing_organization: Vec<Reference>,

  /// A central contact detail for the care team (that applies to all members).
  telecom: Vec<ContactPoint>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Business identifiers assigned to this care team by the performer or other
  /// systems which remain constant as the resource is updated and propagates from
  /// server to server.
  identifier: Vec<Identifier>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// Identifies all people and organizations who are expected to be involved in the
  /// care team.
  participant: Vec<CareTeam_Participant>,

}

#[derive(Debug, Serialize, Deserialize)]
enum CareTeamStatus {
  #[serde(rename = "proposed")]
  Proposed,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "suspended")]
  Suspended,

  #[serde(rename = "inactive")]
  Inactive,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

}
