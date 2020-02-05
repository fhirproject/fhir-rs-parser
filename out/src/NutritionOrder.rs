use serde::{Deserialize, Serialize};

/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NutritionOrder {
  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for instantiatesUri
  #[serde(rename = "_instantiatesUri")]
  _instantiates_uri: Vec<Element>,

  /// Feeding provided through the gastrointestinal tract via a tube, catheter, or
  /// stoma that delivers nutrition distal to the oral cavity.
  #[serde(rename = "enteralFormula")]
  enteral_formula: NutritionOrder_EnteralFormula,

  /// Diet given orally in contrast to enteral (tube) feeding.
  #[serde(rename = "oralDiet")]
  oral_diet: NutritionOrder_OralDiet,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// The workflow status of the nutrition order/request.
  status: String,

  /// The date and time that this nutrition order was requested.
  #[serde(rename = "dateTime")]
  date_time: dateTime,

  /// The URL pointing to a FHIR-defined protocol, guideline, orderset or other
  /// definition that is adhered to in whole or in part by this NutritionOrder.
  #[serde(rename = "instantiatesCanonical")]
  instantiates_canonical: Vec<canonical>,

  /// The URL pointing to a protocol, guideline, orderset or other definition that is
  /// adhered to in whole or in part by this NutritionOrder.
  instantiates: Vec<String>,

  /// Extensions for dateTime
  #[serde(rename = "_dateTime")]
  _date_time: Element,

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
  exclude_food_modifier: Vec<CodeableConcept>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// A link to a record of allergies or intolerances  which should be included in the
  /// nutrition order.
  #[serde(rename = "allergyIntolerance")]
  allergy_intolerance: Vec<Reference>,

  /// Extensions for language
  _language: Element,

  /// Extensions for intent
  _intent: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// The URL pointing to an externally maintained protocol, guideline, orderset or
  /// other definition that is adhered to in whole or in part by this NutritionOrder.
  #[serde(rename = "instantiatesUri")]
  instantiates_uri: Vec<String>,

  /// An encounter that provides additional information about the healthcare context
  /// in which this request is made.
  encounter: Reference,

  /// Extensions for status
  _status: Element,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Oral nutritional products given in order to add further nutritional value to the
  /// patient's diet.
  supplement: Vec<NutritionOrder_Supplement>,

  /// Identifiers assigned to this order by the order sender or by the order receiver.
  identifier: Vec<Identifier>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// The base language in which the resource is written.
  language: String,

  /// Indicates the level of authority/intentionality associated with the NutrionOrder
  /// and where the request fits into the workflow chain.
  intent: String,

  /// The person (patient) who needs the nutrition order for an oral diet, nutritional
  /// supplement and/or enteral or formula feeding.
  patient: Reference,

  /// Extensions for instantiates
  _instantiates: Vec<Element>,

  /// The practitioner that holds legal responsibility for ordering the diet,
  /// nutritional supplement, or formula feedings.
  orderer: Reference,

  /// This modifier is used to convey order-specific modifiers about the type of food
  /// that should be given. These can be derived from patient allergies, intolerances,
  /// or preferences such as Halal, Vegan or Kosher. This modifier applies to the
  /// entire nutrition order inclusive of the oral diet, nutritional supplements and
  /// enteral formula feedings.
  #[serde(rename = "foodPreferenceModifier")]
  food_preference_modifier: Vec<CodeableConcept>,

  /// Comments made about the {{title}} by the requester, performer, subject or other
  /// participants.
  note: Vec<Annotation>,

}
