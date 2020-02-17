#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Quantity::Quantity;
use crate::model::SubstanceSpecification_MolecularWeight::SubstanceSpecification_MolecularWeight;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.

#[derive(Debug)]
pub struct SubstanceSpecification_Isotope<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstanceSpecification_Isotope<'_> {
    /// May be used to represent additional information that is not part of the basic
    /// definition of the element. To make the use of extensions safe and manageable,
    /// there is a strict set of governance  applied to the definition and use of
    /// extensions. Though any implementer can define an extension, there is a set of
    /// requirements that SHALL be met as part of the definition of the extension.
    pub fn extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("extension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Half life - for a non-natural nuclide.
    pub fn half_life(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("halfLife") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Substance identifier for each non-natural or radioisotope.
    pub fn identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifier") {
            return Some(Identifier {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

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
    pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The molecular weight or weight range (for proteins, polymers or nucleic acids).
    pub fn molecular_weight(&self) -> Option<SubstanceSpecification_MolecularWeight> {
        if let Some(val) = self.value.get("molecularWeight") {
            return Some(SubstanceSpecification_MolecularWeight {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Substance name for each non-natural or radioisotope.
    pub fn name(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("name") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The type of isotopic substitution present in a single substance.
    pub fn substitution(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("substitution") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.half_life() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.molecular_weight() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.substitution() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct SubstanceSpecification_IsotopeBuilder {
    pub value: Value,
}

impl SubstanceSpecification_IsotopeBuilder {
    pub fn build(&self) -> SubstanceSpecification_Isotope {
        SubstanceSpecification_Isotope {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn new() -> SubstanceSpecification_IsotopeBuilder {
        let mut __value: Value = json!({});
        return SubstanceSpecification_IsotopeBuilder { value: __value };
    }
}
