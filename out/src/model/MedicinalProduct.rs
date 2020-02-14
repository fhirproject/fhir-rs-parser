#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Coding::Coding;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::MarketingStatus::MarketingStatus;
use crate::model::MedicinalProduct_ManufacturingBusinessOperation::MedicinalProduct_ManufacturingBusinessOperation;
use crate::model::MedicinalProduct_Name::MedicinalProduct_Name;
use crate::model::MedicinalProduct_SpecialDesignation::MedicinalProduct_SpecialDesignation;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// Detailed definition of a medicinal product, typically for uses other than direct
/// patient care (e.g. regulatory use).

#[derive(Debug)]
pub struct MedicinalProduct<'a> {
    pub value: &'a Value,
}

impl MedicinalProduct<'_> {
    /// Whether the Medicinal Product is subject to additional monitoring for regulatory
    /// reasons.
    pub fn additional_monitoring_indicator(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("additionalMonitoringIndicator") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Marketing status of the medicinal product, in contrast to marketing
    /// authorizaton.
    pub fn marketing_status(&self) -> Option<Vec<MarketingStatus>> {
        if let Some(Value::Array(val)) = self.value.get("marketingStatus") {
            return Some(
                val.into_iter()
                    .map(|e| MarketingStatus { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Allows the product to be classified by various systems.
    pub fn product_classification(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("productClassification") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The dose form for a single part product, or combined form of a multiple part
    /// product.
    pub fn combined_pharmaceutical_dose_form(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("combinedPharmaceuticalDoseForm") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Regulatory type, e.g. Investigational or Authorized.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Clinical trials or studies that this product is involved in.
    pub fn clinical_trial(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("clinicalTrial") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The product's name, including full name and possibly coded parts.
    pub fn name(&self) -> Vec<MedicinalProduct_Name> {
        self.value
            .get("name")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| MedicinalProduct_Name { value: e })
            .collect::<Vec<_>>()
    }

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// Pharmaceutical aspects of product.
    pub fn pharmaceutical_product(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("pharmaceuticalProduct") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub fn implicit_rules(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("implicitRules") {
            return Some(string);
        }
        return None;
    }

    /// A master file for to the medicinal product (e.g. Pharmacovigilance System Master
    /// File).
    pub fn master_file(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("masterFile") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the resource. To make the use of extensions safe and manageable,
    /// there is a strict set of governance  applied to the definition and use of
    /// extensions. Though any implementer can define an extension, there is a set of
    /// requirements that SHALL be met as part of the definition of the extension.
    pub fn extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("extension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The legal status of supply of the medicinal product as classified by the
    /// regulator.
    pub fn legal_status_of_supply(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("legalStatusOfSupply") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Whether the Medicinal Product is subject to special measures for regulatory
    /// reasons.
    pub fn special_measures(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("specialMeasures") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// If this medicine applies to human or veterinary uses.
    pub fn domain(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("domain") {
            return Some(Coding { value: val });
        }
        return None;
    }

    /// Supporting documentation, typically for regulatory submission.
    pub fn attached_document(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("attachedDocument") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need not
    /// encode all the structured data, but is required to contain sufficient detail to
    /// make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub fn text(&self) -> Option<Narrative> {
        if let Some(val) = self.value.get("text") {
            return Some(Narrative { value: val });
        }
        return None;
    }

    /// If authorised for use in children.
    pub fn paediatric_use_indicator(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("paediatricUseIndicator") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// A product specific contact, person (in a role), or an organization.
    pub fn contact(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("contact") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Reference to another product, e.g. for linking authorised to investigational
    /// product.
    pub fn cross_reference(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("crossReference") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, and nor can they
    /// have their own independent transaction scope.
    pub fn contained(&self) -> Option<Vec<ResourceList>> {
        if let Some(Value::Array(val)) = self.value.get("contained") {
            return Some(
                val.into_iter()
                    .map(|e| ResourceList { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for specialMeasures
    pub fn _special_measures(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_specialMeasures") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Package representation for the product.
    pub fn packaged_medicinal_product(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("packagedMedicinalProduct") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An operation applied to the product, for manufacturing or adminsitrative
    /// purpose.
    pub fn manufacturing_business_operation(
        &self,
    ) -> Option<Vec<MedicinalProduct_ManufacturingBusinessOperation>> {
        if let Some(Value::Array(val)) = self.value.get("manufacturingBusinessOperation") {
            return Some(
                val.into_iter()
                    .map(|e| MedicinalProduct_ManufacturingBusinessOperation { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
        }
        return None;
    }

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
    pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Business identifier for this product. Could be an MPID.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indicates if the medicinal product has an orphan designation for the treatment
    /// of a rare disease.
    pub fn special_designation(&self) -> Option<Vec<MedicinalProduct_SpecialDesignation>> {
        if let Some(Value::Array(val)) = self.value.get("specialDesignation") {
            return Some(
                val.into_iter()
                    .map(|e| MedicinalProduct_SpecialDesignation { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.additional_monitoring_indicator() {
            _val.validate();
        }
        if let Some(_val) = self.marketing_status() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.product_classification() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.combined_pharmaceutical_dose_form() {
            _val.validate();
        }
        if let Some(_val) = self.fhir_type() {
            _val.validate();
        }
        if let Some(_val) = self.clinical_trial() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.name().into_iter().for_each(|e| {
            e.validate();
        });
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.pharmaceutical_product() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.master_file() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.legal_status_of_supply() {
            _val.validate();
        }
        if let Some(_val) = self.special_measures() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.domain() {
            _val.validate();
        }
        if let Some(_val) = self.attached_document() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.paediatric_use_indicator() {
            _val.validate();
        }
        if let Some(_val) = self.contact() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.cross_reference() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._special_measures() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.packaged_medicinal_product() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.manufacturing_business_operation() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.special_designation() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
