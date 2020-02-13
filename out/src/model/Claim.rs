#![allow(unused_imports, non_camel_case_types)]

use crate::model::Claim_Payee::Claim_Payee;
use crate::model::Narrative::Narrative;
use crate::model::Extension::Extension;
use crate::model::Claim_Procedure::Claim_Procedure;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Reference::Reference;
use crate::model::Money::Money;
use crate::model::Element::Element;
use crate::model::Claim_Item::Claim_Item;
use crate::model::Claim_Accident::Claim_Accident;
use crate::model::Claim_Related::Claim_Related;
use crate::model::ResourceList::ResourceList;
use crate::model::Claim_SupportingInfo::Claim_SupportingInfo;
use crate::model::Claim_Insurance::Claim_Insurance;
use crate::model::Claim_CareTeam::Claim_CareTeam;
use crate::model::Claim_Diagnosis::Claim_Diagnosis;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Period::Period;
use serde_json::value::Value;



/// A provider issued list of professional services and products which have been
/// provided, or are to be provided, to a patient which is sent to an insurer for
/// reimbursement.

#[derive(Debug)]
pub struct Claim<'a> {
  pub value: &'a Value,
}

impl Claim<'_> {
  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  pub fn contained(&self) -> Option<Vec<ResourceList>> {
    if let Some(Value::Array(val)) = self.value.get("contained") {
      return Some(val.into_iter().map(|e| ResourceList { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Procedures performed on the patient relevant to the billing items with the
  /// claim.
  pub fn procedure(&self) -> Option<Vec<Claim_Procedure>> {
    if let Some(Value::Array(val)) = self.value.get("procedure") {
      return Some(val.into_iter().map(|e| Claim_Procedure { value: e }).collect::<Vec<_>>());
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

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The members of the team who provided the products and services.
  pub fn care_team(&self) -> Option<Vec<Claim_CareTeam>> {
    if let Some(Value::Array(val)) = self.value.get("careTeam") {
      return Some(val.into_iter().map(|e| Claim_CareTeam { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The date this resource was created.
  pub fn created(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("created") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Original prescription which has been superseded by this prescription to support
  /// the dispensing of pharmacy services, medications or products.
  pub fn original_prescription(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("originalPrescription") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// The status of the resource instance.
  pub fn status(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("status") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Details of an accident which resulted in injuries which required the products
  /// and services listed in the claim.
  pub fn accident(&self) -> Option<Claim_Accident> {
    if let Some(val) = self.value.get("accident") {
      return Some(Claim_Accident { value: val });
    }
    return None;
  }

  /// A code to indicate whether the nature of the request is: to request adjudication
  /// of products and services previously rendered; or requesting authorization and
  /// adjudication for provision in the future; or requesting the non-binding
  /// adjudication of the listed products and services which could be provided in the
  /// future.
  pub fn fhir_use(&self) -> Option<ClaimUse> {
    if let Some(Value::String(val)) = self.value.get("use") {
      return Some(ClaimUse::from_string(&val).unwrap());
    }
    return None;
  }

  /// A claim line. Either a simple  product or service or a 'group' of details which
  /// can each be a simple items or groups of sub-details.
  pub fn item(&self) -> Option<Vec<Claim_Item>> {
    if let Some(Value::Array(val)) = self.value.get("item") {
      return Some(val.into_iter().map(|e| Claim_Item { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A finer grained suite of claim type codes which may convey additional
  /// information such as Inpatient vs Outpatient and/or a specialty service.
  pub fn sub_type(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("subType") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  pub fn implicit_rules(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("implicitRules") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for status
  pub fn _status(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_status") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A code to indicate whether and for whom funds are to be reserved for future
  /// claims.
  pub fn funds_reserve(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("fundsReserve") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Other claims which are related to this claim such as prior submissions or claims
  /// for related services or for the same event.
  pub fn related(&self) -> Option<Vec<Claim_Related>> {
    if let Some(Value::Array(val)) = self.value.get("related") {
      return Some(val.into_iter().map(|e| Claim_Related { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A reference to a referral resource.
  pub fn referral(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("referral") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Additional information codes regarding exceptions, special considerations, the
  /// condition, situation, prior or concurrent issues.
  pub fn supporting_info(&self) -> Option<Vec<Claim_SupportingInfo>> {
    if let Some(Value::Array(val)) = self.value.get("supportingInfo") {
      return Some(val.into_iter().map(|e| Claim_SupportingInfo { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A unique identifier assigned to this claim.
  pub fn identifier(&self) -> Option<Vec<Identifier>> {
    if let Some(Value::Array(val)) = self.value.get("identifier") {
      return Some(val.into_iter().map(|e| Identifier { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Financial instruments for reimbursement for the health care products and
  /// services specified on the claim.
  pub fn insurance(&self) -> Vec<Claim_Insurance> {
    self.value.get("insurance").unwrap().as_array().unwrap().into_iter().map(|e| Claim_Insurance { value: e }).collect::<Vec<_>>()
  }

  /// Facility where the services were provided.
  pub fn facility(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("facility") {
      return Some(Reference { value: val });
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
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The party to be reimbursed for cost of the products and services according to
  /// the terms of the policy.
  pub fn payee(&self) -> Option<Claim_Payee> {
    if let Some(val) = self.value.get("payee") {
      return Some(Claim_Payee { value: val });
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
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Prescription to support the dispensing of pharmacy, device or vision products.
  pub fn prescription(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("prescription") {
      return Some(Reference { value: val });
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

  /// Extensions for use
  pub fn _use(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_use") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The base language in which the resource is written.
  pub fn language(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("language") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The period for which charges are being submitted.
  pub fn billable_period(&self) -> Option<Period> {
    if let Some(val) = self.value.get("billablePeriod") {
      return Some(Period { value: val });
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

  /// The category of claim, e.g. oral, pharmacy, vision, institutional, professional.
  pub fn fhir_type(&self) -> CodeableConcept {
    CodeableConcept {
      value: &self.value["type"],
    }
  }

  /// Extensions for created
  pub fn _created(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_created") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Individual who created the claim, predetermination or preauthorization.
  pub fn enterer(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("enterer") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// The Insurer who is target of the request.
  pub fn insurer(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("insurer") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// The provider which is responsible for the claim, predetermination or
  /// preauthorization.
  pub fn provider(&self) -> Reference {
    Reference {
      value: &self.value["provider"],
    }
  }

  /// Information about diagnoses relevant to the claim items.
  pub fn diagnosis(&self) -> Option<Vec<Claim_Diagnosis>> {
    if let Some(Value::Array(val)) = self.value.get("diagnosis") {
      return Some(val.into_iter().map(|e| Claim_Diagnosis { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The party to whom the professional services and/or products have been supplied
  /// or are being considered and for whom actual or forecast reimbursement is sought.
  pub fn patient(&self) -> Reference {
    Reference {
      value: &self.value["patient"],
    }
  }

  /// The provider-required urgency of processing the request. Typical values include:
  /// stat, routine deferred.
  pub fn priority(&self) -> CodeableConcept {
    CodeableConcept {
      value: &self.value["priority"],
    }
  }

  /// The total value of the all the items in the claim.
  pub fn total(&self) -> Option<Money> {
    if let Some(val) = self.value.get("total") {
      return Some(Money { value: val });
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

}

#[derive(Debug)]
pub enum ClaimUse {
  Claim,
  Preauthorization,
  Predetermination,
}

impl ClaimUse {
    pub fn from_string(string: &str) -> Option<ClaimUse> {
      match string {
        "claim" => Some(ClaimUse::Claim),
        "preauthorization" => Some(ClaimUse::Preauthorization),
        "predetermination" => Some(ClaimUse::Predetermination),
        _ => None,
    }
  }
}

