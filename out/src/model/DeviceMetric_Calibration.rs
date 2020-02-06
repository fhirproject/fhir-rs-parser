#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;


/// Describes a measurement, calculation or setting capability of a medical device.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceMetric_Calibration {
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

  /// Describes the time last calibration has been performed.
  time: Option<String>,

  /// Extensions for time
  #[serde(rename = "_time")]
  _time: Option<Element>,

  /// Extensions for type
  #[serde(rename = "_type")]
  _type: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Describes the state of the calibration.
  state: Option<DeviceMetric_CalibrationState>,

  /// Extensions for state
  #[serde(rename = "_state")]
  _state: Option<Element>,

  /// Describes the type of the calibration method.
  #[serde(rename = "type")]
  fhir_type: Option<DeviceMetric_CalibrationType>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum DeviceMetric_CalibrationState {
  #[serde(rename = "not-calibrated")]
  NotCalibrated,

  #[serde(rename = "calibration-required")]
  CalibrationRequired,

  #[serde(rename = "calibrated")]
  Calibrated,

  #[serde(rename = "unspecified")]
  Unspecified,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum DeviceMetric_CalibrationType {
  #[serde(rename = "unspecified")]
  Unspecified,

  #[serde(rename = "offset")]
  Offset,

  #[serde(rename = "gain")]
  Gain,

  #[serde(rename = "two-point")]
  TwoPoint,

}
