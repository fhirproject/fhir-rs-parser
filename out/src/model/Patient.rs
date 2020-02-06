#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Patient_Communication::Patient_Communication;
use crate::model::Meta::Meta;
use crate::model::Patient_Contact::Patient_Contact;
use crate::model::Element::Element;
use crate::model::HumanName::HumanName;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Attachment::Attachment;
use crate::model::Identifier::Identifier;
use crate::model::Address::Address;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::Narrative::Narrative;
use crate::model::Extension::Extension;
use crate::model::Patient_Link::Patient_Link;


/// Demographics and other administrative information about an individual or animal
/// receiving care or other health-related services.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Patient {
  /// Extensions for birthDate
  #[serde(rename = "_birthDate")]
  _birth_date: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Extensions for gender
  _gender: Element,

  /// Extensions for active
  _active: Element,

  /// Patient's nominated care provider.
  #[serde(rename = "generalPractitioner")]
  general_practitioner: Vec<Box<Reference>>,

  /// A contact detail (e.g. a telephone number or an email address) by which the
  /// individual may be contacted.
  telecom: Vec<ContactPoint>,

  /// Extensions for deceasedDateTime
  #[serde(rename = "_deceasedDateTime")]
  _deceased_date_time: Element,

  /// Extensions for language
  _language: Element,

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

  /// Indicates whether the patient is part of a multiple (boolean) or indicates the
  /// actual birth order (integer).
  #[serde(rename = "multipleBirthBoolean")]
  multiple_birth_boolean: bool,

  /// The date of birth for the individual.
  #[serde(rename = "birthDate")]
  birth_date: i32,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Extensions for multipleBirthBoolean
  #[serde(rename = "_multipleBirthBoolean")]
  _multiple_birth_boolean: Element,

  /// A contact party (e.g. guardian, partner, friend) for the patient.
  contact: Vec<Patient_Contact>,

  /// Extensions for multipleBirthInteger
  #[serde(rename = "_multipleBirthInteger")]
  _multiple_birth_integer: Element,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// An identifier for this patient.
  identifier: Vec<Identifier>,

  /// The base language in which the resource is written.
  language: String,

  /// An address for the individual.
  address: Vec<Address>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// This field contains a patient's most recent marital (civil) status.
  #[serde(rename = "maritalStatus")]
  marital_status: CodeableConcept,

  /// Indicates if the individual is deceased or not.
  #[serde(rename = "deceasedDateTime")]
  deceased_date_time: String,

  /// A name associated with the individual.
  name: Vec<HumanName>,

  /// Whether this patient record is in active use.   Many systems use this property
  /// to mark as non-current patients, such as those that have not been seen for a
  /// period of time based on an organization's business rules.    It is often used to
  /// filter patient lists to exclude inactive patients    Deceased patients may also
  /// be marked as inactive for the same reasons, but may be active for some time
  /// after death.
  active: bool,

  /// Indicates whether the patient is part of a multiple (boolean) or indicates the
  /// actual birth order (integer).
  #[serde(rename = "multipleBirthInteger")]
  multiple_birth_integer: i32,

  /// Link to another patient resource that concerns the same actual patient.
  link: Vec<Patient_Link>,

  /// A language which may be used to communicate with the patient about his or her
  /// health.
  communication: Vec<Patient_Communication>,

  /// Indicates if the individual is deceased or not.
  #[serde(rename = "deceasedBoolean")]
  deceased_boolean: bool,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Image of the patient.
  photo: Vec<Attachment>,

  /// Extensions for deceasedBoolean
  #[serde(rename = "_deceasedBoolean")]
  _deceased_boolean: Element,

  /// Administrative Gender - the gender that the patient is considered to have for
  /// administration and record keeping purposes.
  gender: PatientGender,

  /// Organization that is the custodian of the patient record.
  #[serde(rename = "managingOrganization")]
  managing_organization: Box<Reference>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum PatientGender {
  #[serde(rename = "male")]
  Male,

  #[serde(rename = "female")]
  Female,

  #[serde(rename = "other")]
  Other,

  #[serde(rename = "unknown")]
  Unknown,

}
