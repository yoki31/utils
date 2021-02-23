//! Pure Rust implementation of Public-Key Cryptography Standards (PKCS) #8:
//!
//! Private-Key Information Syntax Specification (as defined in [RFC 5208]).
//!
//! # About
//! This is a minimalistic library targeting `no_std` platforms and small code
//! size. It supports decoding/encoding of the following types without the use
//! of a heap:
//!
//! - [`EncryptedPrivateKeyInfo`]: (with `pkcs5` feature) encrypted key.
//! - [`PrivateKeyInfo`]: algorithm identifier and data representing a private key.
//! - [`SubjectPublicKeyInfo`]: algorithm identifier and data representing a public key
//!   (re-exported from the [`spki`] crate)
//!
//! When the `alloc` feature is enabled, the following additional types are
//! available which provide more convenient decoding/encoding support:
//!
//! - [`EncryptedPrivateKeyDocument`]: (with `pkcs5` feature) heap-backed encrypted key.
//! - [`PrivateKeyDocument`]: heap-backed storage for serialized [`PrivateKeyInfo`].
//! - [`PublicKeyDocument`]: heap-backed storage for serialized [`SubjectPublicKeyInfo`].
//!
//! When the `pem` feature is enabled, it also supports decoding/encoding
//! documents from "PEM encoding" format as defined in RFC 7468.
//!
//! # Supported Algorithms
//! This crate has been tested against keys generated by OpenSSL for the
//! following algorithms:
//!
//! - ECC (`id-ecPublicKey`)
//! - Ed25519 (`Ed25519`)
//! - RSA (`rsaEncryption`)
//!
//! It may work with other algorithms which use an optional OID for
//! [`AlgorithmIdentifier`] parameters.
//!
//! # Encrypted Private Key Support
//! [`EncryptedPrivateKeyInfo`] supports decoding/encoding encrypted PKCS#8
//! private keys and is gated under the `pkcs5` feature. The corresponding
//! [`EncryptedPrivateKeyDocument`] type provides heap-backed storage
//! (`alloc` feature required).
//!
//! When the `encryption` feature of this crate is enabled, it provides a
//! [`EncryptedPrivateKeyInfo::decrypt`] function which is able to decrypt
//! keys encrypted with the following algorithms:
//!
//! - [PKCS#5v2 Password Based Encryption Scheme 2 (RFC 8018)]
//!   - Key derivation function: PBKDF2 with HMAC-SHA256 as the PRF
//!   - Symmetric encryption: AES-128-CBC or AES-256-CBC
//!
//! # Minimum Supported Rust Version
//!
//! This crate requires **Rust 1.47** at a minimum.
//!
//! [RFC 5208]: https://tools.ietf.org/html/rfc5208
//! [PKCS#5v2 Password Based Encryption Scheme 2 (RFC 8018)]: https://tools.ietf.org/html/rfc8018#section-6.2

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo.svg",
    html_root_url = "https://docs.rs/pkcs8/0.5.2"
)]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod error;
mod private_key_info;
mod traits;

#[cfg(feature = "alloc")]
mod document;

#[cfg(feature = "pem")]
mod pem;

pub use crate::{
    error::{Error, Result},
    private_key_info::PrivateKeyInfo,
    traits::{FromPrivateKey, FromPublicKey},
};
pub use der::{self, ObjectIdentifier};
pub use spki::{AlgorithmIdentifier, SubjectPublicKeyInfo};

#[cfg(feature = "alloc")]
pub use crate::{
    document::{private_key::PrivateKeyDocument, public_key::PublicKeyDocument},
    traits::{ToPrivateKey, ToPublicKey},
};

#[cfg(feature = "pkcs5")]
pub use crate::private_key_info::encrypted::EncryptedPrivateKeyInfo;
#[cfg(feature = "pkcs5")]
pub use pkcs5;

#[cfg(all(feature = "alloc", feature = "pkcs5"))]
pub use crate::document::encrypted_private_key::EncryptedPrivateKeyDocument;
