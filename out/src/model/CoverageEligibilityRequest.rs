#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Identifier::Identifier;
use crate::model::Period::Period;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Narrative::Narrative;
use crate::model::CoverageEligibilityRequest_SupportingInfo::CoverageEligibilityRequest_SupportingInfo;
use crate::model::CoverageEligibilityRequest_Item::CoverageEligibilityRequest_Item;
use crate::model::Element::Element;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::Meta::Meta;
use crate::model::CoverageEligibilityRequest_Insurance::CoverageEligibilityRequest_Insurance;


/// The CoverageEligibilityRequest provides patient and insurance coverage
/// information to an insurer for them to respond, in the form of an
/// CoverageEligibilityResponse, with information regarding whether the stated
/// coverage is valid and in-force and optionally to provide the insurance details
/// of the policy.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityRequest {
  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// The Insurer who issued the coverage in question and is the recipient of the
  /// request.
  insurer: Box<Reference>,

  /// Facility where the services are intended to be provided.
  facility: Option<Box<Reference>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

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

  /// Extensions for servicedDate
  #[serde(rename = "_servicedDate")]
  _serviced_date: Option<Element>,

  /// Person who created the request.
  enterer: Option<Box<Reference>>,

  /// Additional information codes regarding exceptions, special considerations, the
  /// condition, situation, prior or concurrent issues.
  #[serde(rename = "supportingInfo")]
  supporting_info: Option<Vec<CoverageEligibilityRequest_SupportingInfo>>,

  /// Service categories or billable services for which benefit details and/or an
  /// authorization prior to service delivery may be required by the payor.
  item: Option<Vec<CoverageEligibilityRequest_Item>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// The date when this resource was created.
  created: Option<String>,

  /// Financial instruments for reimbursement for the health care products and
  /// services.
  insurance: Option<Vec<CoverageEligibilityRequest_Insurance>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// The date or dates when the enclosed suite of services were performed or
  /// completed.
  #[serde(rename = "servicedDate")]
  serviced_date: Option<String>,

  /// Extensions for purpose
  #[serde(rename = "_purpose")]
  _purpose: Option<Vec<Element>>,

  /// The date or dates when the enclosed suite of services were performed or
  /// completed.
  #[serde(rename = "servicedPeriod")]
  serviced_period: Option<Period>,

  /// Extensions for created
  #[serde(rename = "_created")]
  _created: Option<Element>,

  /// The status of the resource instance.
  status: Option<String>,

  /// The party who is the beneficiary of the supplied coverage and for whom
  /// eligibility is sought.
  patient: Box<Reference>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// A unique identifier assigned to this coverage eligiblity request.
  identifier: Option<Vec<Identifier>>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// When the requestor expects the processor to complete processing.
  priority: Option<CodeableConcept>,

  /// The provider which is responsible for the request.
  provider: Option<Box<Reference>>,

}
