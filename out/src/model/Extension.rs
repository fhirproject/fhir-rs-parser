#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Expression::Expression;
use crate::model::ContactDetail::ContactDetail;
use crate::model::Timing::Timing;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Reference::Reference;
use crate::model::Signature::Signature;
use crate::model::DataRequirement::DataRequirement;
use crate::model::Dosage::Dosage;
use crate::model::Money::Money;
use crate::model::ContactPoint::ContactPoint;
use crate::model::ParameterDefinition::ParameterDefinition;
use crate::model::Duration::Duration;
use crate::model::Address::Address;
use crate::model::Age::Age;
use crate::model::TriggerDefinition::TriggerDefinition;
use crate::model::HumanName::HumanName;
use crate::model::SampledData::SampledData;
use crate::model::UsageContext::UsageContext;
use crate::model::Identifier::Identifier;
use crate::model::Element::Element;
use crate::model::Period::Period;
use crate::model::Meta::Meta;
use crate::model::Contributor::Contributor;
use crate::model::Distance::Distance;
use crate::model::Quantity::Quantity;
use crate::model::Attachment::Attachment;
use crate::model::RelatedArtifact::RelatedArtifact;
use crate::model::Annotation::Annotation;
use crate::model::Ratio::Ratio;
use crate::model::Coding::Coding;
use crate::model::Range::Range;
use crate::model::Count::Count;


/// Optional Extension Element - found in all resources.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extension {
  /// Extensions for valueDecimal
  #[serde(rename = "_valueDecimal")]
  _value_decimal: Option<Element>,

  /// Extensions for url
  #[serde(rename = "_url")]
  _url: Option<Element>,

  /// Extensions for valueDateTime
  #[serde(rename = "_valueDateTime")]
  _value_date_time: Option<Element>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueReference")]
  value_reference: Option<Box<Reference>>,

  /// Extensions for valueCode
  #[serde(rename = "_valueCode")]
  _value_code: Option<Element>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueRange")]
  value_range: Option<Range>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueRelatedArtifact")]
  value_related_artifact: Option<RelatedArtifact>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueId")]
  value_id: Option<String>,

  /// Extensions for valueInstant
  #[serde(rename = "_valueInstant")]
  _value_instant: Option<Element>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueBase64Binary")]
  value_base_6_4_binary: Option<String>,

  /// Extensions for valueMarkdown
  #[serde(rename = "_valueMarkdown")]
  _value_markdown: Option<Element>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueDosage")]
  value_dosage: Option<Dosage>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueCodeableConcept")]
  value_codeable_concept: Option<CodeableConcept>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueCount")]
  value_count: Option<Count>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueMeta")]
  value_meta: Option<Meta>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueAttachment")]
  value_attachment: Option<Attachment>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueContactDetail")]
  value_contact_detail: Option<ContactDetail>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueString")]
  value_string: Option<String>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueCanonical")]
  value_canonical: Option<String>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueIdentifier")]
  value_identifier: Option<Identifier>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueAge")]
  value_age: Option<Age>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueTime")]
  value_time: Option<String>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueUuid")]
  value_uuid: Option<String>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueRatio")]
  value_ratio: Option<Ratio>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueExpression")]
  value_expression: Option<Expression>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueMarkdown")]
  value_markdown: Option<String>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueContactPoint")]
  value_contact_point: Option<ContactPoint>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueDistance")]
  value_distance: Option<Distance>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueUsageContext")]
  value_usage_context: Option<UsageContext>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueDate")]
  value_date: Option<String>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueOid")]
  value_oid: Option<String>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valuePositiveInt")]
  value_positive_int: Option<i32>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueMoney")]
  value_money: Option<Money>,

  /// Extensions for valueInteger
  #[serde(rename = "_valueInteger")]
  _value_integer: Option<Element>,

  /// Extensions for valueUnsignedInt
  #[serde(rename = "_valueUnsignedInt")]
  _value_unsigned_int: Option<Element>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueTriggerDefinition")]
  value_trigger_definition: Option<TriggerDefinition>,

  /// Extensions for valueUuid
  #[serde(rename = "_valueUuid")]
  _value_uuid: Option<Element>,

  /// Extensions for valueDate
  #[serde(rename = "_valueDate")]
  _value_date: Option<Element>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueBoolean")]
  value_boolean: Option<bool>,

  /// Extensions for valueId
  #[serde(rename = "_valueId")]
  _value_id: Option<Element>,

  /// Extensions for valueCanonical
  #[serde(rename = "_valueCanonical")]
  _value_canonical: Option<Element>,

  /// Extensions for valueUri
  #[serde(rename = "_valueUri")]
  _value_uri: Option<Element>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueCoding")]
  value_coding: Option<Coding>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueUri")]
  value_uri: Option<String>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueAddress")]
  value_address: Option<Address>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueHumanName")]
  value_human_name: Option<HumanName>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueUnsignedInt")]
  value_unsigned_int: Option<i32>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueQuantity")]
  value_quantity: Option<Quantity>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valuePeriod")]
  value_period: Option<Period>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueSignature")]
  value_signature: Option<Signature>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueUrl")]
  value_url: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for valueBase64Binary
  #[serde(rename = "_valueBase64Binary")]
  _value_base_6_4_binary: Option<Element>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueParameterDefinition")]
  value_parameter_definition: Option<ParameterDefinition>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueDataRequirement")]
  value_data_requirement: Option<DataRequirement>,

  /// Extensions for valueTime
  #[serde(rename = "_valueTime")]
  _value_time: Option<Element>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueCode")]
  value_code: Option<String>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueInstant")]
  value_instant: Option<String>,

  /// Extensions for valueOid
  #[serde(rename = "_valueOid")]
  _value_oid: Option<Element>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueDecimal")]
  value_decimal: Option<i32>,

  /// Extensions for valueBoolean
  #[serde(rename = "_valueBoolean")]
  _value_boolean: Option<Element>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueTiming")]
  value_timing: Option<Timing>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueDuration")]
  value_duration: Option<Duration>,

  /// Extensions for valueUrl
  #[serde(rename = "_valueUrl")]
  _value_url: Option<Element>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueInteger")]
  value_integer: Option<i32>,

  /// Extensions for valuePositiveInt
  #[serde(rename = "_valuePositiveInt")]
  _value_positive_int: Option<Element>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueAnnotation")]
  value_annotation: Option<Annotation>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueSampledData")]
  value_sampled_data: Option<SampledData>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueDateTime")]
  value_date_time: Option<String>,

  /// Value of extension - must be one of a constrained set of the data types (see
  /// [Extensibility](extensibility.html) for a list).
  #[serde(rename = "valueContributor")]
  value_contributor: Option<Contributor>,

  /// Extensions for valueString
  #[serde(rename = "_valueString")]
  _value_string: Option<Element>,

  /// Source of the definition for the extension code - a logical name or a URL.
  url: Option<String>,

}
