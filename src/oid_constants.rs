// #![allow(clippy::all)]
// #![allow(non_upper_case_globals)] // In case some OID names from the file are not strictly uppercase

use asn1_rs::{Oid, oid};

/// User ID
pub const OID_USERID: Oid<'static> = oid!{0.9.2342.19200300.100.1.1};

/// Domain component
pub const OID_DOMAIN_COMPONENT: Oid<'static> = oid!{0.9.2342.19200300.100.1.25};

/// GOST R 3411-94 with GOST R 3410-2001
pub const OID_SIG_GOST_R3411_94_WITH_R3410_2001: Oid<'static> = oid!{1.2.643.2.2.3};

/// GOST R 34.10-2001
pub const OID_GOST_R3410_2001: Oid<'static> = oid!{1.2.643.2.2.19};

/// GOST R 34.10-2012 public keys with 256 bits private key length
pub const OID_KEY_TYPE_GOST_R3410_2012_256: Oid<'static> = oid!{1.2.643.7.1.1.1.1};

/// GOST R 34.10-2012 public keys with 512 bits private key length
pub const OID_KEY_TYPE_GOST_R3410_2012_512: Oid<'static> = oid!{1.2.643.7.1.1.1.2};

/// GOST R 34.10-2012 signature algorithm with 256-bit key length and GOST R 34.11-2012 hash function with 256-bit hash code
pub const OID_SIG_GOST_R3410_2012_256: Oid<'static> = oid!{1.2.643.7.1.1.3.2};

/// GOST R 34.10-2012 signature algorithm with 512-bit key length and GOST R 34.11-2012 hash function with 512-bit hash code
pub const OID_SIG_GOST_R3410_2012_512: Oid<'static> = oid!{1.2.643.7.1.1.3.3};

/// DSA subject public key
pub const OID_KEY_TYPE_DSA: Oid<'static> = oid!{1.2.840.10040.4.1};

/// DSA signature generated with SHA-1 algorithm
pub const OID_SIG_DSA_WITH_SHA1: Oid<'static> = oid!{1.2.840.10040.4.3};

/// Elliptic curve public key cryptography
pub const OID_KEY_TYPE_EC_PUBLIC_KEY: Oid<'static> = oid!{1.2.840.10045.2.1};

/// Elliptic curve Digital Signature Algorithm (DSA) coupled with the Secure Hash Algorithm 224 (SHA224) algorithm
pub const OID_SIG_ECDSA_WITH_SHA224: Oid<'static> = oid!{1.2.840.10045.4.3.1};

/// Elliptic curve Digital Signature Algorithm (DSA) coupled with the Secure Hash Algorithm 256 (SHA256) algorithm
pub const OID_SIG_ECDSA_WITH_SHA256: Oid<'static> = oid!{1.2.840.10045.4.3.2};

/// Elliptic curve Digital Signature Algorithm (DSA) coupled with the Secure Hash Algorithm 384 (SHA384) algorithm
pub const OID_SIG_ECDSA_WITH_SHA384: Oid<'static> = oid!{1.2.840.10045.4.3.3};

/// Elliptic curve Digital Signature Algorithm (DSA) coupled with the Secure Hash Algorithm 512 (SHA512) algorithm
pub const OID_SIG_ECDSA_WITH_SHA512: Oid<'static> = oid!{1.2.840.10045.4.3.4};

/// P-256 elliptic curve parameter
pub const OID_EC_P256: Oid<'static> = oid!{1.2.840.10045.3.1.7};

/// RSAES-PKCS1-v1_5 encryption scheme
pub const OID_PKCS1_RSAENCRYPTION: Oid<'static> = oid!{1.2.840.113549.1.1.1};

/// MD2 with RSA encryption
pub const OID_PKCS1_MD2WITHRSAENC: Oid<'static> = oid!{1.2.840.113549.1.1.2};

/// MD4 with RSA encryption
pub const OID_PKCS1_MD4WITHRSAENC: Oid<'static> = oid!{1.2.840.113549.1.1.3};

