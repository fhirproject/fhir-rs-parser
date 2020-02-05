use serde::{Deserialize, Serialize};

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used to
/// gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImplementationGuide {
  /// The information needed by an IG publisher tool to publish the whole
  /// implementation guide.
  definition: ImplementationGuide_Definition,

  /// Extensions for version
  _version: Element,

  /// Extensions for license
  _license: Element,

  /// Extensions for fhirVersion
  #[serde(rename = "_fhirVersion")]
  _fhir_version: Vec<Element>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// Extensions for copyright
  _copyright: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for experimental
  _experimental: Element,

  /// Extensions for url
  _url: Element,

  /// The base language in which the resource is written.
  language: String,

  /// The name of the organization or individual that published the implementation
  /// guide.
  publisher: String,

  /// The identifier that is used to identify this version of the implementation guide
  /// when it is referenced in a specification, model, design or instance. This is an
  /// arbitrary value managed by the implementation guide author and is not expected
  /// to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
  /// managed version is not available. There is also no expectation that versions can
  /// be placed in a lexicographical sequence.
  version: String,

  /// Another implementation guide that this implementation depends on. Typically, an
  /// implementation guide uses value sets, profiles etc.defined in other
  /// implementation guides.
  #[serde(rename = "dependsOn")]
  depends_on: Vec<ImplementationGuide_DependsOn>,

  /// Information about an assembled implementation guide, created by the publication
  /// tooling.
  manifest: ImplementationGuide_Manifest,

  /// A set of profiles that all resources covered by this implementation guide must
  /// conform to.
  global: Vec<ImplementationGuide_Global>,

  /// A copyright statement relating to the implementation guide and/or its contents.
  /// Copyright statements are generally legal restrictions on the use and publishing
  /// of the implementation guide.
  copyright: markdown,

  /// A natural language name identifying the implementation guide. This name should
  /// be usable as an identifier for the module by machine processing applications
  /// such as code generation.
  name: String,

  /// The status of this implementation guide. Enables tracking the life-cycle of the
  /// content.
  status: ImplementationGuideStatus,

  /// Extensions for title
  _title: Element,

  /// Extensions for packageId
  #[serde(rename = "_packageId")]
  _package_id: Element,

  /// The NPM package name for this Implementation Guide, used in the NPM package
  /// distribution, which is the primary mechanism by which FHIR based tooling manages
  /// IG dependencies. This value must be globally unique, and should be assigned with
  /// care.
  #[serde(rename = "packageId")]
  package_id: id,

  /// Extensions for name
  _name: Element,

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

  /// Extensions for description
  _description: Element,

  /// The date  (and optionally time) when the implementation guide was published. The
  /// date must change when the business version changes and it must change if the
  /// status code changes. In addition, it should change when the substantive content
  /// of the implementation guide changes.
  date: dateTime,

  /// A legal or geographic region in which the implementation guide is intended to be
  /// used.
  jurisdiction: Vec<CodeableConcept>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// An absolute URI that is used to identify this implementation guide when it is
  /// referenced in a specification, model, design or an instance; also called its
  /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
  /// address at which at which an authoritative instance of this implementation guide
  /// is (or will be) published. This URL can be the target of a canonical reference.
  /// It SHALL remain the same when the implementation guide is stored on different
  /// servers.
  url: String,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  contact: Vec<ContactDetail>,

  /// A Boolean value to indicate that this implementation guide is authored for
  /// testing purposes (or education/evaluation/marketing) and is not intended to be
  /// used for genuine usage.
  experimental: bool,

  /// Extensions for publisher
  _publisher: Element,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate implementation
  /// guide instances.
  #[serde(rename = "useContext")]
  use_context: Vec<UsageContext>,

  /// Extensions for status
  _status: Element,

  /// Extensions for date
  _date: Element,

  /// The license that applies to this Implementation Guide, using an SPDX license
  /// code, or 'not-open-source'.
  license: ImplementationGuideLicense,

  /// A short, descriptive, user-friendly title for the implementation guide.
  title: String,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Extensions for language
  _language: Element,

  /// A free text natural language description of the implementation guide from a
  /// consumer's perspective.
  description: markdown,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

}

#[derive(Debug, Serialize, Deserialize)]
enum ImplementationGuideStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "unknown")]
  Unknown,

}

#[derive(Debug, Serialize, Deserialize)]
enum ImplementationGuideLicense {
  #[serde(rename = "not-open-source")]
  NotOpenSource,

