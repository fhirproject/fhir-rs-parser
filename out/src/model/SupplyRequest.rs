#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Narrative::Narrative;
use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::ResourceList::ResourceList;
use crate::model::Meta::Meta;
use crate::model::SupplyRequest_Parameter::SupplyRequest_Parameter;
use crate::model::Period::Period;
use crate::model::Quantity::Quantity;
use crate::model::Identifier::Identifier;
use crate::model::Reference::Reference;
use crate::model::Timing::Timing;


/// A record of a request for a medication, substance or device used in the
/// healthcare setting.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupplyRequest {
  /// The amount that is being ordered of the indicated item.
  quantity: Quantity,

  /// Business identifiers assigned to this SupplyRequest by the author and/or other
  /// systems. These identifiers remain constant as the resource is updated and
  /// propagates from server to server.
  identifier: Option<Vec<Identifier>>,

  /// The item that is requested to be supplied. This is either a link to a resource
  /// representing the details of the item or a code that identifies the item from a
  /// known list.
  #[serde(rename = "itemReference")]
  item_reference: Option<Box<Reference>>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// Where the supply is expected to come from.
  #[serde(rename = "deliverFrom")]
  deliver_from: Option<Box<Reference>>,

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

  /// Category of supply, e.g.  central, non-stock, etc. This is used to support work
  /// flows associated with the supply process.
  category: Option<CodeableConcept>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// When the request was made.
  #[serde(rename = "authoredOn")]
  authored_on: Option<String>,

  /// When the request should be fulfilled.
  #[serde(rename = "occurrenceDateTime")]
  occurrence_date_time: Option<String>,

  /// When the request should be fulfilled.
  #[serde(rename = "occurrenceTiming")]
  occurrence_timing: Option<Timing>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Where the supply is destined to go.
  #[serde(rename = "deliverTo")]
  deliver_to: Option<Box<Reference>>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Extensions for authoredOn
  #[serde(rename = "_authoredOn")]
  _authored_on: Option<Element>,

  /// The device, practitioner, etc. who initiated the request.
  requester: Option<Box<Reference>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// When the request should be fulfilled.
  #[serde(rename = "occurrencePeriod")]
  occurrence_period: Option<Period>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Extensions for priority
  #[serde(rename = "_priority")]
  _priority: Option<Element>,

  /// The reason why the supply item was requested.
  #[serde(rename = "reasonCode")]
  reason_code: Option<Vec<CodeableConcept>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Indicates how quickly this SupplyRequest should be addressed with respect to
  /// other requests.
  priority: Option<String>,

  /// The item that is requested to be supplied. This is either a link to a resource
  /// representing the details of the item or a code that identifies the item from a
  /// known list.
  #[serde(rename = "itemCodeableConcept")]
  item_codeable_concept: Option<CodeableConcept>,

  /// Status of the supply request.
  status: Option<SupplyRequestStatus>,

  /// Specific parameters for the ordered item.  For example, the size of the
  /// indicated item.
  parameter: Option<Vec<SupplyRequest_Parameter>>,

  /// Extensions for occurrenceDateTime
  #[serde(rename = "_occurrenceDateTime")]
  _occurrence_date_time: Option<Element>,

  /// Who is intended to fulfill the request.
  supplier: Option<Vec<Box<Reference>>>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// The reason why the supply item was requested.
  #[serde(rename = "reasonReference")]
  reason_reference: Option<Vec<Box<Reference>>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum SupplyRequestStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "suspended")]
  Suspended,

  #[serde(rename = "cancelled")]
  Cancelled,

  #[serde(rename = "completed")]
  Completed,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

  #[serde(rename = "unknown")]
  Unknown,

}
