#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::MedicinalProductIngredient_Substance::MedicinalProductIngredient_Substance;
use crate::model::Element::Element;
use crate::model::Meta::Meta;
use crate::model::ResourceList::ResourceList;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::MedicinalProductIngredient_SpecifiedSubstance::MedicinalProductIngredient_SpecifiedSubstance;
use crate::model::Reference::Reference;
use crate::model::Identifier::Identifier;
use crate::model::Narrative::Narrative;


/// An ingredient of a manufactured item or pharmaceutical product.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductIngredient {
  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// The base language in which the resource is written.
  language: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.    Modifier
  /// extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The identifier(s) of this Ingredient that are assigned by business processes
  /// and/or used to refer to it when a direct URL reference to the resource itself is
  /// not appropriate.
  identifier: Identifier,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// If the ingredient is a known or suspected allergen.
  #[serde(rename = "allergenicIndicator")]
  allergenic_indicator: bool,

  /// Extensions for language
  _language: Element,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Extensions for allergenicIndicator
  #[serde(rename = "_allergenicIndicator")]
  _allergenic_indicator: Element,

  /// A specified substance that comprises this ingredient.
  #[serde(rename = "specifiedSubstance")]
  specified_substance: Vec<MedicinalProductIngredient_SpecifiedSubstance>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// The ingredient substance.
  substance: MedicinalProductIngredient_Substance,

  /// Manufacturer of this Ingredient.
  manufacturer: Vec<Box<Reference>>,

  /// Ingredient role e.g. Active ingredient, excipient.
  role: CodeableConcept,

}
