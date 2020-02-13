#![allow(unused_imports, non_camel_case_types)]

use crate::model::UsageContext::UsageContext;
use crate::model::Meta::Meta;
use crate::model::ResourceList::ResourceList;
use crate::model::ContactDetail::ContactDetail;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::CompartmentDefinition_Resource::CompartmentDefinition_Resource;
use crate::model::Narrative::Narrative;
use serde_json::value::Value;



/// A compartment definition that defines how resources are accessed on a server.

#[derive(Debug)]
pub struct CompartmentDefinition<'a> {
  pub value: &'a Value,
}

impl CompartmentDefinition<'_> {
  /// Explanation of why this compartment definition is needed and why it has been
  /// designed as it has.
  pub fn purpose(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("purpose") {
      return Some(string.to_string());
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

  /// Extensions for status
  pub fn _status(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_status") {
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

  /// Extensions for implicitRules
  pub fn _implicit_rules(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_implicitRules") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for name
  pub fn _name(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_name") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for publisher
  pub fn _publisher(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_publisher") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for purpose
  pub fn _purpose(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_purpose") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A natural language name identifying the compartment definition. This name should
  /// be usable as an identifier for the module by machine processing applications
  /// such as code generation.
  pub fn name(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("name") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Which compartment this definition describes.
  pub fn code(&self) -> Option<CompartmentDefinitionCode> {
    if let Some(Value::String(val)) = self.value.get("code") {
      return Some(CompartmentDefinitionCode::from_string(&val).unwrap());
    }
    return None;
  }

  /// Whether the search syntax is supported,.
  pub fn search(&self) -> Option<bool> {
    if let Some(val) = self.value.get("search") {
      return Some(val.as_bool().unwrap());
    }
    return None;
  }

  /// Information about how a resource is related to the compartment.
  pub fn resource(&self) -> Option<Vec<CompartmentDefinition_Resource>> {
    if let Some(Value::Array(val)) = self.value.get("resource") {
      return Some(val.into_iter().map(|e| CompartmentDefinition_Resource { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  pub fn contact(&self) -> Option<Vec<ContactDetail>> {
    if let Some(Value::Array(val)) = self.value.get("contact") {
      return Some(val.into_iter().map(|e| ContactDetail { value: e }).collect::<Vec<_>>());
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

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  pub fn meta(&self) -> Option<Meta> {
    if let Some(val) = self.value.get("meta") {
      return Some(Meta { value: val });
    }
    return None;
  }

  /// A Boolean value to indicate that this compartment definition is authored for
  /// testing purposes (or education/evaluation/marketing) and is not intended to be
  /// used for genuine usage.
  pub fn experimental(&self) -> Option<bool> {
    if let Some(val) = self.value.get("experimental") {
      return Some(val.as_bool().unwrap());
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

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  pub fn contained(&self) -> Option<Vec<ResourceList>> {
    if let Some(Value::Array(val)) = self.value.get("contained") {
      return Some(val.into_iter().map(|e| ResourceList { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for search
  pub fn _search(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_search") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for code
  pub fn _code(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_code") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The name of the organization or individual that published the compartment
  /// definition.
  pub fn publisher(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("publisher") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The date  (and optionally time) when the compartment definition was published.
  /// The date must change when the business version changes and it must change if the
  /// status code changes. In addition, it should change when the substantive content
  /// of the compartment definition changes.
  pub fn date(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("date") {
      return Some(string.to_string());
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

  /// The base language in which the resource is written.
  pub fn language(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("language") {
      return Some(string.to_string());
    }
    return None;
  }

  /// An absolute URI that is used to identify this compartment definition when it is
  /// referenced in a specification, model, design or an instance; also called its
  /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
  /// address at which at which an authoritative instance of this compartment
  /// definition is (or will be) published. This URL can be the target of a canonical
  /// reference. It SHALL remain the same when the compartment definition is stored on
  /// different servers.
  pub fn url(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("url") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for date
  pub fn _date(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_date") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A free text natural language description of the compartment definition from a
  /// consumer's perspective.
  pub fn description(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("description") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for url
  pub fn _url(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_url") {
      return Some(Element { value: val });
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

  /// Extensions for experimental
  pub fn _experimental(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_experimental") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for version
  pub fn _version(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_version") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The status of this compartment definition. Enables tracking the life-cycle of
  /// the content.
  pub fn status(&self) -> Option<CompartmentDefinitionStatus> {
    if let Some(Value::String(val)) = self.value.get("status") {
      return Some(CompartmentDefinitionStatus::from_string(&val).unwrap());
    }
    return None;
  }

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate compartment
  /// definition instances.
  pub fn use_context(&self) -> Option<Vec<UsageContext>> {
    if let Some(Value::Array(val)) = self.value.get("useContext") {
      return Some(val.into_iter().map(|e| UsageContext { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The identifier that is used to identify this version of the compartment
  /// definition when it is referenced in a specification, model, design or instance.
  /// This is an arbitrary value managed by the compartment definition author and is
  /// not expected to be globally unique. For example, it might be a timestamp (e.g.
  /// yyyymmdd) if a managed version is not available. There is also no expectation
  /// that versions can be placed in a lexicographical sequence.
  pub fn version(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("version") {
      return Some(string.to_string());
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
    if let Some(_val) = self.purpose() {
    }
    if let Some(_val) = self.text() {
      _val.validate();
    }
    if let Some(_val) = self._status() {
      _val.validate();
    }
    if let Some(_val) = self.id() {
    }
    if let Some(_val) = self._implicit_rules() {
      _val.validate();
    }
    if let Some(_val) = self._name() {
      _val.validate();
    }
    if let Some(_val) = self._publisher() {
      _val.validate();
    }
    if let Some(_val) = self._purpose() {
      _val.validate();
    }
    if let Some(_val) = self.name() {
    }
    if let Some(_val) = self.code() {
    }
    if let Some(_val) = self.search() {
    }
    if let Some(_val) = self.resource() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.contact() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.modifier_extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.meta() {
      _val.validate();
    }
    if let Some(_val) = self.experimental() {
    }
    if let Some(_val) = self._description() {
      _val.validate();
    }
    if let Some(_val) = self.contained() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self._search() {
      _val.validate();
    }
    if let Some(_val) = self._code() {
      _val.validate();
    }
    if let Some(_val) = self.publisher() {
    }
    if let Some(_val) = self.date() {
    }
    if let Some(_val) = self.implicit_rules() {
    }
    if let Some(_val) = self.language() {
    }
    if let Some(_val) = self.url() {
    }
    if let Some(_val) = self._date() {
      _val.validate();
    }
    if let Some(_val) = self.description() {
    }
    if let Some(_val) = self._url() {
      _val.validate();
    }
    if let Some(_val) = self.extension() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self._experimental() {
      _val.validate();
    }
    if let Some(_val) = self._version() {
      _val.validate();
    }
    if let Some(_val) = self.status() {
    }
    if let Some(_val) = self.use_context() {
      _val.into_iter().for_each(|e| { e.validate(); });
    }
    if let Some(_val) = self.version() {
    }
    if let Some(_val) = self._language() {
      _val.validate();
    }
    return true;
  }

}

#[derive(Debug)]
pub enum CompartmentDefinitionCode {
  Patient,
  Encounter,
  RelatedPerson,
  Practitioner,
  Device,
}

impl CompartmentDefinitionCode {
    pub fn from_string(string: &str) -> Option<CompartmentDefinitionCode> {
      match string {
        "Patient" => Some(CompartmentDefinitionCode::Patient),
        "Encounter" => Some(CompartmentDefinitionCode::Encounter),
        "RelatedPerson" => Some(CompartmentDefinitionCode::RelatedPerson),
        "Practitioner" => Some(CompartmentDefinitionCode::Practitioner),
        "Device" => Some(CompartmentDefinitionCode::Device),
        _ => None,
    }
  }
}


#[derive(Debug)]
pub enum CompartmentDefinitionStatus {
  Draft,
  Active,
  Retired,
  Unknown,
}

impl CompartmentDefinitionStatus {
    pub fn from_string(string: &str) -> Option<CompartmentDefinitionStatus> {
      match string {
        "draft" => Some(CompartmentDefinitionStatus::Draft),
        "active" => Some(CompartmentDefinitionStatus::Active),
        "retired" => Some(CompartmentDefinitionStatus::Retired),
        "unknown" => Some(CompartmentDefinitionStatus::Unknown),
        _ => None,
    }
  }
}

