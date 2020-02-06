#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Period::Period;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::Quantity::Quantity;
use crate::model::Element::Element;
use crate::model::Range::Range;
use crate::model::CodeableConcept::CodeableConcept;


/// Represents a defined collection of entities that may be discussed or acted upon
/// collectively but which are not expected to act collectively, and are not
/// formally or legally recognized; i.e. a collection of entities that isn't an
/// Organization.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group_Characteristic {
  /// Extensions for valueBoolean
  #[serde(rename = "_valueBoolean")]
  _value_boolean: Option<Element>,

  /// The period over which the characteristic is tested; e.g. the patient had an
  /// operation during the month of June.
  period: Option<Period>,

  /// Extensions for exclude
  #[serde(rename = "_exclude")]
  _exclude: Option<Element>,

  /// If true, indicates the characteristic is one that is NOT held by members of the
  /// group.
  exclude: Option<bool>,

  /// A code that identifies the kind of trait being asserted.
  code: CodeableConcept,

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

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The value of the trait that holds (or does not hold - see 'exclude') for members
  /// of the group.
  #[serde(rename = "valueRange")]
  value_range: Option<Range>,

  /// The value of the trait that holds (or does not hold - see 'exclude') for members
  /// of the group.
  #[serde(rename = "valueReference")]
  value_reference: Option<Box<Reference>>,

  /// The value of the trait that holds (or does not hold - see 'exclude') for members
  /// of the group.
  #[serde(rename = "valueBoolean")]
  value_boolean: Option<bool>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The value of the trait that holds (or does not hold - see 'exclude') for members
  /// of the group.
  #[serde(rename = "valueCodeableConcept")]
  value_codeable_concept: Option<CodeableConcept>,

  /// The value of the trait that holds (or does not hold - see 'exclude') for members
  /// of the group.
  #[serde(rename = "valueQuantity")]
  value_quantity: Option<Quantity>,

}
