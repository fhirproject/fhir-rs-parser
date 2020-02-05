use serde::{Deserialize, Serialize};

/// The technical details of an endpoint that can be used for electronic services,
/// such as for web services providing XDS.b or a REST endpoint for another FHIR
/// server. This may include any security context information.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Endpoint {
  /// Extensions for language
  _language: Element,

  /// Contact details for a human to contact about the subscription. The primary use
  /// of this for system administrator troubleshooting.
  contact: Vec<ContactPoint>,

  /// The payload type describes the acceptable content that can be communicated on
  /// the endpoint.
  #[serde(rename = "payloadType")]
  payload_type: Vec<CodeableConcept>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// The base language in which the resource is written.
  language: String,

  /// active | suspended | error | off | test.
  status: EndpointStatus,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// Additional headers / information to send as part of the notification.
  header: Vec<String>,

  /// Extensions for header
  _header: Vec<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The organization that manages this endpoint (even if technically another
  /// organization is hosting this in the cloud, it is the organization associated
  /// with the data).
  #[serde(rename = "managingOrganization")]
  managing_organization: Reference,

  /// The mime type to send the payload in - e.g. application/fhir+xml,
  /// application/fhir+json. If the mime type is not specified, then the sender could
  /// send any content (including no content depending on the connectionType).
  #[serde(rename = "payloadMimeType")]
  payload_mime_type: Vec<String>,

  /// Extensions for address
  _address: Element,

  /// A friendly name that this endpoint can be referred to with.
  name: String,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

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

  /// The interval during which the endpoint is expected to be operational.
  period: Period,

  /// Extensions for status
  _status: Element,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Identifier for the organization that is used to identify the endpoint across
  /// multiple disparate systems.
  identifier: Vec<Identifier>,

  /// A coded value that represents the technical details of the usage of this
  /// endpoint, such as what WSDLs should be used in what way. (e.g. XDS.b/DICOM/cds-
  /// hook).
  #[serde(rename = "connectionType")]
  connection_type: Coding,

  /// The uri that describes the actual end-point to connect to.
  address: url,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Extensions for name
  _name: Element,

  /// Extensions for payloadMimeType
  #[serde(rename = "_payloadMimeType")]
  _payload_mime_type: Vec<Element>,

}

#[derive(Debug, Serialize, Deserialize)]
enum EndpointStatus {
  #[serde(rename = "active")]
  Active,

  #[serde(rename = "suspended")]
  Suspended,

  #[serde(rename = "error")]
  Error,

  #[serde(rename = "off")]
  Off,

  #[serde(rename = "entered-in-error")]
  EnteredInError,

  #[serde(rename = "test")]
  Test,

}
