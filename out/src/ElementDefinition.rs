use serde::{Deserialize, Serialize};

/// Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ElementDefinition {
  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternDataRequirement")]
  pattern_data_requirement: DataRequirement,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueHumanName")]
  default_value_human_name: HumanName,

  /// If present, indicates that the order of the repeating element has meaning and
  /// describes what that meaning is.  If absent, it means that the order of the
  /// element has no meaning.
  #[serde(rename = "orderMeaning")]
  order_meaning: String,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternContactDetail")]
  pattern_contact_detail: ContactDetail,

  /// Extensions for patternId
  #[serde(rename = "_patternId")]
  _pattern_id: Element,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueTime")]
  default_value_time: String,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternCode")]
  pattern_code: String,

  /// The minimum allowed value for the element. The value is inclusive. This is
  /// allowed for the types date, dateTime, instant, time, decimal, integer, and
  /// Quantity.
  #[serde(rename = "minValueDate")]
  min_value_date: String,

  /// The minimum allowed value for the element. The value is inclusive. This is
  /// allowed for the types date, dateTime, instant, time, decimal, integer, and
  /// Quantity.
  #[serde(rename = "minValueInstant")]
  min_value_instant: String,

  /// Extensions for defaultValueDateTime
  #[serde(rename = "_defaultValueDateTime")]
  _default_value_date_time: Element,

  /// Extensions for patternUnsignedInt
  #[serde(rename = "_patternUnsignedInt")]
  _pattern_unsigned_int: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueUsageContext")]
  default_value_usage_context: UsageContext,

  /// Extensions for condition
  _condition: Vec<Element>,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedUuid")]
  fixed_uuid: String,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedCount")]
  fixed_count: Count,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedDistance")]
  fixed_distance: Distance,

  /// If true, indicates that this slice definition is constraining a slice definition
  /// with the same name in an inherited profile. If false, the slice is not
  /// overriding any slice in an inherited profile. If missing, the slice might or
  /// might not be overriding a slice in an inherited profile, depending on the
  /// sliceName.
  #[serde(rename = "sliceIsConstraining")]
  slice_is_constraining: bool,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedDecimal")]
  fixed_decimal: number,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternCanonical")]
  pattern_canonical: String,

  /// Identifies additional names by which this element might also be known.
  alias: Vec<String>,

  /// Extensions for maxValueTime
  #[serde(rename = "_maxValueTime")]
  _max_value_time: Element,

  /// Extensions for defaultValueTime
  #[serde(rename = "_defaultValueTime")]
  _default_value_time: Element,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedUrl")]
  fixed_url: String,

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

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedInstant")]
  fixed_instant: String,

  /// Extensions for fixedUuid
  #[serde(rename = "_fixedUuid")]
  _fixed_uuid: Element,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternPeriod")]
  pattern_period: Period,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternQuantity")]
  pattern_quantity: Quantity,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueContributor")]
  default_value_contributor: Contributor,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedTriggerDefinition")]
  fixed_trigger_definition: TriggerDefinition,

  /// Extensions for defaultValueInstant
  #[serde(rename = "_defaultValueInstant")]
  _default_value_instant: Element,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValuePositiveInt")]
  default_value_positive_int: number,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternRatio")]
  pattern_ratio: Ratio,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueDateTime")]
  default_value_date_time: String,

  /// Extensions for patternInteger
  #[serde(rename = "_patternInteger")]
  _pattern_integer: Element,

  /// Extensions for defaultValueUuid
  #[serde(rename = "_defaultValueUuid")]
  _default_value_uuid: Element,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueSignature")]
  default_value_signature: Signature,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedDate")]
  fixed_date: String,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternUnsignedInt")]
  pattern_unsigned_int: number,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternCoding")]
  pattern_coding: Coding,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedRatio")]
  fixed_ratio: Ratio,

  /// Extensions for patternCode
  #[serde(rename = "_patternCode")]
  _pattern_code: Element,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueInstant")]
  default_value_instant: String,

  /// The maximum allowed value for the element. The value is inclusive. This is
  /// allowed for the types date, dateTime, instant, time, decimal, integer, and
  /// Quantity.
  #[serde(rename = "maxValueDate")]
  max_value_date: String,

  /// Extensions for fixedBase64Binary
  #[serde(rename = "_fixedBase64Binary")]
  _fixed_base_6_4_binary: Element,

  /// Extensions for minValueDecimal
  #[serde(rename = "_minValueDecimal")]
  _min_value_decimal: Element,

  /// Extensions for maxValueInstant
  #[serde(rename = "_maxValueInstant")]
  _max_value_instant: Element,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueId")]
  default_value_id: String,

  /// Extensions for defaultValueUri
  #[serde(rename = "_defaultValueUri")]
  _default_value_uri: Element,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedParameterDefinition")]
  fixed_parameter_definition: ParameterDefinition,

  /// The maximum allowed value for the element. The value is inclusive. This is
  /// allowed for the types date, dateTime, instant, time, decimal, integer, and
  /// Quantity.
  #[serde(rename = "maxValueInteger")]
  max_value_integer: number,

  /// Extensions for maxValueInteger
  #[serde(rename = "_maxValueInteger")]
  _max_value_integer: Element,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedIdentifier")]
  fixed_identifier: Identifier,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueDistance")]
  default_value_distance: Distance,

  /// The maximum number of times this element is permitted to appear in the instance.
  max: String,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternBoolean")]
  pattern_boolean: bool,

  /// Extensions for defaultValueMarkdown
  #[serde(rename = "_defaultValueMarkdown")]
  _default_value_markdown: Element,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternCount")]
  pattern_count: Count,

  /// Extensions for max
  _max: Element,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueOid")]
  default_value_oid: String,

  /// Extensions for path
  _path: Element,

  /// Extensions for defaultValueDecimal
  #[serde(rename = "_defaultValueDecimal")]
  _default_value_decimal: Element,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedBoolean")]
  fixed_boolean: bool,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueTriggerDefinition")]
  default_value_trigger_definition: TriggerDefinition,

  /// Extensions for min
  _min: Element,

  /// If true, the value of this element affects the interpretation of the element or
  /// resource that contains it, and the value of the element cannot be ignored.
  /// Typically, this is used for status, negation and qualification codes. The effect
  /// of this is that the element cannot be ignored by systems: they SHALL either
  /// recognize the element and process it, and/or a pre-determination has been made
  /// that it is not relevant to their particular system.
  #[serde(rename = "isModifier")]
  is_modifier: bool,

  /// The data type or resource that the value of this element is permitted to be.
  type: Vec<ElementDefinition_Type>,

  /// Extensions for defaultValueInteger
  #[serde(rename = "_defaultValueInteger")]
  _default_value_integer: Element,

  /// Extensions for fixedOid
  #[serde(rename = "_fixedOid")]
  _fixed_oid: Element,

  /// Extensions for mustSupport
  #[serde(rename = "_mustSupport")]
  _must_support: Element,

  /// Extensions for defaultValueCode
  #[serde(rename = "_defaultValueCode")]
  _default_value_code: Element,

  /// Extensions for patternUuid
  #[serde(rename = "_patternUuid")]
  _pattern_uuid: Element,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedRange")]
  fixed_range: Range,

  /// Extensions for maxValueDateTime
  #[serde(rename = "_maxValueDateTime")]
  _max_value_date_time: Element,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueDataRequirement")]
  default_value_data_requirement: DataRequirement,

  /// The maximum allowed value for the element. The value is inclusive. This is
  /// allowed for the types date, dateTime, instant, time, decimal, integer, and
  /// Quantity.
  #[serde(rename = "maxValueQuantity")]
  max_value_quantity: Quantity,

  /// Extensions for requirements
  _requirements: Element,

  /// Identifies an element defined elsewhere in the definition whose content rules
  /// should be applied to the current element. ContentReferences bring across all the
  /// rules that are in the ElementDefinition for the element, including definitions,
  /// cardinality constraints, bindings, invariants etc.
  #[serde(rename = "contentReference")]
  content_reference: String,

  /// Extensions for patternTime
  #[serde(rename = "_patternTime")]
  _pattern_time: Element,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueString")]
  default_value_string: String,

  /// The maximum allowed value for the element. The value is inclusive. This is
  /// allowed for the types date, dateTime, instant, time, decimal, integer, and
  /// Quantity.
  #[serde(rename = "maxValueTime")]
  max_value_time: String,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedInteger")]
  fixed_integer: number,

  /// Explains how that element affects the interpretation of the resource or element
  /// that contains it.
  #[serde(rename = "isModifierReason")]
  is_modifier_reason: String,

  /// Extensions for maxValueUnsignedInt
  #[serde(rename = "_maxValueUnsignedInt")]
  _max_value_unsigned_int: Element,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueMoney")]
  default_value_money: Money,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedExpression")]
  fixed_expression: Expression,

  /// The maximum allowed value for the element. The value is inclusive. This is
  /// allowed for the types date, dateTime, instant, time, decimal, integer, and
  /// Quantity.
  #[serde(rename = "maxValuePositiveInt")]
  max_value_positive_int: number,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternUrl")]
  pattern_url: String,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternAnnotation")]
  pattern_annotation: Annotation,

  /// Extensions for fixedMarkdown
  #[serde(rename = "_fixedMarkdown")]
  _fixed_markdown: Element,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternDosage")]
  pattern_dosage: Dosage,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueContactPoint")]
  default_value_contact_point: ContactPoint,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedPositiveInt")]
  fixed_positive_int: number,

  /// Extensions for defaultValueCanonical
  #[serde(rename = "_defaultValueCanonical")]
  _default_value_canonical: Element,

  /// The name of this element definition slice, when slicing is working. The name
  /// must be a token with no dots or spaces. This is a unique name referring to a
  /// specific set of constraints applied to this element, used to provide a name to
  /// different slices of the same element.
  #[serde(rename = "sliceName")]
  slice_name: String,

  /// Extensions for minValueTime
  #[serde(rename = "_minValueTime")]
  _min_value_time: Element,

  /// Extensions for minValueUnsignedInt
  #[serde(rename = "_minValueUnsignedInt")]
  _min_value_unsigned_int: Element,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueInteger")]
  default_value_integer: number,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedCode")]
  fixed_code: String,

  /// Extensions for maxValueDecimal
  #[serde(rename = "_maxValueDecimal")]
  _max_value_decimal: Element,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternDate")]
  pattern_date: String,

  /// A concise description of what this element means (e.g. for use in autogenerated
  /// summaries).
  short: String,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedUnsignedInt")]
  fixed_unsigned_int: number,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternTriggerDefinition")]
  pattern_trigger_definition: TriggerDefinition,

  /// Extensions for defaultValueOid
  #[serde(rename = "_defaultValueOid")]
  _default_value_oid: Element,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueParameterDefinition")]
  default_value_parameter_definition: ParameterDefinition,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedUri")]
  fixed_uri: String,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueUnsignedInt")]
  default_value_unsigned_int: number,

  /// Extensions for patternCanonical
  #[serde(rename = "_patternCanonical")]
  _pattern_canonical: Element,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternParameterDefinition")]
  pattern_parameter_definition: ParameterDefinition,

  /// The minimum allowed value for the element. The value is inclusive. This is
  /// allowed for the types date, dateTime, instant, time, decimal, integer, and
  /// Quantity.
  #[serde(rename = "minValueDateTime")]
  min_value_date_time: String,

  /// This element is for traceability of why the element was created and why the
  /// constraints exist as they do. This may be used to point to source materials or
  /// specifications that drove the structure of this element.
  requirements: markdown,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternReference")]
  pattern_reference: Reference,

  /// Extensions for fixedUri
  #[serde(rename = "_fixedUri")]
  _fixed_uri: Element,

  /// Extensions for minValuePositiveInt
  #[serde(rename = "_minValuePositiveInt")]
  _min_value_positive_int: Element,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedAddress")]
  fixed_address: Address,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueCodeableConcept")]
  default_value_codeable_concept: CodeableConcept,

  /// The Implicit meaning that is to be understood when this element is missing (e.g.
  /// 'when this element is missing, the period is ongoing').
  #[serde(rename = "meaningWhenMissing")]
  meaning_when_missing: markdown,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedTime")]
  fixed_time: String,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternContactPoint")]
  pattern_contact_point: ContactPoint,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternDistance")]
  pattern_distance: Distance,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueSampledData")]
  default_value_sampled_data: SampledData,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternIdentifier")]
  pattern_identifier: Identifier,

  /// Extensions for minValueDate
  #[serde(rename = "_minValueDate")]
  _min_value_date: Element,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedDataRequirement")]
  fixed_data_requirement: DataRequirement,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueCoding")]
  default_value_coding: Coding,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternContributor")]
  pattern_contributor: Contributor,

  /// Extensions for patternDateTime
  #[serde(rename = "_patternDateTime")]
  _pattern_date_time: Element,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternMoney")]
  pattern_money: Money,

  /// Information about the base definition of the element, provided to make it
  /// unnecessary for tools to trace the deviation of the element through the derived
  /// and related profiles. When the element definition is not the original definition
  /// of an element - i.g. either in a constraint on another type, or for elements
  /// from a super type in a snap shot - then the information in provided in the
  /// element definition may be different to the base definition. On the original
  /// definition of the element, it will be same.
  base: ElementDefinition_Base,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedDuration")]
  fixed_duration: Duration,

  /// Extensions for isSummary
  #[serde(rename = "_isSummary")]
  _is_summary: Element,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueBoolean")]
  default_value_boolean: bool,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueMarkdown")]
  default_value_markdown: String,

  /// The maximum allowed value for the element. The value is inclusive. This is
  /// allowed for the types date, dateTime, instant, time, decimal, integer, and
  /// Quantity.
  #[serde(rename = "maxValueUnsignedInt")]
  max_value_unsigned_int: number,

  /// Extensions for fixedInstant
  #[serde(rename = "_fixedInstant")]
  _fixed_instant: Element,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueCount")]
  default_value_count: Count,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValuePeriod")]
  default_value_period: Period,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedDateTime")]
  fixed_date_time: String,

  /// A code that has the same meaning as the element in a particular terminology.
  code: Vec<Coding>,

  /// The maximum allowed value for the element. The value is inclusive. This is
  /// allowed for the types date, dateTime, instant, time, decimal, integer, and
  /// Quantity.
  #[serde(rename = "maxValueInstant")]
  max_value_instant: String,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueContactDetail")]
  default_value_contact_detail: ContactDetail,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternTime")]
  pattern_time: String,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedContactPoint")]
  fixed_contact_point: ContactPoint,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternAttachment")]
  pattern_attachment: Attachment,

  /// Extensions for minValueDateTime
  #[serde(rename = "_minValueDateTime")]
  _min_value_date_time: Element,

  /// The minimum allowed value for the element. The value is inclusive. This is
  /// allowed for the types date, dateTime, instant, time, decimal, integer, and
  /// Quantity.
  #[serde(rename = "minValueInteger")]
  min_value_integer: number,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedMarkdown")]
  fixed_markdown: String,

  /// Extensions for fixedUrl
  #[serde(rename = "_fixedUrl")]
  _fixed_url: Element,

  /// A sample value for this element demonstrating the type of information that would
  /// typically be found in the element.
  example: Vec<ElementDefinition_Example>,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedId")]
  fixed_id: String,

  /// Extensions for patternMarkdown
  #[serde(rename = "_patternMarkdown")]
  _pattern_markdown: Element,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternRelatedArtifact")]
  pattern_related_artifact: RelatedArtifact,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedCodeableConcept")]
  fixed_codeable_concept: CodeableConcept,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedMoney")]
  fixed_money: Money,

  /// The minimum allowed value for the element. The value is inclusive. This is
  /// allowed for the types date, dateTime, instant, time, decimal, integer, and
  /// Quantity.
  #[serde(rename = "minValueQuantity")]
  min_value_quantity: Quantity,

  /// The maximum allowed value for the element. The value is inclusive. This is
  /// allowed for the types date, dateTime, instant, time, decimal, integer, and
  /// Quantity.
  #[serde(rename = "maxValueDecimal")]
  max_value_decimal: number,

  /// Extensions for defaultValueBoolean
  #[serde(rename = "_defaultValueBoolean")]
  _default_value_boolean: Element,

  /// Indicates the maximum length in characters that is permitted to be present in
  /// conformant instances and which is expected to be supported by conformant
  /// consumers that support the element.
  #[serde(rename = "maxLength")]
  max_length: integer,

  /// Extensions for representation
  _representation: Vec<Element>,

  /// Extensions for fixedPositiveInt
  #[serde(rename = "_fixedPositiveInt")]
  _fixed_positive_int: Element,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternBase64Binary")]
  pattern_base_6_4_binary: String,

  /// Indicates that the element is sliced into a set of alternative definitions (i.e.
  /// in a structure definition, there are multiple different constraints on a single
  /// element in the base resource). Slicing can be used in any resource that has
  /// cardinality ..* on the base resource, or any resource with a choice of types.
  /// The set of slices is any elements that come after this in the element sequence
  /// that have the same path, until a shorter path occurs (the shorter path
  /// terminates the set).
  slicing: ElementDefinition_Slicing,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternAge")]
  pattern_age: Age,

  /// Extensions for label
  _label: Element,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueAddress")]
  default_value_address: Address,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternDuration")]
  pattern_duration: Duration,

  /// Extensions for minValueInstant
  #[serde(rename = "_minValueInstant")]
  _min_value_instant: Element,

  /// Extensions for defaultValueBase64Binary
  #[serde(rename = "_defaultValueBase64Binary")]
  _default_value_base_6_4_binary: Element,

  /// Extensions for short
  _short: Element,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueRange")]
  default_value_range: Range,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueAttachment")]
  default_value_attachment: Attachment,

  /// The path identifies the element and is expressed as a "."-separated list of
  /// ancestor elements, beginning with the name of the resource or extension.
  path: String,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueUri")]
  default_value_uri: String,

  /// Extensions for patternBase64Binary
  #[serde(rename = "_patternBase64Binary")]
  _pattern_base_6_4_binary: Element,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternMeta")]
  pattern_meta: Meta,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueDate")]
  default_value_date: String,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueIdentifier")]
  default_value_identifier: Identifier,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternDateTime")]
  pattern_date_time: String,

  /// Extensions for sliceName
  #[serde(rename = "_sliceName")]
  _slice_name: Element,

  /// The minimum allowed value for the element. The value is inclusive. This is
  /// allowed for the types date, dateTime, instant, time, decimal, integer, and
  /// Quantity.
  #[serde(rename = "minValueTime")]
  min_value_time: String,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternTiming")]
  pattern_timing: Timing,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueDuration")]
  default_value_duration: Duration,

  /// The maximum allowed value for the element. The value is inclusive. This is
  /// allowed for the types date, dateTime, instant, time, decimal, integer, and
  /// Quantity.
  #[serde(rename = "maxValueDateTime")]
  max_value_date_time: String,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueReference")]
  default_value_reference: Reference,

  /// Extensions for fixedBoolean
  #[serde(rename = "_fixedBoolean")]
  _fixed_boolean: Element,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternMarkdown")]
  pattern_markdown: String,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternPositiveInt")]
  pattern_positive_int: number,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedReference")]
  fixed_reference: Reference,

  /// Extensions for fixedCanonical
  #[serde(rename = "_fixedCanonical")]
  _fixed_canonical: Element,

  /// Extensions for patternInstant
  #[serde(rename = "_patternInstant")]
  _pattern_instant: Element,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedSampledData")]
  fixed_sampled_data: SampledData,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternAddress")]
  pattern_address: Address,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternUsageContext")]
  pattern_usage_context: UsageContext,

  /// Extensions for fixedId
  #[serde(rename = "_fixedId")]
  _fixed_id: Element,

  /// Extensions for patternDecimal
  #[serde(rename = "_patternDecimal")]
  _pattern_decimal: Element,

  /// The minimum allowed value for the element. The value is inclusive. This is
  /// allowed for the types date, dateTime, instant, time, decimal, integer, and
  /// Quantity.
  #[serde(rename = "minValueUnsignedInt")]
  min_value_unsigned_int: number,

  /// The minimum allowed value for the element. The value is inclusive. This is
  /// allowed for the types date, dateTime, instant, time, decimal, integer, and
  /// Quantity.
  #[serde(rename = "minValuePositiveInt")]
  min_value_positive_int: number,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueQuantity")]
  default_value_quantity: Quantity,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueRatio")]
  default_value_ratio: Ratio,

  /// Extensions for meaningWhenMissing
  #[serde(rename = "_meaningWhenMissing")]
  _meaning_when_missing: Element,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedRelatedArtifact")]
  fixed_related_artifact: RelatedArtifact,

  /// Provides a complete explanation of the meaning of the data element for human
  /// readability.  For the case of elements derived from existing elements (e.g.
  /// constraints), the definition SHALL be consistent with the base definition, but
  /// convey the meaning of the element in the particular context of use of the
  /// resource. (Note: The text you are reading is specified in
  /// ElementDefinition.definition).
  definition: markdown,

  /// Extensions for fixedInteger
  #[serde(rename = "_fixedInteger")]
  _fixed_integer: Element,

  /// Extensions for defaultValueString
  #[serde(rename = "_defaultValueString")]
  _default_value_string: Element,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueTiming")]
  default_value_timing: Timing,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternExpression")]
  pattern_expression: Expression,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedCoding")]
  fixed_coding: Coding,

  /// A single preferred label which is the text to display beside the element
  /// indicating its meaning or to use to prompt for the element in a user display or
  /// form.
  label: String,

  /// Extensions for patternDate
  #[serde(rename = "_patternDate")]
  _pattern_date: Element,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueAge")]
  default_value_age: Age,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternCodeableConcept")]
  pattern_codeable_concept: CodeableConcept,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedDosage")]
  fixed_dosage: Dosage,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternSignature")]
  pattern_signature: Signature,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueRelatedArtifact")]
  default_value_related_artifact: RelatedArtifact,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedSignature")]
  fixed_signature: Signature,

  /// Extensions for fixedDateTime
  #[serde(rename = "_fixedDateTime")]
  _fixed_date_time: Element,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueDecimal")]
  default_value_decimal: number,

  /// Extensions for defaultValueUnsignedInt
  #[serde(rename = "_defaultValueUnsignedInt")]
  _default_value_unsigned_int: Element,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueMeta")]
  default_value_meta: Meta,

  /// Extensions for fixedUnsignedInt
  #[serde(rename = "_fixedUnsignedInt")]
  _fixed_unsigned_int: Element,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedAnnotation")]
  fixed_annotation: Annotation,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternId")]
  pattern_id: String,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternUuid")]
  pattern_uuid: String,

  /// Extensions for defaultValueUrl
  #[serde(rename = "_defaultValueUrl")]
  _default_value_url: Element,

  /// Extensions for fixedDecimal
  #[serde(rename = "_fixedDecimal")]
  _fixed_decimal: Element,

  /// Formal constraints such as co-occurrence and other constraints that can be
  /// computationally evaluated within the context of the instance.
  constraint: Vec<ElementDefinition_Constraint>,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedOid")]
  fixed_oid: String,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedUsageContext")]
  fixed_usage_context: UsageContext,

  /// Extensions for sliceIsConstraining
  #[serde(rename = "_sliceIsConstraining")]
  _slice_is_constraining: Element,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedTiming")]
  fixed_timing: Timing,

  /// Extensions for patternUrl
  #[serde(rename = "_patternUrl")]
  _pattern_url: Element,

  /// The minimum allowed value for the element. The value is inclusive. This is
  /// allowed for the types date, dateTime, instant, time, decimal, integer, and
  /// Quantity.
  #[serde(rename = "minValueDecimal")]
  min_value_decimal: number,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueBase64Binary")]
  default_value_base_6_4_binary: String,

  /// Extensions for contentReference
  #[serde(rename = "_contentReference")]
  _content_reference: Element,

  /// Binds to a value set if this element is coded (code, Coding, CodeableConcept,
  /// Quantity), or the data types (string, uri).
  binding: ElementDefinition_Binding,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueDosage")]
  default_value_dosage: Dosage,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternRange")]
  pattern_range: Range,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedHumanName")]
  fixed_human_name: HumanName,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternInstant")]
  pattern_instant: String,

  /// Whether the element should be included if a client requests a search with the
  /// parameter _summary=true.
  #[serde(rename = "isSummary")]
  is_summary: bool,

  /// Extensions for orderMeaning
  #[serde(rename = "_orderMeaning")]
  _order_meaning: Element,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueCanonical")]
  default_value_canonical: String,

  /// Extensions for maxLength
  #[serde(rename = "_maxLength")]
  _max_length: Element,

  /// Extensions for fixedTime
  #[serde(rename = "_fixedTime")]
  _fixed_time: Element,

  /// Extensions for maxValuePositiveInt
  #[serde(rename = "_maxValuePositiveInt")]
  _max_value_positive_int: Element,

  /// Explanatory notes and implementation guidance about the data element, including
  /// notes about how to use the data properly, exceptions to proper use, etc. (Note:
  /// The text you are reading is specified in ElementDefinition.comment).
  comment: markdown,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternInteger")]
  pattern_integer: number,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedContributor")]
  fixed_contributor: Contributor,

  /// Extensions for defaultValueDate
  #[serde(rename = "_defaultValueDate")]
  _default_value_date: Element,

  /// Extensions for fixedDate
  #[serde(rename = "_fixedDate")]
  _fixed_date: Element,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternOid")]
  pattern_oid: String,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternDecimal")]
  pattern_decimal: number,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedContactDetail")]
  fixed_contact_detail: ContactDetail,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedBase64Binary")]
  fixed_base_6_4_binary: String,

  /// A reference to an invariant that may make additional statements about the
  /// cardinality or value in the instance.
  condition: Vec<id>,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternString")]
  pattern_string: String,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueUrl")]
  default_value_url: String,

  /// Extensions for patternPositiveInt
  #[serde(rename = "_patternPositiveInt")]
  _pattern_positive_int: Element,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternUri")]
  pattern_uri: String,

  /// Identifies a concept from an external specification that roughly corresponds to
  /// this element.
  mapping: Vec<ElementDefinition_Mapping>,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternHumanName")]
  pattern_human_name: HumanName,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedMeta")]
  fixed_meta: Meta,

  /// Extensions for comment
  _comment: Element,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedAttachment")]
  fixed_attachment: Attachment,

  /// If true, implementations that produce or consume resources SHALL provide
  /// "support" for the element in some meaningful way.  If false, the element may be
  /// ignored and not supported. If false, whether to populate or use the data element
  /// in any way is at the discretion of the implementation.
  #[serde(rename = "mustSupport")]
  must_support: bool,

  /// Extensions for definition
  _definition: Element,

  /// Extensions for minValueInteger
  #[serde(rename = "_minValueInteger")]
  _min_value_integer: Element,

  /// Extensions for isModifier
  #[serde(rename = "_isModifier")]
  _is_modifier: Element,

  /// Extensions for defaultValuePositiveInt
  #[serde(rename = "_defaultValuePositiveInt")]
  _default_value_positive_int: Element,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueExpression")]
  default_value_expression: Expression,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedCanonical")]
  fixed_canonical: String,

  /// Extensions for patternString
  #[serde(rename = "_patternString")]
  _pattern_string: Element,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedQuantity")]
  fixed_quantity: Quantity,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueCode")]
  default_value_code: String,

  /// Extensions for fixedCode
  #[serde(rename = "_fixedCode")]
  _fixed_code: Element,

  /// Extensions for alias
  _alias: Vec<Element>,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueUuid")]
  default_value_uuid: String,

  /// Extensions for patternOid
  #[serde(rename = "_patternOid")]
  _pattern_oid: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for isModifierReason
  #[serde(rename = "_isModifierReason")]
  _is_modifier_reason: Element,

  /// Extensions for fixedString
  #[serde(rename = "_fixedString")]
  _fixed_string: Element,

  /// The minimum number of times this element SHALL appear in the instance.
  min: unsignedInt,

  /// Extensions for patternUri
  #[serde(rename = "_patternUri")]
  _pattern_uri: Element,

  /// Extensions for maxValueDate
  #[serde(rename = "_maxValueDate")]
  _max_value_date: Element,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedPeriod")]
  fixed_period: Period,

  /// Extensions for patternBoolean
  #[serde(rename = "_patternBoolean")]
  _pattern_boolean: Element,

  /// Specifies a value that the value in the instance SHALL follow - that is, any
  /// value in the pattern must be found in the instance. Other additional values may
  /// be found too. This is effectively constraint by example.  

  /// When pattern[x] is used to constrain a primitive, it means that the value
  /// provided in the pattern[x] must match the instance value exactly.

  /// When pattern[x] is used to constrain an array, it means that each element
  /// provided in the pattern[x] array must (recursively) match at least one element
  /// from the instance array.

  /// When pattern[x] is used to constrain a complex object, it means that each
  /// property in the pattern must be present in the complex object, and its value
  /// must recursively match -- i.e.,

  /// 1. If primitive: it must match exactly the pattern value
  /// 2. If a complex object: it must match (recursively) the pattern value
  /// 3. If an array: it must match (recursively) the pattern value.
  #[serde(rename = "patternSampledData")]
  pattern_sampled_data: SampledData,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedString")]
  fixed_string: String,

  /// Specifies a value that SHALL be exactly the value  for this element in the
  /// instance. For purposes of comparison, non-significant whitespace is ignored, and
  /// all values must be an exact match (case and accent sensitive). Missing
  /// elements/attributes must also be missing.
  #[serde(rename = "fixedAge")]
  fixed_age: Age,

  /// Extensions for defaultValueId
  #[serde(rename = "_defaultValueId")]
  _default_value_id: Element,

  /// The value that should be used if there is no value stated in the instance (e.g.
  /// 'if not otherwise specified, the abstract is false').
  #[serde(rename = "defaultValueAnnotation")]
  default_value_annotation: Annotation,

}
