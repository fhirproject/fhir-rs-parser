#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Reference::Reference;
use crate::model::Period::Period;
use crate::model::Identifier::Identifier;
use crate::model::Narrative::Narrative;
use crate::model::ResourceList::ResourceList;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Meta::Meta;
use crate::model::Extension::Extension;


/// Prospective warnings of potential issues when providing care to the patient.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flag {
  /// Allows a flag to be divided into different categories like clinical,
  /// administrative etc. Intended to be used as a means of filtering which flags are
  /// displayed to particular user or in a given context.
  category: Option<Vec<CodeableConcept>>,

  /// This alert is only relevant during the encounter.
  encounter: Option<Box<Reference>>,

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

  /// The base language in which the resource is written.
  language: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Business identifiers assigned to this flag by the performer or other systems
  /// which remain constant as the resource is updated and propagates from server to
  /// server.
  identifier: Option<Vec<Identifier>>,

  /// The period of time from the activation of the flag to inactivation of the flag.
  /// If the flag is active, the end of the period should be unspecified.
  period: Option<Period>,

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

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// The person, organization or device that created the flag.
  author: Option<Box<Reference>>,

  /// Supports basic workflow.
  status: Option<FlagStatus>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// The coded value or textual component of the flag to display to the user.
  code: CodeableConcept,

  /// The patient, location, group, organization, or practitioner etc. this is about
  /// record this flag is associated with.
  subject: Box<Reference>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum FlagStatus {
  #[serde(rename = "active")]
  Active,

  #[serde(rename = "inactive")]
  Inactive,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

}
