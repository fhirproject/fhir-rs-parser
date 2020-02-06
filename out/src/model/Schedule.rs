#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::Meta::Meta;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Identifier::Identifier;
use crate::model::Element::Element;


/// A container for slots of time that may be available for booking appointments.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
  /// The specialty of a practitioner that would be required to perform the service
  /// requested in this appointment.
  specialty: Option<Vec<CodeableConcept>>,

  /// Extensions for comment
  #[serde(rename = "_comment")]
  _comment: Option<Element>,

  /// External Ids for this item.
  identifier: Option<Vec<Identifier>>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for active
  #[serde(rename = "_active")]
  _active: Option<Element>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

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
  modifier_extension: Option<Vec<Box<Extension>>>,

  /// Whether this schedule record is in active use or should not be used (such as was
  /// entered in error).
  active: Option<bool>,

  /// Slots that reference this schedule resource provide the availability details to
  /// these referenced resource(s).
  actor: Vec<Box<Reference>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// The period of time that the slots that reference this Schedule resource cover
  /// (even if none exist). These  cover the amount of time that an organization's
  /// planning horizon; the interval for which they are currently accepting
  /// appointments. This does not define a "template" for planning outside these
  /// dates.
  #[serde(rename = "planningHorizon")]
  planning_horizon: Option<Period>,

  /// The specific service that is to be performed during this appointment.
  #[serde(rename = "serviceType")]
  service_type: Option<Vec<CodeableConcept>>,

  /// Comments on the availability to describe any extended information. Such as
  /// custom constraints on the slots that may be associated.
  comment: Option<String>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// The base language in which the resource is written.
  language: Option<String>,

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

}
