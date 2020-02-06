#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Identifier::Identifier;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::NutritionOrder_EnteralFormula::NutritionOrder_EnteralFormula;
use crate::model::NutritionOrder_Supplement::NutritionOrder_Supplement;
use crate::model::Reference::Reference;
use crate::model::NutritionOrder_OralDiet::NutritionOrder_OralDiet;
use crate::model::Narrative::Narrative;
use crate::model::Meta::Meta;
use crate::model::Annotation::Annotation;
use crate::model::Element::Element;
use crate::model::ResourceList::ResourceList;
use crate::model::Extension::Extension;


/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrder {
  /// This modifier is used to convey order-specific modifiers about the type of food
  /// that should be given. These can be derived from patient allergies, intolerances,
  /// or preferences such as Halal, Vegan or Kosher. This modifier applies to the
  /// entire nutrition order inclusive of the oral diet, nutritional supplements and
  /// enteral formula feedings.
  #[serde(rename = "foodPreferenceModifier")]
  food_preference_modifier: Option<Vec<CodeableConcept>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for instantiates
  #[serde(rename = "_instantiates")]
  _instantiates: Option<Vec<Element>>,

  /// Feeding provided through the gastrointestinal tract via a tube, catheter, or
  /// stoma that delivers nutrition distal to the oral cavity.
  #[serde(rename = "enteralFormula")]
  enteral_formula: Option<NutritionOrder_EnteralFormula>,

  /// A link to a record of allergies or intolerances  which should be included in the
  /// nutrition order.
  #[serde(rename = "allergyIntolerance")]
  allergy_intolerance: Option<Vec<Box<Reference>>>,

  /// The date and time that this nutrition order was requested.
  #[serde(rename = "dateTime")]
  date_time: Option<String>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Extensions for dateTime
  #[serde(rename = "_dateTime")]
  _date_time: Option<Element>,

  /// This modifier is used to convey Order-specific modifier about the type of oral
  /// food or oral fluids that should not be given. These can be derived from patient
  /// allergies, intolerances, or preferences such as No Red Meat, No Soy or No Wheat
  /// or  Gluten-Free.  While it should not be necessary to repeat allergy or
  /// intolerance information captured in the referenced AllergyIntolerance resource
  /// in the excludeFoodModifier, this element may be used to convey additional
  /// specificity related to foods that should be eliminated from the patient’s diet
  /// for any reason.  This modifier applies to the entire nutrition order inclusive
  /// of the oral diet, nutritional supplements and enteral formula feedings.
  #[serde(rename = "excludeFoodModifier")]
  exclude_food_modifier: Option<Vec<CodeableConcept>>,

  /// An encounter that provides additional information about the healthcare context
  /// in which this request is made.
  encounter: Option<Box<Reference>>,

  /// Diet given orally in contrast to enteral (tube) feeding.
  #[serde(rename = "oralDiet")]
  oral_diet: Option<NutritionOrder_OralDiet>,

  /// Identifiers assigned to this order by the order sender or by the order receiver.
  identifier: Option<Vec<Identifier>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// The URL pointing to a FHIR-defined protocol, guideline, orderset or other
  /// definition that is adhered to in whole or in part by this NutritionOrder.
  #[serde(rename = "instantiatesCanonical")]
  instantiates_canonical: Option<Vec<String>>,

  /// Comments made about the {{title}} by the requester, performer, subject or other
  /// participants.
  note: Option<Vec<Annotation>>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// The URL pointing to a protocol, guideline, orderset or other definition that is
  /// adhered to in whole or in part by this NutritionOrder.
  instantiates: Option<Vec<String>>,

  /// Extensions for intent
  #[serde(rename = "_intent")]
  _intent: Option<Element>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// The workflow status of the nutrition order/request.
  status: Option<String>,

  /// The URL pointing to an externally maintained protocol, guideline, orderset or
  /// other definition that is adhered to in whole or in part by this NutritionOrder.
  #[serde(rename = "instantiatesUri")]
  instantiates_uri: Option<Vec<String>>,

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

  /// The practitioner that holds legal responsibility for ordering the diet,
  /// nutritional supplement, or formula feedings.
  orderer: Option<Box<Reference>>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// The person (patient) who needs the nutrition order for an oral diet, nutritional
  /// supplement and/or enteral or formula feeding.
  patient: Box<Reference>,

  /// Oral nutritional products given in order to add further nutritional value to the
  /// patient's diet.
  supplement: Option<Vec<NutritionOrder_Supplement>>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// Indicates the level of authority/intentionality associated with the NutrionOrder
  /// and where the request fits into the workflow chain.
  intent: Option<String>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Extensions for instantiatesUri
  #[serde(rename = "_instantiatesUri")]
  _instantiates_uri: Option<Vec<Element>>,

}
