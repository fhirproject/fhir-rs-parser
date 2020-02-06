#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::ElementDefinition_Discriminator::ElementDefinition_Discriminator;


/// Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementDefinition_Slicing {
  /// If the matching elements have to occur in the same order as defined in the
  /// profile.
  ordered: Option<bool>,

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

  /// Designates which child elements are used to discriminate between the slices when
  /// processing an instance. If one or more discriminators are provided, the value of
  /// the child elements in the instance data SHALL completely distinguish which slice
  /// the element in the resource matches based on the allowed values for those
  /// elements in each of the slices.
  discriminator: Option<Vec<ElementDefinition_Discriminator>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// A human-readable text description of how the slicing works. If there is no
  /// discriminator, this is required to be present to provide whatever information is
  /// possible about how the slices can be differentiated.
  description: Option<String>,

  /// Extensions for ordered
  #[serde(rename = "_ordered")]
  _ordered: Option<Element>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// Whether additional slices are allowed or not. When the slices are ordered,
  /// profile authors can also say that additional slices are only allowed at the end.
  rules: Option<ElementDefinition_SlicingRules>,

  /// Extensions for rules
  #[serde(rename = "_rules")]
  _rules: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ElementDefinition_SlicingRules {
  #[serde(rename = "closed")]
  Closed,

  #[serde(rename = "open")]
  Open,

  #[serde(rename = "openAtEnd")]
  OpenAtEnd,

}
