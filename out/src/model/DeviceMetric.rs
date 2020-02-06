#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Narrative::Narrative;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::ResourceList::ResourceList;
use crate::model::Timing::Timing;
use crate::model::Identifier::Identifier;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::Reference::Reference;
use crate::model::DeviceMetric_Calibration::DeviceMetric_Calibration;


/// Describes a measurement, calculation or setting capability of a medical device.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceMetric {
  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// Describes the measurement repetition time. This is not necessarily the same as
  /// the update period. The measurement repetition time can range from milliseconds
  /// up to hours. An example for a measurement repetition time in the range of
  /// milliseconds is the sampling rate of an ECG. An example for a measurement
  /// repetition time in the range of hours is a NIBP that is triggered automatically
  /// every hour. The update period may be different than the measurement repetition
  /// time, if the device does not update the published observed value with the same
  /// frequency as it was measured.
  #[serde(rename = "measurementPeriod")]
  measurement_period: Option<Timing>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Describes the type of the metric. For example: Heart Rate, PEEP Setting, etc.
  #[serde(rename = "type")]
  fhir_type: CodeableConcept,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Describes the link to the  Device that this DeviceMetric belongs to and that
  /// provide information about the location of this DeviceMetric in the containment
  /// structure of the parent Device. An example would be a Device that represents a
  /// Channel. This reference can be used by a client application to distinguish
  /// DeviceMetrics that have the same type, but should be interpreted based on their
  /// containment location.
  parent: Option<Box<Reference>>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Describes the unit that an observed value determined for this metric will have.
  /// For example: Percent, Seconds, etc.
  unit: Option<CodeableConcept>,

  /// Indicates current operational state of the device. For example: On, Off,
  /// Standby, etc.
  #[serde(rename = "operationalStatus")]
  operational_status: Option<DeviceMetricOperationalStatus>,

  /// Extensions for category
  #[serde(rename = "_category")]
  _category: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.    Modifier
  /// extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Option<Vec<Box<Extension>>>,

  /// Extensions for color
  #[serde(rename = "_color")]
  _color: Option<Element>,

  /// Describes the calibrations that have been performed or that are required to be
  /// performed.
  calibration: Option<Vec<DeviceMetric_Calibration>>,

  /// Indicates the category of the observation generation process. A DeviceMetric can
  /// be for example a setting, measurement, or calculation.
  category: Option<DeviceMetricCategory>,

  /// Describes the link to the  Device that this DeviceMetric belongs to and that
  /// contains administrative device information such as manufacturer, serial number,
  /// etc.
  source: Option<Box<Reference>>,

  /// Unique instance identifiers assigned to a device by the device or gateway
  /// software, manufacturers, other organizations or owners. For example: handle ID.
  identifier: Option<Vec<Identifier>>,

  /// Extensions for operationalStatus
  #[serde(rename = "_operationalStatus")]
  _operational_status: Option<Element>,

  /// Describes the color representation for the metric. This is often used to aid
  /// clinicians to track and identify parameter types by color. In practice, consider
  /// a Patient Monitor that has ECG/HR and Pleth for example; the parameters are
  /// displayed in different characteristic colors, such as HR-blue, BP-green, and PR
  /// and SpO2- magenta.
  color: Option<DeviceMetricColor>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum DeviceMetricOperationalStatus {
  #[serde(rename = "on")]
  On,

  #[serde(rename = "off")]
  Off,

  #[serde(rename = "standby")]
  Standby,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum DeviceMetricCategory {
  #[serde(rename = "measurement")]
  Measurement,

  #[serde(rename = "setting")]
  Setting,

  #[serde(rename = "calculation")]
  Calculation,

  #[serde(rename = "unspecified")]
  Unspecified,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum DeviceMetricColor {
  #[serde(rename = "black")]
  Black,

  #[serde(rename = "red")]
  Red,

  #[serde(rename = "green")]
  Green,

  #[serde(rename = "yellow")]
  Yellow,

  #[serde(rename = "blue")]
  Blue,

  #[serde(rename = "magenta")]
  Magenta,

  #[serde(rename = "cyan")]
  Cyan,

  #[serde(rename = "white")]
  White,

}