  #[serde(rename = "0BSD")]
  0BSD,

  #[serde(rename = "AAL")]
  AAL,

  #[serde(rename = "Abstyles")]
  Abstyles,

  #[serde(rename = "Adobe-2006")]
  Adobe2006,

  #[serde(rename = "Adobe-Glyph")]
  AdobeGlyph,

  #[serde(rename = "ADSL")]
  ADSL,

  #[serde(rename = "AFL-1.1")]
  Afl1.1,

  #[serde(rename = "AFL-1.2")]
  Afl1.2,

  #[serde(rename = "AFL-2.0")]
  Afl2.0,

  #[serde(rename = "AFL-2.1")]
  Afl2.1,

  #[serde(rename = "AFL-3.0")]
  Afl3.0,

  #[serde(rename = "Afmparse")]
  Afmparse,

  #[serde(rename = "AGPL-1.0-only")]
  Agpl1.0Only,

  #[serde(rename = "AGPL-1.0-or-later")]
  Agpl1.0OrLater,

  #[serde(rename = "AGPL-3.0-only")]
  Agpl3.0Only,

  #[serde(rename = "AGPL-3.0-or-later")]
  Agpl3.0OrLater,

  #[serde(rename = "Aladdin")]
  Aladdin,

  #[serde(rename = "AMDPLPA")]
  AMDPLPA,

  #[serde(rename = "AML")]
  AML,

  #[serde(rename = "AMPAS")]
  AMPAS,

  #[serde(rename = "ANTLR-PD")]
  AntlrPd,

  #[serde(rename = "Apache-1.0")]
  Apache1.0,

  #[serde(rename = "Apache-1.1")]
  Apache1.1,

  #[serde(rename = "Apache-2.0")]
  Apache2.0,

  #[serde(rename = "APAFML")]
  APAFML,

  #[serde(rename = "APL-1.0")]
  Apl1.0,

  #[serde(rename = "APSL-1.0")]
  Apsl1.0,

  #[serde(rename = "APSL-1.1")]
  Apsl1.1,

  #[serde(rename = "APSL-1.2")]
  Apsl1.2,

  #[serde(rename = "APSL-2.0")]
  Apsl2.0,

  #[serde(rename = "Artistic-1.0-cl8")]
  Artistic1.0Cl8,

  #[serde(rename = "Artistic-1.0-Perl")]
  Artistic1.0Perl,

  #[serde(rename = "Artistic-1.0")]
  Artistic1.0,

  #[serde(rename = "Artistic-2.0")]
  Artistic2.0,

  #[serde(rename = "Bahyph")]
  Bahyph,

  #[serde(rename = "Barr")]
  Barr,

  #[serde(rename = "Beerware")]
  Beerware,

  #[serde(rename = "BitTorrent-1.0")]
  Bittorrent1.0,

  #[serde(rename = "BitTorrent-1.1")]
  Bittorrent1.1,

  #[serde(rename = "Borceux")]
  Borceux,

  #[serde(rename = "BSD-1-Clause")]
  Bsd1Clause,

  #[serde(rename = "BSD-2-Clause-FreeBSD")]
  Bsd2ClauseFreebsd,

  #[serde(rename = "BSD-2-Clause-NetBSD")]
  Bsd2ClauseNetbsd,

  #[serde(rename = "BSD-2-Clause-Patent")]
  Bsd2ClausePatent,

  #[serde(rename = "BSD-2-Clause")]
  Bsd2Clause,

  #[serde(rename = "BSD-3-Clause-Attribution")]
  Bsd3ClauseAttribution,

  #[serde(rename = "BSD-3-Clause-Clear")]
  Bsd3ClauseClear,

  #[serde(rename = "BSD-3-Clause-LBNL")]
  Bsd3ClauseLbnl,

  #[serde(rename = "BSD-3-Clause-No-Nuclear-License-2014")]
  Bsd3ClauseNoNuclearLicense2014,

  #[serde(rename = "BSD-3-Clause-No-Nuclear-License")]
  Bsd3ClauseNoNuclearLicense,

  #[serde(rename = "BSD-3-Clause-No-Nuclear-Warranty")]
  Bsd3ClauseNoNuclearWarranty,

  #[serde(rename = "BSD-3-Clause")]
  Bsd3Clause,

  #[serde(rename = "BSD-4-Clause-UC")]
  Bsd4ClauseUc,

