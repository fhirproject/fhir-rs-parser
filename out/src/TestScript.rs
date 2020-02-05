use serde::{Deserialize, Serialize};

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TestScript {
  /// An abstract server used in operations within this test script in the origin
  /// element.
  origin: Vec<TestScript_Origin>,

  /// A free text natural language description of the test script from a consumer's
  /// perspective.
  description: markdown,

  /// A formal identifier that is used to identify this test script when it is
  /// represented in other formats, or referenced in a specification, model, design or
  /// an instance.
  identifier: Identifier,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Extensions for publisher
  _publisher: Element,

  /// Extensions for url
  _url: Element,

  /// The base language in which the resource is written.
  language: String,

  /// Extensions for title
  _title: Element,

  /// Extensions for purpose
  _purpose: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Variable is set based either on element value in response body or on header
  /// field value in the response headers.
  variable: Vec<TestScript_Variable>,

  /// A short, descriptive, user-friendly title for the test script.
  title: String,

  /// A natural language name identifying the test script. This name should be usable
  /// as an identifier for the module by machine processing applications such as code
  /// generation.
  name: String,

  /// Extensions for name
  _name: Element,

  /// Extensions for description
  _description: Element,

  /// A series of required setup operations before tests are executed.
  setup: TestScript_Setup,

  /// A copyright statement relating to the test script and/or its contents. Copyright
  /// statements are generally legal restrictions on the use and publishing of the
  /// test script.
  copyright: markdown,

  /// Extensions for version
  _version: Element,

  /// The status of this test script. Enables tracking the life-cycle of the content.
  status: TestScriptStatus,

  /// Extensions for experimental
  _experimental: Element,

  /// A Boolean value to indicate that this test script is authored for testing
  /// purposes (or education/evaluation/marketing) and is not intended to be used for
  /// genuine usage.
  experimental: bool,

  /// Fixture in the test script - by reference (uri). All fixtures are required for
  /// the test script to execute.
  fixture: Vec<TestScript_Fixture>,

  /// A test in this script.
  test: Vec<TestScript_Test>,

  /// Extensions for date
  _date: Element,

  /// The name of the organization or individual that published the test script.
  publisher: String,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// The identifier that is used to identify this version of the test script when it
  /// is referenced in a specification, model, design or instance. This is an
  /// arbitrary value managed by the test script author and is not expected to be
  /// globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
  /// managed version is not available. There is also no expectation that versions can
  /// be placed in a lexicographical sequence.
  version: String,

  /// Extensions for language
  _language: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// Explanation of why this test script is needed and why it has been designed as it
  /// has.
  purpose: markdown,

  /// The date  (and optionally time) when the test script was published. The date
  /// must change when the business version changes and it must change if the status
  /// code changes. In addition, it should change when the substantive content of the
  /// test script changes.
  date: dateTime,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The required capability must exist and are assumed to function correctly on the
  /// FHIR server being tested.
  metadata: TestScript_Metadata,

  /// Extensions for status
  _status: Element,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  contact: Vec<ContactDetail>,

  /// Extensions for copyright
  _copyright: Element,

  /// A series of operations required to clean up after all the tests are executed
  /// (successfully or otherwise).
  teardown: TestScript_Teardown,

  /// A legal or geographic region in which the test script is intended to be used.
  jurisdiction: Vec<CodeableConcept>,

  /// An abstract server used in operations within this test script in the destination
  /// element.
  destination: Vec<TestScript_Destination>,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate test script
  /// instances.
  #[serde(rename = "useContext")]
  use_context: Vec<UsageContext>,

  /// An absolute URI that is used to identify this test script when it is referenced
  /// in a specification, model, design or an instance; also called its canonical
  /// identifier. This SHOULD be globally unique and SHOULD be a literal address at
  /// which at which an authoritative instance of this test script is (or will be)
  /// published. This URL can be the target of a canonical reference. It SHALL remain
  /// the same when the test script is stored on different servers.
  url: String,

  /// Reference to the profile to be used for validation.
  profile: Vec<Reference>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

}

#[derive(Debug, Serialize, Deserialize)]
enum TestScriptStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "unknown")]
  Unknown,

}
