#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Identifier::Identifier;
use crate::model::Extension::Extension;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Address::Address;
use crate::model::ResourceList::ResourceList;
use crate::model::Element::Element;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::Organization_Contact::Organization_Contact;
use crate::model::Meta::Meta;
use crate::model::CodeableConcept::CodeableConcept;


/// A formally or informally recognized grouping of people or organizations formed
/// for the purpose of achieving some form of collective action.  Includes
/// companies, institutions, corporations, departments, community groups, healthcare
/// practice groups, payer/insurer, etc.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
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

  /// An address for the organization.
  address: Option<Vec<Address>>,

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

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Extensions for active
  #[serde(rename = "_active")]
  _active: Option<Element>,

  /// The kind(s) of organization that this is.
  #[serde(rename = "type")]
  fhir_type: Option<Vec<CodeableConcept>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// Identifier for the organization that is used to identify the organization across
  /// multiple disparate systems.
  identifier: Option<Vec<Identifier>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// A list of alternate names that the organization is known as, or was known as in
  /// the past.
  alias: Option<Vec<String>>,

  /// Extensions for alias
  #[serde(rename = "_alias")]
  _alias: Option<Vec<Element>>,

  /// A name associated with the organization.
  name: Option<String>,

  /// A contact detail for the organization.
  telecom: Option<Vec<ContactPoint>>,

  /// Whether the organization's record is still in active use.
  active: Option<bool>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// The organization of which this organization forms a part.
  #[serde(rename = "partOf")]
  part_of: Option<Box<Reference>>,

  /// Contact for the organization for a certain purpose.
  contact: Option<Vec<Organization_Contact>>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Technical endpoints providing access to services operated for the organization.
  endpoint: Option<Vec<Box<Reference>>>,

}
