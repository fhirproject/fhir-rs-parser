#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Reference::Reference;
use crate::model::Element::Element;
use crate::model::Narrative::Narrative;
use crate::model::Dosage::Dosage;
use crate::model::Meta::Meta;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Identifier::Identifier;
use crate::model::Annotation::Annotation;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;



/// A record of a medication that is being consumed by a patient.   A
/// MedicationStatement may indicate that the patient may be taking the medication
/// now or has taken the medication in the past or will be taking the medication in
/// the future.  The source of this information can be the patient, significant
/// other (such as a family member or spouse), or a clinician.  A common scenario
/// where this information is captured is during the history taking process during a
/// patient visit or stay.   The medication information may come from sources such
/// as the patient's memory, from a prescription bottle,  or from a list of
/// medications the patient, clinician or other party maintains. 

/// The primary difference between a medication statement and a medication
/// administration is that the medication administration has complete administration
/// information and is based on actual administration information from the person
/// who administered the medication.  A medication statement is often, if not
/// always, less specific.  There is no required date/time when the medication was
/// administered, in fact we only know that a source has reported the patient is
/// taking this medication, where details such as time, quantity, or rate or even
/// medication product may be incomplete or missing or less precise.  As stated
/// earlier, the medication statement information may come from the patient's
/// memory, from a prescription bottle or from a list of medications the patient,
/// clinician or other party maintains.  Medication administration is more formal
/// and is not missing detailed information.

#[derive(Debug)]
pub struct MedicationStatement<'a> {
  pub value: &'a Value,
}

impl MedicationStatement<'_> {
  /// Extensions for status
  pub fn _status(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_status") {
      return Some(Element { value: val });
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
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
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

  /// The interval of time during which it is being asserted that the patient
  /// is/was/will be taking the medication (or was not taking, when the
  /// MedicationStatement.taken element is No).
  pub fn effective_date_time(&self) -> Option<&str> {
    if let Some(Value::String(string)) = self.value.get("effectiveDateTime") {
      return Some(string);
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

  /// Extensions for effectiveDateTime
  pub fn _effective_date_time(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_effectiveDateTime") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A plan, proposal or order that is fulfilled in whole or in part by this event.
  pub fn based_on(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("basedOn") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The person or organization that provided the information about the taking of
  /// this medication. Note: Use derivedFrom when a MedicationStatement is derived
  /// from other resources, e.g. Claim or MedicationRequest.
  pub fn information_source(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("informationSource") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Indicates where the medication is expected to be consumed or administered.
  pub fn category(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("category") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// The interval of time during which it is being asserted that the patient
  /// is/was/will be taking the medication (or was not taking, when the
  /// MedicationStatement.taken element is No).
  pub fn effective_period(&self) -> Option<Period> {
    if let Some(val) = self.value.get("effectivePeriod") {
      return Some(Period { value: val });
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

  /// Extensions for language
  pub fn _language(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_language") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for dateAsserted
  pub fn _date_asserted(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_dateAsserted") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A reason for why the medication is being/was taken.
  pub fn reason_code(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("reasonCode") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Provides extra information about the medication statement that is not conveyed
  /// by the other attributes.
  pub fn note(&self) -> Option<Vec<Annotation>> {
    if let Some(Value::Array(val)) = self.value.get("note") {
      return Some(val.into_iter().map(|e| Annotation { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A code representing the patient or other source's judgment about the state of
  /// the medication used that this statement is about.  Generally, this will be
  /// active or completed.
  pub fn status(&self) -> Option<&str> {
    if let Some(Value::String(string)) = self.value.get("status") {
      return Some(string);
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

  /// Identifies the medication being administered. This is either a link to a
  /// resource representing the details of the medication or a simple attribute
  /// carrying a code that identifies the medication from a known list of medications.
  pub fn medication_codeable_concept(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("medicationCodeableConcept") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Identifiers associated with this Medication Statement that are defined by
  /// business processes and/or used to refer to it when a direct URL reference to the
  /// resource itself is not appropriate. They are business identifiers assigned to
  /// this resource by the performer or other systems and remain constant as the
  /// resource is updated and propagates from server to server.
  pub fn identifier(&self) -> Option<Vec<Identifier>> {
    if let Some(Value::Array(val)) = self.value.get("identifier") {
      return Some(val.into_iter().map(|e| Identifier { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The encounter or episode of care that establishes the context for this
  /// MedicationStatement.
  pub fn context(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("context") {
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

  /// Captures the reason for the current state of the MedicationStatement.
  pub fn status_reason(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("statusReason") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Identifies the medication being administered. This is either a link to a
  /// resource representing the details of the medication or a simple attribute
  /// carrying a code that identifies the medication from a known list of medications.
  pub fn medication_reference(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("medicationReference") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// The date when the medication statement was asserted by the information source.
  pub fn date_asserted(&self) -> Option<&str> {
    if let Some(Value::String(string)) = self.value.get("dateAsserted") {
      return Some(string);
    }
    return None;
  }

  /// Allows linking the MedicationStatement to the underlying MedicationRequest, or
  /// to other information that supports or is used to derive the MedicationStatement.
  pub fn derived_from(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("derivedFrom") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The person, animal or group who is/was taking the medication.
  pub fn subject(&self) -> Reference {
    Reference {
      value: &self.value["subject"],
    }
  }

  /// Condition or observation that supports why the medication is being/was taken.
  pub fn reason_reference(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("reasonReference") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Indicates how the medication is/was or should be taken by the patient.
  pub fn dosage(&self) -> Option<Vec<Dosage>> {
    if let Some(Value::Array(val)) = self.value.get("dosage") {
      return Some(val.into_iter().map(|e| Dosage { value: e }).collect::<Vec<_>>());
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

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  pub fn meta(&self) -> Option<Meta> {
    if let Some(val) = self.value.get("meta") {
      return Some(Meta { value: val });
    }
    return None;
  }

  /// A larger event of which this particular event is a component or step.
  pub fn part_of(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("partOf") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  pub fn validate(&self) -> bool {
    if let Some(_val) = self._status() {
      _val.validate();
    }
    if let Some(_val) = self.language() {
    }
    if let Some(_val) = self.modifier_extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.contained() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.effective_date_time() {
    }
    if let Some(_val) = self.text() {
      _val.validate();
    }
    if let Some(_val) = self._effective_date_time() {
      _val.validate();
    }
    if let Some(_val) = self.based_on() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.information_source() {
      _val.validate();
    }
    if let Some(_val) = self.category() {
      _val.validate();
    }
    if let Some(_val) = self.effective_period() {
      _val.validate();
    }
    if let Some(_val) = self.implicit_rules() {
    }
    if let Some(_val) = self._language() {
      _val.validate();
    }
    if let Some(_val) = self._date_asserted() {
      _val.validate();
    }
    if let Some(_val) = self.reason_code() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.note() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.status() {
    }
    if let Some(_val) = self.id() {
    }
    if let Some(_val) = self.medication_codeable_concept() {
      _val.validate();
    }
    if let Some(_val) = self.identifier() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.context() {
      _val.validate();
    }
    if let Some(_val) = self.extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.status_reason() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.medication_reference() {
      _val.validate();
    }
    if let Some(_val) = self.date_asserted() {
    }
    if let Some(_val) = self.derived_from() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    let _ = self.subject().validate();
    if let Some(_val) = self.reason_reference() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.dosage() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self._implicit_rules() {
      _val.validate();
    }
    if let Some(_val) = self.meta() {
      _val.validate();
    }
    if let Some(_val) = self.part_of() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    return true;
  }

}
