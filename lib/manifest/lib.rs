// Copyright (c) Douglas Reis.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ManifestVersion {
    pub minor: u16,
    pub major: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ManifestUsageConstraints {
    pub selector_bits: u32,
    pub device_id: [u32; 8], // lifecycle_device_id_t usually 32-bit array
    pub manuf_state_creator: u32,
    pub manuf_state_owner: u32,
    pub life_cycle_state: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union SignatureUnion {
    pub rsa_signature: [u32; 96],  // 3072-bit RSA signature
    pub ecdsa_signature: [u8; 96], // ECDSA P256 signature (first 64 bytes used)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union PublicKeyUnion {
    pub rsa_modulus: [u32; 96],     // RSA public key modulus
    pub ecdsa_public_key: [u8; 96], // ECDSA P256 public key
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ManifestTimestamp {
    pub secs_low: u32,
    pub secs_high: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Manifest {
    pub signature: SignatureUnion, // signature for the image
    pub usage_constraints: ManifestUsageConstraints,
    pub public_key: PublicKeyUnion,        // signer public key
    pub address_translation: u32,          // hardened boolean
    pub identifier: u32,                   // manifest identifier
    pub manifest_version: ManifestVersion, // format version
    pub signed_region_end: u32,            // end of signed region
    pub length: u32,                       // image length including manifest
    pub version_major: u32,
    pub version_minor: u32,
    pub security_version: u32,        // anti-rollback version
    pub timestamp: ManifestTimestamp, // Unix epoch timestamp
    pub binding_value: [u32; 8],      // for key manager binding
    pub max_key_version: u32,
    pub code_start: u32,        // offset of executable code
    pub code_end: u32,          // end (exclusive) of code
    pub entry_point: *const u8, // entry offset
    pub extensions: [u32; 30],  // extension table
}

// Safety: The Manifest is a read-only static structure that is safe to share across the system.
unsafe impl Sync for Manifest {}

unsafe extern "C" {
    static _manifest_entry_point: u8;
}

#[unsafe(link_section = ".manifest")]
#[used]
pub static MANIFEST: Manifest = Manifest {
    entry_point: unsafe { &_manifest_entry_point as *const u8 },
    ..unsafe { core::mem::zeroed() }
};
