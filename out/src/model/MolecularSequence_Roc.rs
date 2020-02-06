#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Extension::Extension;


/// Raw data describing a biological sequence.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequence_Roc {
  /// The number of false positives if the GQ score threshold was set to "score" field
  /// value.
  #[serde(rename = "numFP")]
  num_f_p: Vec<i32>,

  /// The number of true positives if the GQ score threshold was set to "score" field
  /// value.
  #[serde(rename = "numTP")]
  num_t_p: Vec<i32>,

  /// Calculated precision if the GQ score threshold was set to "score" field value.
  precision: Vec<f32>,

  /// Extensions for sensitivity
  _sensitivity: Vec<Element>,

  /// Extensions for precision
  _precision: Vec<Element>,

  /// Extensions for numTP
  #[serde(rename = "_numTP")]
  _num_t_p: Vec<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

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

  /// Calculated fScore if the GQ score threshold was set to "score" field value.
  #[serde(rename = "fMeasure")]
  f_measure: Vec<f32>,

  /// Extensions for fMeasure
  #[serde(rename = "_fMeasure")]
  _f_measure: Vec<Element>,

  /// Invidual data point representing the GQ (genotype quality) score threshold.
  score: Vec<i32>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Calculated sensitivity if the GQ score threshold was set to "score" field value.
  sensitivity: Vec<f32>,

  /// Extensions for score
  _score: Vec<Element>,

  /// Extensions for numFN
  #[serde(rename = "_numFN")]
  _num_f_n: Vec<Element>,

  /// Extensions for numFP
  #[serde(rename = "_numFP")]
  _num_f_p: Vec<Element>,

  /// The number of false negatives if the GQ score threshold was set to "score" field
  /// value.
  #[serde(rename = "numFN")]
  num_f_n: Vec<i32>,

}
