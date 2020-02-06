#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Subscription_Channel::Subscription_Channel;
use crate::model::ResourceList::ResourceList;
use crate::model::Element::Element;
use crate::model::Narrative::Narrative;
use crate::model::Extension::Extension;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Meta::Meta;


/// The subscription resource is used to define a push-based subscription from a
/// server to another system. Once a subscription is registered with the server, the
/// server checks every resource that is created or updated, and if the resource
/// matches the given criteria, it sends a message on the defined "channel" so that
/// another system can take an appropriate action.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// The time for the server to turn the subscription off.
  end: Option<String>,

  /// The status of the subscription, which marks the server state for managing the
  /// subscription.
  status: Option<SubscriptionStatus>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// A description of why this subscription is defined.
  reason: Option<String>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// The rules that the server should use to determine when to generate notifications
  /// for this subscription.
  criteria: Option<String>,

  /// Extensions for error
  #[serde(rename = "_error")]
  _error: Option<Element>,

  /// Extensions for criteria
  #[serde(rename = "_criteria")]
  _criteria: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Extension>>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// Contact details for a human to contact about the subscription. The primary use
  /// of this for system administrator troubleshooting.
  contact: Option<Vec<ContactPoint>>,

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
  modifier_extension: Option<Vec<Extension>>,

  /// A record of the last error that occurred when the server processed a
  /// notification.
  error: Option<String>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// Details where to send notifications when resources are received that meet the
  /// criteria.
  channel: Subscription_Channel,

  /// Extensions for end
  #[serde(rename = "_end")]
  _end: Option<Element>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// Extensions for reason
  #[serde(rename = "_reason")]
  _reason: Option<Element>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscriptionStatus {
  #[serde(rename = "requested")]
  Requested,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "error")]
  Error,

  #[serde(rename = "off")]
  Off,

}
