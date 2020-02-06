#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Identifier::Identifier;
use crate::model::Element::Element;
use crate::model::Narrative::Narrative;
use crate::model::Account_Guarantor::Account_Guarantor;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::Period::Period;
use crate::model::Meta::Meta;
use crate::model::ResourceList::ResourceList;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Account_Coverage::Account_Coverage;


/// A financial tool for tracking value accrued for a particular purpose.  In the
/// healthcare field, used to track charges for a patient, cost centers, etc.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Categorizes the account for reporting and searching purposes.
  #[serde(rename = "type")]
  fhir_type: Option<CodeableConcept>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

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

  /// Provides additional information about what the account tracks and how it is
  /// used.
  description: Option<String>,

  /// Unique identifier used to reference the account.  Might or might not be intended
  /// for human use (e.g. credit card number).
  identifier: Option<Vec<Identifier>>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// Identifies the entity which incurs the expenses. While the immediate recipients
  /// of services or goods might be entities related to the subject, the expenses were
  /// ultimately incurred by the subject of the Account.
  subject: Option<Vec<Box<Reference>>>,

  /// The date range of services associated with this account.
  #[serde(rename = "servicePeriod")]
  service_period: Option<Period>,

  /// The parties responsible for balancing the account if other payment options fall
  /// short.
  guarantor: Option<Vec<Account_Guarantor>>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// The party(s) that are responsible for covering the payment of this account, and
  /// what order should they be applied to the account.
  coverage: Option<Vec<Account_Coverage>>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Reference to a parent Account.
  #[serde(rename = "partOf")]
  part_of: Option<Box<Reference>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// Indicates whether the account is presently used/usable or not.
  status: Option<AccountStatus>,

  /// Name used for the account when displaying it to humans in reports, etc.
  name: Option<String>,

  /// Indicates the service area, hospital, department, etc. with responsibility for
  /// managing the Account.
  owner: Option<Box<Reference>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum AccountStatus {
  #[serde(rename = "active")]
  Active,

  #[serde(rename = "inactive")]
  Inactive,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

  #[serde(rename = "on-hold")]
  OnHold,

  #[serde(rename = "unknown")]
  Unknown,

}
