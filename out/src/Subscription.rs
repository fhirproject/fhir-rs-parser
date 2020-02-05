use serde::{Deserialize, Serialize};

/// The subscription resource is used to define a push-based subscription from a
/// server to another system. Once a subscription is registered with the server, the
/// server checks every resource that is created or updated, and if the resource
/// matches the given criteria, it sends a message on the defined "channel" so that
/// another system can take an appropriate action.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Subscription {
  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// The rules that the server should use to determine when to generate notifications
  /// for this subscription.
  criteria: String,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Extensions for status
  _status: Element,

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

  /// The status of the subscription, which marks the server state for managing the
  /// subscription.
  status: SubscriptionStatus,

  /// The time for the server to turn the subscription off.
  end: instant,

  /// Extensions for criteria
  _criteria: Element,

  /// Extensions for error
  _error: Element,

  /// Contact details for a human to contact about the subscription. The primary use
  /// of this for system administrator troubleshooting.
  contact: Vec<ContactPoint>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// A description of why this subscription is defined.
  reason: String,

  /// Extensions for language
  _language: Element,

  /// Extensions for reason
  _reason: Element,

  /// A record of the last error that occurred when the server processed a
  /// notification.
  error: String,

  /// Extensions for end
  _end: Element,

  /// Details where to send notifications when resources are received that meet the
  /// criteria.
  channel: Subscription_Channel,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// The base language in which the resource is written.
  language: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

}

#[derive(Debug, Serialize, Deserialize)]
enum SubscriptionStatus {
  #[serde(rename = "requested")]
  Requested,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "error")]
  Error,

  #[serde(rename = "off")]
  Off,

}
