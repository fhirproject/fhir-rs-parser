#![allow(unused_imports, non_camel_case_types)]

use crate::model::RelatedPerson_Communication::RelatedPerson_Communication;
use crate::model::ResourceList::ResourceList;
use crate::model::Meta::Meta;
use crate::model::Reference::Reference;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Narrative::Narrative;
use crate::model::Attachment::Attachment;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Period::Period;
use crate::model::HumanName::HumanName;
use crate::model::Identifier::Identifier;
use crate::model::Address::Address;
use crate::model::Extension::Extension;
use serde_json::value::Value;



/// Information about a person that is involved in the care for a patient, but who
/// is not the target of healthcare, nor has a formal responsibility in the care
/// process.

#[derive(Debug)]
pub struct RelatedPerson<'a> {
  pub value: &'a Value,
}

impl RelatedPerson<'_> {
  /// Address where the related person can be contacted or visited.
  pub fn address(&self) -> Option<Vec<Address>> {
    if let Some(Value::Array(val)) = self.value.get("address") {
      return Some(val.into_iter().map(|e| Address { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for gender
  pub fn _gender(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_gender") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A name associated with the person.
  pub fn name(&self) -> Option<Vec<HumanName>> {
    if let Some(Value::Array(val)) = self.value.get("name") {
      return Some(val.into_iter().map(|e| HumanName { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Image of the person.
  pub fn photo(&self) -> Option<Vec<Attachment>> {
    if let Some(Value::Array(val)) = self.value.get("photo") {
      return Some(val.into_iter().map(|e| Attachment { value: e }).collect::<Vec<_>>());
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

  /// A contact detail for the person, e.g. a telephone number or an email address.
  pub fn telecom(&self) -> Option<Vec<ContactPoint>> {
    if let Some(Value::Array(val)) = self.value.get("telecom") {
      return Some(val.into_iter().map(|e| ContactPoint { value: e }).collect::<Vec<_>>());
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

  /// The period of time during which this relationship is or was active. If there are
  /// no dates defined, then the interval is unknown.
  pub fn period(&self) -> Option<Period> {
    if let Some(val) = self.value.get("period") {
      return Some(Period { value: val });
    }
    return None;
  }

  /// A language which may be used to communicate with about the patient's health.
  pub fn communication(&self) -> Option<Vec<RelatedPerson_Communication>> {
    if let Some(Value::Array(val)) = self.value.get("communication") {
      return Some(val.into_iter().map(|e| RelatedPerson_Communication { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Whether this related person record is in active use.
  pub fn active(&self) -> Option<bool> {
    if let Some(val) = self.value.get("active") {
      return Some(val.as_bool().unwrap());
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

  /// Extensions for implicitRules
  pub fn _implicit_rules(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_implicitRules") {
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

  /// Extensions for birthDate
  pub fn _birth_date(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_birthDate") {
      return Some(Element { value: val });
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

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  pub fn meta(&self) -> Option<Meta> {
    if let Some(val) = self.value.get("meta") {
      return Some(Meta { value: val });
    }
    return None;
  }

  /// The patient this person is related to.
  pub fn patient(&self) -> Reference {
    Reference {
      value: &self.value["patient"],
    }
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

  /// Identifier for a person within a particular scope.
  pub fn identifier(&self) -> Option<Vec<Identifier>> {
    if let Some(Value::Array(val)) = self.value.get("identifier") {
      return Some(val.into_iter().map(|e| Identifier { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The nature of the relationship between a patient and the related person.
  pub fn relationship(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("relationship") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
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

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for active
  pub fn _active(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_active") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Administrative Gender - the gender that the person is considered to have for
  /// administration and record keeping purposes.
  pub fn gender(&self) -> Option<RelatedPersonGender> {
    if let Some(Value::String(val)) = self.value.get("gender") {
      return Some(RelatedPersonGender::from_string(&val).unwrap());
    }
    return None;
  }

  /// The date on which the related person was born.
  pub fn birth_date(&self) -> Option<i64> {
    if let Some(val) = self.value.get("birthDate") {
      return Some(val.as_i64().unwrap());
    }
    return None;
  }

}

#[derive(Debug)]
pub enum RelatedPersonGender {
  Male,
  Female,
  Other,
  Unknown,
}

impl RelatedPersonGender {
    pub fn from_string(string: &str) -> Option<RelatedPersonGender> {
      match string {
        "male" => Some(RelatedPersonGender::Male),
        "female" => Some(RelatedPersonGender::Female),
        "other" => Some(RelatedPersonGender::Other),
        "unknown" => Some(RelatedPersonGender::Unknown),
        _ => None,
    }
  }
}

