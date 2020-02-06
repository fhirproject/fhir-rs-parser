#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Identifier::Identifier;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Substance_Ingredient::Substance_Ingredient;
use crate::model::ResourceList::ResourceList;
use crate::model::Narrative::Narrative;
use crate::model::Meta::Meta;
use crate::model::Substance_Instance::Substance_Instance;


/// A homogeneous material with a definite composition.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Substance {
  /// Unique identifier for the substance.
  identifier: Option<Vec<Identifier>>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// A code to indicate if the substance is actively used.
  status: Option<SubstanceStatus>,

  /// A code that classifies the general type of substance.  This is used  for
  /// searching, sorting and display purposes.
  category: Option<Vec<CodeableConcept>>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// A code (or set of codes) that identify this substance.
  code: CodeableConcept,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Substance may be used to describe a kind of substance, or a specific
  /// package/container of the substance: an instance.
  instance: Option<Vec<Substance_Instance>>,

  /// A description of the substance - its appearance, handling requirements, and
  /// other usage notes.
  description: Option<String>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

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
  modifier_extension: Option<Vec<Box<Extension>>>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// A substance can be composed of other substances.
  ingredient: Option<Vec<Substance_Ingredient>>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// The base language in which the resource is written.
  language: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubstanceStatus {
  #[serde(rename = "active")]
  Active,

  #[serde(rename = "inactive")]
  Inactive,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

}