/// MD5 with RSA encryption
pub const OID_PKCS1_MD5WITHRSAENC: Oid<'static> = oid!{1.2.840.113549.1.1.4};

/// SHA1 with RSA encryption
pub const OID_PKCS1_SHA1WITHRSA: Oid<'static> = oid!{1.2.840.113549.1.1.5};

/// Mask Generator Function 1 (MGF1)
pub const OID_PKCS1_MGF1: Oid<'static> = oid!{1.2.840.113549.1.1.8};

/// RSA Signature Scheme with Probabilistic Signature Scheme (RSASSA-PSS)
pub const OID_PKCS1_RSASSAPSS: Oid<'static> = oid!{1.2.840.113549.1.1.10};

/// SHA256 with RSA encryption
pub const OID_PKCS1_SHA256WITHRSA: Oid<'static> = oid!{1.2.840.113549.1.1.11};

/// SHA384 with RSA encryption
pub const OID_PKCS1_SHA384WITHRSA: Oid<'static> = oid!{1.2.840.113549.1.1.12};

/// SHA512 with RSA encryption
pub const OID_PKCS1_SHA512WITHRSA: Oid<'static> = oid!{1.2.840.113549.1.1.13};

/// SHA224 with RSA encryption
pub const OID_PKCS1_SHA224WITHRSA: Oid<'static> = oid!{1.2.840.113549.1.1.14};

/// pkcs7-data
pub const OID_PKCS7_ID_DATA: Oid<'static> = oid!{1.2.840.113549.1.7.1};

/// PKCS#7 Signed Data
pub const OID_PKCS7_ID_SIGNED_DATA: Oid<'static> = oid!{1.2.840.113549.1.7.2};

/// PKCS#7 Enveloped Data
pub const OID_PKCS7_ID_ENVELOPED_DATA: Oid<'static> = oid!{1.2.840.113549.1.7.3};

/// PKCS#7 Signed and Enveloped Data
pub const OID_PKCS7_ID_SIGNED_ENVELOPED_DATA: Oid<'static> = oid!{1.2.840.113549.1.7.4};

/// PKCS#7 Digested Data
pub const OID_PKCS7_ID_DIGESTED_DATA: Oid<'static> = oid!{1.2.840.113549.1.7.5};

/// PKCS#7 Encrypted Data
pub const OID_PKCS7_ID_ENCRYPTED_DATA: Oid<'static> = oid!{1.2.840.113549.1.7.6};

/// Email Address attribute for use in signatures
pub const OID_PKCS9_EMAIL_ADDRESS: Oid<'static> = oid!{1.2.840.113549.1.9.1};

/// PKCS#9 unstructuredName
pub const OID_PKCS9_UNSTRUCTURED_NAME: Oid<'static> = oid!{1.2.840.113549.1.9.2};

/// id-contentType
pub const OID_PKCS9_CONTENT_TYPE: Oid<'static> = oid!{1.2.840.113549.1.9.3};

/// id-messageDigest
pub const OID_PKCS9_ID_MESSAGE_DIGEST: Oid<'static> = oid!{1.2.840.113549.1.9.4};

/// id-signingTime
pub const OID_PKCS9_SIGNING_TIME: Oid<'static> = oid!{1.2.840.113549.1.9.5};

/// PKCS #9 challenge password (as specified for PKSC#10 in RFC2986)
pub const OID_PKCS9_CHALLENGE_PASSWORD: Oid<'static> = oid!{1.2.840.113549.1.9.7};

/// Extension list for Certification Requests
pub const OID_PKCS9_EXTENSION_REQUEST: Oid<'static> = oid!{1.2.840.113549.1.9.14};

/// aa-smimeCapabilities
pub const OID_PKCS9_SMIME_CAPABILITIES: Oid<'static> = oid!{1.2.840.113549.1.9.15};

/// PKCS #9 attribute friendlyName (for PKCS #12)
pub const OID_PKCS9_FRIENDLY_NAME: Oid<'static> = oid!{1.2.840.113549.1.9.20};

