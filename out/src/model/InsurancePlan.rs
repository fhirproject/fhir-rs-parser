#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Narrative::Narrative;
use crate::model::Element::Element;
use crate::model::InsurancePlan_Contact::InsurancePlan_Contact;
use crate::model::InsurancePlan_Coverage::InsurancePlan_Coverage;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::InsurancePlan_Plan::InsurancePlan_Plan;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::Meta::Meta;
use crate::model::Period::Period;


/// Details of a Health Insurance product/plan provided by an organization.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlan {
  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// The base language in which the resource is written.
  language: String,

  /// Details about an insurance plan.
  plan: Vec<InsurancePlan_Plan>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The entity that is providing  the health insurance product and underwriting the
  /// risk.  This is typically an insurance carriers, other third-party payers, or
  /// health plan sponsors comonly referred to as 'payers'.
  #[serde(rename = "ownedBy")]
  owned_by: Box<Reference>,

  /// An organization which administer other services such as underwriting, customer
  /// service and/or claims processing on behalf of the health insurance product
  /// owner.
  #[serde(rename = "administeredBy")]
  administered_by: Box<Reference>,

  /// Reference to the network included in the health insurance product.
  network: Vec<Box<Reference>>,

  /// The technical endpoints providing access to services operated for the health
  /// insurance product.
  endpoint: Vec<Box<Reference>>,

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

  /// Official name of the health insurance product as designated by the owner.
  name: String,

  /// The period of time that the health insurance product is available.
  period: Period,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Business identifiers assigned to this health insurance product which remain
  /// constant as the resource is updated and propagates from server to server.
  identifier: Vec<Identifier>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// The kind of health insurance product.
  #[serde(rename = "type")]
  fhir_type: Vec<CodeableConcept>,

  /// The current state of the health insurance product.
  status: InsurancePlanStatus,

  /// The geographic region in which a health insurance product's benefits apply.
  #[serde(rename = "coverageArea")]
  coverage_area: Vec<Box<Reference>>,

  /// Extensions for alias
  _alias: Vec<Element>,

  /// Extensions for name
  _name: Element,

  /// The contact for the health insurance product for a certain purpose.
  contact: Vec<InsurancePlan_Contact>,

  /// Details about the coverage offered by the insurance product.
  coverage: Vec<InsurancePlan_Coverage>,

  /// Extensions for status
  _status: Element,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// Extensions for language
  _language: Element,

  /// A list of alternate names that the product is known as, or was known as in the
  /// past.
  alias: Vec<String>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum InsurancePlanStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "unknown")]
  Unknown,

}
