#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Reference::Reference;
use crate::model::Timing::Timing;
use crate::model::Annotation::Annotation;
use crate::model::Meta::Meta;
use crate::model::Quantity::Quantity;
use crate::model::Range::Range;
use crate::model::SampledData::SampledData;
use crate::model::Address::Address;
use crate::model::Contributor::Contributor;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Distance::Distance;
use crate::model::RelatedArtifact::RelatedArtifact;
use crate::model::Ratio::Ratio;
use crate::model::ParameterDefinition::ParameterDefinition;
use crate::model::ContactDetail::ContactDetail;
use crate::model::Expression::Expression;
use crate::model::Period::Period;
use crate::model::UsageContext::UsageContext;
use crate::model::DataRequirement::DataRequirement;
use crate::model::HumanName::HumanName;
use crate::model::Duration::Duration;
use crate::model::Element::Element;
use crate::model::Identifier::Identifier;
use crate::model::Signature::Signature;
use crate::model::Money::Money;
use crate::model::TriggerDefinition::TriggerDefinition;
use crate::model::Coding::Coding;
use crate::model::Count::Count;
use crate::model::Attachment::Attachment;
use crate::model::Age::Age;
use crate::model::Extension::Extension;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Dosage::Dosage;


/// A task to be performed.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task_Output {
  /// Extensions for valueMarkdown
  #[serde(rename = "_valueMarkdown")]
  _value_markdown: Element,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueUri")]
  value_uri: String,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueUsageContext")]
  value_usage_context: UsageContext,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueUnsignedInt")]
  value_unsigned_int: i32,

  /// Extensions for valueOid
  #[serde(rename = "_valueOid")]
  _value_oid: Element,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueBoolean")]
  value_boolean: bool,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueMeta")]
  value_meta: Meta,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueSignature")]
  value_signature: Signature,

  /// Extensions for valueId
  #[serde(rename = "_valueId")]
  _value_id: Element,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueCodeableConcept")]
  value_codeable_concept: CodeableConcept,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valuePositiveInt")]
  value_positive_int: i32,

  /// Extensions for valueUuid
  #[serde(rename = "_valueUuid")]
  _value_uuid: Element,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueDistance")]
  value_distance: Distance,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueMoney")]
  value_money: Money,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueString")]
  value_string: String,

  /// Extensions for valueCanonical
  #[serde(rename = "_valueCanonical")]
  _value_canonical: Element,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueSampledData")]
  value_sampled_data: SampledData,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueCount")]
  value_count: Count,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueMarkdown")]
  value_markdown: String,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueAttachment")]
  value_attachment: Attachment,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueAge")]
  value_age: Age,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueReference")]
  value_reference: Box<Reference>,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueIdentifier")]
  value_identifier: Identifier,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for valueBoolean
  #[serde(rename = "_valueBoolean")]
  _value_boolean: Element,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueQuantity")]
  value_quantity: Quantity,

  /// The name of the Output parameter.
  #[serde(rename = "type")]
  fhir_type: CodeableConcept,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueCode")]
  value_code: String,

  /// Extensions for valueInteger
  #[serde(rename = "_valueInteger")]
  _value_integer: Element,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueInstant")]
  value_instant: String,

  /// Extensions for valueString
  #[serde(rename = "_valueString")]
  _value_string: Element,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueAddress")]
  value_address: Address,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueDateTime")]
  value_date_time: String,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueRelatedArtifact")]
  value_related_artifact: RelatedArtifact,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueBase64Binary")]
  value_base_6_4_binary: String,

  /// Extensions for valuePositiveInt
  #[serde(rename = "_valuePositiveInt")]
  _value_positive_int: Element,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueExpression")]
  value_expression: Expression,

  /// Extensions for valueDate
  #[serde(rename = "_valueDate")]
  _value_date: Element,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueContactDetail")]
  value_contact_detail: ContactDetail,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueDuration")]
  value_duration: Duration,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueContributor")]
  value_contributor: Contributor,

  /// Extensions for valueTime
  #[serde(rename = "_valueTime")]
  _value_time: Element,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valuePeriod")]
  value_period: Period,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueUuid")]
  value_uuid: String,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueAnnotation")]
  value_annotation: Annotation,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueId")]
  value_id: String,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueParameterDefinition")]
  value_parameter_definition: ParameterDefinition,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueCanonical")]
  value_canonical: String,

  /// Extensions for valueUnsignedInt
  #[serde(rename = "_valueUnsignedInt")]
  _value_unsigned_int: Element,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueContactPoint")]
  value_contact_point: ContactPoint,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueOid")]
  value_oid: String,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueRange")]
  value_range: Range,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueTime")]
  value_time: String,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueInteger")]
  value_integer: i32,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueDataRequirement")]
  value_data_requirement: DataRequirement,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueUrl")]
  value_url: String,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueDecimal")]
  value_decimal: i32,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueTiming")]
  value_timing: Timing,

  /// Extensions for valueCode
  #[serde(rename = "_valueCode")]
  _value_code: Element,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueHumanName")]
  value_human_name: HumanName,

  /// Extensions for valueBase64Binary
  #[serde(rename = "_valueBase64Binary")]
  _value_base_6_4_binary: Element,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueCoding")]
  value_coding: Coding,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueDate")]
  value_date: String,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueDosage")]
  value_dosage: Dosage,

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueTriggerDefinition")]
  value_trigger_definition: TriggerDefinition,

  /// Extensions for valueDecimal
  #[serde(rename = "_valueDecimal")]
  _value_decimal: Element,

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

  /// The value of the Output parameter as a basic type.
  #[serde(rename = "valueRatio")]
  value_ratio: Ratio,

  /// Extensions for valueInstant
  #[serde(rename = "_valueInstant")]
  _value_instant: Element,

  /// Extensions for valueUrl
  #[serde(rename = "_valueUrl")]
  _value_url: Element,

  /// Extensions for valueDateTime
  #[serde(rename = "_valueDateTime")]
  _value_date_time: Element,

  /// Extensions for valueUri
  #[serde(rename = "_valueUri")]
  _value_uri: Element,

}
