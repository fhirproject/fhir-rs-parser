#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Range::Range;
use crate::model::Duration::Duration;
use crate::model::Annotation::Annotation;
use crate::model::Timing::Timing;
use crate::model::Ratio::Ratio;
use crate::model::Identifier::Identifier;
use crate::model::Distance::Distance;
use crate::model::ParameterDefinition::ParameterDefinition;
use crate::model::Signature::Signature;
use crate::model::Coding::Coding;
use crate::model::ContactDetail::ContactDetail;
use crate::model::DataRequirement::DataRequirement;
use crate::model::HumanName::HumanName;
use crate::model::Meta::Meta;
use crate::model::Money::Money;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Dosage::Dosage;
use crate::model::Period::Period;
use crate::model::Count::Count;
use crate::model::TriggerDefinition::TriggerDefinition;
use crate::model::Attachment::Attachment;
use crate::model::SampledData::SampledData;
use crate::model::Age::Age;
use crate::model::Contributor::Contributor;
use crate::model::RelatedArtifact::RelatedArtifact;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Address::Address;
use crate::model::Reference::Reference;
use crate::model::Quantity::Quantity;
use crate::model::Expression::Expression;
use crate::model::UsageContext::UsageContext;


/// Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementDefinition_Example {
  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueId")]
  value_id: Option<String>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueDateTime")]
  value_date_time: Option<String>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueSignature")]
  value_signature: Option<Signature>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueDosage")]
  value_dosage: Option<Dosage>,

  /// Extensions for valueTime
  #[serde(rename = "_valueTime")]
  _value_time: Option<Element>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueCodeableConcept")]
  value_codeable_concept: Option<CodeableConcept>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueMoney")]
  value_money: Option<Money>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueCoding")]
  value_coding: Option<Coding>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueContactPoint")]
  value_contact_point: Option<ContactPoint>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueUnsignedInt")]
  value_unsigned_int: Option<i32>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueUri")]
  value_uri: Option<String>,

  /// Extensions for valueUnsignedInt
  #[serde(rename = "_valueUnsignedInt")]
  _value_unsigned_int: Option<Element>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueAddress")]
  value_address: Option<Address>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valuePeriod")]
  value_period: Option<Period>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueExpression")]
  value_expression: Option<Expression>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueParameterDefinition")]
  value_parameter_definition: Option<ParameterDefinition>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueRelatedArtifact")]
  value_related_artifact: Option<RelatedArtifact>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueUsageContext")]
  value_usage_context: Option<UsageContext>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueUuid")]
  value_uuid: Option<String>,

  /// Extensions for valueString
  #[serde(rename = "_valueString")]
  _value_string: Option<Element>,

  /// Extensions for label
  #[serde(rename = "_label")]
  _label: Option<Element>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueInteger")]
  value_integer: Option<i32>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueOid")]
  value_oid: Option<String>,

  /// Extensions for valueOid
  #[serde(rename = "_valueOid")]
  _value_oid: Option<Element>,

  /// Extensions for valueMarkdown
  #[serde(rename = "_valueMarkdown")]
  _value_markdown: Option<Element>,

  /// Extensions for valueInstant
  #[serde(rename = "_valueInstant")]
  _value_instant: Option<Element>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueHumanName")]
  value_human_name: Option<HumanName>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueTriggerDefinition")]
  value_trigger_definition: Option<TriggerDefinition>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueContributor")]
  value_contributor: Option<Contributor>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueDuration")]
  value_duration: Option<Duration>,

  /// Extensions for valueDecimal
  #[serde(rename = "_valueDecimal")]
  _value_decimal: Option<Element>,

  /// Extensions for valueBoolean
  #[serde(rename = "_valueBoolean")]
  _value_boolean: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueReference")]
  value_reference: Option<Box<Reference>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Extensions for valueDateTime
  #[serde(rename = "_valueDateTime")]
  _value_date_time: Option<Element>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueInstant")]
  value_instant: Option<String>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueAge")]
  value_age: Option<Age>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueRatio")]
  value_ratio: Option<Ratio>,

  /// Extensions for valueCode
  #[serde(rename = "_valueCode")]
  _value_code: Option<Element>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueQuantity")]
  value_quantity: Option<Quantity>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueSampledData")]
  value_sampled_data: Option<SampledData>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueString")]
  value_string: Option<String>,

  /// Extensions for valuePositiveInt
  #[serde(rename = "_valuePositiveInt")]
  _value_positive_int: Option<Element>,

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

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueDecimal")]
  value_decimal: Option<i32>,

  /// Extensions for valueUuid
  #[serde(rename = "_valueUuid")]
  _value_uuid: Option<Element>,

  /// Extensions for valueInteger
  #[serde(rename = "_valueInteger")]
  _value_integer: Option<Element>,

  /// Extensions for valueCanonical
  #[serde(rename = "_valueCanonical")]
  _value_canonical: Option<Element>,

  /// Extensions for valueId
  #[serde(rename = "_valueId")]
  _value_id: Option<Element>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueMarkdown")]
  value_markdown: Option<String>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueCanonical")]
  value_canonical: Option<String>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueCode")]
  value_code: Option<String>,

  /// Extensions for valueUrl
  #[serde(rename = "_valueUrl")]
  _value_url: Option<Element>,

  /// Extensions for valueUri
  #[serde(rename = "_valueUri")]
  _value_uri: Option<Element>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueAnnotation")]
  value_annotation: Option<Annotation>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueTiming")]
  value_timing: Option<Timing>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueContactDetail")]
  value_contact_detail: Option<ContactDetail>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueDataRequirement")]
  value_data_requirement: Option<DataRequirement>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueTime")]
  value_time: Option<String>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueCount")]
  value_count: Option<Count>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueRange")]
  value_range: Option<Range>,

  /// Extensions for valueDate
  #[serde(rename = "_valueDate")]
  _value_date: Option<Element>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueMeta")]
  value_meta: Option<Meta>,

  /// Extensions for valueBase64Binary
  #[serde(rename = "_valueBase64Binary")]
  _value_base_6_4_binary: Option<Element>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueBase64Binary")]
  value_base_6_4_binary: Option<String>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueIdentifier")]
  value_identifier: Option<Identifier>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valuePositiveInt")]
  value_positive_int: Option<i32>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueAttachment")]
  value_attachment: Option<Attachment>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueDate")]
  value_date: Option<String>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueDistance")]
  value_distance: Option<Distance>,

  /// Describes the purpose of this example amoung the set of examples.
  label: Option<String>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueBoolean")]
  value_boolean: Option<bool>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueUrl")]
  value_url: Option<String>,

}
