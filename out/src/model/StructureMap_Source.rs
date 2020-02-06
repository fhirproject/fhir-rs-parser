#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Expression::Expression;
use crate::model::Identifier::Identifier;
use crate::model::HumanName::HumanName;
use crate::model::Annotation::Annotation;
use crate::model::Money::Money;
use crate::model::Period::Period;
use crate::model::RelatedArtifact::RelatedArtifact;
use crate::model::Meta::Meta;
use crate::model::Attachment::Attachment;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Timing::Timing;
use crate::model::Element::Element;
use crate::model::Dosage::Dosage;
use crate::model::ParameterDefinition::ParameterDefinition;
use crate::model::Duration::Duration;
use crate::model::Ratio::Ratio;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactDetail::ContactDetail;
use crate::model::Distance::Distance;
use crate::model::Quantity::Quantity;
use crate::model::Address::Address;
use crate::model::DataRequirement::DataRequirement;
use crate::model::UsageContext::UsageContext;
use crate::model::Range::Range;
use crate::model::SampledData::SampledData;
use crate::model::Reference::Reference;
use crate::model::Extension::Extension;
use crate::model::Count::Count;
use crate::model::Signature::Signature;
use crate::model::Coding::Coding;
use crate::model::Contributor::Contributor;
use crate::model::Age::Age;
use crate::model::TriggerDefinition::TriggerDefinition;


