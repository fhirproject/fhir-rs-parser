use serde::{Deserialize, Serialize};

/// A person who is directly or indirectly involved in the provisioning of
/// healthcare.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Practitioner {
  /// The date of birth for the practitioner.
  #[serde(rename = "birthDate")]
  birth_date: date,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// An identifier that applies to this person in this role.
  identifier: Vec<Identifier>,

  /// Whether this practitioner's record is in active use.
  active: bool,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Image of the person.
  photo: Vec<Attachment>,

  /// A language the practitioner can use in patient communication.
  communication: Vec<CodeableConcept>,

  /// A contact detail for the practitioner, e.g. a telephone number or an email
  /// address.
  telecom: Vec<ContactPoint>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

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

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Extensions for active
  _active: Element,

  /// Administrative Gender - the gender that the person is considered to have for
  /// administration and record keeping purposes.
  gender: PractitionerGender,

  /// Extensions for gender
  _gender: Element,

  /// The official certifications, training, and licenses that authorize or otherwise
  /// pertain to the provision of care by the practitioner.  For example, a medical
  /// license issued by a medical board authorizing the practitioner to practice
  /// medicine within a certian locality.
  qualification: Vec<Practitioner_Qualification>,

  /// The name(s) associated with the practitioner.
  name: Vec<HumanName>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// Address(es) of the practitioner that are not role specific (typically home
  /// address). Work addresses are not typically entered in this property as they are
  /// usually role dependent.
  address: Vec<Address>,

  /// Extensions for language
  _language: Element,

  /// The base language in which the resource is written.
  language: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for birthDate
  #[serde(rename = "_birthDate")]
  _birth_date: Element,

}

#[derive(Debug, Serialize, Deserialize)]
enum PractitionerGender {
  #[serde(rename = "male")]
  Male,

  #[serde(rename = "female")]
  Female,

  #[serde(rename = "other")]
  Other,

  #[serde(rename = "unknown")]
  Unknown,

}
