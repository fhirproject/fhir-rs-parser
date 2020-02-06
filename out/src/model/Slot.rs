#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::ResourceList::ResourceList;
use crate::model::Meta::Meta;
use crate::model::Extension::Extension;
use crate::model::Narrative::Narrative;
use crate::model::Identifier::Identifier;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Reference::Reference;


/// A slot of time on a schedule that may be available for booking appointments.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Slot {
  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

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

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// The style of appointment or patient that may be booked in the slot (not service
  /// type).
  #[serde(rename = "appointmentType")]
  appointment_type: Option<CodeableConcept>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Comments on the slot to describe any extended information. Such as custom
  /// constraints on the slot.
  comment: Option<String>,

  /// Extensions for overbooked
  #[serde(rename = "_overbooked")]
  _overbooked: Option<Element>,

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

  /// The base language in which the resource is written.
  language: Option<String>,

  /// The specialty of a practitioner that would be required to perform the service
  /// requested in this appointment.
  specialty: Option<Vec<CodeableConcept>>,

  /// busy | free | busy-unavailable | busy-tentative | entered-in-error.
  status: Option<SlotStatus>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// Extensions for end
  #[serde(rename = "_end")]
  _end: Option<Element>,

  /// External Ids for this item.
  identifier: Option<Vec<Identifier>>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// A broad categorization of the service that is to be performed during this
  /// appointment.
  #[serde(rename = "serviceCategory")]
  service_category: Option<Vec<CodeableConcept>>,

  /// Extensions for start
  #[serde(rename = "_start")]
  _start: Option<Element>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The type of appointments that can be booked into this slot (ideally this would
  /// be an identifiable service - which is at a location, rather than the location
  /// itself). If provided then this overrides the value provided on the availability
  /// resource.
  #[serde(rename = "serviceType")]
  service_type: Option<Vec<CodeableConcept>>,

  /// The schedule resource that this slot defines an interval of status information.
  schedule: Box<Reference>,

  /// Date/Time that the slot is to begin.
  start: Option<String>,

  /// This slot has already been overbooked, appointments are unlikely to be accepted
  /// for this time.
  overbooked: Option<bool>,

  /// Extensions for comment
  #[serde(rename = "_comment")]
  _comment: Option<Element>,

  /// Date/Time that the slot is to conclude.
  end: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum SlotStatus {
  #[serde(rename = "busy")]
  Busy,

  #[serde(rename = "free")]
  Free,

  #[serde(rename = "busy-unavailable")]
  BusyUnavailable,

  #[serde(rename = "busy-tentative")]
  BusyTentative,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

}
