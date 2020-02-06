#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Identifier::Identifier;
use crate::model::Narrative::Narrative;
use crate::model::Element::Element;
use crate::model::ResourceList::ResourceList;
use crate::model::Extension::Extension;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Meta::Meta;
use crate::model::Reference::Reference;
use crate::model::Organization_Contact::Organization_Contact;
use crate::model::Address::Address;


/// A formally or informally recognized grouping of people or organizations formed
/// for the purpose of achieving some form of collective action.  Includes
/// companies, institutions, corporations, departments, community groups, healthcare
/// practice groups, payer/insurer, etc.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
  /// Whether the organization's record is still in active use.
  active: bool,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// A contact detail for the organization.
  telecom: Vec<ContactPoint>,

  /// An address for the organization.
  address: Vec<Address>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Technical endpoints providing access to services operated for the organization.
  endpoint: Vec<Box<Reference>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

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

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// The base language in which the resource is written.
  language: String,

  /// The kind(s) of organization that this is.
  #[serde(rename = "type")]
  fhir_type: Vec<CodeableConcept>,

  /// Identifier for the organization that is used to identify the organization across
  /// multiple disparate systems.
  identifier: Vec<Identifier>,

  /// A name associated with the organization.
  name: String,

  /// Contact for the organization for a certain purpose.
  contact: Vec<Organization_Contact>,

  /// Extensions for language
  _language: Element,

  /// Extensions for name
  _name: Element,

  /// The organization of which this organization forms a part.
  #[serde(rename = "partOf")]
  part_of: Box<Reference>,

  /// Extensions for active
  _active: Element,

  /// A list of alternate names that the organization is known as, or was known as in
  /// the past.
  alias: Vec<String>,

  /// Extensions for alias
  _alias: Vec<Element>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

}
