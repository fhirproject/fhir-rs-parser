#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Identifier::Identifier;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;


/// A sample to be used for analysis.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Specimen_Container {
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

  /// The capacity (volume or other measure) the container may contain.
  capacity: Option<Quantity>,

  /// Id for container. There may be multiple; a manufacturer's bar code, lab assigned
  /// identifier, etc. The container ID may differ from the specimen id in some
  /// circumstances.
  identifier: Option<Vec<Identifier>>,

  /// Introduced substance to preserve, maintain or enhance the specimen. Examples:
  /// Formalin, Citrate, EDTA.
  #[serde(rename = "additiveCodeableConcept")]
  additive_codeable_concept: Option<CodeableConcept>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Introduced substance to preserve, maintain or enhance the specimen. Examples:
  /// Formalin, Citrate, EDTA.
  #[serde(rename = "additiveReference")]
  additive_reference: Option<Box<Reference>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Textual description of the container.
  description: Option<String>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// The type of container associated with the specimen (e.g. slide, aliquot, etc.).
  #[serde(rename = "type")]
  fhir_type: Option<CodeableConcept>,

  /// The quantity of specimen in the container; may be volume, dimensions, or other
  /// appropriate measurements, depending on the specimen type.
  #[serde(rename = "specimenQuantity")]
  specimen_quantity: Option<Quantity>,

}
