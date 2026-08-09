use base64::Engine;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::fs;

type HmacSha256 = Hmac<Sha256>;
pub const LICENSE_SECRET_SALT: &[u8] = b"NANTARA_SUPPORT_PRO_SECRET_SALT_2026_KEYGEN";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LicenseStatus {
    pub is_pro: bool,
    pub tier_name: String,
    pub client_name: String,
    pub expiry_date: String,
}

pub fn generate_signature(client_name: &str, expiry_date: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(LICENSE_SECRET_SALT)
        .expect("HMAC can take key of any size");
    let payload = format!("CLIENT={}|EXPIRY={}", client_name.trim(), expiry_date.trim());
    mac.update(payload.as_bytes());
    let result = mac.finalize().into_bytes();
    result.iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn generate_pro_key(client_name: &str, expiry_date: &str) -> String {
    let client_b64 = base64::engine::general_purpose::STANDARD.encode(client_name.trim());
    let sig = generate_signature(client_name, expiry_date);
    format!("NANTARA-PRO.{}.{}.{}", client_b64, expiry_date.trim(), sig)
}

pub fn parse_and_verify_license(license_key: &str) -> LicenseStatus {
    let trimmed = license_key.trim();
    let parts: Vec<&str> = trimmed.split('.').collect();

    if parts.len() == 4 && parts[0] == "NANTARA-PRO" {
        let client_b64 = parts[1];
        let expiry = parts[2];
        let signature = parts[3];

        if let Ok(client_bytes) = base64::engine::general_purpose::STANDARD.decode(client_b64) {
            if let Ok(client_name) = String::from_utf8(client_bytes) {
                let expected_sig = generate_signature(&client_name, expiry);

                if signature.to_lowercase() == expected_sig.to_lowercase() {
                    return LicenseStatus {
                        is_pro: true,
                        tier_name: "PRO LICENSE (Active)".into(),
                        client_name,
                        expiry_date: expiry.into(),
                    };
                }
            }
        }
    }

    LicenseStatus {
        is_pro: false,
        tier_name: "COMMUNITY EDITION (Free)".into(),
        client_name: "Open Source User".into(),
        expiry_date: "Lifetime Free".into(),
    }
}

pub fn load_local_license() -> LicenseStatus {
    if let Ok(key) = fs::read_to_string("license.key") {
        let status = parse_and_verify_license(&key);
        if status.is_pro {
            return status;
        }
    }
    LicenseStatus {
        is_pro: false,
        tier_name: "COMMUNITY EDITION (Free)".into(),
        client_name: "Open Source User".into(),
        expiry_date: "Lifetime Free".into(),
    }
}
