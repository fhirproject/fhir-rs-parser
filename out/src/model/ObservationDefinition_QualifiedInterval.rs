#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Range::Range;


/// Set of definitional characteristics for a kind of observation or measurement
/// produced or consumed by an orderable health care service.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservationDefinition_QualifiedInterval {
  /// Extensions for gender
  #[serde(rename = "_gender")]
  _gender: Option<Element>,

  /// The gestational age to which this reference range is applicable, in the context
  /// of pregnancy.
  #[serde(rename = "gestationalAge")]
  gestational_age: Option<Range>,

  /// The category of interval of values for continuous or ordinal observations
  /// conforming to this ObservationDefinition.
  category: Option<ObservationDefinition_QualifiedIntervalCategory>,

  /// Text based condition for which the reference range is valid.
  condition: Option<String>,

  /// Extensions for condition
  #[serde(rename = "_condition")]
  _condition: Option<Element>,

  /// Sex of the population the range applies to.
  gender: Option<ObservationDefinition_QualifiedIntervalGender>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

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

  /// Codes to indicate the target population this reference range applies to.
  #[serde(rename = "appliesTo")]
  applies_to: Option<Vec<CodeableConcept>>,

  /// The age at which this reference range is applicable. This is a neonatal age
  /// (e.g. number of weeks at term) if the meaning says so.
  age: Option<Range>,

  /// Extensions for category
  #[serde(rename = "_category")]
  _category: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// The low and high values determining the interval. There may be only one of the
  /// two.
  range: Option<Range>,

  /// Codes to indicate the health context the range applies to. For example, the
  /// normal or therapeutic range.
  context: Option<CodeableConcept>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ObservationDefinition_QualifiedIntervalCategory {
  #[serde(rename = "reference")]
  Reference,

  #[serde(rename = "critical")]
  Critical,

  #[serde(rename = "absolute")]
  Absolute,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ObservationDefinition_QualifiedIntervalGender {
  #[serde(rename = "male")]
  Male,

  #[serde(rename = "female")]
  Female,

  #[serde(rename = "other")]
  Other,

  #[serde(rename = "unknown")]
  Unknown,

}