  #[serde(rename = "BSD-4-Clause")]
  Bsd4Clause,

  #[serde(rename = "BSD-Protection")]
  BsdProtection,

  #[serde(rename = "BSD-Source-Code")]
  BsdSourceCode,

  #[serde(rename = "BSL-1.0")]
  Bsl1.0,

  #[serde(rename = "bzip2-1.0.5")]
  Bzip21.0.5,

  #[serde(rename = "bzip2-1.0.6")]
  Bzip21.0.6,

  #[serde(rename = "Caldera")]
  Caldera,

  #[serde(rename = "CATOSL-1.1")]
  Catosl1.1,

  #[serde(rename = "CC-BY-1.0")]
  CcBy1.0,

  #[serde(rename = "CC-BY-2.0")]
  CcBy2.0,

  #[serde(rename = "CC-BY-2.5")]
  CcBy2.5,

  #[serde(rename = "CC-BY-3.0")]
  CcBy3.0,

  #[serde(rename = "CC-BY-4.0")]
  CcBy4.0,

  #[serde(rename = "CC-BY-NC-1.0")]
  CcByNc1.0,

  #[serde(rename = "CC-BY-NC-2.0")]
  CcByNc2.0,

  #[serde(rename = "CC-BY-NC-2.5")]
  CcByNc2.5,

  #[serde(rename = "CC-BY-NC-3.0")]
  CcByNc3.0,

  #[serde(rename = "CC-BY-NC-4.0")]
  CcByNc4.0,

  #[serde(rename = "CC-BY-NC-ND-1.0")]
  CcByNcNd1.0,

  #[serde(rename = "CC-BY-NC-ND-2.0")]
  CcByNcNd2.0,

  #[serde(rename = "CC-BY-NC-ND-2.5")]
  CcByNcNd2.5,

  #[serde(rename = "CC-BY-NC-ND-3.0")]
  CcByNcNd3.0,

  #[serde(rename = "CC-BY-NC-ND-4.0")]
  CcByNcNd4.0,

  #[serde(rename = "CC-BY-NC-SA-1.0")]
  CcByNcSa1.0,

  #[serde(rename = "CC-BY-NC-SA-2.0")]
  CcByNcSa2.0,

  #[serde(rename = "CC-BY-NC-SA-2.5")]
  CcByNcSa2.5,

  #[serde(rename = "CC-BY-NC-SA-3.0")]
  CcByNcSa3.0,

  #[serde(rename = "CC-BY-NC-SA-4.0")]
  CcByNcSa4.0,

  #[serde(rename = "CC-BY-ND-1.0")]
  CcByNd1.0,

  #[serde(rename = "CC-BY-ND-2.0")]
  CcByNd2.0,

  #[serde(rename = "CC-BY-ND-2.5")]
  CcByNd2.5,

  #[serde(rename = "CC-BY-ND-3.0")]
  CcByNd3.0,

  #[serde(rename = "CC-BY-ND-4.0")]
  CcByNd4.0,

  #[serde(rename = "CC-BY-SA-1.0")]
  CcBySa1.0,

  #[serde(rename = "CC-BY-SA-2.0")]
  CcBySa2.0,

  #[serde(rename = "CC-BY-SA-2.5")]
  CcBySa2.5,

  #[serde(rename = "CC-BY-SA-3.0")]
  CcBySa3.0,

  #[serde(rename = "CC-BY-SA-4.0")]
  CcBySa4.0,

  #[serde(rename = "CC0-1.0")]
  Cc01.0,

  #[serde(rename = "CDDL-1.0")]
  Cddl1.0,

  #[serde(rename = "CDDL-1.1")]
  Cddl1.1,

  #[serde(rename = "CDLA-Permissive-1.0")]
  CdlaPermissive1.0,

  #[serde(rename = "CDLA-Sharing-1.0")]
  CdlaSharing1.0,

  #[serde(rename = "CECILL-1.0")]
  Cecill1.0,

  #[serde(rename = "CECILL-1.1")]
  Cecill1.1,

  #[serde(rename = "CECILL-2.0")]
  Cecill2.0,

  #[serde(rename = "CECILL-2.1")]
  Cecill2.1,

  #[serde(rename = "CECILL-B")]
  CecillB,

  #[serde(rename = "CECILL-C")]
  CecillC,

  #[serde(rename = "ClArtistic")]
  ClArtistic,

