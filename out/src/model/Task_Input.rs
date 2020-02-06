#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Signature::Signature;
use crate::model::DataRequirement::DataRequirement;
use crate::model::Attachment::Attachment;
use crate::model::HumanName::HumanName;
use crate::model::Contributor::Contributor;
use crate::model::Address::Address;
use crate::model::TriggerDefinition::TriggerDefinition;
use crate::model::Duration::Duration;
use crate::model::Age::Age;
use crate::model::Range::Range;
use crate::model::ContactPoint::ContactPoint;
use crate::model::ParameterDefinition::ParameterDefinition;
use crate::model::UsageContext::UsageContext;
use crate::model::Expression::Expression;
use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::SampledData::SampledData;
use crate::model::RelatedArtifact::RelatedArtifact;
use crate::model::Timing::Timing;
use crate::model::Distance::Distance;
use crate::model::Reference::Reference;
use crate::model::Dosage::Dosage;
use crate::model::Quantity::Quantity;
use crate::model::Money::Money;
use crate::model::Ratio::Ratio;
use crate::model::ContactDetail::ContactDetail;
use crate::model::Count::Count;
use crate::model::Extension::Extension;
use crate::model::Coding::Coding;
use crate::model::Element::Element;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Period::Period;


/// A task to be performed.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task_Input {
  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueDosage")]
  value_dosage: Option<Dosage>,

  /// Extensions for valueBoolean
  #[serde(rename = "_valueBoolean")]
  _value_boolean: Option<Element>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueParameterDefinition")]
  value_parameter_definition: Option<ParameterDefinition>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueCanonical")]
  value_canonical: Option<String>,

  /// Extensions for valueString
  #[serde(rename = "_valueString")]
  _value_string: Option<Element>,

  /// Extensions for valueUnsignedInt
  #[serde(rename = "_valueUnsignedInt")]
  _value_unsigned_int: Option<Element>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueInstant")]
  value_instant: Option<String>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueAnnotation")]
  value_annotation: Option<Annotation>,

  /// Extensions for valuePositiveInt
  #[serde(rename = "_valuePositiveInt")]
  _value_positive_int: Option<Element>,

  /// Extensions for valueCode
  #[serde(rename = "_valueCode")]
  _value_code: Option<Element>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueCodeableConcept")]
  value_codeable_concept: Option<CodeableConcept>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueContactDetail")]
  value_contact_detail: Option<ContactDetail>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueCode")]
  value_code: Option<String>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueDate")]
  value_date: Option<String>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueAddress")]
  value_address: Option<Address>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueCount")]
  value_count: Option<Count>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueAttachment")]
  value_attachment: Option<Attachment>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valuePositiveInt")]
  value_positive_int: Option<i32>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueTriggerDefinition")]
  value_trigger_definition: Option<TriggerDefinition>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueDuration")]
  value_duration: Option<Duration>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueRatio")]
  value_ratio: Option<Ratio>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueContributor")]
  value_contributor: Option<Contributor>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueBoolean")]
  value_boolean: Option<bool>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueId")]
  value_id: Option<String>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueTime")]
  value_time: Option<String>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueBase64Binary")]
  value_base_6_4_binary: Option<String>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueString")]
  value_string: Option<String>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueCoding")]
  value_coding: Option<Coding>,

  /// Extensions for valueUri
  #[serde(rename = "_valueUri")]
  _value_uri: Option<Element>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueUnsignedInt")]
  value_unsigned_int: Option<i32>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueUrl")]
  value_url: Option<String>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueSignature")]
  value_signature: Option<Signature>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for valueCanonical
  #[serde(rename = "_valueCanonical")]
  _value_canonical: Option<Element>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueDateTime")]
  value_date_time: Option<String>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueDecimal")]
  value_decimal: Option<i32>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueMarkdown")]
  value_markdown: Option<String>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueRelatedArtifact")]
  value_related_artifact: Option<RelatedArtifact>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueUsageContext")]
  value_usage_context: Option<UsageContext>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueUri")]
  value_uri: Option<String>,

  /// Extensions for valueDateTime
  #[serde(rename = "_valueDateTime")]
  _value_date_time: Option<Element>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueReference")]
  value_reference: Option<Box<Reference>>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueInteger")]
  value_integer: Option<i32>,

  /// Extensions for valueOid
  #[serde(rename = "_valueOid")]
  _value_oid: Option<Element>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueDataRequirement")]
  value_data_requirement: Option<DataRequirement>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueOid")]
  value_oid: Option<String>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueTiming")]
  value_timing: Option<Timing>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueMeta")]
  value_meta: Option<Meta>,

  /// Extensions for valueTime
  #[serde(rename = "_valueTime")]
  _value_time: Option<Element>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueIdentifier")]
  value_identifier: Option<Identifier>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueExpression")]
  value_expression: Option<Expression>,

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

  /// Extensions for valueUrl
  #[serde(rename = "_valueUrl")]
  _value_url: Option<Element>,

  /// A code or description indicating how the input is intended to be used as part of
  /// the task execution.
  #[serde(rename = "type")]
  fhir_type: CodeableConcept,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueAge")]
  value_age: Option<Age>,

  /// Extensions for valueDate
  #[serde(rename = "_valueDate")]
  _value_date: Option<Element>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueRange")]
  value_range: Option<Range>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueDistance")]
  value_distance: Option<Distance>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueQuantity")]
  value_quantity: Option<Quantity>,

  /// Extensions for valueBase64Binary
  #[serde(rename = "_valueBase64Binary")]
  _value_base_6_4_binary: Option<Element>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueSampledData")]
  value_sampled_data: Option<SampledData>,

  /// Extensions for valueUuid
  #[serde(rename = "_valueUuid")]
  _value_uuid: Option<Element>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueMoney")]
  value_money: Option<Money>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valuePeriod")]
  value_period: Option<Period>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueContactPoint")]
  value_contact_point: Option<ContactPoint>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueHumanName")]
  value_human_name: Option<HumanName>,

  /// Extensions for valueId
  #[serde(rename = "_valueId")]
  _value_id: Option<Element>,

  /// Extensions for valueInstant
  #[serde(rename = "_valueInstant")]
  _value_instant: Option<Element>,

  /// Extensions for valueInteger
  #[serde(rename = "_valueInteger")]
  _value_integer: Option<Element>,

  /// Extensions for valueMarkdown
  #[serde(rename = "_valueMarkdown")]
  _value_markdown: Option<Element>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueUuid")]
  value_uuid: Option<String>,

  /// Extensions for valueDecimal
  #[serde(rename = "_valueDecimal")]
  _value_decimal: Option<Element>,

}
