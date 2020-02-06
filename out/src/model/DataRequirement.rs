#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::DataRequirement_DateFilter::DataRequirement_DateFilter;
use crate::model::DataRequirement_Sort::DataRequirement_Sort;
use crate::model::Reference::Reference;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::DataRequirement_CodeFilter::DataRequirement_CodeFilter;


/// Describes a required data item for evaluation in terms of the type of data, and
/// optional code or date-based filters of the data.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataRequirement {
  /// The intended subjects of the data requirement. If this element is not provided,
  /// a Patient subject is assumed.
  #[serde(rename = "subjectCodeableConcept")]
  subject_codeable_concept: CodeableConcept,

  /// Indicates that specific elements of the type are referenced by the knowledge
  /// module and must be supported by the consumer in order to obtain an effective
  /// evaluation. This does not mean that a value is required for this element, only
  /// that the consuming system must understand the element and be able to provide
  /// values for it if they are available.     The value of mustSupport SHALL be a
  /// FHIRPath resolveable on the type of the DataRequirement. The path SHALL consist
  /// only of identifiers, constant indexers, and .resolve() (see the [Simple FHIRPath
  /// Profile](fhirpath.html#simple) for full details).
  #[serde(rename = "mustSupport")]
  must_support: Vec<String>,

  /// Date filters specify additional constraints on the data in terms of the
  /// applicable date range for specific elements. Each date filter specifies an
  /// additional constraint on the data, i.e. date filters are AND'ed, not OR'ed.
  #[serde(rename = "dateFilter")]
  date_filter: Vec<DataRequirement_DateFilter>,

  /// Specifies a maximum number of results that are required (uses the _count search
  /// parameter).
  limit: i32,

  /// Extensions for type
  _type: Element,

  /// The intended subjects of the data requirement. If this element is not provided,
  /// a Patient subject is assumed.
  #[serde(rename = "subjectReference")]
  subject_reference: Box<Reference>,

  /// Extensions for limit
  _limit: Element,

  /// Specifies the order of the results to be returned.
  sort: Vec<DataRequirement_Sort>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The type of the required data, specified as the type name of a resource. For
  /// profiles, this value is set to the type of the base resource of the profile.
  #[serde(rename = "type")]
  fhir_type: String,

  /// The profile of the required data, specified as the uri of the profile
  /// definition.
  profile: Vec<String>,

  /// Extensions for mustSupport
  #[serde(rename = "_mustSupport")]
  _must_support: Vec<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Code filters specify additional constraints on the data, specifying the value
  /// set of interest for a particular element of the data. Each code filter defines
  /// an additional constraint on the data, i.e. code filters are AND'ed, not OR'ed.
  #[serde(rename = "codeFilter")]
  code_filter: Vec<DataRequirement_CodeFilter>,

}
