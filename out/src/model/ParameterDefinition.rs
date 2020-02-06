#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// The parameters to the module. This collection specifies both the input and
/// output parameters. Input parameters are provided by the caller as part of the
/// $evaluate operation. Output parameters are included in the GuidanceResponse.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterDefinition {
  /// Extensions for use
  _use: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for name
  _name: Element,

  /// A brief discussion of what the parameter is for and how it is used by the
  /// module.
  documentation: String,

  /// The type of the parameter.
  #[serde(rename = "type")]
  fhir_type: String,

  /// If specified, this indicates a profile that the input data must conform to, or
  /// that the output data will conform to.
  profile: String,

  /// Extensions for min
  _min: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// The name of the parameter used to allow access to the value of the parameter in
  /// evaluation contexts.
  name: String,

  /// Extensions for type
  _type: Element,

  /// Extensions for max
  _max: Element,

  /// The maximum number of times this element is permitted to appear in the request
  /// or response.
  max: String,

  /// Extensions for documentation
  _documentation: Element,

  /// The minimum number of times this parameter SHALL appear in the request or
  /// response.
  min: i32,

  /// Whether the parameter is input or output for the module.
  #[serde(rename = "use")]
  fhir_use: String,

}
