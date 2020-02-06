#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::MeasureReport_Group::MeasureReport_Group;
use crate::model::Period::Period;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::Identifier::Identifier;
use crate::model::Narrative::Narrative;


/// The MeasureReport resource contains the results of the calculation of a measure;
/// and optionally a reference to the resources involved in that calculation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeasureReport {
  /// The individual, location, or organization that is reporting the data.
  reporter: Option<Box<Reference>>,

  /// The MeasureReport status. No data will be available until the MeasureReport
  /// status is complete.
  status: Option<MeasureReportStatus>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// The type of measure report. This may be an individual report, which provides the
  /// score for the measure for an individual member of the population; a subject-
  /// listing, which returns the list of members that meet the various
  /// criteria in the measure; a summary report, which returns a population count for
  /// each of the criteria in the measure; or a data-collection, which enables the
  /// MeasureReport to be used to exchange the data-of-interest for a quality measure.
  #[serde(rename = "type")]
  fhir_type: Option<MeasureReportType>,

  /// The reporting period for which the report was calculated.
  period: Period,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Whether improvement in the measure is noted by an increase or decrease in the
  /// measure score.
  #[serde(rename = "improvementNotation")]
  improvement_notation: Option<CodeableConcept>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

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

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// A reference to the Measure that was calculated to produce this report.
  measure: String,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Extensions for type
  #[serde(rename = "_type")]
  _type: Option<Element>,

  /// Optional subject identifying the individual or individuals the report is for.
  subject: Option<Box<Reference>>,

  /// The date this measure report was generated.
  date: Option<String>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// Extensions for date
  #[serde(rename = "_date")]
  _date: Option<Element>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// A formal identifier that is used to identify this MeasureReport when it is
  /// represented in other formats or referenced in a specification, model, design or
  /// an instance.
  identifier: Option<Vec<Identifier>>,

  /// A reference to a Bundle containing the Resources that were used in the
  /// calculation of this measure.
  #[serde(rename = "evaluatedResource")]
  evaluated_resource: Option<Vec<Box<Reference>>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// The results of the calculation, one for each population group in the measure.
  group: Option<Vec<MeasureReport_Group>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum MeasureReportStatus {
  #[serde(rename = "complete")]
  Complete,

  #[serde(rename = "pending")]
  Pending,

  #[serde(rename = "error")]
  Error,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum MeasureReportType {
  #[serde(rename = "individual")]
  Individual,

  #[serde(rename = "subject-list")]
  SubjectList,

  #[serde(rename = "summary")]
  Summary,

  #[serde(rename = "data-collection")]
  DataCollection,

}
