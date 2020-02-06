#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ResourceList::ResourceList;
use crate::model::Extension::Extension;
use crate::model::Attachment::Attachment;
use crate::model::HumanName::HumanName;
use crate::model::Reference::Reference;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Element::Element;
use crate::model::Narrative::Narrative;
use crate::model::Meta::Meta;
use crate::model::Patient_Communication::Patient_Communication;
use crate::model::Identifier::Identifier;
use crate::model::Patient_Contact::Patient_Contact;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Address::Address;
use crate::model::Patient_Link::Patient_Link;


/// Demographics and other administrative information about an individual or animal
/// receiving care or other health-related services.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Patient {
  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Indicates if the individual is deceased or not.
  #[serde(rename = "deceasedDateTime")]
  deceased_date_time: Option<String>,

  /// Extensions for gender
  #[serde(rename = "_gender")]
  _gender: Option<Element>,

  /// An address for the individual.
  address: Option<Vec<Address>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Extensions for active
  #[serde(rename = "_active")]
  _active: Option<Element>,

  /// Extensions for deceasedDateTime
  #[serde(rename = "_deceasedDateTime")]
  _deceased_date_time: Option<Element>,

  /// Administrative Gender - the gender that the patient is considered to have for
  /// administration and record keeping purposes.
  gender: Option<PatientGender>,

  /// Indicates whether the patient is part of a multiple (boolean) or indicates the
  /// actual birth order (integer).
  #[serde(rename = "multipleBirthInteger")]
  multiple_birth_integer: Option<i32>,

  /// Patient's nominated care provider.
  #[serde(rename = "generalPractitioner")]
  general_practitioner: Option<Vec<Box<Reference>>>,

  /// Image of the patient.
  photo: Option<Vec<Attachment>>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// A name associated with the individual.
  name: Option<Vec<HumanName>>,

  /// Indicates whether the patient is part of a multiple (boolean) or indicates the
  /// actual birth order (integer).
  #[serde(rename = "multipleBirthBoolean")]
  multiple_birth_boolean: Option<bool>,

  /// Extensions for multipleBirthInteger
  #[serde(rename = "_multipleBirthInteger")]
  _multiple_birth_integer: Option<Element>,

  /// A language which may be used to communicate with the patient about his or her
  /// health.
  communication: Option<Vec<Patient_Communication>>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

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
  modifier_extension: Option<Vec<Extension>>,

  /// This field contains a patient's most recent marital (civil) status.
  #[serde(rename = "maritalStatus")]
  marital_status: Option<CodeableConcept>,

  /// Extensions for birthDate
  #[serde(rename = "_birthDate")]
  _birth_date: Option<Element>,

  /// A contact party (e.g. guardian, partner, friend) for the patient.
  contact: Option<Vec<Patient_Contact>>,

  /// Whether this patient record is in active use.   Many systems use this property
  /// to mark as non-current patients, such as those that have not been seen for a
  /// period of time based on an organization's business rules.    It is often used to
  /// filter patient lists to exclude inactive patients    Deceased patients may also
  /// be marked as inactive for the same reasons, but may be active for some time
  /// after death.
  active: Option<bool>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// Link to another patient resource that concerns the same actual patient.
  link: Option<Vec<Patient_Link>>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Extensions for deceasedBoolean
  #[serde(rename = "_deceasedBoolean")]
  _deceased_boolean: Option<Element>,

  /// A contact detail (e.g. a telephone number or an email address) by which the
  /// individual may be contacted.
  telecom: Option<Vec<ContactPoint>>,

  /// Organization that is the custodian of the patient record.
  #[serde(rename = "managingOrganization")]
  managing_organization: Option<Box<Reference>>,

  /// An identifier for this patient.
  identifier: Option<Vec<Identifier>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The date of birth for the individual.
  #[serde(rename = "birthDate")]
  birth_date: Option<i32>,

  /// Indicates if the individual is deceased or not.
  #[serde(rename = "deceasedBoolean")]
  deceased_boolean: Option<bool>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// Extensions for multipleBirthBoolean
  #[serde(rename = "_multipleBirthBoolean")]
  _multiple_birth_boolean: Option<Element>,

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