/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureMap_Source {
  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueDuration")]
  default_value_duration: Duration,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueSignature")]
  default_value_signature: Signature,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueDecimal")]
  default_value_decimal: i32,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueRelatedArtifact")]
  default_value_related_artifact: RelatedArtifact,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueBase64Binary")]
  default_value_base_6_4_binary: String,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueString")]
  default_value_string: String,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueId")]
  default_value_id: String,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueUrl")]
  default_value_url: String,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueExpression")]
  default_value_expression: Expression,

  /// Extensions for defaultValueBoolean
  #[serde(rename = "_defaultValueBoolean")]
  _default_value_boolean: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValuePeriod")]
  default_value_period: Period,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueCode")]
  default_value_code: String,

  /// Extensions for defaultValueTime
  #[serde(rename = "_defaultValueTime")]
  _default_value_time: Element,

  /// Extensions for defaultValueString
  #[serde(rename = "_defaultValueString")]
  _default_value_string: Element,

  /// Type or variable this rule applies to.
  context: String,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueAge")]
  default_value_age: Age,

  /// Specified type for the element. This works as a condition on the mapping - use
  /// for polymorphic elements.
  #[serde(rename = "type")]
  fhir_type: String,

  /// Specified minimum cardinality for the element. This is optional; if present, it
  /// acts an implicit check on the input content.
  min: i32,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueDistance")]
  default_value_distance: Distance,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueAttachment")]
  default_value_attachment: Attachment,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueRatio")]
  default_value_ratio: Ratio,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueUuid")]
  default_value_uuid: String,

  /// Extensions for defaultValueMarkdown
  #[serde(rename = "_defaultValueMarkdown")]
  _default_value_markdown: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueCanonical")]
  default_value_canonical: String,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueContactDetail")]
  default_value_contact_detail: ContactDetail,

  /// Extensions for max
  _max: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueDataRequirement")]
  default_value_data_requirement: DataRequirement,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueMeta")]
  default_value_meta: Meta,

  /// Extensions for defaultValueInteger
  #[serde(rename = "_defaultValueInteger")]
  _default_value_integer: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueParameterDefinition")]
  default_value_parameter_definition: ParameterDefinition,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueTiming")]
  default_value_timing: Timing,

  /// Extensions for context
  _context: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueInteger")]
  default_value_integer: i32,

  /// Extensions for defaultValueCanonical
  #[serde(rename = "_defaultValueCanonical")]
  _default_value_canonical: Element,

  /// Extensions for defaultValueUnsignedInt
  #[serde(rename = "_defaultValueUnsignedInt")]
  _default_value_unsigned_int: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueReference")]
  default_value_reference: Box<Reference>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueContributor")]
  default_value_contributor: Contributor,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueDosage")]
  default_value_dosage: Dosage,

  /// Extensions for defaultValueDate
  #[serde(rename = "_defaultValueDate")]
  _default_value_date: Element,

  /// Extensions for element
  _element: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueTime")]
  default_value_time: String,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueAddress")]
  default_value_address: Address,

  /// Extensions for defaultValueDecimal
  #[serde(rename = "_defaultValueDecimal")]
  _default_value_decimal: Element,

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
  modifier_extension: Vec<Extension>,

  /// Extensions for check
  _check: Element,

  /// Extensions for condition
  _condition: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueRange")]
  default_value_range: Range,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueSampledData")]
  default_value_sampled_data: SampledData,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueIdentifier")]
  default_value_identifier: Identifier,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueMarkdown")]
  default_value_markdown: String,

  /// Extensions for defaultValueOid
  #[serde(rename = "_defaultValueOid")]
  _default_value_oid: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueDate")]
  default_value_date: String,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueDateTime")]
  default_value_date_time: String,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueContactPoint")]
  default_value_contact_point: ContactPoint,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueTriggerDefinition")]
  default_value_trigger_definition: TriggerDefinition,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueUnsignedInt")]
  default_value_unsigned_int: i32,

  /// Extensions for variable
  _variable: Element,

  /// Extensions for defaultValuePositiveInt
  #[serde(rename = "_defaultValuePositiveInt")]
  _default_value_positive_int: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueOid")]
  default_value_oid: String,

  /// Named context for field, if a field is specified.
  variable: String,

  /// How to handle the list mode for this element.
  #[serde(rename = "listMode")]
  list_mode: StructureMap_SourceListMode,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for min
  _min: Element,

  /// Extensions for defaultValueCode
  #[serde(rename = "_defaultValueCode")]
  _default_value_code: Element,

  /// Extensions for defaultValueInstant
  #[serde(rename = "_defaultValueInstant")]
  _default_value_instant: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValuePositiveInt")]
  default_value_positive_int: i32,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueAnnotation")]
  default_value_annotation: Annotation,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueCodeableConcept")]
  default_value_codeable_concept: CodeableConcept,

  /// Extensions for defaultValueBase64Binary
  #[serde(rename = "_defaultValueBase64Binary")]
  _default_value_base_6_4_binary: Element,

  /// Extensions for defaultValueId
  #[serde(rename = "_defaultValueId")]
  _default_value_id: Element,

  /// Extensions for defaultValueUri
  #[serde(rename = "_defaultValueUri")]
  _default_value_uri: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueBoolean")]
  default_value_boolean: bool,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueHumanName")]
  default_value_human_name: HumanName,

  /// Extensions for defaultValueUuid
  #[serde(rename = "_defaultValueUuid")]
  _default_value_uuid: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueCount")]
  default_value_count: Count,

  /// A FHIRPath expression which specifies a message to put in the transform log when
  /// content matching the source rule is found.
  #[serde(rename = "logMessage")]
  log_message: String,

  /// FHIRPath expression  - must be true or the mapping engine throws an error
  /// instead of completing.
  check: String,

  /// Specified maximum cardinality for the element - a number or a "*". This is
  /// optional; if present, it acts an implicit check on the input content (* just
  /// serves as documentation; it's the default value).
  max: String,

  /// FHIRPath expression  - must be true or the rule does not apply.
  condition: String,

  /// Optional field for this source.
  element: String,

  /// Extensions for listMode
  #[serde(rename = "_listMode")]
  _list_mode: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueCoding")]
  default_value_coding: Coding,

  /// Extensions for logMessage
  #[serde(rename = "_logMessage")]
  _log_message: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueInstant")]
  default_value_instant: String,

  /// Extensions for type
  _type: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueUri")]
  default_value_uri: String,

  /// Extensions for defaultValueUrl
  #[serde(rename = "_defaultValueUrl")]
  _default_value_url: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueMoney")]
  default_value_money: Money,

  /// Extensions for defaultValueDateTime
  #[serde(rename = "_defaultValueDateTime")]
  _default_value_date_time: Element,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueQuantity")]
  default_value_quantity: Quantity,

  /// A value to use if there is no existing value in the source object.
  #[serde(rename = "defaultValueUsageContext")]
  default_value_usage_context: UsageContext,

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
