#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::MedicinalProductIngredient_ReferenceStrength::MedicinalProductIngredient_ReferenceStrength;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Ratio::Ratio;


/// An ingredient of a manufactured item or pharmaceutical product.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductIngredient_Strength {
  /// A lower limit for the strength per unitary volume (or mass), for when there is a
  /// range. The concentration attribute then becomes the upper limit.
  #[serde(rename = "concentrationLowLimit")]
  concentration_low_limit: Option<Ratio>,

  /// A lower limit for the quantity of substance in the unit of presentation. For use
  /// when there is a range of strengths, this is the lower limit, with the
  /// presentation attribute becoming the upper limit.
  #[serde(rename = "presentationLowLimit")]
  presentation_low_limit: Option<Ratio>,

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

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// For when strength is measured at a particular point or distance.
  #[serde(rename = "measurementPoint")]
  measurement_point: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// The quantity of substance in the unit of presentation, or in the volume (or
  /// mass) of the single pharmaceutical product or manufactured item.
  presentation: Ratio,

  /// The country or countries for which the strength range applies.
  country: Option<Vec<CodeableConcept>>,

  /// The strength per unitary volume (or mass).
  concentration: Option<Ratio>,

  /// Strength expressed in terms of a reference substance.
  #[serde(rename = "referenceStrength")]
  reference_strength: Option<Vec<MedicinalProductIngredient_ReferenceStrength>>,

  /// Extensions for measurementPoint
  #[serde(rename = "_measurementPoint")]
  _measurement_point: Option<Element>,

}
