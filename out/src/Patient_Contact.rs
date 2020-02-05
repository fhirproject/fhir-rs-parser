use serde::{Deserialize, Serialize};

/// Demographics and other administrative information about an individual or animal
/// receiving care or other health-related services.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Patient_Contact {
  /// The period during which this contact person or organization is valid to be
  /// contacted relating to this patient.
  period: Period,

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

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Address for the contact person.
  address: Address,

  /// A contact detail for the person, e.g. a telephone number or an email address.
  telecom: Vec<ContactPoint>,

  /// The nature of the relationship between the patient and the contact person.
  relationship: Vec<CodeableConcept>,

  /// Extensions for gender
  _gender: Element,

  /// A name associated with the contact person.
  name: HumanName,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Administrative Gender - the gender that the contact person is considered to have
  /// for administration and record keeping purposes.
  gender: Patient_ContactGender,

  /// Organization on behalf of which the contact is acting or for which the contact
  /// is working.
  organization: Reference,

}

#[derive(Debug, Serialize, Deserialize)]
enum Patient_ContactGender {
  #[serde(rename = "male")]
  Male,

  #[serde(rename = "female")]
  Female,

  #[serde(rename = "other")]
  Other,

  #[serde(rename = "unknown")]
  Unknown,

}
