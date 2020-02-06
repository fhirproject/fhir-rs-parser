#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Meta::Meta;
use crate::model::Element::Element;
use crate::model::ResourceList::ResourceList;
use crate::model::Identifier::Identifier;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::Period::Period;


/// Defines an affiliation/assotiation/relationship between 2 distinct oganizations,
/// that is not a part-of relationship/sub-division relationship.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationAffiliation {
  /// Definition of the role the participatingOrganization plays in the association.
  code: Option<Vec<CodeableConcept>>,

  /// Healthcare services provided through the role.
  #[serde(rename = "healthcareService")]
  healthcare_service: Option<Vec<Box<Reference>>>,

  /// Whether this organization affiliation record is in active use.
  active: Option<bool>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// The Participating Organization provides/performs the role(s) defined by the code
  /// to the Primary Organization (e.g. providing services or is a member of).
  #[serde(rename = "participatingOrganization")]
  participating_organization: Option<Box<Reference>>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// Business identifiers that are specific to this role.
  identifier: Option<Vec<Identifier>>,

  /// Specific specialty of the participatingOrganization in the context of the role.
  specialty: Option<Vec<CodeableConcept>>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Extensions for active
  #[serde(rename = "_active")]
  _active: Option<Element>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Health insurance provider network in which the participatingOrganization
  /// provides the role's services (if defined) at the indicated locations (if
  /// defined).
  network: Option<Vec<Box<Reference>>>,

  /// Contact details at the participatingOrganization relevant to this Affiliation.
  telecom: Option<Vec<ContactPoint>>,

  /// The period during which the participatingOrganization is affiliated with the
  /// primary organization.
  period: Option<Period>,

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

  /// Organization where the role is available (primary organization/has members).
  organization: Option<Box<Reference>>,

  /// The location(s) at which the role occurs.
  location: Option<Vec<Box<Reference>>>,

  /// Technical endpoints providing access to services operated for this role.
  endpoint: Option<Vec<Box<Reference>>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

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
  modifier_extension: Option<Vec<Box<Extension>>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

}
