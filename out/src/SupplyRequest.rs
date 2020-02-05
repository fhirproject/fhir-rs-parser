use serde::{Deserialize, Serialize};

/// A record of a request for a medication, substance or device used in the
/// healthcare setting.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SupplyRequest {
  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Extensions for status
  _status: Element,

  /// The reason why the supply item was requested.
  #[serde(rename = "reasonCode")]
  reason_code: Vec<CodeableConcept>,

  /// Status of the supply request.
  status: SupplyRequestStatus,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// When the request should be fulfilled.
  #[serde(rename = "occurrenceDateTime")]
  occurrence_date_time: String,

  /// The amount that is being ordered of the indicated item.
  quantity: Quantity,

  /// When the request was made.
  #[serde(rename = "authoredOn")]
  authored_on: dateTime,

  /// Category of supply, e.g.  central, non-stock, etc. This is used to support work
  /// flows associated with the supply process.
  category: CodeableConcept,

  /// Extensions for language
  _language: Element,

  /// The item that is requested to be supplied. This is either a link to a resource
  /// representing the details of the item or a code that identifies the item from a
  /// known list.
  #[serde(rename = "itemReference")]
  item_reference: Reference,

  /// Where the supply is destined to go.
  #[serde(rename = "deliverTo")]
  deliver_to: Reference,

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

  /// Who is intended to fulfill the request.
  supplier: Vec<Reference>,

  /// The device, practitioner, etc. who initiated the request.
  requester: Reference,

  /// Indicates how quickly this SupplyRequest should be addressed with respect to
  /// other requests.
  priority: String,

  /// Extensions for priority
  _priority: Element,

  /// Extensions for authoredOn
  #[serde(rename = "_authoredOn")]
  _authored_on: Element,

  /// Where the supply is expected to come from.
  #[serde(rename = "deliverFrom")]
  deliver_from: Reference,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// When the request should be fulfilled.
  #[serde(rename = "occurrencePeriod")]
  occurrence_period: Period,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Extensions for occurrenceDateTime
  #[serde(rename = "_occurrenceDateTime")]
  _occurrence_date_time: Element,

  /// Specific parameters for the ordered item.  For example, the size of the
  /// indicated item.
  parameter: Vec<SupplyRequest_Parameter>,

  /// Business identifiers assigned to this SupplyRequest by the author and/or other
  /// systems. These identifiers remain constant as the resource is updated and
  /// propagates from server to server.
  identifier: Vec<Identifier>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The reason why the supply item was requested.
  #[serde(rename = "reasonReference")]
  reason_reference: Vec<Reference>,

  /// The item that is requested to be supplied. This is either a link to a resource
  /// representing the details of the item or a code that identifies the item from a
  /// known list.
  #[serde(rename = "itemCodeableConcept")]
  item_codeable_concept: CodeableConcept,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// The base language in which the resource is written.
  language: String,

  /// When the request should be fulfilled.
  #[serde(rename = "occurrenceTiming")]
  occurrence_timing: Timing,

}

#[derive(Debug, Serialize, Deserialize)]
enum SupplyRequestStatus {
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
