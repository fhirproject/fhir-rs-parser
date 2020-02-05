use serde::{Deserialize, Serialize};

/// Indicates that a medication product is to be or has been dispensed for a named
/// person/patient.  This includes a description of the medication product (supply)
/// provided and the instructions for administering the medication.  The medication
/// dispense is the result of a pharmacy system responding to a medication order.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MedicationDispense {
  /// Additional information that supports the medication being dispensed.
  #[serde(rename = "supportingInformation")]
  supporting_information: Vec<Reference>,

  /// The time the dispensed product was provided to the patient or their
  /// representative.
  #[serde(rename = "whenHandedOver")]
  when_handed_over: dateTime,

  /// A summary of the events of interest that have occurred, such as when the
  /// dispense was verified.
  #[serde(rename = "eventHistory")]
  event_history: Vec<Reference>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Indicates the medication order that is being dispensed against.
  #[serde(rename = "authorizingPrescription")]
  authorizing_prescription: Vec<Reference>,

  /// Extra information about the dispense that could not be conveyed in the other
  /// attributes.
  note: Vec<Annotation>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// The time when the dispensed product was packaged and reviewed.
  #[serde(rename = "whenPrepared")]
  when_prepared: dateTime,

  /// The principal physical location where the dispense was performed.
  location: Reference,

  /// Indicates an actual or potential clinical issue with or between one or more
  /// active or proposed clinical actions for a patient; e.g. drug-drug interaction,
  /// duplicate therapy, dosage alert etc.
  #[serde(rename = "detectedIssue")]
  detected_issue: Vec<Reference>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Indicates how the medication is to be used by the patient.
  #[serde(rename = "dosageInstruction")]
  dosage_instruction: Vec<Dosage>,

  /// Indicates the type of medication dispense (for example, where the medication is
  /// expected to be consumed or administered (i.e. inpatient or outpatient)).
  category: CodeableConcept,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Extensions for whenHandedOver
  #[serde(rename = "_whenHandedOver")]
  _when_handed_over: Element,

  /// The procedure that trigger the dispense.
  #[serde(rename = "partOf")]
  part_of: Vec<Reference>,

  /// Indicates the type of dispensing event that is performed. For example, Trial
  /// Fill, Completion of Trial, Partial Fill, Emergency Fill, Samples, etc.
  type: CodeableConcept,

  /// The base language in which the resource is written.
  language: String,

  /// Extensions for status
  _status: Element,

  /// The amount of medication that has been dispensed. Includes unit of measure.
  quantity: Quantity,

  /// Indicates the reason why a dispense was not performed.
  #[serde(rename = "statusReasonReference")]
  status_reason_reference: Reference,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Extensions for whenPrepared
  #[serde(rename = "_whenPrepared")]
  _when_prepared: Element,

  /// The amount of medication expressed as a timing amount.
  #[serde(rename = "daysSupply")]
  days_supply: Quantity,

  /// Extensions for language
  _language: Element,

  /// Identification of the facility/location where the medication was shipped to, as
  /// part of the dispense event.
  destination: Reference,

  /// Identifiers associated with this Medication Dispense that are defined by
  /// business processes and/or used to refer to it when a direct URL reference to the
  /// resource itself is not appropriate. They are business identifiers assigned to
  /// this resource by the performer or other systems and remain constant as the
  /// resource is updated and propagates from server to server.
  identifier: Vec<Identifier>,

  /// Identifies the person who picked up the medication.  This will usually be a
  /// patient or their caregiver, but some cases exist where it can be a healthcare
  /// professional.
  receiver: Vec<Reference>,

  /// Indicates the reason why a dispense was not performed.
  #[serde(rename = "statusReasonCodeableConcept")]
  status_reason_codeable_concept: CodeableConcept,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Indicates whether or not substitution was made as part of the dispense.  In some
  /// cases, substitution will be expected but does not happen, in other cases
  /// substitution is not expected but does happen.  This block explains what
  /// substitution did or did not happen and why.  If nothing is specified,
  /// substitution was not done.
  substitution: MedicationDispense_Substitution,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

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

  /// A code specifying the state of the set of dispense events.
  status: String,

  /// Indicates who or what performed the event.
  performer: Vec<MedicationDispense_Performer>,

  /// The encounter or episode of care that establishes the context for this event.
  context: Reference,

  /// Identifies the medication being administered. This is either a link to a
  /// resource representing the details of the medication or a simple attribute
  /// carrying a code that identifies the medication from a known list of medications.
  #[serde(rename = "medicationCodeableConcept")]
  medication_codeable_concept: CodeableConcept,

  /// Identifies the medication being administered. This is either a link to a
  /// resource representing the details of the medication or a simple attribute
  /// carrying a code that identifies the medication from a known list of medications.
  #[serde(rename = "medicationReference")]
  medication_reference: Reference,

  /// A link to a resource representing the person or the group to whom the medication
  /// will be given.
  subject: Reference,

}