/// Public-Key Cryptography Standard (PKCS) #12
pub const OID_PKCS12: Oid<'static> = oid!{1.2.840.113549.1.12};

/// PKCS #12 Password Based Encryption IDs
pub const OID_PKCS12_PBEIDS: Oid<'static> = oid!{1.2.840.113549.1.12.1};

/// PKCS #12 Password Based Encryption With SHA-1 and 128-bit RC4
pub const OID_PKCS12_PBE_SHA1_128RC4: Oid<'static> = oid!{1.2.840.113549.1.12.1.1};

/// PKCS #12 Password Based Encryption With SHA-1 and 40-bit RC4
pub const OID_PKCS12_PBE_SHA1_40RC4: Oid<'static> = oid!{1.2.840.113549.1.12.1.2};

/// PKCS #12 Password Based Encryption With SHA-1 and 3-key Triple DES in CBC mode
pub const OID_PKCS12_PBE_SHA1_3K_3DES_CBC: Oid<'static> = oid!{1.2.840.113549.1.12.1.3};

/// PKCS #12 Password Based Encryption With SHA-1 and 2-key Triple DES in CBC mode
pub const OID_PKCS12_PBE_SHA1_2K_3DES_CBC: Oid<'static> = oid!{1.2.840.113549.1.12.1.4};

/// PKCS #12 Password Based Encryption With SHA-1 and 128-bit RC2-CBC
pub const OID_PKCS12_PBE_SHA1_128RC2_CBC: Oid<'static> = oid!{1.2.840.113549.1.12.1.5};

/// PKCS #12 Password Based Encryption With SHA-1 and 40-bit RC2-CBC
pub const OID_PKCS12_PBE_SHA1_40RC2_CBC: Oid<'static> = oid!{1.2.840.113549.1.12.1.6};

/// RSA signature in combination with hash algorithm RIPEMD-160
pub const OID_SIG_RSA_RIPE_MD160: Oid<'static> = oid!{1.3.36.3.3.1.2};

/// Edwards-curve Digital Signature Algorithm (EdDSA) Ed25519
pub const OID_SIG_ED25519: Oid<'static> = oid!{1.3.101.112};

/// Edwards-curve Digital Signature Algorithm (EdDSA) Ed448
pub const OID_SIG_ED448: Oid<'static> = oid!{1.3.101.113};

/// P-384 elliptic curve parameter
pub const OID_NIST_EC_P384: Oid<'static> = oid!{1.3.132.0.34};

/// P-521 elliptic curve parameter
pub const OID_NIST_EC_P521: Oid<'static> = oid!{1.3.132.0.35};

/// Single pass Secure Hash Algorithm 1 (SHA1) key derivation
pub const OID_KDF_SHA1_SINGLE: Oid<'static> = oid!{1.3.133.16.840.63.0.2};

/// The SPC_INDIRECT_DATA_CONTENT structure is used in Authenticode signatures to store the digest and other attributes of the signed file
pub const SPC_INDIRECT_DATA_OBJID: Oid<'static> = oid!{1.3.6.1.4.1.311.2.1.4};

/// spcStatementType
pub const SPC_STATEMENT_TYPE_OBJID: Oid<'static> = oid!{1.3.6.1.4.1.311.2.1.11};

/// SpcSpOpusInfo
pub const SPC_SP_OPUS_INFO_OBJID: Oid<'static> = oid!{1.3.6.1.4.1.311.2.1.12};

/// spcPEImageData
pub const SPC_PE_IMAGE_DATA: Oid<'static> = oid!{1.3.6.1.4.1.311.2.1.15};

/// MsCodeInd (SPC_INDIVIDUAL_SP_KEY_PURPOSE_OBJID) is a ExtendedKeyUsage for Certificate Extensions which indicates Microsoft Individual Code Signing (authenticode)
pub const SPC_INDIVIDUAL_SP_KEY_PURPOSE_OBJID: Oid<'static> = oid!{1.3.6.1.4.1.311.2.1.21};

/// MS_CTL
pub const MS_CTL: Oid<'static> = oid!{1.3.6.1.4.1.311.10.1};

