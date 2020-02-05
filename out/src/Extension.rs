use serde::{Deserialize, Serialize};

/// Optional Extension Element - found in all resources.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Extension {
  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueTime")]
  value_time: String,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueRange")]
  value_range: Range,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueMeta")]
  value_meta: Meta,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueUnsignedInt")]
  value_unsigned_int: number,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueDistance")]
  value_distance: Distance,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueContributor")]
  value_contributor: Contributor,

  /// Extensions for valueDecimal
  #[serde(rename = "_valueDecimal")]
  _value_decimal: Element,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueCanonical")]
  value_canonical: String,

  /// Extensions for valueInteger
  #[serde(rename = "_valueInteger")]
  _value_integer: Element,

  /// Extensions for valueUrl
  #[serde(rename = "_valueUrl")]
  _value_url: Element,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueContactPoint")]
  value_contact_point: ContactPoint,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueRelatedArtifact")]
  value_related_artifact: RelatedArtifact,

  /// Source of the definition for the extension code - a logical name or a URL.
  url: String,

  /// Extensions for valueUnsignedInt
  #[serde(rename = "_valueUnsignedInt")]
  _value_unsigned_int: Element,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueAddress")]
  value_address: Address,

  /// Extensions for valueCanonical
  #[serde(rename = "_valueCanonical")]
  _value_canonical: Element,

  /// Extensions for valueDateTime
  #[serde(rename = "_valueDateTime")]
  _value_date_time: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueDosage")]
  value_dosage: Dosage,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueString")]
  value_string: String,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueTiming")]
  value_timing: Timing,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueContactDetail")]
  value_contact_detail: ContactDetail,

  /// Extensions for valueOid
  #[serde(rename = "_valueOid")]
  _value_oid: Element,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueAge")]
  value_age: Age,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueIdentifier")]
  value_identifier: Identifier,

  /// Extensions for valueBoolean
  #[serde(rename = "_valueBoolean")]
  _value_boolean: Element,

  /// Extensions for valueId
  #[serde(rename = "_valueId")]
  _value_id: Element,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueUrl")]
  value_url: String,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueId")]
  value_id: String,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueInstant")]
  value_instant: String,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueOid")]
  value_oid: String,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueBoolean")]
  value_boolean: bool,

  /// Extensions for url
  _url: Element,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueInteger")]
  value_integer: number,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueSampledData")]
  value_sampled_data: SampledData,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueCoding")]
  value_coding: Coding,

  /// Extensions for valueTime
  #[serde(rename = "_valueTime")]
  _value_time: Element,

  /// Extensions for valuePositiveInt
  #[serde(rename = "_valuePositiveInt")]
  _value_positive_int: Element,

  /// Extensions for valueUuid
  #[serde(rename = "_valueUuid")]
  _value_uuid: Element,

  /// Extensions for valueInstant
  #[serde(rename = "_valueInstant")]
  _value_instant: Element,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueAnnotation")]
  value_annotation: Annotation,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueMoney")]
  value_money: Money,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueTriggerDefinition")]
  value_trigger_definition: TriggerDefinition,

  /// Extensions for valueCode
  #[serde(rename = "_valueCode")]
  _value_code: Element,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueExpression")]
  value_expression: Expression,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueDateTime")]
  value_date_time: String,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueAttachment")]
  value_attachment: Attachment,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueDecimal")]
  value_decimal: number,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueDuration")]
  value_duration: Duration,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueHumanName")]
  value_human_name: HumanName,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueUuid")]
  value_uuid: String,

  /// Extensions for valueBase64Binary
  #[serde(rename = "_valueBase64Binary")]
  _value_base_6_4_binary: Element,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueCode")]
  value_code: String,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueDate")]
  value_date: String,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valuePeriod")]
  value_period: Period,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueParameterDefinition")]
  value_parameter_definition: ParameterDefinition,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueCodeableConcept")]
  value_codeable_concept: CodeableConcept,

  /// Extensions for valueMarkdown
  #[serde(rename = "_valueMarkdown")]
  _value_markdown: Element,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueBase64Binary")]
  value_base_6_4_binary: String,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueReference")]
  value_reference: Reference,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueSignature")]
  value_signature: Signature,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueUsageContext")]
  value_usage_context: UsageContext,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueDataRequirement")]
  value_data_requirement: DataRequirement,

  /// Extensions for valueString
  #[serde(rename = "_valueString")]
  _value_string: Element,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueUri")]
  value_uri: String,

  /// Extensions for valueDate
  #[serde(rename = "_valueDate")]
  _value_date: Element,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueRatio")]
  value_ratio: Ratio,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueQuantity")]
  value_quantity: Quantity,

  /// Extensions for valueUri
  #[serde(rename = "_valueUri")]
  _value_uri: Element,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueMarkdown")]
  value_markdown: String,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueCount")]
  value_count: Count,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valuePositiveInt")]
  value_positive_int: number,

}
