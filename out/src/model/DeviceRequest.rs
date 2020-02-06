#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Timing::Timing;
use crate::model::Extension::Extension;
use crate::model::Narrative::Narrative;
use crate::model::Identifier::Identifier;
use crate::model::Annotation::Annotation;
use crate::model::ResourceList::ResourceList;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::DeviceRequest_Parameter::DeviceRequest_Parameter;
use crate::model::Meta::Meta;


/// Represents a request for a patient to employ a medical device. The device may be
/// an implantable device, or an external assistive device, such as a walker.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRequest {
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

  /// The timing schedule for the use of the device. The Schedule data type allows
  /// many different expressions, for example. "Every 8 hours"; "Three times a day";
  /// "1/2 an hour before breakfast for 10 days from 23-Dec 2011:"; "15 Oct 2013, 17
  /// Oct 2013 and 1 Nov 2013".
  #[serde(rename = "occurrenceTiming")]
  occurrence_timing: Option<Timing>,

  /// Extensions for instantiatesUri
  #[serde(rename = "_instantiatesUri")]
  _instantiates_uri: Option<Vec<Element>>,

  /// Extensions for intent
  #[serde(rename = "_intent")]
  _intent: Option<Element>,

  /// The desired performer for doing the diagnostic testing.
  performer: Option<Box<Reference>>,

  /// Reason or justification for the use of this device.
  #[serde(rename = "reasonReference")]
  reason_reference: Option<Vec<Box<Reference>>>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// Composite request this is part of.
  #[serde(rename = "groupIdentifier")]
  group_identifier: Option<Identifier>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// The details of the device to be used.
  #[serde(rename = "codeCodeableConcept")]
  code_codeable_concept: Option<CodeableConcept>,

  /// The patient who will use the device.
  subject: Box<Reference>,

  /// Extensions for occurrenceDateTime
  #[serde(rename = "_occurrenceDateTime")]
  _occurrence_date_time: Option<Element>,

  /// The request takes the place of the referenced completed or terminated
  /// request(s).
  #[serde(rename = "priorRequest")]
  prior_request: Option<Vec<Box<Reference>>>,

  /// The individual who initiated the request and has responsibility for its
  /// activation.
  requester: Option<Box<Reference>>,

  /// The URL pointing to an externally maintained protocol, guideline, orderset or
  /// other definition that is adhered to in whole or in part by this DeviceRequest.
  #[serde(rename = "instantiatesUri")]
  instantiates_uri: Option<Vec<String>>,

  /// An encounter that provides additional context in which this request is made.
  encounter: Option<Box<Reference>>,

  /// Extensions for priority
  #[serde(rename = "_priority")]
  _priority: Option<Element>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Extensions for authoredOn
  #[serde(rename = "_authoredOn")]
  _authored_on: Option<Element>,

  /// Additional clinical information about the patient that may influence the request
  /// fulfilment.  For example, this may include where on the subject's body the
  /// device will be used (i.e. the target site).
  #[serde(rename = "supportingInfo")]
  supporting_info: Option<Vec<Box<Reference>>>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// The status of the request.
  status: Option<String>,

  /// Indicates how quickly the {{title}} should be addressed with respect to other
  /// requests.
  priority: Option<String>,

  /// The URL pointing to a FHIR-defined protocol, guideline, orderset or other
  /// definition that is adhered to in whole or in part by this DeviceRequest.
  #[serde(rename = "instantiatesCanonical")]
  instantiates_canonical: Option<Vec<String>>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Details about this request that were not represented at all or sufficiently in
  /// one of the attributes provided in a class. These may include for example a
  /// comment, an instruction, or a note associated with the statement.
  note: Option<Vec<Annotation>>,

  /// When the request transitioned to being actionable.
  #[serde(rename = "authoredOn")]
  authored_on: Option<String>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Identifiers assigned to this order by the orderer or by the receiver.
  identifier: Option<Vec<Identifier>>,

  /// Whether the request is a proposal, plan, an original order or a reflex order.
  intent: Option<String>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// The timing schedule for the use of the device. The Schedule data type allows
  /// many different expressions, for example. "Every 8 hours"; "Three times a day";
  /// "1/2 an hour before breakfast for 10 days from 23-Dec 2011:"; "15 Oct 2013, 17
  /// Oct 2013 and 1 Nov 2013".
  #[serde(rename = "occurrenceDateTime")]
  occurrence_date_time: Option<String>,

  /// The details of the device to be used.
  #[serde(rename = "codeReference")]
  code_reference: Option<Box<Reference>>,

  /// Reason or justification for the use of this device.
  #[serde(rename = "reasonCode")]
  reason_code: Option<Vec<CodeableConcept>>,

  /// Specific parameters for the ordered item.  For example, the prism value for
  /// lenses.
  parameter: Option<Vec<DeviceRequest_Parameter>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The timing schedule for the use of the device. The Schedule data type allows
  /// many different expressions, for example. "Every 8 hours"; "Three times a day";
  /// "1/2 an hour before breakfast for 10 days from 23-Dec 2011:"; "15 Oct 2013, 17
  /// Oct 2013 and 1 Nov 2013".
  #[serde(rename = "occurrencePeriod")]
  occurrence_period: Option<Period>,

  /// Desired type of performer for doing the diagnostic testing.
  #[serde(rename = "performerType")]
  performer_type: Option<CodeableConcept>,

  /// Plan/proposal/order fulfilled by this request.
  #[serde(rename = "basedOn")]
  based_on: Option<Vec<Box<Reference>>>,

  /// Insurance plans, coverage extensions, pre-authorizations and/or pre-
  /// determinations that may be required for delivering the requested service.
  insurance: Option<Vec<Box<Reference>>>,

  /// Key events in the history of the request.
  #[serde(rename = "relevantHistory")]
  relevant_history: Option<Vec<Box<Reference>>>,

}