/// X520LocalityName as specified in RFC 3280
pub const MS_JURISDICTION_LOCALITY: Oid<'static> = oid!{1.3.6.1.4.1.311.60.2.1.1};

/// X520StateOrProvinceName as specified in RFC 3280
pub const MS_JURISDICTION_STATE_OR_PROVINCE: Oid<'static> = oid!{1.3.6.1.4.1.311.60.2.1.2};

/// X520countryName as specified in RFC 3280
pub const MS_JURISDICTION_COUNTRY: Oid<'static> = oid!{1.3.6.1.4.1.311.60.2.1.3};

// Certificate Transparency: https://tools.ietf.org/html/rfc6962#section-3.3
/// Certificate Transparency Signed Certificate Timestamp List
pub const OID_CT_LIST_SCT: Oid<'static> = oid!{1.3.6.1.4.1.11129.2.4.2};

// PKIX Certificate Extension
// https://www.iana.org/assignments/smi-numbers/smi-numbers.xhtml#smi-numbers-1.3.6.1.5.5.7.1
/// Certificate Authority Information Access
pub const OID_PKIX_AUTHORITY_INFO_ACCESS: Oid<'static> = oid!{1.3.6.1.5.5.7.1.1};

/// Certificate Subject Information Access
pub const OID_PKIX_SUBJECT_INFO_ACCESS: Oid<'static> = oid!{1.3.6.1.5.5.7.1.11};

// PKIX Access Descriptor
// https://www.iana.org/assignments/smi-numbers/smi-numbers.xhtml#smi-numbers-1.3.6.1.5.5.7.48
/// PKIX Access Descriptor OCSP
pub const OID_PKIX_ACCESS_DESCRIPTOR_OCSP: Oid<'static> = oid!{1.3.6.1.5.5.7.48.1};

/// PKIX Access Descriptor CA Issuers
pub const OID_PKIX_ACCESS_DESCRIPTOR_CA_ISSUERS: Oid<'static> = oid!{1.3.6.1.5.5.7.48.2};

/// PKIX Access Descriptor Timestamping
pub const OID_PKIX_ACCESS_DESCRIPTOR_TIMESTAMPING: Oid<'static> = oid!{1.3.6.1.5.5.7.48.3};

/// PKIX Access Descriptor DVCS
pub const OID_PKIX_ACCESS_DESCRIPTOR_DVCS: Oid<'static> = oid!{1.3.6.1.5.5.7.48.4};

/// PKIX Access Descriptor CA Repository
pub const OID_PKIX_ACCESS_DESCRIPTOR_CA_REPOSITORY: Oid<'static> = oid!{1.3.6.1.5.5.7.48.5};

/// PKIX Access Descriptor HTTP Certificates
pub const OID_PKIX_ACCESS_DESCRIPTOR_HTTP_CERTS: Oid<'static> = oid!{1.3.6.1.5.5.7.48.6};

/// PKIX Access Descriptor HTTP Certificate Revocation Lists
pub const OID_PKIX_ACCESS_DESCRIPTOR_HTTP_CRLS: Oid<'static> = oid!{1.3.6.1.5.5.7.48.7};

//
/// PKIX Access Descriptor RPKI Manifest
pub const OID_PKIX_ACCESS_DESCRIPTOR_RPKI_MANIFEST: Oid<'static> = oid!{1.3.6.1.5.5.7.48.10};

/// PKIX Access Descriptor Signed Object
pub const OID_PKIX_ACCESS_DESCRIPTOR_SIGNED_OBJECT: Oid<'static> = oid!{1.3.6.1.5.5.7.48.11};

/// PKIX Access Descriptor CMC
pub const OID_PKIX_ACCESS_DESCRIPTOR_CMC: Oid<'static> = oid!{1.3.6.1.5.5.7.48.12};

/// PKIX Access Descriptor RPKI Notify
pub const OID_PKIX_ACCESS_DESCRIPTOR_RPKI_NOTIFY: Oid<'static> = oid!{1.3.6.1.5.5.7.48.13};

