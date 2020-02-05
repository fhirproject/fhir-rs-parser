use serde::{Deserialize, Serialize};

/// Information about a medication that is used to support knowledge.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MedicationKnowledge {
  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// A code to indicate if the medication is in active use.  The status refers to the
  /// validity about the information of the medication and not to its medicinal
  /// properties.
  status: String,

  /// Additional names for a medication, for example, the name(s) given to a
  /// medication in different countries.  For example, acetaminophen and paracetamol
  /// or salbutamol and albuterol.
  synonym: Vec<String>,

  /// Extensions for synonym
  _synonym: Vec<Element>,

  /// Potential clinical issue with or between medication(s) (for example, drug-drug
  /// interaction, drug-disease contraindication, drug-allergy interaction, etc.).
  contraindication: Vec<Reference>,

  /// Associated or related medications.  For example, if the medication is a branded
  /// product (e.g. Crestor), this is the Therapeutic Moeity (e.g. Rosuvastatin) or if
  /// this is a generic medication (e.g. Rosuvastatin), this would link to a branded
  /// product (e.g. Crestor).
  #[serde(rename = "associatedMedication")]
  associated_medication: Vec<Reference>,

  /// Regulatory information about a medication.
  regulatory: Vec<MedicationKnowledge_Regulatory>,

  /// The base language in which the resource is written.
  language: String,

  /// A code that specifies this medication, or a textual description if no code is
  /// available. Usage note: This could be a standard medication code such as a code
  /// from RxNorm, SNOMED CT, IDMP etc. It could also be a national or local formulary
  /// code, optionally with translations to other code systems.
  code: CodeableConcept,

  /// Categorization of the medication within a formulary or classification system.
  #[serde(rename = "medicineClassification")]
  medicine_classification: Vec<MedicationKnowledge_MedicineClassification>,

  /// The program under which the medication is reviewed.
  #[serde(rename = "monitoringProgram")]
  monitoring_program: Vec<MedicationKnowledge_MonitoringProgram>,

  /// Describes the details of the manufacturer of the medication product.  This is
  /// not intended to represent the distributor of a medication product.
  manufacturer: Reference,

  /// Describes the form of the item.  Powder; tablets; capsule.
  #[serde(rename = "doseForm")]
  dose_form: CodeableConcept,

  /// Category of the medication or product (e.g. branded product, therapeutic moeity,
  /// generic product, innovator product, etc.).
  #[serde(rename = "productType")]
  product_type: Vec<CodeableConcept>,

  /// Guidelines for the administration of the medication.
  #[serde(rename = "administrationGuidelines")]
  administration_guidelines: Vec<MedicationKnowledge_AdministrationGuidelines>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// The intended or approved route of administration.
  #[serde(rename = "intendedRoute")]
  intended_route: Vec<CodeableConcept>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Specifies descriptive properties of the medicine, such as color, shape,
  /// imprints, etc.
  #[serde(rename = "drugCharacteristic")]
  drug_characteristic: Vec<MedicationKnowledge_DrugCharacteristic>,

  /// The time course of drug absorption, distribution, metabolism and excretion of a
  /// medication from the body.
  kinetics: Vec<MedicationKnowledge_Kinetics>,

  /// Extensions for status
  _status: Element,

  /// Associated or related knowledge about a medication.
  #[serde(rename = "relatedMedicationKnowledge")]
  related_medication_knowledge: Vec<MedicationKnowledge_RelatedMedicationKnowledge>,

  /// Identifies a particular constituent of interest in the product.
  ingredient: Vec<MedicationKnowledge_Ingredient>,

  /// Extensions for preparationInstruction
  #[serde(rename = "_preparationInstruction")]
  _preparation_instruction: Element,

  /// The price of the medication.
  cost: Vec<MedicationKnowledge_Cost>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Associated documentation about the medication.
  monograph: Vec<MedicationKnowledge_Monograph>,

  /// Specific amount of the drug in the packaged product.  For example, when
  /// specifying a product that has the same strength (For example, Insulin glargine
  /// 100 unit per mL solution for injection), this attribute provides additional
  /// clarification of the package amount (For example, 3 mL, 10mL, etc.).
  amount: Quantity,

  /// Information that only applies to packages (not products).
  packaging: MedicationKnowledge_Packaging,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// Extensions for language
  _language: Element,

  /// The instructions for preparing the medication.
  #[serde(rename = "preparationInstruction")]
  preparation_instruction: markdown,

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

}
