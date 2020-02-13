#![allow(unused_imports, non_camel_case_types)]

use crate::model::Reference::Reference;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Meta::Meta;
use crate::model::EpisodeOfCare_StatusHistory::EpisodeOfCare_StatusHistory;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::EpisodeOfCare_Diagnosis::EpisodeOfCare_Diagnosis;
use crate::model::Narrative::Narrative;
use crate::model::ResourceList::ResourceList;
use crate::model::Period::Period;
use serde_json::value::Value;



/// An association between a patient and an organization / healthcare provider(s)
/// during which time encounters may occur. The managing organization assumes a
/// level of responsibility for the patient during this time.

#[derive(Debug)]
pub struct EpisodeOfCare<'a> {
  pub value: &'a Value,
}

impl EpisodeOfCare<'_> {
  /// The history of statuses that the EpisodeOfCare has been through (without
  /// requiring processing the history of the resource).
  pub fn status_history(&self) -> Option<Vec<EpisodeOfCare_StatusHistory>> {
    if let Some(Value::Array(val)) = self.value.get("statusHistory") {
      return Some(val.into_iter().map(|e| EpisodeOfCare_StatusHistory { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The set of accounts that may be used for billing for this EpisodeOfCare.
  pub fn account(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("account") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
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

  /// Extensions for language
  pub fn _language(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_language") {
      return Some(Element { value: val });
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

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  pub fn meta(&self) -> Option<Meta> {
    if let Some(val) = self.value.get("meta") {
      return Some(Meta { value: val });
    }
    return None;
  }

  /// The organization that has assumed the specific responsibilities for the
  /// specified duration.
  pub fn managing_organization(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("managingOrganization") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// The patient who is the focus of this episode of care.
  pub fn patient(&self) -> Reference {
    Reference {
      value: &self.value["patient"],
    }
  }

  /// Referral Request(s) that are fulfilled by this EpisodeOfCare, incoming
  /// referrals.
  pub fn referral_request(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("referralRequest") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// planned | waitlist | active | onhold | finished | cancelled.
  pub fn status(&self) -> Option<EpisodeOfCareStatus> {
    if let Some(Value::String(val)) = self.value.get("status") {
      return Some(EpisodeOfCareStatus::from_string(&val).unwrap());
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

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  pub fn contained(&self) -> Option<Vec<ResourceList>> {
    if let Some(Value::Array(val)) = self.value.get("contained") {
      return Some(val.into_iter().map(|e| ResourceList { value: e }).collect::<Vec<_>>());
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

  /// The list of diagnosis relevant to this episode of care.
  pub fn diagnosis(&self) -> Option<Vec<EpisodeOfCare_Diagnosis>> {
    if let Some(Value::Array(val)) = self.value.get("diagnosis") {
      return Some(val.into_iter().map(|e| EpisodeOfCare_Diagnosis { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The practitioner that is the care manager/care coordinator for this patient.
  pub fn care_manager(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("careManager") {
      return Some(Reference { value: val });
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

  /// The list of practitioners that may be facilitating this episode of care for
  /// specific purposes.
  pub fn team(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("team") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The EpisodeOfCare may be known by different identifiers for different contexts
  /// of use, such as when an external agency is tracking the Episode for funding
  /// purposes.
  pub fn identifier(&self) -> Option<Vec<Identifier>> {
    if let Some(Value::Array(val)) = self.value.get("identifier") {
      return Some(val.into_iter().map(|e| Identifier { value: e }).collect::<Vec<_>>());
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

  /// A classification of the type of episode of care; e.g. specialist referral,
  /// disease management, type of funded care.
  pub fn fhir_type(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("type") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The interval during which the managing organization assumes the defined
  /// responsibility.
  pub fn period(&self) -> Option<Period> {
    if let Some(val) = self.value.get("period") {
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

  pub fn validate(&self) -> bool {
    if let Some(_val) = self.status_history() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.account() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.implicit_rules() {
    }
    if let Some(_val) = self.modifier_extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self._language() {
      _val.validate();
    }
    if let Some(_val) = self._status() {
      _val.validate();
    }
    if let Some(_val) = self.meta() {
      _val.validate();
    }
    if let Some(_val) = self.managing_organization() {
      _val.validate();
    }
    let _ = self.patient().validate();
    if let Some(_val) = self.referral_request() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.status() {
    }
    if let Some(_val) = self.id() {
    }
    if let Some(_val) = self.contained() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self._implicit_rules() {
      _val.validate();
    }
    if let Some(_val) = self.diagnosis() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.care_manager() {
      _val.validate();
    }
    if let Some(_val) = self.language() {
    }
    if let Some(_val) = self.team() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.identifier() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.fhir_type() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.period() {
      _val.validate();
    }
    if let Some(_val) = self.text() {
      _val.validate();
    }
    return true;
  }

}

#[derive(Debug)]
pub enum EpisodeOfCareStatus {
  Planned,
  Waitlist,
  Active,
  Onhold,
  Finished,
  Cancelled,
  EnteredInError,
}

impl EpisodeOfCareStatus {
    pub fn from_string(string: &str) -> Option<EpisodeOfCareStatus> {
      match string {
        "planned" => Some(EpisodeOfCareStatus::Planned),
        "waitlist" => Some(EpisodeOfCareStatus::Waitlist),
        "active" => Some(EpisodeOfCareStatus::Active),
        "onhold" => Some(EpisodeOfCareStatus::Onhold),
        "finished" => Some(EpisodeOfCareStatus::Finished),
        "cancelled" => Some(EpisodeOfCareStatus::Cancelled),
        "entered-in-error" => Some(EpisodeOfCareStatus::EnteredInError),
        _ => None,
    }
  }
}