/// PKIX Access Descriptor STIRTNLIST
pub const OID_PKIX_ACCESS_DESCRIPTOR_STIRTNLIST: Oid<'static> = oid!{1.3.6.1.5.5.7.48.14};

/// RSA algorithm coupled with the MD5 hashing algorithm (Oddball using ISO/IEC 9796-2 padding rules)
pub const OID_MD5_WITH_RSA: Oid<'static> = oid!{1.3.14.3.2.25};

/// SHA-1 hash algorithm
pub const OID_HASH_SHA1: Oid<'static> = oid!{1.3.14.3.2.26};

/// RSA algorithm that uses the Secure Hash Algorithm 1 (SHA1) (obsolete)
pub const OID_SHA1_WITH_RSA: Oid<'static> = oid!{1.3.14.3.2.29};

/// X.500
pub const OID_X500: Oid<'static> = oid!{2.5};

/// X.509
pub const OID_X509: Oid<'static> = oid!{2.5.4};

/// Object classes
pub const OID_X509_OBJECT_CLASS: Oid<'static> = oid!{2.5.4.0};

/// Aliased entry/object name
pub const OID_X509_ALIASED_ENTRY_NAME: Oid<'static> = oid!{2.5.4.1};

/// 'knowledgeInformation' attribute type
pub const OID_X509_KNOWLEDGE_INFORMATION: Oid<'static> = oid!{2.5.4.2};

/// Common Name
pub const OID_X509_COMMON_NAME: Oid<'static> = oid!{2.5.4.3};

/// Surname
pub const OID_X509_SURNAME: Oid<'static> = oid!{2.5.4.4};

/// Serial Number
pub const OID_X509_SERIALNUMBER: Oid<'static> = oid!{2.5.4.5};

/// Country Name
pub const OID_X509_COUNTRY_NAME: Oid<'static> = oid!{2.5.4.6};

/// Locality Name
pub const OID_X509_LOCALITY_NAME: Oid<'static> = oid!{2.5.4.7};

/// State or Province name
pub const OID_X509_STATE_OR_PROVINCE_NAME: Oid<'static> = oid!{2.5.4.8};

/// Street Address
pub const OID_X509_STREET_ADDRESS: Oid<'static> = oid!{2.5.4.9};

/// Organization Name
pub const OID_X509_ORGANIZATION_NAME: Oid<'static> = oid!{2.5.4.10};

/// Organizational Unit
pub const OID_X509_ORGANIZATIONAL_UNIT: Oid<'static> = oid!{2.5.4.11};

/// Title
pub const OID_X509_TITLE: Oid<'static> = oid!{2.5.4.12};

/// Description
pub const OID_X509_DESCRIPTION: Oid<'static> = oid!{2.5.4.13};

/// Search Guide
pub const OID_X509_SEARCH_GUIDE: Oid<'static> = oid!{2.5.4.14};

/// Business Category
pub const OID_X509_BUSINESS_CATEGORY: Oid<'static> = oid!{2.5.4.15};

/// Postal Address
pub const OID_X509_POSTAL_ADDRESS: Oid<'static> = oid!{2.5.4.16};

/// Postal Code
pub const OID_X509_POSTAL_CODE: Oid<'static> = oid!{2.5.4.17};

//
/// Name
pub const OID_X509_NAME: Oid<'static> = oid!{2.5.4.41};

/// Given Name
pub const OID_X509_GIVEN_NAME: Oid<'static> = oid!{2.5.4.42};

/// Initials of an individual's name
pub const OID_X509_INITIALS: Oid<'static> = oid!{2.5.4.43};

/// Generation information to qualify an individual's name
pub const OID_X509_GENERATION_QUALIFIER: Oid<'static> = oid!{2.5.4.44};

/// Unique Identifier
pub const OID_X509_UNIQUE_IDENTIFIER: Oid<'static> = oid!{2.5.4.45};

/// DN Qualifier
pub const OID_X509_DN_QUALIFIER: Oid<'static> = oid!{2.5.4.46};

