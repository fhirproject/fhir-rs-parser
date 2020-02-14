#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeSystem_Concept::CodeSystem_Concept;
use crate::model::CodeSystem_Filter::CodeSystem_Filter;
use crate::model::CodeSystem_Property::CodeSystem_Property;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactDetail::ContactDetail;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::ResourceList::ResourceList;
use crate::model::UsageContext::UsageContext;
use serde_json::value::Value;

/// The CodeSystem resource is used to declare the existence of and describe a code
/// system or code system supplement and its key properties, and optionally define a
/// part or all of its content.

#[derive(Debug)]
pub struct CodeSystem<'a> {
    pub value: &'a Value,
}

impl CodeSystem<'_> {
    /// Extensions for copyright
    pub fn _copyright(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_copyright") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A formal identifier that is used to identify this code system when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
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

    /// The total number of concepts defined by the code system. Where the code system
    /// has a compositional grammar, the basis of this count is defined by the system
    /// steward.
    pub fn count(&self) -> Option<u64> {
        if let Some(val) = self.value.get("count") {
            return Some(val.as_u64().unwrap());
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

    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate code system
    /// instances.
    pub fn use_context(&self) -> Option<Vec<UsageContext>> {
        if let Some(Value::Array(val)) = self.value.get("useContext") {
            return Some(
                val.into_iter()
                    .map(|e| UsageContext { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A legal or geographic region in which the code system is intended to be used.
    pub fn jurisdiction(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("jurisdiction") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The name of the organization or individual that published the code system.
    pub fn publisher(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("publisher") {
            return Some(string);
        }
        return None;
    }

    /// The canonical URL of the code system that this code system supplement is adding
    /// designations and properties to.
    pub fn supplements(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("supplements") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for count
    pub fn _count(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_count") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The date (and optionally time) when the code system resource was created or
    /// revised.
    pub fn status(&self) -> Option<CodeSystemStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(CodeSystemStatus::from_string(&val).unwrap());
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

    /// A filter that can be used in a value set compose statement when selecting
    /// concepts using a filter.
    pub fn filter(&self) -> Option<Vec<CodeSystem_Filter>> {
        if let Some(Value::Array(val)) = self.value.get("filter") {
            return Some(
                val.into_iter()
                    .map(|e| CodeSystem_Filter { value: e })
                    .collect::<Vec<_>>(),
            );
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
            return Some(
                val.into_iter()
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// If code comparison is case sensitive when codes within this system are compared
    /// to each other.
    pub fn case_sensitive(&self) -> Option<bool> {
        if let Some(val) = self.value.get("caseSensitive") {
            return Some(val.as_bool().unwrap());
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

    /// An absolute URI that is used to identify this code system when it is referenced
    /// in a specification, model, design or an instance; also called its canonical
    /// identifier. This SHOULD be globally unique and SHOULD be a literal address at
    /// which at which an authoritative instance of this code system is (or will be)
    /// published. This URL can be the target of a canonical reference. It SHALL remain
    /// the same when the code system is stored on different servers. This is used in
    /// [Coding](datatypes.html#Coding).system.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
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

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
            return Some(Element { value: val });
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

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
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

    /// A free text natural language description of the code system from a consumer's
    /// perspective.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
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

    /// A short, descriptive, user-friendly title for the code system.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
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

    /// Extensions for caseSensitive
    pub fn _case_sensitive(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_caseSensitive") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for hierarchyMeaning
    pub fn _hierarchy_meaning(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_hierarchyMeaning") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub fn contact(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("contact") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The code system defines a compositional (post-coordination) grammar.
    pub fn compositional(&self) -> Option<bool> {
        if let Some(val) = self.value.get("compositional") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Extensions for versionNeeded
    pub fn _version_needed(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_versionNeeded") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Canonical reference to the value set that contains the entire code system.
    pub fn value_set(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueSet") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for content
    pub fn _content(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_content") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A property defines an additional slot through which additional information can
    /// be provided about a concept.
    pub fn property(&self) -> Option<Vec<CodeSystem_Property>> {
        if let Some(Value::Array(val)) = self.value.get("property") {
            return Some(
                val.into_iter()
                    .map(|e| CodeSystem_Property { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Concepts that are in the code system. The concept definitions are inherently
    /// hierarchical, but the definitions must be consulted to determine what the
    /// meanings of the hierarchical relationships are.
    pub fn concept(&self) -> Option<Vec<CodeSystem_Concept>> {
        if let Some(Value::Array(val)) = self.value.get("concept") {
            return Some(
                val.into_iter()
                    .map(|e| CodeSystem_Concept { value: e })
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

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The date  (and optionally time) when the code system was published. The date
    /// must change when the business version changes and it must change if the status
    /// code changes. In addition, it should change when the substantive content of the
    /// code system changes.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
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

    /// The extent of the content of the code system (the concepts and codes it defines)
    /// are represented in this resource instance.
    pub fn content(&self) -> Option<CodeSystemContent> {
        if let Some(Value::String(val)) = self.value.get("content") {
            return Some(CodeSystemContent::from_string(&val).unwrap());
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

    /// The meaning of the hierarchy of concepts as represented in this resource.
    pub fn hierarchy_meaning(&self) -> Option<CodeSystemHierarchyMeaning> {
        if let Some(Value::String(val)) = self.value.get("hierarchyMeaning") {
            return Some(CodeSystemHierarchyMeaning::from_string(&val).unwrap());
        }
        return None;
    }

    /// This flag is used to signify that the code system does not commit to concept
    /// permanence across versions. If true, a version must be specified when
    /// referencing this code system.
    pub fn version_needed(&self) -> Option<bool> {
        if let Some(val) = self.value.get("versionNeeded") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Explanation of why this code system is needed and why it has been designed as it
    /// has.
    pub fn purpose(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("purpose") {
            return Some(string);
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

    /// A Boolean value to indicate that this code system is authored for testing
    /// purposes (or education/evaluation/marketing) and is not intended to be used for
    /// genuine usage.
    pub fn experimental(&self) -> Option<bool> {
        if let Some(val) = self.value.get("experimental") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Extensions for compositional
    pub fn _compositional(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_compositional") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A copyright statement relating to the code system and/or its contents. Copyright
    /// statements are generally legal restrictions on the use and publishing of the
    /// code system.
    pub fn copyright(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("copyright") {
            return Some(string);
        }
        return None;
    }

    /// The identifier that is used to identify this version of the code system when it
    /// is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the code system author and is not expected to be
    /// globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
    /// managed version is not available. There is also no expectation that versions can
    /// be placed in a lexicographical sequence. This is used in
    /// [Coding](datatypes.html#Coding).version.
    pub fn version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("version") {
            return Some(string);
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

    /// A natural language name identifying the code system. This name should be usable
    /// as an identifier for the module by machine processing applications such as code
    /// generation.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._copyright() {
            _val.validate();
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.count() {}
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.use_context() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.jurisdiction() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.publisher() {}
        if let Some(_val) = self.supplements() {}
        if let Some(_val) = self._count() {
            _val.validate();
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self._date() {
            _val.validate();
        }
        if let Some(_val) = self.filter() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._url() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.case_sensitive() {}
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.url() {}
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self._name() {
            _val.validate();
        }
        if let Some(_val) = self._title() {
            _val.validate();
        }
        if let Some(_val) = self._experimental() {
            _val.validate();
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self.title() {}
        if let Some(_val) = self._purpose() {
            _val.validate();
        }
        if let Some(_val) = self._case_sensitive() {
            _val.validate();
        }
        if let Some(_val) = self._hierarchy_meaning() {
            _val.validate();
        }
        if let Some(_val) = self.contact() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.compositional() {}
        if let Some(_val) = self._version_needed() {
            _val.validate();
        }
        if let Some(_val) = self.value_set() {}
        if let Some(_val) = self._content() {
            _val.validate();
        }
        if let Some(_val) = self.property() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.concept() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.content() {}
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.hierarchy_meaning() {}
        if let Some(_val) = self.version_needed() {}
        if let Some(_val) = self.purpose() {}
        if let Some(_val) = self._version() {
            _val.validate();
        }
        if let Some(_val) = self.experimental() {}
        if let Some(_val) = self._compositional() {
            _val.validate();
        }
        if let Some(_val) = self.copyright() {}
        if let Some(_val) = self.version() {}
        if let Some(_val) = self._publisher() {
            _val.validate();
        }
        if let Some(_val) = self.name() {}
        return true;
    }
}

#[derive(Debug)]
pub enum CodeSystemStatus {
    Draft,
    Active,
    Retired,
    Unknown,
}

impl CodeSystemStatus {
    pub fn from_string(string: &str) -> Option<CodeSystemStatus> {
        match string {
            "draft" => Some(CodeSystemStatus::Draft),
            "active" => Some(CodeSystemStatus::Active),
            "retired" => Some(CodeSystemStatus::Retired),
            "unknown" => Some(CodeSystemStatus::Unknown),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum CodeSystemContent {
    NotPresent,
    Example,
    Fragment,
    Complete,
    Supplement,
}

impl CodeSystemContent {
    pub fn from_string(string: &str) -> Option<CodeSystemContent> {
        match string {
            "not-present" => Some(CodeSystemContent::NotPresent),
            "example" => Some(CodeSystemContent::Example),
            "fragment" => Some(CodeSystemContent::Fragment),
            "complete" => Some(CodeSystemContent::Complete),
            "supplement" => Some(CodeSystemContent::Supplement),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum CodeSystemHierarchyMeaning {
    GroupedBy,
    IsA,
    PartOf,
    ClassifiedWith,
}

impl CodeSystemHierarchyMeaning {
    pub fn from_string(string: &str) -> Option<CodeSystemHierarchyMeaning> {
        match string {
            "grouped-by" => Some(CodeSystemHierarchyMeaning::GroupedBy),
            "is-a" => Some(CodeSystemHierarchyMeaning::IsA),
            "part-of" => Some(CodeSystemHierarchyMeaning::PartOf),
            "classified-with" => Some(CodeSystemHierarchyMeaning::ClassifiedWith),
            _ => None,
        }
    }
}