  #[serde(rename = "CNRI-Jython")]
  CnriJython,

  #[serde(rename = "CNRI-Python-GPL-Compatible")]
  CnriPythonGplCompatible,

  #[serde(rename = "CNRI-Python")]
  CnriPython,

  #[serde(rename = "Condor-1.1")]
  Condor1.1,

  #[serde(rename = "CPAL-1.0")]
  Cpal1.0,

  #[serde(rename = "CPL-1.0")]
  Cpl1.0,

  #[serde(rename = "CPOL-1.02")]
  Cpol1.02,

  #[serde(rename = "Crossword")]
  Crossword,

  #[serde(rename = "CrystalStacker")]
  CrystalStacker,

  #[serde(rename = "CUA-OPL-1.0")]
  CuaOpl1.0,

  #[serde(rename = "Cube")]
  Cube,

  #[serde(rename = "curl")]
  Curl,

  #[serde(rename = "D-FSL-1.0")]
  DFsl1.0,

  #[serde(rename = "diffmark")]
  Diffmark,

  #[serde(rename = "DOC")]
  DOC,

  #[serde(rename = "Dotseqn")]
  Dotseqn,

  #[serde(rename = "DSDP")]
  DSDP,

  #[serde(rename = "dvipdfm")]
  Dvipdfm,

  #[serde(rename = "ECL-1.0")]
  Ecl1.0,

  #[serde(rename = "ECL-2.0")]
  Ecl2.0,

  #[serde(rename = "EFL-1.0")]
  Efl1.0,

  #[serde(rename = "EFL-2.0")]
  Efl2.0,

  #[serde(rename = "eGenix")]
  EGenix,

  #[serde(rename = "Entessa")]
  Entessa,

  #[serde(rename = "EPL-1.0")]
  Epl1.0,

  #[serde(rename = "EPL-2.0")]
  Epl2.0,

  #[serde(rename = "ErlPL-1.1")]
  Erlpl1.1,

  #[serde(rename = "EUDatagrid")]
  EUDatagrid,

  #[serde(rename = "EUPL-1.0")]
  Eupl1.0,

  #[serde(rename = "EUPL-1.1")]
  Eupl1.1,

  #[serde(rename = "EUPL-1.2")]
  Eupl1.2,

  #[serde(rename = "Eurosym")]
  Eurosym,

  #[serde(rename = "Fair")]
  Fair,

  #[serde(rename = "Frameworx-1.0")]
  Frameworx1.0,

  #[serde(rename = "FreeImage")]
  FreeImage,

  #[serde(rename = "FSFAP")]
  FSFAP,

  #[serde(rename = "FSFUL")]
  FSFUL,

  #[serde(rename = "FSFULLR")]
  FSFULLR,

  #[serde(rename = "FTL")]
  FTL,

  #[serde(rename = "GFDL-1.1-only")]
  Gfdl1.1Only,

  #[serde(rename = "GFDL-1.1-or-later")]
  Gfdl1.1OrLater,

  #[serde(rename = "GFDL-1.2-only")]
  Gfdl1.2Only,

  #[serde(rename = "GFDL-1.2-or-later")]
  Gfdl1.2OrLater,

  #[serde(rename = "GFDL-1.3-only")]
  Gfdl1.3Only,

  #[serde(rename = "GFDL-1.3-or-later")]
  Gfdl1.3OrLater,

  #[serde(rename = "Giftware")]
  Giftware,

  #[serde(rename = "GL2PS")]
  GL2PS,

  #[serde(rename = "Glide")]
  Glide,

  #[serde(rename = "Glulxe")]
  Glulxe,

  #[serde(rename = "gnuplot")]
  Gnuplot,

  #[serde(rename = "GPL-1.0-only")]
  Gpl1.0Only,

  #[serde(rename = "GPL-1.0-or-later")]
  Gpl1.0OrLater,

  #[serde(rename = "GPL-2.0-only")]
  Gpl2.0Only,

  #[serde(rename = "GPL-2.0-or-later")]
  Gpl2.0OrLater,

  #[serde(rename = "GPL-3.0-only")]
  Gpl3.0Only,

  #[serde(rename = "GPL-3.0-or-later")]
  Gpl3.0OrLater,

  #[serde(rename = "gSOAP-1.3b")]
  Gsoap1.3b,

  #[serde(rename = "HaskellReport")]
  HaskellReport,

  #[serde(rename = "HPND")]
  HPND,

