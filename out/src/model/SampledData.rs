#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Quantity::Quantity;
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// A series of measurements taken by a device, with upper and lower limits. There
/// may be more than one dimension in the data.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SampledData {
  /// The length of time between sampling times, measured in milliseconds.
  period: Option<f32>,

  /// A correction factor that is applied to the sampled data points before they are
  /// added to the origin.
  factor: Option<f32>,

  /// Extensions for factor
  #[serde(rename = "_factor")]
  _factor: Option<Element>,

  /// The lower limit of detection of the measured points. This is needed if any of
  /// the data points have the value "L" (lower than detection limit).
  #[serde(rename = "lowerLimit")]
  lower_limit: Option<f32>,

  /// The upper limit of detection of the measured points. This is needed if any of
  /// the data points have the value "U" (higher than detection limit).
  #[serde(rename = "upperLimit")]
  upper_limit: Option<f32>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for period
  #[serde(rename = "_period")]
  _period: Option<Element>,

  /// Extensions for upperLimit
  #[serde(rename = "_upperLimit")]
  _upper_limit: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The number of sample points at each time point. If this value is greater than
  /// one, then the dimensions will be interlaced - all the sample points for a point
  /// in time will be recorded at once.
  dimensions: Option<i32>,

  /// A series of data points which are decimal values separated by a single space
  /// (character u20). The special values "E" (error), "L" (below detection limit) and
  /// "U" (above detection limit) can also be used in place of a decimal value.
  data: Option<String>,

  /// The base quantity that a measured value of zero represents. In addition, this
  /// provides the units of the entire measurement series.
  origin: Quantity,

  /// Extensions for lowerLimit
  #[serde(rename = "_lowerLimit")]
  _lower_limit: Option<Element>,

  /// Extensions for dimensions
  #[serde(rename = "_dimensions")]
  _dimensions: Option<Element>,

  /// Extensions for data
  #[serde(rename = "_data")]
  _data: Option<Element>,

}