//
// https://www.alvestrand.no/objectid/2.5.29.html
/// X509v3 Authority Key Identifier (obsolete)
pub const OID_X509_OBSOLETE_AUTHORITY_KEY_IDENTIFIER: Oid<'static> = oid!{2.5.29.1};

/// X509v3 Key Attributes (obsolete)
pub const OID_X509_OBSOLETE_KEY_ATTRIBUTES: Oid<'static> = oid!{2.5.29.2};

/// X509v3 Certificate Policies (obsolete)
pub const OID_X509_OBSOLETE_CERTIFICATE_POLICIES: Oid<'static> = oid!{2.5.29.3};

/// X509v3 Key Usage Restriction (obsolete)
pub const OID_X509_OBSOLETE_KEY_USAGE: Oid<'static> = oid!{2.5.29.4};

/// X509v3 Policy Mapping (obsolete)
pub const OID_X509_OBSOLETE_POLICY_MAPPING: Oid<'static> = oid!{2.5.29.5};

/// X509v3 Subtrees Constraint (obsolete)
pub const OID_X509_OBSOLETE_SUBTREES_CONSTRAINT: Oid<'static> = oid!{2.5.29.6};

/// X509v3 Subject Alternative Name (obsolete)
pub const OID_X509_OBSOLETE_SUBJECT_ALT_NAME: Oid<'static> = oid!{2.5.29.7};

/// X509v3 Issuer Alternative Name (obsolete)
pub const OID_X509_OBSOLETE_ISSUER_ALT_NAME: Oid<'static> = oid!{2.5.29.8};

/// X509v3 Subject Key Identifier
pub const OID_X509_EXT_SUBJECT_KEY_IDENTIFIER: Oid<'static> = oid!{2.5.29.14};

/// X509v3 Key Usage
pub const OID_X509_EXT_KEY_USAGE: Oid<'static> = oid!{2.5.29.15};

/// X509v3 Private Key Usage Period
pub const OID_X509_EXT_PRIVATE_KEY_USAGE_PERIOD: Oid<'static> = oid!{2.5.29.16};

/// X509v3 Subject Alternative Name
pub const OID_X509_EXT_SUBJECT_ALT_NAME: Oid<'static> = oid!{2.5.29.17};

/// X509v3 Issuer Alternative Name
pub const OID_X509_EXT_ISSUER_ALT_NAME: Oid<'static> = oid!{2.5.29.18};

/// X509v3 Basic Constraints
pub const OID_X509_EXT_BASIC_CONSTRAINTS: Oid<'static> = oid!{2.5.29.19};

/// X509v3 CRL Number
pub const OID_X509_EXT_CRL_NUMBER: Oid<'static> = oid!{2.5.29.20};

/// X509v3 Reason Code
pub const OID_X509_EXT_REASON_CODE: Oid<'static> = oid!{2.5.29.21};

// no 2.5.29.22
/// X509v3 Hold Instruction Code
pub const OID_X509_EXT_HOLD_INSTRUCTION_CODE: Oid<'static> = oid!{2.5.29.23};

/// X509v3 Invalidity Date
pub const OID_X509_EXT_INVALIDITY_DATE: Oid<'static> = oid!{2.5.29.24};

// no 2.5.29.25 2.5.29.26
/// X509v3 Delta CRL Indicator
pub const OID_X509_EXT_DELTA_CRL_INDICATOR: Oid<'static> = oid!{2.5.29.27};

/// X509v3 Issuer Distribution Point
pub const OID_X509_EXT_ISSUER_DISTRIBUTION_POINT: Oid<'static> = oid!{2.5.29.28};

/// X509v3 Issuer
pub const OID_X509_EXT_ISSUER: Oid<'static> = oid!{2.5.29.29};

/// X509v3 Name Constraints
pub const OID_X509_EXT_NAME_CONSTRAINTS: Oid<'static> = oid!{2.5.29.30};

/// X509v3 CRL Distribution Points
pub const OID_X509_EXT_CRL_DISTRIBUTION_POINTS: Oid<'static> = oid!{2.5.29.31};

