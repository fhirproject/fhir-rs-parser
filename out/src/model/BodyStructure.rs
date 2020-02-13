#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Identifier::Identifier;
use crate::model::Attachment::Attachment;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::CodeableConcept::CodeableConcept;
use serde_json::value::Value;



/// Record details about an anatomical structure.  This resource may be used when a
/// coded concept does not provide the necessary detail needed for the use case.

#[derive(Debug)]
pub struct BodyStructure<'a> {
  pub value: &'a Value,
}

impl BodyStructure<'_> {
  /// Whether this body site is in active use.
  pub fn active(&self) -> Option<bool> {
    if let Some(val) = self.value.get("active") {
      return Some(val.as_bool().unwrap());
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

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  pub fn contained(&self) -> Option<Vec<ResourceList>> {
    if let Some(Value::Array(val)) = self.value.get("contained") {
      return Some(val.into_iter().map(|e| ResourceList { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Qualifier to refine the anatomical location.  These include qualifiers for
  /// laterality, relative location, directionality, number, and plane.
  pub fn location_qualifier(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("locationQualifier") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
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

  /// Identifier for this instance of the anatomical structure.
  pub fn identifier(&self) -> Option<Vec<Identifier>> {
    if let Some(Value::Array(val)) = self.value.get("identifier") {
      return Some(val.into_iter().map(|e| Identifier { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The anatomical location or region of the specimen, lesion, or body structure.
  pub fn location(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("location") {
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

  /// A summary, characterization or explanation of the body structure.
  pub fn description(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("description") {
      return Some(string.to_string());
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

  /// The person to which the body site belongs.
  pub fn patient(&self) -> Reference {
    Reference {
      value: &self.value["patient"],
    }
  }

  /// Extensions for active
  pub fn _active(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_active") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for description
  pub fn _description(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_description") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Image or images used to identify a location.
  pub fn image(&self) -> Option<Vec<Attachment>> {
    if let Some(Value::Array(val)) = self.value.get("image") {
      return Some(val.into_iter().map(|e| Attachment { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The kind of structure being represented by the body structure at
  /// `BodyStructure.location`.  This can define both normal and abnormal
  /// morphologies.
  pub fn morphology(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("morphology") {
      return Some(CodeableConcept { value: val });
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

  /// Extensions for language
  pub fn _language(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_language") {
      return Some(Element { value: val });
    }
    return None;
  }

}
