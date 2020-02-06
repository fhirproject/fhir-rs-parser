#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;


/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMap_DependsOn {
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

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for property
  #[serde(rename = "_property")]
  _property: Option<Element>,

  /// A reference to an element that holds a coded value that corresponds to a code
  /// system property. The idea is that the information model carries an element
  /// somewhere that is labeled to correspond with a code system property.
  property: Option<String>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The display for the code. The display is only provided to help editors when
  /// editing the concept map.
  display: Option<String>,

  /// Extensions for value
  #[serde(rename = "_value")]
  _value: Option<Element>,

  /// Extensions for display
  #[serde(rename = "_display")]
  _display: Option<Element>,

  /// An absolute URI that identifies the code system of the dependency code (if the
  /// source/dependency is a value set that crosses code systems).
  system: Option<String>,

  /// Identity (code or path) or the element/item/ValueSet/text that the map depends
  /// on / refers to.
  value: Option<String>,

}
