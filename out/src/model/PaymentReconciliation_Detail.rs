#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Money::Money;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// This resource provides the details including amount of a payment and allocates
/// the payment items being paid.

#[derive(Debug)]
pub struct PaymentReconciliation_Detail<'a> {
    pub value: &'a Value,
}

impl PaymentReconciliation_Detail<'_> {
    /// Extensions for date
    pub fn _date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_date") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The monetary amount allocated from the total payment to the payable.
    pub fn amount(&self) -> Option<Money> {
        if let Some(val) = self.value.get("amount") {
            return Some(Money { value: val });
        }
        return None;
    }

    /// The date from the response resource containing a commitment to pay.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element. To make the use of extensions safe and manageable,
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Unique identifier for the current payment item for the referenced payable.
    pub fn identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifier") {
            return Some(Identifier { value: val });
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The party which is receiving the payment.
    pub fn payee(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("payee") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Unique identifier for the prior payment item for the referenced payable.
    pub fn predecessor(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("predecessor") {
            return Some(Identifier { value: val });
        }
        return None;
    }

    /// A resource, such as a Claim, the evaluation of which could lead to payment.
    pub fn request(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("request") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// A resource, such as a ClaimResponse, which contains a commitment to payment.
    pub fn response(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("response") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// A reference to the individual who is responsible for inquiries regarding the
    /// response and its payment.
    pub fn responsible(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("responsible") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The party which submitted the claim or financial transaction.
    pub fn submitter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("submitter") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Code to indicate the nature of the payment.
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["type"],
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.amount() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
        if let Some(_val) = self.payee() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.predecessor() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.request() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.response() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.responsible() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.submitter() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.fhir_type().validate() {
            return false;
        }
        return true;
    }
}
