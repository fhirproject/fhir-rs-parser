#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Element::Element;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::Narrative::Narrative;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Meta::Meta;
use crate::model::Period::Period;
use crate::model::Coding::Coding;
use crate::model::Identifier::Identifier;


/// The technical details of an endpoint that can be used for electronic services,
/// such as for web services providing XDS.b or a REST endpoint for another FHIR
/// server. This may include any security context information.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Contact details for a human to contact about the subscription. The primary use
  /// of this for system administrator troubleshooting.
  contact: Option<Vec<ContactPoint>>,

  /// Additional headers / information to send as part of the notification.
  header: Option<Vec<String>>,

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
  #[serde(rename = "modifierExtension")]
  modifier_extension: Option<Vec<Box<Extension>>>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// The payload type describes the acceptable content that can be communicated on
  /// the endpoint.
  #[serde(rename = "payloadType")]
  payload_type: Vec<CodeableConcept>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// Extensions for payloadMimeType
  #[serde(rename = "_payloadMimeType")]
  _payload_mime_type: Option<Vec<Element>>,

  /// A friendly name that this endpoint can be referred to with.
  name: Option<String>,

  /// The mime type to send the payload in - e.g. application/fhir+xml,
  /// application/fhir+json. If the mime type is not specified, then the sender could
  /// send any content (including no content depending on the connectionType).
  #[serde(rename = "payloadMimeType")]
  payload_mime_type: Option<Vec<String>>,

  /// A coded value that represents the technical details of the usage of this
  /// endpoint, such as what WSDLs should be used in what way. (e.g. XDS.b/DICOM/cds-
  /// hook).
  #[serde(rename = "connectionType")]
  connection_type: Coding,

  /// The uri that describes the actual end-point to connect to.
  address: Option<String>,

  /// active | suspended | error | off | test.
  status: Option<EndpointStatus>,

  /// Extensions for header
  #[serde(rename = "_header")]
  _header: Option<Vec<Element>>,

  /// Identifier for the organization that is used to identify the endpoint across
  /// multiple disparate systems.
  identifier: Option<Vec<Identifier>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// Extensions for address
  #[serde(rename = "_address")]
  _address: Option<Element>,

  /// The organization that manages this endpoint (even if technically another
  /// organization is hosting this in the cloud, it is the organization associated
  /// with the data).
  #[serde(rename = "managingOrganization")]
  managing_organization: Option<Box<Reference>>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// The interval during which the endpoint is expected to be operational.
  period: Option<Period>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum EndpointStatus {
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
