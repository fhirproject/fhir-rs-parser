#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::ObservationDefinition_QualifiedInterval::ObservationDefinition_QualifiedInterval;
use crate::model::ObservationDefinition_QuantitativeDetails::ObservationDefinition_QuantitativeDetails;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// Set of definitional characteristics for a kind of observation or measurement
/// produced or consumed by an orderable health care service.

#[derive(Debug)]
pub struct ObservationDefinition<'a> {
    pub value: &'a Value,
}

impl ObservationDefinition<'_> {
    /// Multiple results allowed for observations conforming to this
    /// ObservationDefinition.
    pub fn multiple_results_allowed(&self) -> Option<bool> {
        if let Some(val) = self.value.get("multipleResultsAllowed") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Extensions for multipleResultsAllowed
    pub fn _multiple_results_allowed(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_multipleResultsAllowed") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Characteristics for quantitative results of this observation.
    pub fn quantitative_details(&self) -> Option<ObservationDefinition_QuantitativeDetails> {
        if let Some(val) = self.value.get("quantitativeDetails") {
            return Some(ObservationDefinition_QuantitativeDetails { value: val });
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

    /// Describes what will be observed. Sometimes this is called the observation
    /// "name".
    pub fn code(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["code"],
        }
    }

    /// Multiple  ranges of results qualified by different contexts for ordinal or
    /// continuous observations conforming to this ObservationDefinition.
    pub fn qualified_interval(&self) -> Option<Vec<ObservationDefinition_QualifiedInterval>> {
        if let Some(Value::Array(val)) = self.value.get("qualifiedInterval") {
            return Some(
                val.into_iter()
                    .map(|e| ObservationDefinition_QualifiedInterval { value: e })
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

    /// The set of normal coded results for the observations conforming to this
    /// ObservationDefinition.
    pub fn normal_coded_value_set(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("normalCodedValueSet") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Extensions for permittedDataType
    pub fn _permitted_data_type(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_permittedDataType") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The set of abnormal coded results for the observation conforming to this
    /// ObservationDefinition.
    pub fn abnormal_coded_value_set(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("abnormalCodedValueSet") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Extensions for preferredReportName
    pub fn _preferred_report_name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_preferredReportName") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The set of critical coded results for the observation conforming to this
    /// ObservationDefinition.
    pub fn critical_coded_value_set(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("criticalCodedValueSet") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// A unique identifier assigned to this ObservationDefinition artifact.
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

    /// The set of valid coded results for the observations  conforming to this
    /// ObservationDefinition.
    pub fn valid_coded_value_set(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("validCodedValueSet") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// A code that classifies the general type of observation.
    pub fn category(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("category") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
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

    /// The preferred name to be used when reporting the results of observations
    /// conforming to this ObservationDefinition.
    pub fn preferred_report_name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("preferredReportName") {
            return Some(string);
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

    /// The method or technique used to perform the observation.
    pub fn method(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("method") {
            return Some(CodeableConcept { value: val });
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.multiple_results_allowed() {}
        if let Some(_val) = self._multiple_results_allowed() {
            _val.validate();
        }
        if let Some(_val) = self.quantitative_details() {
            _val.validate();
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        let _ = self.code().validate();
        if let Some(_val) = self.qualified_interval() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.normal_coded_value_set() {
            _val.validate();
        }
        if let Some(_val) = self._permitted_data_type() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.abnormal_coded_value_set() {
            _val.validate();
        }
        if let Some(_val) = self._preferred_report_name() {
            _val.validate();
        }
        if let Some(_val) = self.critical_coded_value_set() {
            _val.validate();
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.valid_coded_value_set() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.category() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.preferred_report_name() {}
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.method() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        return true;
    }
}
