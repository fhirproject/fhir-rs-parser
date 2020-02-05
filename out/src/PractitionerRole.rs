use serde::{Deserialize, Serialize};

/// A specific set of Roles/Locations/specialties/services that a practitioner may
/// perform at an organization for a period of time.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PractitionerRole {
  /// Technical endpoints providing access to services operated for the practitioner
  /// with this role.
  endpoint: Vec<Reference>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Roles which this practitioner is authorized to perform for the organization.
  code: Vec<CodeableConcept>,

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

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// The organization where the Practitioner performs the roles associated.
  organization: Reference,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// Extensions for language
  _language: Element,

  /// Specific specialty of the practitioner.
  specialty: Vec<CodeableConcept>,

  /// The location(s) at which this practitioner provides care.
  location: Vec<Reference>,

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

  /// Contact details that are specific to the role/location/service.
  telecom: Vec<ContactPoint>,

  /// A collection of times the practitioner is available or performing this role at
  /// the location and/or healthcareservice.
  #[serde(rename = "availableTime")]
  available_time: Vec<PractitionerRole_AvailableTime>,

  /// The base language in which the resource is written.
  language: String,

  /// Extensions for active
  _active: Element,

  /// The practitioner is not available or performing this role during this period of
  /// time due to the provided reason.
  #[serde(rename = "notAvailable")]
  not_available: Vec<PractitionerRole_NotAvailable>,

  /// A description of site availability exceptions, e.g. public holiday availability.
  /// Succinctly describing all possible exceptions to normal site availability as
  /// details in the available Times and not available Times.
  #[serde(rename = "availabilityExceptions")]
  availability_exceptions: String,

  /// Extensions for availabilityExceptions
  #[serde(rename = "_availabilityExceptions")]
  _availability_exceptions: Element,

  /// The list of healthcare services that this worker provides for this role's
  /// Organization/Location(s).
  #[serde(rename = "healthcareService")]
  healthcare_service: Vec<Reference>,

  /// Business Identifiers that are specific to a role/location.
  identifier: Vec<Identifier>,

  /// Whether this practitioner role record is in active use.
  active: bool,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// The period during which the person is authorized to act as a practitioner in
  /// these role(s) for the organization.
  period: Period,

  /// Practitioner that is able to provide the defined services for the organization.
  practitioner: Reference,

}
