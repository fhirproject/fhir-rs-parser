#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::EffectEvidenceSynthesis_EffectEstimate::EffectEvidenceSynthesis_EffectEstimate;
use crate::model::EffectEvidenceSynthesis_Certainty::EffectEvidenceSynthesis_Certainty;
use crate::model::ResourceList::ResourceList;
use crate::model::EffectEvidenceSynthesis_SampleSize::EffectEvidenceSynthesis_SampleSize;
use crate::model::Element::Element;
use crate::model::Identifier::Identifier;
use crate::model::Narrative::Narrative;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::RelatedArtifact::RelatedArtifact;
use crate::model::Meta::Meta;
use crate::model::Annotation::Annotation;
use crate::model::ContactDetail::ContactDetail;
use crate::model::EffectEvidenceSynthesis_ResultsByExposure::EffectEvidenceSynthesis_ResultsByExposure;
use crate::model::UsageContext::UsageContext;
use crate::model::Extension::Extension;


/// The EffectEvidenceSynthesis resource describes the difference in an outcome
/// between exposures states in a population where the effect estimate is derived
/// from a combination of research studies.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectEvidenceSynthesis {
  /// A human-readable string to clarify or explain concepts about the resource.
  note: Option<Vec<Annotation>>,

  /// The date on which the resource content was last reviewed. Review happens
  /// periodically after approval but does not change the original approval date.
  #[serde(rename = "lastReviewDate")]
  last_review_date: Option<i32>,

  /// Extensions for lastReviewDate
  #[serde(rename = "_lastReviewDate")]
  _last_review_date: Option<Element>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// Extensions for title
  #[serde(rename = "_title")]
  _title: Option<Element>,

  /// Type of synthesis eg meta-analysis.
  #[serde(rename = "synthesisType")]
  synthesis_type: Option<CodeableConcept>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Type of study eg randomized trial.
  #[serde(rename = "studyType")]
  study_type: Option<CodeableConcept>,

  /// Extensions for approvalDate
  #[serde(rename = "_approvalDate")]
  _approval_date: Option<Element>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// The period during which the effect evidence synthesis content was or is planned
  /// to be in active use.
  #[serde(rename = "effectivePeriod")]
  effective_period: Option<Period>,

  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  contact: Option<Vec<ContactDetail>>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// A reference to a EvidenceVariable resource that defines the comparison exposure
  /// for the research.
  #[serde(rename = "exposureAlternative")]
  exposure_alternative: Box<Reference>,

  /// The status of this effect evidence synthesis. Enables tracking the life-cycle of
  /// the content.
  status: Option<EffectEvidenceSynthesisStatus>,

  /// A reference to a EvidenceVariable resource that defines the exposure for the
  /// research.
  exposure: Box<Reference>,

  /// A reference to a EvidenceVariable resomece that defines the outcome for the
  /// research.
  outcome: Box<Reference>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// An individual or organization primarily responsible for internal coherence of
  /// the content.
  editor: Option<Vec<ContactDetail>>,

  /// Extensions for date
  #[serde(rename = "_date")]
  _date: Option<Element>,

  /// A description of the results for each exposure considered in the effect
  /// estimate.
  #[serde(rename = "resultsByExposure")]
  results_by_exposure: Option<Vec<EffectEvidenceSynthesis_ResultsByExposure>>,

  /// The estimated effect of the exposure variant.
  #[serde(rename = "effectEstimate")]
  effect_estimate: Option<Vec<EffectEvidenceSynthesis_EffectEstimate>>,

  /// Extensions for copyright
  #[serde(rename = "_copyright")]
  _copyright: Option<Element>,

  /// An individiual or organization primarily involved in the creation and
  /// maintenance of the content.
  author: Option<Vec<ContactDetail>>,

  /// A formal identifier that is used to identify this effect evidence synthesis when
  /// it is represented in other formats, or referenced in a specification, model,
  /// design or an instance.
  identifier: Option<Vec<Identifier>>,

  /// Extensions for version
  #[serde(rename = "_version")]
  _version: Option<Element>,

  /// A short, descriptive, user-friendly title for the effect evidence synthesis.
  title: Option<String>,

  /// Extensions for url
  #[serde(rename = "_url")]
  _url: Option<Element>,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate effect evidence
  /// synthesis instances.
  #[serde(rename = "useContext")]
  use_context: Option<Vec<UsageContext>>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Related artifacts such as additional documentation, justification, or
  /// bibliographic references.
  #[serde(rename = "relatedArtifact")]
  related_artifact: Option<Vec<RelatedArtifact>>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// An absolute URI that is used to identify this effect evidence synthesis when it
  /// is referenced in a specification, model, design or an instance; also called its
  /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
  /// address at which at which an authoritative instance of this effect evidence
  /// synthesis is (or will be) published. This URL can be the target of a canonical
  /// reference. It SHALL remain the same when the effect evidence synthesis is stored
  /// on different servers.
  url: Option<String>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// A reference to a EvidenceVariable resource that defines the population for the
  /// research.
  population: Box<Reference>,

  /// A description of the certainty of the effect estimate.
  certainty: Option<Vec<EffectEvidenceSynthesis_Certainty>>,

  /// The date on which the resource content was approved by the publisher. Approval
  /// happens once when the content is officially approved for usage.
  #[serde(rename = "approvalDate")]
  approval_date: Option<i32>,

  /// A free text natural language description of the effect evidence synthesis from a
  /// consumer's perspective.
  description: Option<String>,

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

  /// A copyright statement relating to the effect evidence synthesis and/or its
  /// contents. Copyright statements are generally legal restrictions on the use and
  /// publishing of the effect evidence synthesis.
  copyright: Option<String>,

  /// The date  (and optionally time) when the effect evidence synthesis was
  /// published. The date must change when the business version changes and it must
  /// change if the status code changes. In addition, it should change when the
  /// substantive content of the effect evidence synthesis changes.
  date: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Descriptive topics related to the content of the EffectEvidenceSynthesis. Topics
  /// provide a high-level categorization grouping types of EffectEvidenceSynthesiss
  /// that can be useful for filtering and searching.
  topic: Option<Vec<CodeableConcept>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// The identifier that is used to identify this version of the effect evidence
  /// synthesis when it is referenced in a specification, model, design or instance.
  /// This is an arbitrary value managed by the effect evidence synthesis author and
  /// is not expected to be globally unique. For example, it might be a timestamp
  /// (e.g. yyyymmdd) if a managed version is not available. There is also no
  /// expectation that versions can be placed in a lexicographical sequence.
  version: Option<String>,

  /// An individual or organization responsible for officially endorsing the content
  /// for use in some setting.
  endorser: Option<Vec<ContactDetail>>,

  /// A natural language name identifying the effect evidence synthesis. This name
  /// should be usable as an identifier for the module by machine processing
  /// applications such as code generation.
  name: Option<String>,

  /// A description of the size of the sample involved in the synthesis.
  #[serde(rename = "sampleSize")]
  sample_size: Option<EffectEvidenceSynthesis_SampleSize>,

  /// An individual or organization primarily responsible for review of some aspect of
  /// the content.
  reviewer: Option<Vec<ContactDetail>>,

  /// Extensions for publisher
  #[serde(rename = "_publisher")]
  _publisher: Option<Element>,

  /// The name of the organization or individual that published the effect evidence
  /// synthesis.
  publisher: Option<String>,

  /// A legal or geographic region in which the effect evidence synthesis is intended
  /// to be used.
  jurisdiction: Option<Vec<CodeableConcept>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum EffectEvidenceSynthesisStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "unknown")]
  Unknown,

}
