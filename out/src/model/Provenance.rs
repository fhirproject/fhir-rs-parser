#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Meta::Meta;
use crate::model::Signature::Signature;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Provenance_Entity::Provenance_Entity;
use crate::model::ResourceList::ResourceList;
use crate::model::Period::Period;
use crate::model::Provenance_Agent::Provenance_Agent;
use crate::model::Narrative::Narrative;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::Element::Element;


/// Provenance of a resource is a record that describes entities and processes
/// involved in producing and delivering or otherwise influencing that resource.
/// Provenance provides a critical foundation for assessing authenticity, enabling
/// trust, and allowing reproducibility. Provenance assertions are a form of
/// contextual metadata and can themselves become important records with their own
/// provenance. Provenance statement indicates clinical significance in terms of
/// confidence in authenticity, reliability, and trustworthiness, integrity, and
/// stage in lifecycle (e.g. Document Completion - has the artifact been legally
/// authenticated), all of which may impact security, privacy, and trust policies.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Provenance {
  /// An activity is something that occurs over a period of time and acts upon or with
  /// entities; it may include consuming, processing, transforming, modifying,
  /// relocating, using, or generating entities.
  activity: Option<CodeableConcept>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

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

  /// The period during which the activity occurred.
  #[serde(rename = "occurredPeriod")]
  occurred_period: Option<Period>,

  /// An actor taking a role in an activity  for which it can be assigned some degree
  /// of responsibility for the activity taking place.
  agent: Vec<Provenance_Agent>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Policy or plan the activity was defined by. Typically, a single activity may
  /// have multiple applicable policy documents, such as patient consent, guarantor
  /// funding, etc.
  policy: Option<Vec<String>>,

  /// The Reference(s) that were generated or updated by  the activity described in
  /// this resource. A provenance can point to more than one target if multiple
  /// resources were created/updated by the same activity.
  target: Vec<Box<Reference>>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

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

  /// The base language in which the resource is written.
  language: Option<String>,

  /// An entity used in this activity.
  entity: Option<Vec<Provenance_Entity>>,

  /// A digital signature on the target Reference(s). The signer should match a
  /// Provenance.agent. The purpose of the signature is indicated.
  signature: Option<Vec<Signature>>,

  /// The period during which the activity occurred.
  #[serde(rename = "occurredDateTime")]
  occurred_date_time: Option<String>,

  /// The instant of time at which the activity was recorded.
  recorded: Option<String>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// Extensions for recorded
  #[serde(rename = "_recorded")]
  _recorded: Option<Element>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// The reason that the activity was taking place.
  reason: Option<Vec<CodeableConcept>>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Extensions for occurredDateTime
  #[serde(rename = "_occurredDateTime")]
  _occurred_date_time: Option<Element>,

  /// Extensions for policy
  #[serde(rename = "_policy")]
  _policy: Option<Vec<Element>>,

  /// Where the activity occurred, if relevant.
  location: Option<Box<Reference>>,

}
