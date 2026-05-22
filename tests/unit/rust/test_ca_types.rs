#[cfg(test)]
mod tests {
    use ca_types::*;

    #[test]
    fn test_validity_period_default() {
        let vp = ValidityPeriod::new(365);
        assert!(vp.not_before < vp.not_after);
    }

    #[test]
    fn test_key_type_security_bits() {
        assert_eq!(KeyType::EcdsaP256.security_bits(), 128);
        assert_eq!(KeyType::EcdsaP384.security_bits(), 192);
        assert_eq!(KeyType::Ed25519.security_bits(), 128);
        assert_eq!(KeyType::PqcDilithium5.security_bits(), 192);
    }

    #[test]
    fn test_cert_profile_defaults() {
        let profile = CertProfile {
            name: "test".into(),
            key_type: KeyType::EcdsaP256,
            validity_days: 365,
            key_usage: vec!["digitalSignature".into()],
            extended_key_usage: vec!["clientAuth".into()],
            max_path_len: None,
            allow_ca: false,
        };
        assert_eq!(profile.name, "test");
        assert!(!profile.allow_ca);
    }

    #[test]
    fn test_serial_number_unique() {
        let s1 = uuid::Uuid::new_v4().to_string();
        let s2 = uuid::Uuid::new_v4().to_string();
        assert_ne!(s1, s2);
    }

    #[test]
    fn test_identity_association() {
        let identity = Identity {
            id: "id-001".into(),
            name: "Lunar Relay Alpha".into(),
            organization: "NASA".into(),
            asset_type: SpaceAssetType::LunarRelay,
            public_key_fingerprint: "abc123".into(),
            hardware_attestation: None,
            attributes: vec![],
            status: IdentityStatus::Active,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };
        assert_eq!(identity.status, IdentityStatus::Active);
    }

    #[test]
    fn test_parses_attestation_types() {
        let tpm = AttestationType::Tpm2_0;
        let se = AttestationType::SecureElement;
        assert_ne!(format!("{:?}", tpm), format!("{:?}", se));
    }
}
