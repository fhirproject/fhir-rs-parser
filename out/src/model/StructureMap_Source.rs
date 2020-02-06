#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Signature::Signature;
use crate::model::Money::Money;
use crate::model::DataRequirement::DataRequirement;
use crate::model::Distance::Distance;
use crate::model::TriggerDefinition::TriggerDefinition;
use crate::model::RelatedArtifact::RelatedArtifact;
use crate::model::Annotation::Annotation;
use crate::model::Duration::Duration;
use crate::model::ParameterDefinition::ParameterDefinition;
use crate::model::Range::Range;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::Coding::Coding;
use crate::model::ContactDetail::ContactDetail;
use crate::model::Contributor::Contributor;
use crate::model::SampledData::SampledData;
use crate::model::Period::Period;
use crate::model::Count::Count;
use crate::model::Attachment::Attachment;
use crate::model::HumanName::HumanName;
use crate::model::Ratio::Ratio;
use crate::model::Age::Age;
use crate::model::Expression::Expression;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use crate::model::Dosage::Dosage;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Identifier::Identifier;
use crate::model::Timing::Timing;
use crate::model::Address::Address;
use crate::model::UsageContext::UsageContext;


/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureMap_Source {
  /// Extensions for defaultValueUuid
  #[serde(rename = "_defaultValueUuid")]
  _default_value_uuid: Option<Element>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueAge")]
  default_value_age: Option<Age>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueBoolean")]
  default_value_boolean: Option<bool>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueUri")]
  default_value_uri: Option<String>,

  /// How to handle the list mode for this element.
  #[serde(rename = "listMode")]
  list_mode: Option<StructureMap_SourceListMode>,

  /// FHIRPath expression  - must be true or the mapping engine throws an error
  /// instead of completing.
  check: Option<String>,

  /// Extensions for defaultValueBase64Binary
  #[serde(rename = "_defaultValueBase64Binary")]
  _default_value_base_6_4_binary: Option<Element>,

  /// Extensions for defaultValueInteger
  #[serde(rename = "_defaultValueInteger")]
  _default_value_integer: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for defaultValueTime
  #[serde(rename = "_defaultValueTime")]
  _default_value_time: Option<Element>,

  /// Extensions for logMessage
  #[serde(rename = "_logMessage")]
  _log_message: Option<Element>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueId")]
  default_value_id: Option<String>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueDecimal")]
  default_value_decimal: Option<i32>,

  /// Extensions for defaultValueUnsignedInt
  #[serde(rename = "_defaultValueUnsignedInt")]
  _default_value_unsigned_int: Option<Element>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueCount")]
  default_value_count: Option<Count>,

  /// Named context for field, if a field is specified.
  variable: Option<String>,

  /// Type or variable this rule applies to.
  context: Option<String>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueUnsignedInt")]
  default_value_unsigned_int: Option<i32>,

  /// Extensions for defaultValueDecimal
  #[serde(rename = "_defaultValueDecimal")]
  _default_value_decimal: Option<Element>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueAttachment")]
  default_value_attachment: Option<Attachment>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Extensions for min
  #[serde(rename = "_min")]
  _min: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.    Modifier extensions
  /// SHALL NOT change the meaning of any elements on Resource or DomainResource
  /// (including cannot change the meaning of modifierExtension itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Option<Vec<Extension>>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueRatio")]
  default_value_ratio: Option<Ratio>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueOid")]
  default_value_oid: Option<String>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueQuantity")]
  default_value_quantity: Option<Quantity>,

  /// Extensions for condition
  #[serde(rename = "_condition")]
  _condition: Option<Element>,

  /// Extensions for defaultValueOid
  #[serde(rename = "_defaultValueOid")]
  _default_value_oid: Option<Element>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueUuid")]
  default_value_uuid: Option<String>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueDataRequirement")]
  default_value_data_requirement: Option<DataRequirement>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueUrl")]
  default_value_url: Option<String>,

  /// Extensions for listMode
  #[serde(rename = "_listMode")]
  _list_mode: Option<Element>,

  /// Specified minimum cardinality for the element. This is optional; if present, it
  /// acts an implicit check on the input content.
  min: Option<i32>,

  /// Extensions for defaultValuePositiveInt
  #[serde(rename = "_defaultValuePositiveInt")]
  _default_value_positive_int: Option<Element>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueRelatedArtifact")]
  default_value_related_artifact: Option<RelatedArtifact>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueInstant")]
  default_value_instant: Option<String>,

  /// Extensions for type
  #[serde(rename = "_type")]
  _type: Option<Element>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueRange")]
  default_value_range: Option<Range>,

  /// Extensions for defaultValueUri
  #[serde(rename = "_defaultValueUri")]
  _default_value_uri: Option<Element>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueCode")]
  default_value_code: Option<String>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueDistance")]
  default_value_distance: Option<Distance>,

  /// Extensions for defaultValueBoolean
  #[serde(rename = "_defaultValueBoolean")]
  _default_value_boolean: Option<Element>,

  /// Extensions for defaultValueId
  #[serde(rename = "_defaultValueId")]
  _default_value_id: Option<Element>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueCoding")]
  default_value_coding: Option<Coding>,

  /// Extensions for defaultValueCanonical
  #[serde(rename = "_defaultValueCanonical")]
  _default_value_canonical: Option<Element>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueMarkdown")]
  default_value_markdown: Option<String>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueTime")]
  default_value_time: Option<String>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueHumanName")]
  default_value_human_name: Option<HumanName>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueString")]
  default_value_string: Option<String>,

  /// Specified type for the element. This works as a condition on the mapping - use
  /// for polymorphic elements.
  #[serde(rename = "type")]
  fhir_type: Option<String>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueBase64Binary")]
  default_value_base_6_4_binary: Option<String>,

  /// Optional field for this source.
  element: Option<String>,

  /// Extensions for check
  #[serde(rename = "_check")]
  _check: Option<Element>,

  /// Extensions for defaultValueString
  #[serde(rename = "_defaultValueString")]
  _default_value_string: Option<Element>,

  /// FHIRPath expression  - must be true or the rule does not apply.
  condition: Option<String>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueTriggerDefinition")]
  default_value_trigger_definition: Option<TriggerDefinition>,

  /// A FHIRPath expression which specifies a message to put in the transform log when
  /// content matching the source rule is found.
  #[serde(rename = "logMessage")]
  log_message: Option<String>,

  /// Extensions for max
  #[serde(rename = "_max")]
  _max: Option<Element>,

  /// Extensions for defaultValueDate
  #[serde(rename = "_defaultValueDate")]
  _default_value_date: Option<Element>,

  /// Extensions for defaultValueCode
  #[serde(rename = "_defaultValueCode")]
  _default_value_code: Option<Element>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueContributor")]
  default_value_contributor: Option<Contributor>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueInteger")]
  default_value_integer: Option<i32>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueCanonical")]
  default_value_canonical: Option<String>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueParameterDefinition")]
  default_value_parameter_definition: Option<ParameterDefinition>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueSampledData")]
  default_value_sampled_data: Option<SampledData>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueMeta")]
  default_value_meta: Option<Meta>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueUsageContext")]
  default_value_usage_context: Option<UsageContext>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValuePositiveInt")]
  default_value_positive_int: Option<i32>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueDate")]
  default_value_date: Option<String>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueIdentifier")]
  default_value_identifier: Option<Identifier>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueMoney")]
  default_value_money: Option<Money>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueExpression")]
  default_value_expression: Option<Expression>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueReference")]
  default_value_reference: Option<Box<Reference>>,

  /// Extensions for defaultValueInstant
  #[serde(rename = "_defaultValueInstant")]
  _default_value_instant: Option<Element>,

  /// Extensions for context
  #[serde(rename = "_context")]
  _context: Option<Element>,

  /// Extensions for defaultValueUrl
  #[serde(rename = "_defaultValueUrl")]
  _default_value_url: Option<Element>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValuePeriod")]
  default_value_period: Option<Period>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueSignature")]
  default_value_signature: Option<Signature>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueTiming")]
  default_value_timing: Option<Timing>,

  /// Extensions for variable
  #[serde(rename = "_variable")]
  _variable: Option<Element>,

  /// Extensions for defaultValueMarkdown
  #[serde(rename = "_defaultValueMarkdown")]
  _default_value_markdown: Option<Element>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueAddress")]
  default_value_address: Option<Address>,

  /// Specified maximum cardinality for the element - a number or a "*". This is
  /// optional; if present, it acts an implicit check on the input content (* just
  /// serves as documentation; it's the default value).
  max: Option<String>,

  /// Extensions for defaultValueDateTime
  #[serde(rename = "_defaultValueDateTime")]
  _default_value_date_time: Option<Element>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueCodeableConcept")]
  default_value_codeable_concept: Option<CodeableConcept>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueContactPoint")]
  default_value_contact_point: Option<ContactPoint>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueDuration")]
  default_value_duration: Option<Duration>,

  /// Extensions for element
  #[serde(rename = "_element")]
  _element: Option<Element>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueContactDetail")]
  default_value_contact_detail: Option<ContactDetail>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueAnnotation")]
  default_value_annotation: Option<Annotation>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueDateTime")]
  default_value_date_time: Option<String>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueDosage")]
  default_value_dosage: Option<Dosage>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum StructureMap_SourceListMode {
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