/// X509v3 Certificate Policies
pub const OID_X509_EXT_CERTIFICATE_POLICIES: Oid<'static> = oid!{2.5.29.32};

/// X509v3 Policy Mappings
pub const OID_X509_EXT_POLICY_MAPPINGS: Oid<'static> = oid!{2.5.29.33};

// no 2.5.29.34
/// X509v3 Authority Key Identifier
pub const OID_X509_EXT_AUTHORITY_KEY_IDENTIFIER: Oid<'static> = oid!{2.5.29.35};

/// X509v3 Policy Constraints
pub const OID_X509_EXT_POLICY_CONSTRAINTS: Oid<'static> = oid!{2.5.29.36};

/// X509v3 Extended Key Usage
pub const OID_X509_EXT_EXTENDED_KEY_USAGE: Oid<'static> = oid!{2.5.29.37};

//
/// X509v3 Freshest CRL
pub const OID_X509_EXT_FRESHEST_CRL: Oid<'static> = oid!{2.5.29.46};

//
/// X509v3 Inhibit Any-policy
pub const OID_X509_EXT_INHIBIT_ANY_POLICY: Oid<'static> = oid!{2.5.29.54};

/// 256-bit Advanced Encryption Standard (AES) algorithm with Cipher-Block Chaining (CBC) mode of operation
pub const OID_NIST_ENC_AES256_CBC: Oid<'static> = oid!{2.16.840.1.101.3.4.1.42};

/// Secure Hash Algorithm that uses a 256 bit key (SHA256)
pub const OID_NIST_HASH_SHA256: Oid<'static> = oid!{2.16.840.1.101.3.4.2.1};

/// Secure Hash Algorithm that uses a 384 bit key (SHA384)
pub const OID_NIST_HASH_SHA384: Oid<'static> = oid!{2.16.840.1.101.3.4.2.2};

/// Secure Hash Algorithm that uses a 512 bit key (SHA512)
pub const OID_NIST_HASH_SHA512: Oid<'static> = oid!{2.16.840.1.101.3.4.2.3};

/// X.509 v3 Certificate Type
pub const OID_X509_EXT_CERT_TYPE: Oid<'static> = oid!{2.16.840.1.113730.1.1};

/// Base URL
pub const OID_X509_EXT_BASE_URL: Oid<'static> = oid!{2.16.840.1.113730.1.2};

/// Revocation URL
pub const OID_X509_EXT_REVOCATION_URL: Oid<'static> = oid!{2.16.840.1.113730.1.3};

/// CA Revocation URL
pub const OID_X509_EXT_CA_REVOCATION_URL: Oid<'static> = oid!{2.16.840.1.113730.1.4};

/// CA CRL URL
pub const OID_X509_EXT_CA_CRL_URL: Oid<'static> = oid!{2.16.840.1.113730.1.5};

/// CA Certificate URL
pub const OID_X509_EXT_CA_CERT_URL: Oid<'static> = oid!{2.16.840.1.113730.1.6};

/// Renewal URL
pub const OID_X509_EXT_RENEWAL_URL: Oid<'static> = oid!{2.16.840.1.113730.1.7};

/// CA Policy URL
pub const OID_X509_EXT_CA_POLICY_URL: Oid<'static> = oid!{2.16.840.1.113730.1.8};

/// Certificate Homepage URL
pub const OID_X509_EXT_HOMEPAGE_URL: Oid<'static> = oid!{2.16.840.1.113730.1.9};

/// Certificate Entity Logo
pub const OID_X509_EXT_ENTITY_LOGO: Oid<'static> = oid!{2.16.840.1.113730.1.10};

/// Certificate User Picture
pub const OID_X509_EXT_USER_PICTURE: Oid<'static> = oid!{2.16.840.1.113730.1.11};

/// SSL Server Name
pub const OID_X509_EXT_SSL_SERVER_NAME: Oid<'static> = oid!{2.16.840.1.113730.1.12};

/// Certificate Comment
pub const OID_X509_EXT_CERT_COMMENT: Oid<'static> = oid!{2.16.840.1.113730.1.13};