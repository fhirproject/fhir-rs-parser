#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Reference::Reference;
use crate::model::Quantity::Quantity;
use crate::model::Extension::Extension;
use crate::model::MedicationKnowledge_Regulatory::MedicationKnowledge_Regulatory;
use crate::model::ResourceList::ResourceList;
use crate::model::MedicationKnowledge_Kinetics::MedicationKnowledge_Kinetics;
use crate::model::Narrative::Narrative;
use crate::model::MedicationKnowledge_MonitoringProgram::MedicationKnowledge_MonitoringProgram;
use crate::model::MedicationKnowledge_Cost::MedicationKnowledge_Cost;
use crate::model::MedicationKnowledge_Ingredient::MedicationKnowledge_Ingredient;
use crate::model::MedicationKnowledge_MedicineClassification::MedicationKnowledge_MedicineClassification;
use crate::model::MedicationKnowledge_Monograph::MedicationKnowledge_Monograph;
use crate::model::Element::Element;
use crate::model::MedicationKnowledge_AdministrationGuidelines::MedicationKnowledge_AdministrationGuidelines;
use crate::model::MedicationKnowledge_DrugCharacteristic::MedicationKnowledge_DrugCharacteristic;
use crate::model::MedicationKnowledge_Packaging::MedicationKnowledge_Packaging;
use crate::model::Meta::Meta;
use crate::model::MedicationKnowledge_RelatedMedicationKnowledge::MedicationKnowledge_RelatedMedicationKnowledge;


/// Information about a medication that is used to support knowledge.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationKnowledge {
  /// The time course of drug absorption, distribution, metabolism and excretion of a
  /// medication from the body.
  kinetics: Option<Vec<MedicationKnowledge_Kinetics>>,

  /// Associated or related knowledge about a medication.
  #[serde(rename = "relatedMedicationKnowledge")]
  related_medication_knowledge: Option<Vec<MedicationKnowledge_RelatedMedicationKnowledge>>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Category of the medication or product (e.g. branded product, therapeutic moeity,
  /// generic product, innovator product, etc.).
  #[serde(rename = "productType")]
  product_type: Option<Vec<CodeableConcept>>,

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

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Associated documentation about the medication.
  monograph: Option<Vec<MedicationKnowledge_Monograph>>,

  /// Additional names for a medication, for example, the name(s) given to a
  /// medication in different countries.  For example, acetaminophen and paracetamol
  /// or salbutamol and albuterol.
  synonym: Option<Vec<String>>,

  /// A code that specifies this medication, or a textual description if no code is
  /// available. Usage note: This could be a standard medication code such as a code
  /// from RxNorm, SNOMED CT, IDMP etc. It could also be a national or local formulary
  /// code, optionally with translations to other code systems.
  code: Option<CodeableConcept>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// A code to indicate if the medication is in active use.  The status refers to the
  /// validity about the information of the medication and not to its medicinal
  /// properties.
  status: Option<String>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Specific amount of the drug in the packaged product.  For example, when
  /// specifying a product that has the same strength (For example, Insulin glargine
  /// 100 unit per mL solution for injection), this attribute provides additional
  /// clarification of the package amount (For example, 3 mL, 10mL, etc.).
  amount: Option<Quantity>,

  /// Regulatory information about a medication.
  regulatory: Option<Vec<MedicationKnowledge_Regulatory>>,

  /// Describes the form of the item.  Powder; tablets; capsule.
  #[serde(rename = "doseForm")]
  dose_form: Option<CodeableConcept>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// Extensions for preparationInstruction
  #[serde(rename = "_preparationInstruction")]
  _preparation_instruction: Option<Element>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Potential clinical issue with or between medication(s) (for example, drug-drug
  /// interaction, drug-disease contraindication, drug-allergy interaction, etc.).
  contraindication: Option<Vec<Box<Reference>>>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// The price of the medication.
  cost: Option<Vec<MedicationKnowledge_Cost>>,

  /// The intended or approved route of administration.
  #[serde(rename = "intendedRoute")]
  intended_route: Option<Vec<CodeableConcept>>,

  /// The instructions for preparing the medication.
  #[serde(rename = "preparationInstruction")]
  preparation_instruction: Option<String>,

  /// Guidelines for the administration of the medication.
  #[serde(rename = "administrationGuidelines")]
  administration_guidelines: Option<Vec<MedicationKnowledge_AdministrationGuidelines>>,

  /// Associated or related medications.  For example, if the medication is a branded
  /// product (e.g. Crestor), this is the Therapeutic Moeity (e.g. Rosuvastatin) or if
  /// this is a generic medication (e.g. Rosuvastatin), this would link to a branded
  /// product (e.g. Crestor).
  #[serde(rename = "associatedMedication")]
  associated_medication: Option<Vec<Box<Reference>>>,

  /// Extensions for synonym
  #[serde(rename = "_synonym")]
  _synonym: Option<Vec<Element>>,

  /// Describes the details of the manufacturer of the medication product.  This is
  /// not intended to represent the distributor of a medication product.
  manufacturer: Option<Box<Reference>>,

  /// Identifies a particular constituent of interest in the product.
  ingredient: Option<Vec<MedicationKnowledge_Ingredient>>,

  /// Information that only applies to packages (not products).
  packaging: Option<MedicationKnowledge_Packaging>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// The program under which the medication is reviewed.
  #[serde(rename = "monitoringProgram")]
  monitoring_program: Option<Vec<MedicationKnowledge_MonitoringProgram>>,

  /// Categorization of the medication within a formulary or classification system.
  #[serde(rename = "medicineClassification")]
  medicine_classification: Option<Vec<MedicationKnowledge_MedicineClassification>>,

  /// Specifies descriptive properties of the medicine, such as color, shape,
  /// imprints, etc.
  #[serde(rename = "drugCharacteristic")]
  drug_characteristic: Option<Vec<MedicationKnowledge_DrugCharacteristic>>,

}