  #[serde(rename = "IBM-pibs")]
  IbmPibs,

  #[serde(rename = "ICU")]
  ICU,

  #[serde(rename = "IJG")]
  IJG,

  #[serde(rename = "ImageMagick")]
  ImageMagick,

  #[serde(rename = "iMatix")]
  IMatix,

  #[serde(rename = "Imlib2")]
  Imlib2,

  #[serde(rename = "Info-ZIP")]
  InfoZip,

  #[serde(rename = "Intel-ACPI")]
  IntelAcpi,

  #[serde(rename = "Intel")]
  Intel,

  #[serde(rename = "Interbase-1.0")]
  Interbase1.0,

  #[serde(rename = "IPA")]
  IPA,

  #[serde(rename = "IPL-1.0")]
  Ipl1.0,

  #[serde(rename = "ISC")]
  ISC,

  #[serde(rename = "JasPer-2.0")]
  Jasper2.0,

  #[serde(rename = "JSON")]
  JSON,

  #[serde(rename = "LAL-1.2")]
  Lal1.2,

  #[serde(rename = "LAL-1.3")]
  Lal1.3,

  #[serde(rename = "Latex2e")]
  Latex2e,

  #[serde(rename = "Leptonica")]
  Leptonica,

  #[serde(rename = "LGPL-2.0-only")]
  Lgpl2.0Only,

  #[serde(rename = "LGPL-2.0-or-later")]
  Lgpl2.0OrLater,

  #[serde(rename = "LGPL-2.1-only")]
  Lgpl2.1Only,

  #[serde(rename = "LGPL-2.1-or-later")]
  Lgpl2.1OrLater,

  #[serde(rename = "LGPL-3.0-only")]
  Lgpl3.0Only,

  #[serde(rename = "LGPL-3.0-or-later")]
  Lgpl3.0OrLater,

  #[serde(rename = "LGPLLR")]
  LGPLLR,

  #[serde(rename = "Libpng")]
  Libpng,

  #[serde(rename = "libtiff")]
  Libtiff,

  #[serde(rename = "LiLiQ-P-1.1")]
  LiliqP1.1,

  #[serde(rename = "LiLiQ-R-1.1")]
  LiliqR1.1,

  #[serde(rename = "LiLiQ-Rplus-1.1")]
  LiliqRplus1.1,

  #[serde(rename = "Linux-OpenIB")]
  LinuxOpenib,

  #[serde(rename = "LPL-1.0")]
  Lpl1.0,

  #[serde(rename = "LPL-1.02")]
  Lpl1.02,

  #[serde(rename = "LPPL-1.0")]
  Lppl1.0,

  #[serde(rename = "LPPL-1.1")]
  Lppl1.1,

  #[serde(rename = "LPPL-1.2")]
  Lppl1.2,

  #[serde(rename = "LPPL-1.3a")]
  Lppl1.3a,

  #[serde(rename = "LPPL-1.3c")]
  Lppl1.3c,

  #[serde(rename = "MakeIndex")]
  MakeIndex,

  #[serde(rename = "MirOS")]
  MirOS,

  #[serde(rename = "MIT-0")]
  Mit0,

  #[serde(rename = "MIT-advertising")]
  MitAdvertising,

  #[serde(rename = "MIT-CMU")]
  MitCmu,

  #[serde(rename = "MIT-enna")]
  MitEnna,

  #[serde(rename = "MIT-feh")]
  MitFeh,

  #[serde(rename = "MIT")]
  MIT,

  #[serde(rename = "MITNFA")]
  MITNFA,

  #[serde(rename = "Motosoto")]
  Motosoto,

  #[serde(rename = "mpich2")]
  Mpich2,

  #[serde(rename = "MPL-1.0")]
  Mpl1.0,

  #[serde(rename = "MPL-1.1")]
  Mpl1.1,

  #[serde(rename = "MPL-2.0-no-copyleft-exception")]
  Mpl2.0NoCopyleftException,

  #[serde(rename = "MPL-2.0")]
  Mpl2.0,

  #[serde(rename = "MS-PL")]
  MsPl,

  #[serde(rename = "MS-RL")]
  MsRl,

  #[serde(rename = "MTLL")]
  MTLL,

  #[serde(rename = "Multics")]
  Multics,

  #[serde(rename = "Mup")]
  Mup,

  #[serde(rename = "NASA-1.3")]
  Nasa1.3,

