#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Identifier::Identifier;
use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::HealthcareService_Eligibility::HealthcareService_Eligibility;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::ContactPoint::ContactPoint;
use crate::model::HealthcareService_NotAvailable::HealthcareService_NotAvailable;
use crate::model::HealthcareService_AvailableTime::HealthcareService_AvailableTime;
use crate::model::Attachment::Attachment;


/// The details of a healthcare service available at a location.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthcareService {
  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Extensions for name
  _name: Element,

  /// Extensions for language
  _language: Element,

  /// Ways that the service accepts referrals, if this is not provided then it is
  /// implied that no referral is required.
  #[serde(rename = "referralMethod")]
  referral_method: Vec<CodeableConcept>,

  /// The location(s) that this service is available to (not where the service is
  /// provided).
  #[serde(rename = "coverageArea")]
  coverage_area: Vec<Box<Reference>>,

  /// The specific type of service that may be delivered or performed.
  #[serde(rename = "type")]
  fhir_type: Vec<CodeableConcept>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// List of contacts related to this specific healthcare service.
  telecom: Vec<ContactPoint>,

  /// The organization that provides this healthcare service.
  #[serde(rename = "providedBy")]
  provided_by: Box<Reference>,

  /// This flag is used to mark the record to not be used. This is not used when a
  /// center is closed for maintenance, or for holidays, the notAvailable period is to
  /// be used for this.
  active: bool,

  /// Extensions for availabilityExceptions
  #[serde(rename = "_availabilityExceptions")]
  _availability_exceptions: Element,

  /// The location(s) where this healthcare service may be provided.
  location: Vec<Box<Reference>>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// The base language in which the resource is written.
  language: String,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// Programs that this service is applicable to.
  program: Vec<CodeableConcept>,

  /// Extensions for appointmentRequired
  #[serde(rename = "_appointmentRequired")]
  _appointment_required: Element,

  /// Extra details about the service that can't be placed in the other fields.
  #[serde(rename = "extraDetails")]
  extra_details: String,

  /// The HealthcareService is not available during this period of time due to the
  /// provided reason.
  #[serde(rename = "notAvailable")]
  not_available: Vec<HealthcareService_NotAvailable>,

  /// Does this service have specific eligibility requirements that need to be met in
  /// order to use the service?
  eligibility: Vec<HealthcareService_Eligibility>,

  /// Extensions for comment
  _comment: Element,

  /// Extensions for extraDetails
  #[serde(rename = "_extraDetails")]
  _extra_details: Element,

  /// The code(s) that detail the conditions under which the healthcare service is
  /// available/offered.
  #[serde(rename = "serviceProvisionCode")]
  service_provision_code: Vec<CodeableConcept>,

  /// Further description of the service as it would be presented to a consumer while
  /// searching.
  name: String,

  /// A collection of times that the Service Site is available.
  #[serde(rename = "availableTime")]
  available_time: Vec<HealthcareService_AvailableTime>,

  /// Technical endpoints providing access to services operated for the specific
  /// healthcare services defined at this resource.
  endpoint: Vec<Box<Reference>>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// A description of site availability exceptions, e.g. public holiday availability.
  /// Succinctly describing all possible exceptions to normal site availability as
  /// details in the available Times and not available Times.
  #[serde(rename = "availabilityExceptions")]
  availability_exceptions: String,

  /// Collection of characteristics (attributes).
  characteristic: Vec<CodeableConcept>,

  /// If there is a photo/symbol associated with this HealthcareService, it may be
  /// included here to facilitate quick identification of the service in a list.
  photo: Attachment,

  /// Some services are specifically made available in multiple languages, this
  /// property permits a directory to declare the languages this is offered in.
  /// Typically this is only provided where a service operates in communities with
  /// mixed languages used.
  communication: Vec<CodeableConcept>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.    Modifier
  /// extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// External identifiers for this item.
  identifier: Vec<Identifier>,

  /// Identifies the broad category of service being performed or delivered.
  category: Vec<CodeableConcept>,

  /// Collection of specialties handled by the service site. This is more of a medical
  /// term.
  specialty: Vec<CodeableConcept>,

  /// Indicates whether or not a prospective consumer will require an appointment for
  /// a particular service at a site to be provided by the Organization. Indicates if
  /// an appointment is required for access to this service.
  #[serde(rename = "appointmentRequired")]
  appointment_required: bool,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Any additional description of the service and/or any specific issues not covered
  /// by the other attributes, which can be displayed as further detail under the
  /// serviceName.
  comment: String,

  /// Extensions for active
  _active: Element,

}
