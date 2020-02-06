#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ResourceList::ResourceList;
use crate::model::ObservationDefinition_QuantitativeDetails::ObservationDefinition_QuantitativeDetails;
use crate::model::Narrative::Narrative;
use crate::model::Meta::Meta;
use crate::model::ObservationDefinition_QualifiedInterval::ObservationDefinition_QualifiedInterval;
use crate::model::Reference::Reference;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Identifier::Identifier;


/// Set of definitional characteristics for a kind of observation or measurement
/// produced or consumed by an orderable health care service.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservationDefinition {
  /// Describes what will be observed. Sometimes this is called the observation
  /// "name".
  code: CodeableConcept,

  /// A unique identifier assigned to this ObservationDefinition artifact.
  identifier: Vec<Identifier>,

  /// Extensions for preferredReportName
  #[serde(rename = "_preferredReportName")]
  _preferred_report_name: Element,

  /// The preferred name to be used when reporting the results of observations
  /// conforming to this ObservationDefinition.
  #[serde(rename = "preferredReportName")]
  preferred_report_name: String,

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

  /// A code that classifies the general type of observation.
  category: Vec<CodeableConcept>,

  /// Extensions for permittedDataType
  #[serde(rename = "_permittedDataType")]
  _permitted_data_type: Vec<Element>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// The base language in which the resource is written.
  language: String,

  /// Multiple results allowed for observations conforming to this
  /// ObservationDefinition.
  #[serde(rename = "multipleResultsAllowed")]
  multiple_results_allowed: bool,

  /// Multiple  ranges of results qualified by different contexts for ordinal or
  /// continuous observations conforming to this ObservationDefinition.
  #[serde(rename = "qualifiedInterval")]
  qualified_interval: Vec<ObservationDefinition_QualifiedInterval>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The method or technique used to perform the observation.
  method: CodeableConcept,

  /// The set of normal coded results for the observations conforming to this
  /// ObservationDefinition.
  #[serde(rename = "normalCodedValueSet")]
  normal_coded_value_set: Box<Reference>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The set of critical coded results for the observation conforming to this
  /// ObservationDefinition.
  #[serde(rename = "criticalCodedValueSet")]
  critical_coded_value_set: Box<Reference>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// Characteristics for quantitative results of this observation.
  #[serde(rename = "quantitativeDetails")]
  quantitative_details: ObservationDefinition_QuantitativeDetails,

  /// The set of abnormal coded results for the observation conforming to this
  /// ObservationDefinition.
  #[serde(rename = "abnormalCodedValueSet")]
  abnormal_coded_value_set: Box<Reference>,

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

  /// Extensions for multipleResultsAllowed
  #[serde(rename = "_multipleResultsAllowed")]
  _multiple_results_allowed: Element,

  /// The set of valid coded results for the observations  conforming to this
  /// ObservationDefinition.
  #[serde(rename = "validCodedValueSet")]
  valid_coded_value_set: Box<Reference>,

  /// Extensions for language
  _language: Element,

}
