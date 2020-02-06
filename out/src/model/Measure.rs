#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Reference::Reference;
use crate::model::Meta::Meta;
use crate::model::ContactDetail::ContactDetail;
use crate::model::RelatedArtifact::RelatedArtifact;
use crate::model::Identifier::Identifier;
use crate::model::Narrative::Narrative;
use crate::model::ResourceList::ResourceList;
use crate::model::Measure_SupplementalData::Measure_SupplementalData;
use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::UsageContext::UsageContext;
use crate::model::Measure_Group::Measure_Group;
use crate::model::Period::Period;


/// The Measure resource provides the definition of a quality measure.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Measure {
  /// A description of the risk adjustment factors that may impact the resulting score
  /// for the measure and how they may be accounted for when computing and reporting
  /// measure results.
  #[serde(rename = "riskAdjustment")]
  risk_adjustment: Option<String>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Extensions for url
  #[serde(rename = "_url")]
  _url: Option<Element>,

  /// A legal or geographic region in which the measure is intended to be used.
  jurisdiction: Option<Vec<CodeableConcept>>,

  /// Extensions for disclaimer
  #[serde(rename = "_disclaimer")]
  _disclaimer: Option<Element>,

  /// Extensions for clinicalRecommendationStatement
  #[serde(rename = "_clinicalRecommendationStatement")]
  _clinical_recommendation_statement: Option<Element>,

  /// Extensions for date
  #[serde(rename = "_date")]
  _date: Option<Element>,

  /// Indicates how the calculation is performed for the measure, including
  /// proportion, ratio, continuous-variable, and cohort. The value set is extensible,
  /// allowing additional measure scoring types to be represented.
  scoring: Option<CodeableConcept>,

  /// Descriptive topics related to the content of the measure. Topics provide a high-
  /// level categorization grouping types of measures that can be useful for
  /// filtering and searching.
  topic: Option<Vec<CodeableConcept>>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// Indicates whether the measure is used to examine a process, an outcome over
  /// time, a patient-reported outcome, or a structure measure such as utilization.
  #[serde(rename = "type")]
  fhir_type: Option<Vec<CodeableConcept>>,

  /// Extensions for rationale
  #[serde(rename = "_rationale")]
  _rationale: Option<Element>,

  /// Extensions for copyright
  #[serde(rename = "_copyright")]
  _copyright: Option<Element>,

  /// A free text natural language description of the measure from a consumer's
  /// perspective.
  description: Option<String>,

  /// Extensions for lastReviewDate
  #[serde(rename = "_lastReviewDate")]
  _last_review_date: Option<Element>,

  /// Provides a description of an individual term used within the measure.
  definition: Option<Vec<String>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// The identifier that is used to identify this version of the measure when it is
  /// referenced in a specification, model, design or instance. This is an arbitrary
  /// value managed by the measure author and is not expected to be globally unique.
  /// For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not
  /// available. There is also no expectation that versions can be placed in a
  /// lexicographical sequence. To provide a version consistent with the Decision
  /// Support Service specification, use the format Major.Minor.Revision (e.g. 1.0.0).
  /// For more information on versioning knowledge assets, refer to the Decision
  /// Support Service specification. Note that a version is required for non-
  /// experimental active artifacts.
  version: Option<String>,

  /// Related artifacts such as additional documentation, justification, or
  /// bibliographic references.
  #[serde(rename = "relatedArtifact")]
  related_artifact: Option<Vec<RelatedArtifact>>,

  /// Extensions for subtitle
  #[serde(rename = "_subtitle")]
  _subtitle: Option<Element>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// An explanatory or alternate title for the measure giving additional information
  /// about its content.
  subtitle: Option<String>,

  /// Extensions for guidance
  #[serde(rename = "_guidance")]
  _guidance: Option<Element>,

  /// The intended subjects for the measure. If this element is not provided, a
  /// Patient subject is assumed, but the subject of the measure can be anything.
  #[serde(rename = "subjectCodeableConcept")]
  subject_codeable_concept: Option<CodeableConcept>,

  /// If this is a composite measure, the scoring method used to combine the component
  /// measures to determine the composite score.
  #[serde(rename = "compositeScoring")]
  composite_scoring: Option<CodeableConcept>,

  /// An individiual or organization primarily involved in the creation and
  /// maintenance of the content.
  author: Option<Vec<ContactDetail>>,

  /// A Boolean value to indicate that this measure is authored for testing purposes
  /// (or education/evaluation/marketing) and is not intended to be used for genuine
  /// usage.
  experimental: Option<bool>,

  /// The status of this measure. Enables tracking the life-cycle of the content.
  status: Option<MeasureStatus>,

  /// Extensions for version
  #[serde(rename = "_version")]
  _version: Option<Element>,

  /// Describes how to combine the information calculated, based on logic in each of
  /// several populations, into one summarized result.
  #[serde(rename = "rateAggregation")]
  rate_aggregation: Option<String>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Extensions for title
  #[serde(rename = "_title")]
  _title: Option<Element>,

  /// A reference to a Library resource containing the formal logic used by the
  /// measure.
  library: Option<Vec<String>>,

  /// A group of population criteria for the measure.
  group: Option<Vec<Measure_Group>>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// The period during which the measure content was or is planned to be in active
  /// use.
  #[serde(rename = "effectivePeriod")]
  effective_period: Option<Period>,

  /// A formal identifier that is used to identify this measure when it is represented
  /// in other formats, or referenced in a specification, model, design or an
  /// instance.
  identifier: Option<Vec<Identifier>>,

  /// Extensions for riskAdjustment
  #[serde(rename = "_riskAdjustment")]
  _risk_adjustment: Option<Element>,

  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  contact: Option<Vec<ContactDetail>>,

  /// The supplemental data criteria for the measure report, specified as either the
  /// name of a valid CQL expression within a referenced library, or a valid FHIR
  /// Resource Path.
  #[serde(rename = "supplementalData")]
  supplemental_data: Option<Vec<Measure_SupplementalData>>,

  /// The date  (and optionally time) when the measure was published. The date must
  /// change when the business version changes and it must change if the status code
  /// changes. In addition, it should change when the substantive content of the
  /// measure changes.
  date: Option<String>,

  /// The date on which the resource content was last reviewed. Review happens
  /// periodically after approval but does not change the original approval date.
  #[serde(rename = "lastReviewDate")]
  last_review_date: Option<i32>,

  /// A copyright statement relating to the measure and/or its contents. Copyright
  /// statements are generally legal restrictions on the use and publishing of the
  /// measure.
  copyright: Option<String>,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate measure instances.
  #[serde(rename = "useContext")]
  use_context: Option<Vec<UsageContext>>,

  /// A detailed description, from a clinical perspective, of how the measure is used.
  usage: Option<String>,

  /// Information on whether an increase or decrease in score is the preferred result
  /// (e.g., a higher score indicates better quality OR a lower score indicates better
  /// quality OR quality is within a range).
  #[serde(rename = "improvementNotation")]
  improvement_notation: Option<CodeableConcept>,

  /// The name of the organization or individual that published the measure.
  publisher: Option<String>,

  /// Provides a summary of relevant clinical guidelines or other clinical
  /// recommendations supporting the measure.
  #[serde(rename = "clinicalRecommendationStatement")]
  clinical_recommendation_statement: Option<String>,

  /// Extensions for usage
  #[serde(rename = "_usage")]
  _usage: Option<Element>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Extensions for definition
  #[serde(rename = "_definition")]
  _definition: Option<Vec<Element>>,

  /// Extensions for publisher
  #[serde(rename = "_publisher")]
  _publisher: Option<Element>,

  /// An individual or organization responsible for officially endorsing the content
  /// for use in some setting.
  endorser: Option<Vec<ContactDetail>>,

  /// An individual or organization primarily responsible for internal coherence of
  /// the content.
  editor: Option<Vec<ContactDetail>>,

  /// An absolute URI that is used to identify this measure when it is referenced in a
  /// specification, model, design or an instance; also called its canonical
  /// identifier. This SHOULD be globally unique and SHOULD be a literal address at
  /// which at which an authoritative instance of this measure is (or will be)
  /// published. This URL can be the target of a canonical reference. It SHALL remain
  /// the same when the measure is stored on different servers.
  url: Option<String>,

  /// The intended subjects for the measure. If this element is not provided, a
  /// Patient subject is assumed, but the subject of the measure can be anything.
  #[serde(rename = "subjectReference")]
  subject_reference: Option<Box<Reference>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Extensions for purpose
  #[serde(rename = "_purpose")]
  _purpose: Option<Element>,

  /// A natural language name identifying the measure. This name should be usable as
  /// an identifier for the module by machine processing applications such as code
  /// generation.
  name: Option<String>,

  /// The date on which the resource content was approved by the publisher. Approval
  /// happens once when the content is officially approved for usage.
  #[serde(rename = "approvalDate")]
  approval_date: Option<i32>,

  /// Additional guidance for the measure including how it can be used in a clinical
  /// context, and the intent of the measure.
  guidance: Option<String>,

  /// Extensions for rateAggregation
  #[serde(rename = "_rateAggregation")]
  _rate_aggregation: Option<Element>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Extensions for experimental
  #[serde(rename = "_experimental")]
  _experimental: Option<Element>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

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

  /// Explanation of why this measure is needed and why it has been designed as it
  /// has.
  purpose: Option<String>,

  /// A short, descriptive, user-friendly title for the measure.
  title: Option<String>,

  /// Extensions for approvalDate
  #[serde(rename = "_approvalDate")]
  _approval_date: Option<Element>,

  /// Notices and disclaimers regarding the use of the measure or related to
  /// intellectual property (such as code systems) referenced by the measure.
  disclaimer: Option<String>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// Provides a succinct statement of the need for the measure. Usually includes
  /// statements pertaining to importance criterion: impact, gap in care, and
  /// evidence.
  rationale: Option<String>,

  /// An individual or organization primarily responsible for review of some aspect of
  /// the content.
  reviewer: Option<Vec<ContactDetail>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum MeasureStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "unknown")]
  Unknown,

}