  #[serde(rename = "Naumen")]
  Naumen,

  #[serde(rename = "NBPL-1.0")]
  Nbpl1.0,

  #[serde(rename = "NCSA")]
  NCSA,

  #[serde(rename = "Net-SNMP")]
  NetSnmp,

  #[serde(rename = "NetCDF")]
  NetCDF,

  #[serde(rename = "Newsletr")]
  Newsletr,

  #[serde(rename = "NGPL")]
  NGPL,

  #[serde(rename = "NLOD-1.0")]
  Nlod1.0,

  #[serde(rename = "NLPL")]
  NLPL,

  #[serde(rename = "Nokia")]
  Nokia,

  #[serde(rename = "NOSL")]
  NOSL,

  #[serde(rename = "Noweb")]
  Noweb,

  #[serde(rename = "NPL-1.0")]
  Npl1.0,

  #[serde(rename = "NPL-1.1")]
  Npl1.1,

  #[serde(rename = "NPOSL-3.0")]
  Nposl3.0,

  #[serde(rename = "NRL")]
  NRL,

  #[serde(rename = "NTP")]
  NTP,

  #[serde(rename = "OCCT-PL")]
  OcctPl,

  #[serde(rename = "OCLC-2.0")]
  Oclc2.0,

  #[serde(rename = "ODbL-1.0")]
  Odbl1.0,

  #[serde(rename = "OFL-1.0")]
  Ofl1.0,

  #[serde(rename = "OFL-1.1")]
  Ofl1.1,

  #[serde(rename = "OGTSL")]
  OGTSL,

  #[serde(rename = "OLDAP-1.1")]
  Oldap1.1,

  #[serde(rename = "OLDAP-1.2")]
  Oldap1.2,

  #[serde(rename = "OLDAP-1.3")]
  Oldap1.3,

  #[serde(rename = "OLDAP-1.4")]
  Oldap1.4,

  #[serde(rename = "OLDAP-2.0.1")]
  Oldap2.0.1,

  #[serde(rename = "OLDAP-2.0")]
  Oldap2.0,

  #[serde(rename = "OLDAP-2.1")]
  Oldap2.1,

  #[serde(rename = "OLDAP-2.2.1")]
  Oldap2.2.1,

  #[serde(rename = "OLDAP-2.2.2")]
  Oldap2.2.2,

  #[serde(rename = "OLDAP-2.2")]
  Oldap2.2,

  #[serde(rename = "OLDAP-2.3")]
  Oldap2.3,

  #[serde(rename = "OLDAP-2.4")]
  Oldap2.4,

  #[serde(rename = "OLDAP-2.5")]
  Oldap2.5,

  #[serde(rename = "OLDAP-2.6")]
  Oldap2.6,

  #[serde(rename = "OLDAP-2.7")]
  Oldap2.7,

  #[serde(rename = "OLDAP-2.8")]
  Oldap2.8,

  #[serde(rename = "OML")]
  OML,

  #[serde(rename = "OpenSSL")]
  OpenSSL,

  #[serde(rename = "OPL-1.0")]
  Opl1.0,

  #[serde(rename = "OSET-PL-2.1")]
  OsetPl2.1,

  #[serde(rename = "OSL-1.0")]
  Osl1.0,

  #[serde(rename = "OSL-1.1")]
  Osl1.1,

  #[serde(rename = "OSL-2.0")]
  Osl2.0,

  #[serde(rename = "OSL-2.1")]
  Osl2.1,

  #[serde(rename = "OSL-3.0")]
  Osl3.0,

  #[serde(rename = "PDDL-1.0")]
  Pddl1.0,

  #[serde(rename = "PHP-3.0")]
  Php3.0,

  #[serde(rename = "PHP-3.01")]
  Php3.01,

  #[serde(rename = "Plexus")]
  Plexus,

  #[serde(rename = "PostgreSQL")]
  PostgreSQL,

  #[serde(rename = "psfrag")]
  Psfrag,

  #[serde(rename = "psutils")]
  Psutils,

  #[serde(rename = "Python-2.0")]
  Python2.0,

  #[serde(rename = "Qhull")]
  Qhull,

  #[serde(rename = "QPL-1.0")]
  Qpl1.0,

  #[serde(rename = "Rdisc")]
  Rdisc,

  #[serde(rename = "RHeCos-1.1")]
  Rhecos1.1,

  #[serde(rename = "RPL-1.1")]
  Rpl1.1,

