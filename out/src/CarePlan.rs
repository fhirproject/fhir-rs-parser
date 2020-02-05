use serde::{Deserialize, Serialize};

/// Describes the intention of how one or more practitioners intend to deliver care
/// for a particular patient, group or community for a period of time, possibly
/// limited to care for a specific condition or set of conditions.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CarePlan {
  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Identifies a planned action to occur as part of the plan.  For example, a
  /// medication to be used, lab tests to perform, self-monitoring, education, etc.
  activity: Vec<CarePlan_Activity>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Extensions for title
  _title: Element,

  /// Represents when this particular CarePlan record was created in the system, which
  /// is often a system-generated date.
  created: dateTime,

  /// Extensions for language
  _language: Element,

  /// Business identifiers assigned to this care plan by the performer or other
  /// systems which remain constant as the resource is updated and propagates from
  /// server to server.
  identifier: Vec<Identifier>,

  /// When populated, the author is responsible for the care plan.  The care plan is
  /// attributed to the author.
  author: Reference,

  /// Identifies portions of the patient's record that specifically influenced the
  /// formation of the plan.  These might include comorbidities, recent procedures,
  /// limitations, recent assessments, etc.
  #[serde(rename = "supportingInfo")]
  supporting_info: Vec<Reference>,

  /// Identifies the patient or group whose intended care is described by the plan.
  subject: Reference,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// General notes about the care plan not covered elsewhere.
  note: Vec<Annotation>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Indicates the level of authority/intentionality associated with the care plan
  /// and where the care plan fits into the workflow chain.
  intent: String,

  /// Identifies the individual(s) or organization who provided the contents of the
  /// care plan.
  contributor: Vec<Reference>,

  /// A larger care plan of which this particular care plan is a component or step.
  #[serde(rename = "partOf")]
  part_of: Vec<Reference>,

  /// Indicates whether the plan is currently being acted upon, represents future
  /// intentions or is now a historical record.
  status: String,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// A description of the scope and nature of the plan.
  description: String,

  /// Human-friendly name for the care plan.
  title: String,

  /// The URL pointing to a FHIR-defined protocol, guideline, questionnaire or other
  /// definition that is adhered to in whole or in part by this CarePlan.
  #[serde(rename = "instantiatesCanonical")]
  instantiates_canonical: Vec<canonical>,

  /// Extensions for instantiatesUri
  #[serde(rename = "_instantiatesUri")]
  _instantiates_uri: Vec<Element>,

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

  /// Extensions for description
  _description: Element,

  /// The Encounter during which this CarePlan was created or to which the creation of
  /// this record is tightly associated.
  encounter: Reference,

  /// Describes the intended objective(s) of carrying out the care plan.
  goal: Vec<Reference>,

  /// Indicates when the plan did (or is intended to) come into effect and end.
  period: Period,

  /// Identifies the conditions/problems/concerns/diagnoses/etc. whose management
  /// and/or mitigation are handled by this plan.
  addresses: Vec<Reference>,

  /// Identifies all people and organizations who are expected to be involved in the
  /// care envisioned by this plan.
  #[serde(rename = "careTeam")]
  care_team: Vec<Reference>,

  /// The base language in which the resource is written.
  language: String,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// Identifies what "kind" of plan this is to support differentiation between
  /// multiple co-existing plans; e.g. "Home health", "psychiatric", "asthma",
  /// "disease management", "wellness plan", etc.
  category: Vec<CodeableConcept>,

  /// A care plan that is fulfilled in whole or in part by this care plan.
  #[serde(rename = "basedOn")]
  based_on: Vec<Reference>,

  /// Completed or terminated care plan whose function is taken by this new care plan.
  replaces: Vec<Reference>,

  /// Extensions for created
  _created: Element,

  /// Extensions for intent
  _intent: Element,

  /// The URL pointing to an externally maintained protocol, guideline, questionnaire
  /// or other definition that is adhered to in whole or in part by this CarePlan.
  #[serde(rename = "instantiatesUri")]
  instantiates_uri: Vec<String>,

  /// Extensions for status
  _status: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

}
