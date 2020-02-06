#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::MedicationAdministration_Performer::MedicationAdministration_Performer;
use crate::model::Reference::Reference;
use crate::model::Annotation::Annotation;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::Element::Element;
use crate::model::Identifier::Identifier;
use crate::model::MedicationAdministration_Dosage::MedicationAdministration_Dosage;
use crate::model::ResourceList::ResourceList;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;


/// Describes the event of a patient consuming or otherwise being administered a
/// medication.  This may be as simple as swallowing a tablet or it may be a long
/// running infusion.  Related resources tie this event to the authorizing
/// prescription, and the specific encounter between patient and health care
/// practitioner.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationAdministration {
  /// The person or animal or group receiving the medication.
  subject: Box<Reference>,

  /// A protocol, guideline, orderset, or other definition that was adhered to in
  /// whole or in part by this event.
  instantiates: Vec<String>,

  /// Identifiers associated with this Medication Administration that are defined by
  /// business processes and/or used to refer to it when a direct URL reference to the
  /// resource itself is not appropriate. They are business identifiers assigned to
  /// this resource by the performer or other systems and remain constant as the
  /// resource is updated and propagates from server to server.
  identifier: Vec<Identifier>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Extensions for effectiveDateTime
  #[serde(rename = "_effectiveDateTime")]
  _effective_date_time: Element,

  /// The base language in which the resource is written.
  language: String,

  /// A specific date/time or interval of time during which the administration took
  /// place (or did not take place, when the 'notGiven' attribute is true). For many
  /// administrations, such as swallowing a tablet the use of dateTime is more
  /// appropriate.
  #[serde(rename = "effectiveDateTime")]
  effective_date_time: String,

  /// Indicates who or what performed the medication administration and how they were
  /// involved.
  performer: Vec<MedicationAdministration_Performer>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Additional information (for example, patient height and weight) that supports
  /// the administration of the medication.
  #[serde(rename = "supportingInformation")]
  supporting_information: Vec<Box<Reference>>,

  /// A code indicating why the medication was given.
  #[serde(rename = "reasonCode")]
  reason_code: Vec<CodeableConcept>,

  /// The original request, instruction or authority to perform the administration.
  request: Box<Reference>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Extra information about the medication administration that is not conveyed by
  /// the other attributes.
  note: Vec<Annotation>,

  /// Will generally be set to show that the administration has been completed.  For
  /// some long running administrations such as infusions, it is possible for an
  /// administration to be started but not completed or it may be paused while some
  /// other process is under way.
  status: String,

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

  /// The visit, admission, or other contact between patient and health care provider
  /// during which the medication administration was performed.
  context: Box<Reference>,

  /// Describes the medication dosage information details e.g. dose, rate, site,
  /// route, etc.
  dosage: MedicationAdministration_Dosage,

  /// A summary of the events of interest that have occurred, such as when the
  /// administration was verified.
  #[serde(rename = "eventHistory")]
  event_history: Vec<Box<Reference>>,

  /// A code indicating why the administration was not performed.
  #[serde(rename = "statusReason")]
  status_reason: Vec<CodeableConcept>,

  /// Identifies the medication that was administered. This is either a link to a
  /// resource representing the details of the medication or a simple attribute
  /// carrying a code that identifies the medication from a known list of medications.
  #[serde(rename = "medicationReference")]
  medication_reference: Box<Reference>,

  /// Condition or observation that supports why the medication was administered.
  #[serde(rename = "reasonReference")]
  reason_reference: Vec<Box<Reference>>,

  /// The device used in administering the medication to the patient.  For example, a
  /// particular infusion pump.
  device: Vec<Box<Reference>>,

  /// A larger event of which this particular event is a component or step.
  #[serde(rename = "partOf")]
  part_of: Vec<Box<Reference>>,

  /// Extensions for status
  _status: Element,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Extensions for instantiates
  _instantiates: Vec<Element>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Identifies the medication that was administered. This is either a link to a
  /// resource representing the details of the medication or a simple attribute
  /// carrying a code that identifies the medication from a known list of medications.
  #[serde(rename = "medicationCodeableConcept")]
  medication_codeable_concept: CodeableConcept,

  /// A specific date/time or interval of time during which the administration took
  /// place (or did not take place, when the 'notGiven' attribute is true). For many
  /// administrations, such as swallowing a tablet the use of dateTime is more
  /// appropriate.
  #[serde(rename = "effectivePeriod")]
  effective_period: Period,

  /// Indicates where the medication is expected to be consumed or administered.
  category: CodeableConcept,

  /// Extensions for language
  _language: Element,

}
