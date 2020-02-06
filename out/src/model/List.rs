#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::List_Entry::List_Entry;
use crate::model::Identifier::Identifier;
use crate::model::Reference::Reference;
use crate::model::Extension::Extension;
use crate::model::Annotation::Annotation;
use crate::model::Element::Element;
use crate::model::ResourceList::ResourceList;
use crate::model::Narrative::Narrative;
use crate::model::Meta::Meta;
use crate::model::CodeableConcept::CodeableConcept;


/// A list is a curated collection of resources.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct List {
  /// The encounter that is the context in which this list was created.
  encounter: Box<Reference>,

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

  /// A label for the list assigned by the author.
  title: String,

  /// Comments that apply to the overall list.
  note: Vec<Annotation>,

  /// This code defines the purpose of the list - why it was created.
  code: CodeableConcept,

  /// Extensions for mode
  _mode: Element,

  /// Extensions for title
  _title: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Indicates the current state of this list.
  status: ListStatus,

  /// The date that the list was prepared.
  date: String,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Entries in this list.
  entry: Vec<List_Entry>,

  /// The base language in which the resource is written.
  language: String,

  /// Extensions for language
  _language: Element,

  /// Identifier for the List assigned for business purposes outside the context of
  /// FHIR.
  identifier: Vec<Identifier>,

  /// How this list was prepared - whether it is a working list that is suitable for
  /// being maintained on an ongoing basis, or if it represents a snapshot of a list
  /// of items from another source, or whether it is a prepared list where items may
  /// be marked as added, modified or deleted.
  mode: ListMode,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// The common subject (or patient) of the resources that are in the list if there
  /// is one.
  subject: Box<Reference>,

  /// What order applies to the items in the list.
  #[serde(rename = "orderedBy")]
  ordered_by: CodeableConcept,

  /// Extensions for status
  _status: Element,

  /// The entity responsible for deciding what the contents of the list were. Where
  /// the list was created by a human, this is the same as the author of the list.
  source: Box<Reference>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Extensions for date
  _date: Element,

  /// If the list is empty, why the list is empty.
  #[serde(rename = "emptyReason")]
  empty_reason: CodeableConcept,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ListStatus {
  #[serde(rename = "current")]
  Current,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ListMode {
  #[serde(rename = "working")]
  Working,

  #[serde(rename = "snapshot")]
  Snapshot,

  #[serde(rename = "changes")]
  Changes,

}
