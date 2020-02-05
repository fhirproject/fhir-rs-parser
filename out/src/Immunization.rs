use serde::{Deserialize, Serialize};

/// Describes the event of a patient being administered a vaccine or a record of an
/// immunization as reported by a patient, a clinician or another party.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Immunization {
  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Indicates who performed the immunization event.
  performer: Vec<Immunization_Performer>,

  /// Indicates the reason the immunization event was not performed.
  #[serde(rename = "statusReason")]
  status_reason: CodeableConcept,

  /// Extensions for occurrenceDateTime
  #[serde(rename = "_occurrenceDateTime")]
  _occurrence_date_time: Element,

  /// Date vaccine administered or was to be administered.
  #[serde(rename = "occurrenceDateTime")]
  occurrence_date_time: String,

  /// Vaccine that was administered or was to be administered.
  #[serde(rename = "vaccineCode")]
  vaccine_code: CodeableConcept,

  /// Reasons why the vaccine was administered.
  #[serde(rename = "reasonCode")]
  reason_code: Vec<CodeableConcept>,

  /// Extra information about the immunization that is not conveyed by the other
  /// attributes.
  note: Vec<Annotation>,

  /// Lot number of the  vaccine product.
  #[serde(rename = "lotNumber")]
  lot_number: String,

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

  /// Indication if a dose is considered to be subpotent. By default, a dose should be
  /// considered to be potent.
  #[serde(rename = "isSubpotent")]
  is_subpotent: bool,

  /// The base language in which the resource is written.
  language: String,

  /// Indicates a patient's eligibility for a funding program.
  #[serde(rename = "programEligibility")]
  program_eligibility: Vec<CodeableConcept>,

  /// The protocol (set of recommendations) being followed by the provider who
  /// administered the dose.
  #[serde(rename = "protocolApplied")]
  protocol_applied: Vec<Immunization_ProtocolApplied>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Indicates the current status of the immunization event.
  status: String,

  /// Extensions for status
  _status: Element,

  /// Reason why a dose is considered to be subpotent.
  #[serde(rename = "subpotentReason")]
  subpotent_reason: Vec<CodeableConcept>,

  /// The patient who either received or did not receive the immunization.
  patient: Reference,

  /// A unique identifier assigned to this immunization record.
  identifier: Vec<Identifier>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Extensions for language
  _language: Element,

  /// The visit or admission or other contact between patient and health care provider
  /// the immunization was performed as part of.
  encounter: Reference,

  /// Name of vaccine manufacturer.
  manufacturer: Reference,

  /// Extensions for primarySource
  #[serde(rename = "_primarySource")]
  _primary_source: Element,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// The date the occurrence of the immunization was first captured in the record -
  /// potentially significantly after the occurrence of the event.
  recorded: dateTime,

  /// The source of the data when the report of the immunization event is not based on
  /// information from the person who administered the vaccine.
  #[serde(rename = "reportOrigin")]
  report_origin: CodeableConcept,

  /// Extensions for lotNumber
  #[serde(rename = "_lotNumber")]
  _lot_number: Element,

  /// Date vaccine batch expires.
  #[serde(rename = "expirationDate")]
  expiration_date: date,

  /// Extensions for expirationDate
  #[serde(rename = "_expirationDate")]
  _expiration_date: Element,

  /// Condition, Observation or DiagnosticReport that supports why the immunization
  /// was administered.
  #[serde(rename = "reasonReference")]
  reason_reference: Vec<Reference>,

  /// Date vaccine administered or was to be administered.
  #[serde(rename = "occurrenceString")]
  occurrence_string: String,

  /// Extensions for recorded
  _recorded: Element,

  /// Extensions for occurrenceString
  #[serde(rename = "_occurrenceString")]
  _occurrence_string: Element,

  /// An indication that the content of the record is based on information from the
  /// person who administered the vaccine. This reflects the context under which the
  /// data was originally recorded.
  #[serde(rename = "primarySource")]
  primary_source: bool,

  /// Extensions for isSubpotent
  #[serde(rename = "_isSubpotent")]
  _is_subpotent: Element,

  /// Educational material presented to the patient (or guardian) at the time of
  /// vaccine administration.
  education: Vec<Immunization_Education>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The service delivery location where the vaccine administration occurred.
  location: Reference,

  /// The quantity of vaccine product that was administered.
  #[serde(rename = "doseQuantity")]
  dose_quantity: Quantity,

  /// Indicates the source of the vaccine actually administered. This may be different
  /// than the patient eligibility (e.g. the patient may be eligible for a publically
  /// purchased vaccine but due to inventory issues, vaccine purchased with private
  /// funds was actually administered).
  #[serde(rename = "fundingSource")]
  funding_source: CodeableConcept,

  /// Categorical data indicating that an adverse event is associated in time to an
  /// immunization.
  reaction: Vec<Immunization_Reaction>,

  /// The path by which the vaccine product is taken into the body.
  route: CodeableConcept,

  /// Body site where vaccine was administered.
  site: CodeableConcept,

}
