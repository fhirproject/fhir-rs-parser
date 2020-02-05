use serde::{Deserialize, Serialize};

/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StructureMap_Source {
  /// Extensions for check
  _check: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueRatio")]
  default_value_ratio: Ratio,

  /// Extensions for defaultValueMarkdown
  #[serde(rename = "_defaultValueMarkdown")]
  _default_value_markdown: Element,

  /// Extensions for condition
  _condition: Element,

  /// Extensions for defaultValueDecimal
  #[serde(rename = "_defaultValueDecimal")]
  _default_value_decimal: Element,

  /// Extensions for defaultValueUuid
  #[serde(rename = "_defaultValueUuid")]
  _default_value_uuid: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueMeta")]
  default_value_meta: Meta,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueUrl")]
  default_value_url: String,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueDecimal")]
  default_value_decimal: number,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueContactDetail")]
  default_value_contact_detail: ContactDetail,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueCount")]
  default_value_count: Count,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueCoding")]
  default_value_coding: Coding,

  /// Extensions for min
  _min: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueSignature")]
  default_value_signature: Signature,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueCodeableConcept")]
  default_value_codeable_concept: CodeableConcept,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueUuid")]
  default_value_uuid: String,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueRange")]
  default_value_range: Range,

  /// Optional field for this source.
  element: String,

  /// Extensions for defaultValueDate
  #[serde(rename = "_defaultValueDate")]
  _default_value_date: Element,

  /// A FHIRPath expression which specifies a message to put in the transform log when
  /// content matching the source rule is found.
  #[serde(rename = "logMessage")]
  log_message: String,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueRelatedArtifact")]
  default_value_related_artifact: RelatedArtifact,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueOid")]
  default_value_oid: String,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueAddress")]
  default_value_address: Address,

  /// Extensions for defaultValueUri
  #[serde(rename = "_defaultValueUri")]
  _default_value_uri: Element,

  /// Extensions for defaultValuePositiveInt
  #[serde(rename = "_defaultValuePositiveInt")]
  _default_value_positive_int: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueExpression")]
  default_value_expression: Expression,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueTiming")]
  default_value_timing: Timing,

  /// Specified minimum cardinality for the element. This is optional; if present, it
  /// acts an implicit check on the input content.
  min: integer,

  /// Extensions for defaultValueUnsignedInt
  #[serde(rename = "_defaultValueUnsignedInt")]
  _default_value_unsigned_int: Element,

  /// Extensions for logMessage
  #[serde(rename = "_logMessage")]
  _log_message: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueBase64Binary")]
  default_value_base_6_4_binary: String,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueDate")]
  default_value_date: String,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueSampledData")]
  default_value_sampled_data: SampledData,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueDosage")]
  default_value_dosage: Dosage,

  /// Extensions for defaultValueTime
  #[serde(rename = "_defaultValueTime")]
  _default_value_time: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValuePeriod")]
  default_value_period: Period,

  /// Specified type for the element. This works as a condition on the mapping - use
  /// for polymorphic elements.
  type: String,

  /// Extensions for defaultValueBase64Binary
  #[serde(rename = "_defaultValueBase64Binary")]
  _default_value_base_6_4_binary: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueCode")]
  default_value_code: String,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueInstant")]
  default_value_instant: String,

  /// Extensions for defaultValueInstant
  #[serde(rename = "_defaultValueInstant")]
  _default_value_instant: Element,

  /// Extensions for defaultValueString
  #[serde(rename = "_defaultValueString")]
  _default_value_string: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueAge")]
  default_value_age: Age,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueDataRequirement")]
  default_value_data_requirement: DataRequirement,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueAttachment")]
  default_value_attachment: Attachment,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueParameterDefinition")]
  default_value_parameter_definition: ParameterDefinition,

  /// Extensions for element
  _element: Element,

  /// Named context for field, if a field is specified.
  variable: id,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueUnsignedInt")]
  default_value_unsigned_int: number,

  /// Extensions for defaultValueCanonical
  #[serde(rename = "_defaultValueCanonical")]
  _default_value_canonical: Element,

  /// Extensions for defaultValueCode
  #[serde(rename = "_defaultValueCode")]
  _default_value_code: Element,

  /// Extensions for defaultValueInteger
  #[serde(rename = "_defaultValueInteger")]
  _default_value_integer: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for context
  _context: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueInteger")]
  default_value_integer: number,

  /// FHIRPath expression  - must be true or the rule does not apply.
  condition: String,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueTriggerDefinition")]
  default_value_trigger_definition: TriggerDefinition,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueString")]
  default_value_string: String,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueQuantity")]
  default_value_quantity: Quantity,

  /// Specified maximum cardinality for the element - a number or a "*". This is
  /// optional; if present, it acts an implicit check on the input content (* just
  /// serves as documentation; it's the default value).
  max: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueDateTime")]
  default_value_date_time: String,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueHumanName")]
  default_value_human_name: HumanName,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueUsageContext")]
  default_value_usage_context: UsageContext,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueCanonical")]
  default_value_canonical: String,

  /// Extensions for type
  _type: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueContributor")]
  default_value_contributor: Contributor,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueReference")]
  default_value_reference: Reference,

  /// Extensions for defaultValueId
  #[serde(rename = "_defaultValueId")]
  _default_value_id: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueBoolean")]
  default_value_boolean: bool,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueAnnotation")]
  default_value_annotation: Annotation,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueDistance")]
  default_value_distance: Distance,

  /// Extensions for listMode
  #[serde(rename = "_listMode")]
  _list_mode: Element,

  /// FHIRPath expression  - must be true or the mapping engine throws an error
  /// instead of completing.
  check: String,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueMarkdown")]
  default_value_markdown: String,

  /// Extensions for defaultValueOid
  #[serde(rename = "_defaultValueOid")]
  _default_value_oid: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValuePositiveInt")]
  default_value_positive_int: number,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueUri")]
  default_value_uri: String,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueContactPoint")]
  default_value_contact_point: ContactPoint,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueMoney")]
  default_value_money: Money,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueIdentifier")]
  default_value_identifier: Identifier,

  /// Extensions for defaultValueBoolean
  #[serde(rename = "_defaultValueBoolean")]
  _default_value_boolean: Element,

  /// Extensions for max
  _max: Element,

  /// Type or variable this rule applies to.
  context: id,

  /// How to handle the list mode for this element.
  #[serde(rename = "listMode")]
  list_mode: StructureMap_SourceListMode,

  /// Extensions for variable
  _variable: Element,

  /// Extensions for defaultValueUrl
  #[serde(rename = "_defaultValueUrl")]
  _default_value_url: Element,

  /// Extensions for defaultValueDateTime
  #[serde(rename = "_defaultValueDateTime")]
  _default_value_date_time: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueId")]
  default_value_id: String,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueDuration")]
  default_value_duration: Duration,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueTime")]
  default_value_time: String,

}

#[derive(Debug, Serialize, Deserialize)]
enum StructureMap_SourceListMode {
  #[serde(rename = "first")]
  First,

  #[serde(rename = "not_first")]
  NotFirst,

  #[serde(rename = "last")]
  Last,

  #[serde(rename = "not_last")]
  NotLast,

  #[serde(rename = "only_one")]
  OnlyOne,

}
