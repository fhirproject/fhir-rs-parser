#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ResourceList::ResourceList;
use crate::model::TriggerDefinition::TriggerDefinition;
use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Meta::Meta;
use crate::model::Element::Element;
use crate::model::Reference::Reference;
use crate::model::ContactDetail::ContactDetail;
use crate::model::RelatedArtifact::RelatedArtifact;
use crate::model::Identifier::Identifier;
use crate::model::UsageContext::UsageContext;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;


/// The EventDefinition resource provides a reusable description of when a
/// particular event can occur.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventDefinition {
  /// A free text natural language description of the event definition from a
  /// consumer's perspective.
  description: String,

  /// Extensions for experimental
  _experimental: Element,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate event definition
  /// instances.
  #[serde(rename = "useContext")]
  use_context: Vec<UsageContext>,

  /// Extensions for usage
  _usage: Element,

  /// A formal identifier that is used to identify this event definition when it is
  /// represented in other formats, or referenced in a specification, model, design or
  /// an instance.
  identifier: Vec<Identifier>,

  /// A natural language name identifying the event definition. This name should be
  /// usable as an identifier for the module by machine processing applications such
  /// as code generation.
  name: String,

  /// The status of this event definition. Enables tracking the life-cycle of the
  /// content.
  status: EventDefinitionStatus,

  /// An individiual or organization primarily involved in the creation and
  /// maintenance of the content.
  author: Vec<ContactDetail>,

  /// Extensions for publisher
  _publisher: Element,

  /// A Boolean value to indicate that this event definition is authored for testing
  /// purposes (or education/evaluation/marketing) and is not intended to be used for
  /// genuine usage.
  experimental: bool,

  /// An individual or organization primarily responsible for review of some aspect of
  /// the content.
  reviewer: Vec<ContactDetail>,

  /// Extensions for description
  _description: Element,

  /// An individual or organization primarily responsible for internal coherence of
  /// the content.
  editor: Vec<ContactDetail>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// A short, descriptive, user-friendly title for the event definition.
  title: String,

  /// Explanation of why this event definition is needed and why it has been designed
  /// as it has.
  purpose: String,

  /// A copyright statement relating to the event definition and/or its contents.
  /// Copyright statements are generally legal restrictions on the use and publishing
  /// of the event definition.
  copyright: String,

  /// A code or group definition that describes the intended subject of the event
  /// definition.
  #[serde(rename = "subjectCodeableConcept")]
  subject_codeable_concept: CodeableConcept,

  /// Descriptive topics related to the module. Topics provide a high-level
  /// categorization of the module that can be useful for filtering and searching.
  topic: Vec<CodeableConcept>,

  /// The trigger element defines when the event occurs. If more than one trigger
  /// condition is specified, the event fires whenever any one of the trigger
  /// conditions is met.
  trigger: Vec<TriggerDefinition>,

  /// A legal or geographic region in which the event definition is intended to be
  /// used.
  jurisdiction: Vec<CodeableConcept>,

  /// Extensions for copyright
  _copyright: Element,

  /// Extensions for lastReviewDate
  #[serde(rename = "_lastReviewDate")]
  _last_review_date: Element,

  /// Related resources such as additional documentation, justification, or
  /// bibliographic references.
  #[serde(rename = "relatedArtifact")]
  related_artifact: Vec<RelatedArtifact>,

  /// Extensions for status
  _status: Element,

  /// The base language in which the resource is written.
  language: String,

  /// The period during which the event definition content was or is planned to be in
  /// active use.
  #[serde(rename = "effectivePeriod")]
  effective_period: Period,

  /// Extensions for language
  _language: Element,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// The identifier that is used to identify this version of the event definition
  /// when it is referenced in a specification, model, design or instance. This is an
  /// arbitrary value managed by the event definition author and is not expected to be
  /// globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
  /// managed version is not available. There is also no expectation that versions can
  /// be placed in a lexicographical sequence.
  version: String,

  /// The date on which the resource content was approved by the publisher. Approval
  /// happens once when the content is officially approved for usage.
  #[serde(rename = "approvalDate")]
  approval_date: i32,

  /// The date on which the resource content was last reviewed. Review happens
  /// periodically after approval but does not change the original approval date.
  #[serde(rename = "lastReviewDate")]
  last_review_date: i32,

  /// An individual or organization responsible for officially endorsing the content
  /// for use in some setting.
  endorser: Vec<ContactDetail>,

  /// Extensions for url
  _url: Element,

  /// Extensions for purpose
  _purpose: Element,

  /// A detailed description of how the event definition is used from a clinical
  /// perspective.
  usage: String,

  /// Extensions for date
  _date: Element,

  /// Extensions for title
  _title: Element,

  /// Extensions for approvalDate
  #[serde(rename = "_approvalDate")]
  _approval_date: Element,

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

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Extensions for subtitle
  _subtitle: Element,

  /// Extensions for version
  _version: Element,

  /// A code or group definition that describes the intended subject of the event
  /// definition.
  #[serde(rename = "subjectReference")]
  subject_reference: Box<Reference>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Extensions for name
  _name: Element,

  /// An explanatory or alternate title for the event definition giving additional
  /// information about its content.
  subtitle: String,

  /// The date  (and optionally time) when the event definition was published. The
  /// date must change when the business version changes and it must change if the
  /// status code changes. In addition, it should change when the substantive content
  /// of the event definition changes.
  date: String,

  /// An absolute URI that is used to identify this event definition when it is
  /// referenced in a specification, model, design or an instance; also called its
  /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
  /// address at which at which an authoritative instance of this event definition is
  /// (or will be) published. This URL can be the target of a canonical reference. It
  /// SHALL remain the same when the event definition is stored on different servers.
  url: String,

  /// The name of the organization or individual that published the event definition.
  publisher: String,

  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  contact: Vec<ContactDetail>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum EventDefinitionStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "unknown")]
  Unknown,

}
