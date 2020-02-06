#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Quantity::Quantity;
use crate::model::Element::Element;
use crate::model::Attachment::Attachment;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;


/// The marketing status describes the date when a medicinal product is actually put
/// on the market or the date as of which it is no longer available.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProdCharacteristic {
  /// Where applicable, the height can be specified using a numerical value and its
  /// unit of measurement The unit of measurement shall be specified in accordance
  /// with ISO 11240 and the resulting terminology The symbol and the symbol
  /// identifier shall be used.
  height: Option<Quantity>,

  /// Where applicable, the depth can be specified using a numerical value and its
  /// unit of measurement The unit of measurement shall be specified in accordance
  /// with ISO 11240 and the resulting terminology The symbol and the symbol
  /// identifier shall be used.
  depth: Option<Quantity>,

  /// Where applicable, the nominal volume can be specified using a numerical value
  /// and its unit of measurement The unit of measurement shall be specified in
  /// accordance with ISO 11240 and the resulting terminology The symbol and the
  /// symbol identifier shall be used.
  #[serde(rename = "nominalVolume")]
  nominal_volume: Option<Quantity>,

  /// Extensions for shape
  #[serde(rename = "_shape")]
  _shape: Option<Element>,

  /// Where applicable, the shape can be specified An appropriate controlled
  /// vocabulary shall be used The term and the term identifier shall be used.
  shape: Option<String>,

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

  /// Where applicable, the imprint can be specified as text.
  imprint: Option<Vec<String>>,

  /// Where applicable, the weight can be specified using a numerical value and its
  /// unit of measurement The unit of measurement shall be specified in accordance
  /// with ISO 11240 and the resulting terminology The symbol and the symbol
  /// identifier shall be used.
  weight: Option<Quantity>,

  /// Where applicable, the color can be specified An appropriate controlled
  /// vocabulary shall be used The term and the term identifier shall be used.
  color: Option<Vec<String>>,

  /// Extensions for imprint
  #[serde(rename = "_imprint")]
  _imprint: Option<Vec<Element>>,

  /// Where applicable, the scoring can be specified An appropriate controlled
  /// vocabulary shall be used The term and the term identifier shall be used.
  scoring: Option<CodeableConcept>,

  /// Where applicable, the width can be specified using a numerical value and its
  /// unit of measurement The unit of measurement shall be specified in accordance
  /// with ISO 11240 and the resulting terminology The symbol and the symbol
  /// identifier shall be used.
  width: Option<Quantity>,

  /// Extensions for color
  #[serde(rename = "_color")]
  _color: Option<Vec<Element>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Where applicable, the external diameter can be specified using a numerical value
  /// and its unit of measurement The unit of measurement shall be specified in
  /// accordance with ISO 11240 and the resulting terminology The symbol and the
  /// symbol identifier shall be used.
  #[serde(rename = "externalDiameter")]
  external_diameter: Option<Quantity>,

  /// Where applicable, the image can be provided The format of the image attachment
  /// shall be specified by regional implementations.
  image: Option<Vec<Attachment>>,

}
