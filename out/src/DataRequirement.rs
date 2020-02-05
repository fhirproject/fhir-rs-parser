use serde::{Deserialize, Serialize};

/// Describes a required data item for evaluation in terms of the type of data, and
/// optional code or date-based filters of the data.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DataRequirement {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for type
  _type: Element,

  /// Specifies the order of the results to be returned.
  sort: Vec<DataRequirement_Sort>,

  /// Code filters specify additional constraints on the data, specifying the value
  /// set of interest for a particular element of the data. Each code filter defines
  /// an additional constraint on the data, i.e. code filters are AND'ed, not OR'ed.
  #[serde(rename = "codeFilter")]
  code_filter: Vec<DataRequirement_CodeFilter>,

  /// Date filters specify additional constraints on the data in terms of the
  /// applicable date range for specific elements. Each date filter specifies an
  /// additional constraint on the data, i.e. date filters are AND'ed, not OR'ed.
  #[serde(rename = "dateFilter")]
  date_filter: Vec<DataRequirement_DateFilter>,

  /// Indicates that specific elements of the type are referenced by the knowledge
  /// module and must be supported by the consumer in order to obtain an effective
  /// evaluation. This does not mean that a value is required for this element, only
  /// that the consuming system must understand the element and be able to provide
  /// values for it if they are available. 

  /// The value of mustSupport SHALL be a FHIRPath resolveable on the type of the
  /// DataRequirement. The path SHALL consist only of identifiers, constant indexers,
  /// and .resolve() (see the [Simple FHIRPath Profile](fhirpath.html#simple) for full
  /// details).
  #[serde(rename = "mustSupport")]
  must_support: Vec<String>,

  /// Specifies a maximum number of results that are required (uses the _count search
  /// parameter).
  limit: positiveInt,

  /// Extensions for limit
  _limit: Element,

  /// The intended subjects of the data requirement. If this element is not provided,
  /// a Patient subject is assumed.
  #[serde(rename = "subjectReference")]
  subject_reference: Reference,

  /// The intended subjects of the data requirement. If this element is not provided,
  /// a Patient subject is assumed.
  #[serde(rename = "subjectCodeableConcept")]
  subject_codeable_concept: CodeableConcept,

  /// Extensions for mustSupport
  #[serde(rename = "_mustSupport")]
  _must_support: Vec<Element>,

  /// The type of the required data, specified as the type name of a resource. For
  /// profiles, this value is set to the type of the base resource of the profile.
  type: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The profile of the required data, specified as the uri of the profile
  /// definition.
  profile: Vec<canonical>,

}