  #[serde(rename = "RPL-1.5")]
  Rpl1.5,

  #[serde(rename = "RPSL-1.0")]
  Rpsl1.0,

  #[serde(rename = "RSA-MD")]
  RsaMd,

  #[serde(rename = "RSCPL")]
  RSCPL,

  #[serde(rename = "Ruby")]
  Ruby,

  #[serde(rename = "SAX-PD")]
  SaxPd,

  #[serde(rename = "Saxpath")]
  Saxpath,

  #[serde(rename = "SCEA")]
  SCEA,

  #[serde(rename = "Sendmail")]
  Sendmail,

  #[serde(rename = "SGI-B-1.0")]
  SgiB1.0,

  #[serde(rename = "SGI-B-1.1")]
  SgiB1.1,

  #[serde(rename = "SGI-B-2.0")]
  SgiB2.0,

  #[serde(rename = "SimPL-2.0")]
  Simpl2.0,

  #[serde(rename = "SISSL-1.2")]
  Sissl1.2,

  #[serde(rename = "SISSL")]
  SISSL,

  #[serde(rename = "Sleepycat")]
  Sleepycat,

  #[serde(rename = "SMLNJ")]
  SMLNJ,

  #[serde(rename = "SMPPL")]
  SMPPL,

  #[serde(rename = "SNIA")]
  SNIA,

  #[serde(rename = "Spencer-86")]
  Spencer86,

  #[serde(rename = "Spencer-94")]
  Spencer94,

  #[serde(rename = "Spencer-99")]
  Spencer99,

  #[serde(rename = "SPL-1.0")]
  Spl1.0,

  #[serde(rename = "SugarCRM-1.1.3")]
  Sugarcrm1.1.3,

  #[serde(rename = "SWL")]
  SWL,

  #[serde(rename = "TCL")]
  TCL,

  #[serde(rename = "TCP-wrappers")]
  TcpWrappers,

  #[serde(rename = "TMate")]
  TMate,

  #[serde(rename = "TORQUE-1.1")]
  Torque1.1,

  #[serde(rename = "TOSL")]
  TOSL,

  #[serde(rename = "Unicode-DFS-2015")]
  UnicodeDfs2015,

  #[serde(rename = "Unicode-DFS-2016")]
  UnicodeDfs2016,

  #[serde(rename = "Unicode-TOU")]
  UnicodeTou,

  #[serde(rename = "Unlicense")]
  Unlicense,

  #[serde(rename = "UPL-1.0")]
  Upl1.0,

  #[serde(rename = "Vim")]
  Vim,

  #[serde(rename = "VOSTROM")]
  VOSTROM,

  #[serde(rename = "VSL-1.0")]
  Vsl1.0,

  #[serde(rename = "W3C-19980720")]
  W3c19980720,

  #[serde(rename = "W3C-20150513")]
  W3c20150513,

  #[serde(rename = "W3C")]
  W3C,

  #[serde(rename = "Watcom-1.0")]
  Watcom1.0,

  #[serde(rename = "Wsuipa")]
  Wsuipa,

  #[serde(rename = "WTFPL")]
  WTFPL,

  #[serde(rename = "X11")]
  X11,

  #[serde(rename = "Xerox")]
  Xerox,

  #[serde(rename = "XFree86-1.1")]
  Xfree861.1,

  #[serde(rename = "xinetd")]
  Xinetd,

  #[serde(rename = "Xnet")]
  Xnet,

  #[serde(rename = "xpp")]
  Xpp,

  #[serde(rename = "XSkat")]
  XSkat,

  #[serde(rename = "YPL-1.0")]
  Ypl1.0,

  #[serde(rename = "YPL-1.1")]
  Ypl1.1,

  #[serde(rename = "Zed")]
  Zed,

  #[serde(rename = "Zend-2.0")]
  Zend2.0,

  #[serde(rename = "Zimbra-1.3")]
  Zimbra1.3,

  #[serde(rename = "Zimbra-1.4")]
  Zimbra1.4,

  #[serde(rename = "zlib-acknowledgement")]
  ZlibAcknowledgement,

  #[serde(rename = "Zlib")]
  Zlib,

  #[serde(rename = "ZPL-1.1")]
  Zpl1.1,

  #[serde(rename = "ZPL-2.0")]
  Zpl2.0,

  #[serde(rename = "ZPL-2.1")]
  Zpl2.1,

}
