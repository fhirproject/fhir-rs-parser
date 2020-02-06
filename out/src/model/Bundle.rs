#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Element::Element;
use crate::model::Bundle_Entry::Bundle_Entry;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Signature::Signature;
use crate::model::Bundle_Link::Bundle_Link;


/// A container for a collection of resources.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: String,

  /// Extensions for type
  _type: Element,

  /// Extensions for timestamp
  _timestamp: Element,

  /// An entry in a bundle resource - will either contain a resource or information
  /// about a resource (transactions and history only).
  entry: Vec<Bundle_Entry>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Extensions for language
  _language: Element,

  /// A persistent identifier for the bundle that won't change as a bundle is copied
  /// from server to server.
  identifier: Identifier,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Digital Signature - base64 encoded. XML-DSig or a JWT.
  signature: Signature,

  /// The base language in which the resource is written.
  language: String,

  /// Extensions for total
  _total: Element,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// A series of links that provide context to this bundle.
  link: Vec<Bundle_Link>,

  /// Indicates the purpose of this bundle - how it is intended to be used.
  #[serde(rename = "type")]
  fhir_type: BundleType,

  /// If a set of search matches, this is the total number of entries of type 'match'
  /// across all pages in the search.  It does not include search.mode = 'include' or
  /// 'outcome' entries and it does not provide a count of the number of entries in
  /// the Bundle.
  total: u32,

  /// The date/time that the bundle was assembled - i.e. when the resources were
  /// placed in the bundle.
  timestamp: i32,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum BundleType {
  #[serde(rename = "document")]
  Document,

  #[serde(rename = "message")]
  Message,

  #[serde(rename = "transaction")]
  Transaction,

  #[serde(rename = "transaction-response")]
  TransactionResponse,

  #[serde(rename = "batch")]
  Batch,

  #[serde(rename = "batch-response")]
  BatchResponse,

  #[serde(rename = "history")]
  History,

  #[serde(rename = "searchset")]
  Searchset,

  #[serde(rename = "collection")]
  Collection,

}
