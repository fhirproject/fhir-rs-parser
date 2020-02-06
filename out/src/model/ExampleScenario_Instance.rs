#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ExampleScenario_ContainedInstance::ExampleScenario_ContainedInstance;
use crate::model::Extension::Extension;
use crate::model::ExampleScenario_Version::ExampleScenario_Version;
use crate::model::Element::Element;


/// Example of workflow instance.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenario_Instance {
  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// Extensions for resourceId
  #[serde(rename = "_resourceId")]
  _resource_id: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The type of the resource.
  #[serde(rename = "resourceType")]
  resource_type: Option<String>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// Extensions for resourceType
  #[serde(rename = "_resourceType")]
  _resource_type: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Resources contained in the instance (e.g. the observations contained in a
  /// bundle).
  #[serde(rename = "containedInstance")]
  contained_instance: Option<Vec<ExampleScenario_ContainedInstance>>,

  /// Human-friendly description of the resource instance.
  description: Option<String>,

  /// The id of the resource for referencing.
  #[serde(rename = "resourceId")]
  resource_id: Option<String>,

  /// A short name for the resource instance.
  name: Option<String>,

  /// A specific version of the resource.
  version: Option<Vec<ExampleScenario_Version>>,

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
  modifier_extension: Option<Vec<Box<Extension>>>,

}
