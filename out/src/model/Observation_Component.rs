#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Period::Period;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Range::Range;
use crate::model::Extension::Extension;
use crate::model::Observation_ReferenceRange::Observation_ReferenceRange;
use crate::model::Element::Element;
use crate::model::Ratio::Ratio;
use crate::model::SampledData::SampledData;
use crate::model::Quantity::Quantity;


/// Measurements and simple assertions made about a patient, device or other
/// subject.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Observation_Component {
  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  #[serde(rename = "valueQuantity")]
  value_quantity: Quantity,

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  #[serde(rename = "valueDateTime")]
  value_date_time: String,

  /// Guidance on how to interpret the value by comparison to a normal or recommended
  /// range.
  #[serde(rename = "referenceRange")]
  reference_range: Vec<Observation_ReferenceRange>,

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  #[serde(rename = "valueInteger")]
  value_integer: i32,

  /// Describes what was observed. Sometimes this is called the observation "code".
  code: CodeableConcept,

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  #[serde(rename = "valueSampledData")]
  value_sampled_data: SampledData,

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  #[serde(rename = "valueTime")]
  value_time: String,

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  #[serde(rename = "valueRange")]
  value_range: Range,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for valueBoolean
  #[serde(rename = "_valueBoolean")]
  _value_boolean: Element,

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  #[serde(rename = "valueRatio")]
  value_ratio: Ratio,

  /// Extensions for valueTime
  #[serde(rename = "_valueTime")]
  _value_time: Element,

  /// Provides a reason why the expected value in the element
  /// Observation.component.value[x] is missing.
  #[serde(rename = "dataAbsentReason")]
  data_absent_reason: CodeableConcept,

  /// A categorical assessment of an observation value.  For example, high, low,
  /// normal.
  interpretation: Vec<CodeableConcept>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  #[serde(rename = "valueBoolean")]
  value_boolean: bool,

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

  /// Extensions for valueInteger
  #[serde(rename = "_valueInteger")]
  _value_integer: Element,

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  #[serde(rename = "valueCodeableConcept")]
  value_codeable_concept: CodeableConcept,

  /// Extensions for valueString
  #[serde(rename = "_valueString")]
  _value_string: Element,

  /// Extensions for valueDateTime
  #[serde(rename = "_valueDateTime")]
  _value_date_time: Element,

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  #[serde(rename = "valuePeriod")]
  value_period: Period,

  /// The information determined as a result of making the observation, if the
  /// information has a simple value.
  #[serde(rename = "valueString")]
  value_string: String,

}
