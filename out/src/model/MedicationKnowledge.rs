#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::MedicationKnowledge_MonitoringProgram::MedicationKnowledge_MonitoringProgram;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::Quantity::Quantity;
use crate::model::MedicationKnowledge_Kinetics::MedicationKnowledge_Kinetics;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ResourceList::ResourceList;
use crate::model::MedicationKnowledge_DrugCharacteristic::MedicationKnowledge_DrugCharacteristic;
use crate::model::MedicationKnowledge_Monograph::MedicationKnowledge_Monograph;
use crate::model::MedicationKnowledge_Cost::MedicationKnowledge_Cost;
use crate::model::MedicationKnowledge_Regulatory::MedicationKnowledge_Regulatory;
use crate::model::MedicationKnowledge_MedicineClassification::MedicationKnowledge_MedicineClassification;
use crate::model::Extension::Extension;
use crate::model::MedicationKnowledge_Packaging::MedicationKnowledge_Packaging;
use crate::model::Meta::Meta;
use crate::model::MedicationKnowledge_Ingredient::MedicationKnowledge_Ingredient;
use crate::model::MedicationKnowledge_AdministrationGuidelines::MedicationKnowledge_AdministrationGuidelines;
use crate::model::MedicationKnowledge_RelatedMedicationKnowledge::MedicationKnowledge_RelatedMedicationKnowledge;
use crate::model::Element::Element;


/// Information about a medication that is used to support knowledge.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledge {
  /// Associated or related medications.  For example, if the medication is a branded
  /// product (e.g. Crestor), this is the Therapeutic Moeity (e.g. Rosuvastatin) or if
  /// this is a generic medication (e.g. Rosuvastatin), this would link to a branded
  /// product (e.g. Crestor).
  #[serde(rename = "associatedMedication")]
  associated_medication: Vec<Box<Reference>>,

  /// The intended or approved route of administration.
  #[serde(rename = "intendedRoute")]
  intended_route: Vec<CodeableConcept>,

  /// Guidelines for the administration of the medication.
  #[serde(rename = "administrationGuidelines")]
  administration_guidelines: Vec<MedicationKnowledge_AdministrationGuidelines>,

  /// Extensions for language
  _language: Element,

  /// Categorization of the medication within a formulary or classification system.
  #[serde(rename = "medicineClassification")]
  medicine_classification: Vec<MedicationKnowledge_MedicineClassification>,

  /// Extensions for status
  _status: Element,

  /// Associated or related knowledge about a medication.
  #[serde(rename = "relatedMedicationKnowledge")]
  related_medication_knowledge: Vec<MedicationKnowledge_RelatedMedicationKnowledge>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Associated documentation about the medication.
  monograph: Vec<MedicationKnowledge_Monograph>,

  /// Extensions for preparationInstruction
  #[serde(rename = "_preparationInstruction")]
  _preparation_instruction: Element,

  /// Regulatory information about a medication.
  regulatory: Vec<MedicationKnowledge_Regulatory>,

  /// A code to indicate if the medication is in active use.  The status refers to the
  /// validity about the information of the medication and not to its medicinal
  /// properties.
  status: String,

  /// Describes the form of the item.  Powder; tablets; capsule.
  #[serde(rename = "doseForm")]
  dose_form: CodeableConcept,

  /// The instructions for preparing the medication.
  #[serde(rename = "preparationInstruction")]
  preparation_instruction: String,

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

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Category of the medication or product (e.g. branded product, therapeutic moeity,
  /// generic product, innovator product, etc.).
  #[serde(rename = "productType")]
  product_type: Vec<CodeableConcept>,

  /// The program under which the medication is reviewed.
  #[serde(rename = "monitoringProgram")]
  monitoring_program: Vec<MedicationKnowledge_MonitoringProgram>,

  /// Specific amount of the drug in the packaged product.  For example, when
  /// specifying a product that has the same strength (For example, Insulin glargine
  /// 100 unit per mL solution for injection), this attribute provides additional
  /// clarification of the package amount (For example, 3 mL, 10mL, etc.).
  amount: Quantity,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// Information that only applies to packages (not products).
  packaging: MedicationKnowledge_Packaging,

  /// The time course of drug absorption, distribution, metabolism and excretion of a
  /// medication from the body.
  kinetics: Vec<MedicationKnowledge_Kinetics>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Identifies a particular constituent of interest in the product.
  ingredient: Vec<MedicationKnowledge_Ingredient>,

  /// Additional names for a medication, for example, the name(s) given to a
  /// medication in different countries.  For example, acetaminophen and paracetamol
  /// or salbutamol and albuterol.
  synonym: Vec<String>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// A code that specifies this medication, or a textual description if no code is
  /// available. Usage note: This could be a standard medication code such as a code
  /// from RxNorm, SNOMED CT, IDMP etc. It could also be a national or local formulary
  /// code, optionally with translations to other code systems.
  code: CodeableConcept,

  /// The price of the medication.
  cost: Vec<MedicationKnowledge_Cost>,

  /// Describes the details of the manufacturer of the medication product.  This is
  /// not intended to represent the distributor of a medication product.
  manufacturer: Box<Reference>,

  /// The base language in which the resource is written.
  language: String,

  /// Potential clinical issue with or between medication(s) (for example, drug-drug
  /// interaction, drug-disease contraindication, drug-allergy interaction, etc.).
  contraindication: Vec<Box<Reference>>,

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

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Extensions for synonym
  _synonym: Vec<Element>,

}
