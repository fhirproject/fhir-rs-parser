#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::MedicinalProductAuthorization_JurisdictionalAuthorization::MedicinalProductAuthorization_JurisdictionalAuthorization;
use crate::model::MedicinalProductAuthorization_Procedure::MedicinalProductAuthorization_Procedure;
use crate::model::Identifier::Identifier;
use crate::model::ResourceList::ResourceList;
use crate::model::Reference::Reference;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::Element::Element;
use crate::model::Period::Period;
use crate::model::Narrative::Narrative;


/// The regulatory authorization of a medicinal product.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductAuthorization {
  /// The regulatory procedure for granting or amending a marketing authorization.
  procedure: Option<MedicinalProductAuthorization_Procedure>,

  /// The date at which the given status has become applicable.
  #[serde(rename = "statusDate")]
  status_date: Option<String>,

  /// A period of time after authorization before generic product applicatiosn can be
  /// submitted.
  #[serde(rename = "dataExclusivityPeriod")]
  data_exclusivity_period: Option<Period>,

  /// Extensions for restoreDate
  #[serde(rename = "_restoreDate")]
  _restore_date: Option<Element>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Extensions for statusDate
  #[serde(rename = "_statusDate")]
  _status_date: Option<Element>,

  /// The country in which the marketing authorization has been granted.
  country: Option<Vec<CodeableConcept>>,

  /// The status of the marketing authorization.
  status: Option<CodeableConcept>,

  /// Extensions for dateOfFirstAuthorization
  #[serde(rename = "_dateOfFirstAuthorization")]
  _date_of_first_authorization: Option<Element>,

  /// Marketing Authorization Holder.
  holder: Option<Box<Reference>>,

  /// The date when the first authorization was granted by a Medicines Regulatory
  /// Agency.
  #[serde(rename = "dateOfFirstAuthorization")]
  date_of_first_authorization: Option<String>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// The beginning of the time period in which the marketing authorization is in the
  /// specific status shall be specified A complete date consisting of day, month and
  /// year shall be specified using the ISO 8601 date format.
  #[serde(rename = "validityPeriod")]
  validity_period: Option<Period>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

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

  /// The medicinal product that is being authorized.
  subject: Option<Box<Reference>>,

  /// The date when a suspended the marketing or the marketing authorization of the
  /// product is anticipated to be restored.
  #[serde(rename = "restoreDate")]
  restore_date: Option<String>,

  /// Medicines Regulatory Agency.
  regulator: Option<Box<Reference>>,

  /// Business identifier for the marketing authorization, as assigned by a regulator.
  identifier: Option<Vec<Identifier>>,

  /// Jurisdiction within a country.
  jurisdiction: Option<Vec<CodeableConcept>>,

  /// The legal framework against which this authorization is granted.
  #[serde(rename = "legalBasis")]
  legal_basis: Option<CodeableConcept>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Date of first marketing authorization for a company's new medicinal product in
  /// any country in the World.
  #[serde(rename = "internationalBirthDate")]
  international_birth_date: Option<String>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Extensions for internationalBirthDate
  #[serde(rename = "_internationalBirthDate")]
  _international_birth_date: Option<Element>,

  /// Authorization in areas within a country.
  #[serde(rename = "jurisdictionalAuthorization")]
  jurisdictional_authorization: Option<Vec<MedicinalProductAuthorization_JurisdictionalAuthorization>>,

}
