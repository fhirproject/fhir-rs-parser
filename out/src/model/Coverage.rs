#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::Identifier::Identifier;
use crate::model::ResourceList::ResourceList;
use crate::model::Coverage_CostToBeneficiary::Coverage_CostToBeneficiary;
use crate::model::Narrative::Narrative;
use crate::model::Coverage_Class::Coverage_Class;
use crate::model::Meta::Meta;


/// Financial instrument which may be used to reimburse or pay for health care
/// products and services. Includes both insurance and self-payment.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coverage {
  /// The status of the resource instance.
  status: Option<String>,

  /// The insurer-specific identifier for the insurer-defined network of providers to
  /// which the beneficiary may seek treatment which will be covered at the 'in-
  /// network' rate, otherwise 'out of network' terms and conditions apply.
  network: Option<String>,

  /// The party who has signed-up for or 'owns' the contractual relationship to the
  /// policy or to whom the benefit of the policy for services rendered to them or
  /// their family is due.
  subscriber: Option<Box<Reference>>,

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

  /// The program or plan underwriter or payor including both insurance and non-
  /// insurance agreements, such as patient-pay agreements.
  payor: Vec<Box<Reference>>,

  /// Extensions for dependent
  #[serde(rename = "_dependent")]
  _dependent: Option<Element>,

  /// The party who benefits from the insurance coverage; the patient when products
  /// and/or services are provided.
  beneficiary: Box<Reference>,

  /// Extensions for subrogation
  #[serde(rename = "_subrogation")]
  _subrogation: Option<Element>,

  /// The policy(s) which constitute this insurance coverage.
  contract: Option<Vec<Box<Reference>>>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// When 'subrogation=true' this insurance instance has been included not for
  /// adjudication but to provide insurers with the details to recover costs.
  subrogation: Option<bool>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Time period during which the coverage is in force. A missing start date
  /// indicates the start date isn't known, a missing end date means the coverage is
  /// continuing to be in force.
  period: Option<Period>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// The relationship of beneficiary (patient) to the subscriber.
  relationship: Option<CodeableConcept>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Extensions for order
  #[serde(rename = "_order")]
  _order: Option<Element>,

  /// A unique identifier assigned to this coverage.
  identifier: Option<Vec<Identifier>>,

  /// Extensions for network
  #[serde(rename = "_network")]
  _network: Option<Element>,

  /// A suite of codes indicating the cost category and associated amount which have
  /// been detailed in the policy and may have been  included on the health card.
  #[serde(rename = "costToBeneficiary")]
  cost_to_beneficiary: Option<Vec<Coverage_CostToBeneficiary>>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// A suite of underwriter specific classifiers.
  class: Option<Vec<Coverage_Class>>,

  /// A unique identifier for a dependent under the coverage.
  dependent: Option<String>,

  /// The insurer assigned ID for the Subscriber.
  #[serde(rename = "subscriberId")]
  subscriber_id: Option<String>,

  /// Extensions for subscriberId
  #[serde(rename = "_subscriberId")]
  _subscriber_id: Option<Element>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// The order of applicability of this coverage relative to other coverages which
  /// are currently in force. Note, there may be gaps in the numbering and this does
  /// not imply primary, secondary etc. as the specific positioning of coverages
  /// depends upon the episode of care.
  order: Option<i32>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The party who 'owns' the insurance policy.
  #[serde(rename = "policyHolder")]
  policy_holder: Option<Box<Reference>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// The type of coverage: social program, medical plan, accident coverage (workers
  /// compensation, auto), group health or payment by an individual or organization.
  #[serde(rename = "type")]
  fhir_type: Option<CodeableConcept>,

}
