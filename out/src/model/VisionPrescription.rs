#![allow(unused_imports, non_camel_case_types)]

use crate::model::Meta::Meta;
use crate::model::Reference::Reference;
use crate::model::Element::Element;
use crate::model::Narrative::Narrative;
use crate::model::Identifier::Identifier;
use crate::model::ResourceList::ResourceList;
use crate::model::VisionPrescription_LensSpecification::VisionPrescription_LensSpecification;
use crate::model::Extension::Extension;
use serde_json::value::Value;



/// An authorization for the provision of glasses and/or contact lenses to a
/// patient.

#[derive(Debug)]
pub struct VisionPrescription<'a> {
  pub value: &'a Value,
}

impl VisionPrescription<'_> {
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

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  pub fn meta(&self) -> Option<Meta> {
    if let Some(val) = self.value.get("meta") {
      return Some(Meta { value: val });
    }
    return None;
  }

  /// Extensions for dateWritten
  pub fn _date_written(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_dateWritten") {
      return Some(Element { value: val });
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

  /// The date this resource was created.
  pub fn created(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("created") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The healthcare professional responsible for authorizing the prescription.
  pub fn prescriber(&self) -> Reference {
    Reference {
      value: &self.value["prescriber"],
    }
  }

  /// A reference to a resource that identifies the particular occurrence of contact
  /// between patient and health care provider during which the prescription was
  /// issued.
  pub fn encounter(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("encounter") {
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

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  pub fn contained(&self) -> Option<Vec<ResourceList>> {
    if let Some(Value::Array(val)) = self.value.get("contained") {
      return Some(val.into_iter().map(|e| ResourceList { value: e }).collect::<Vec<_>>());
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

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
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

  /// Extensions for status
  pub fn _status(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_status") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Contain the details of  the individual lens specifications and serves as the
  /// authorization for the fullfillment by certified professionals.
  pub fn lens_specification(&self) -> Vec<VisionPrescription_LensSpecification> {
    self.value.get("lensSpecification").unwrap().as_array().unwrap().into_iter().map(|e| VisionPrescription_LensSpecification { value: e }).collect::<Vec<_>>()
  }

  /// A resource reference to the person to whom the vision prescription applies.
  pub fn patient(&self) -> Reference {
    Reference {
      value: &self.value["patient"],
    }
  }

  /// The base language in which the resource is written.
  pub fn language(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("language") {
      return Some(string.to_string());
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

  /// Extensions for created
  pub fn _created(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_created") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The date (and perhaps time) when the prescription was written.
  pub fn date_written(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("dateWritten") {
      return Some(string.to_string());
    }
    return None;
  }

  /// A unique identifier assigned to this vision prescription.
  pub fn identifier(&self) -> Option<Vec<Identifier>> {
    if let Some(Value::Array(val)) = self.value.get("identifier") {
      return Some(val.into_iter().map(|e| Identifier { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

}
