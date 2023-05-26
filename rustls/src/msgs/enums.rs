#![allow(clippy::upper_case_acronyms)]
#![allow(non_camel_case_types)]
/// This file is autogenerated.  See https://github.com/ctz/tls-hacking/
use crate::msgs::codec::{Codec, ListLength, Reader, TlsListElement};

enum_builder! {
    /// The `HashAlgorithm` TLS protocol enum.  Values in this enum are taken
    /// from the various RFCs covering TLS, and are listed by IANA.
    /// The `Unknown` item is used when processing unrecognised ordinals.
    @U8
    EnumName: HashAlgorithm;
    EnumVal{
        NONE => 0x00,
        MD5 => 0x01,
        SHA1 => 0x02,
        SHA224 => 0x03,
        SHA256 => 0x04,
        SHA384 => 0x05,
        SHA512 => 0x06
    }
}

enum_builder! {
    /// The `ClientCertificateType` TLS protocol enum.  Values in this enum are taken
    /// from the various RFCs covering TLS, and are listed by IANA.
    /// The `Unknown` item is used when processing unrecognised ordinals.
    @U8
    EnumName: ClientCertificateType;
    EnumVal{
        RSASign => 0x01,
        DSSSign => 0x02,
        RSAFixedDH => 0x03,
        DSSFixedDH => 0x04,
        RSAEphemeralDH => 0x05,
        DSSEphemeralDH => 0x06,
        FortezzaDMS => 0x14,
        ECDSASign => 0x40,
        RSAFixedECDH => 0x41,
        ECDSAFixedECDH => 0x42
    }
}

enum_builder! {
    /// The `Compression` TLS protocol enum.  Values in this enum are taken
    /// from the various RFCs covering TLS, and are listed by IANA.
    /// The `Unknown` item is used when processing unrecognised ordinals.
    @U8
    EnumName: Compression;
    EnumVal{
        Null => 0x00,
        Deflate => 0x01,
        LSZ => 0x40
    }
}

enum_builder! {
    /// The `AlertLevel` TLS protocol enum.  Values in this enum are taken
    /// from the various RFCs covering TLS, and are listed by IANA.
    /// The `Unknown` item is used when processing unrecognised ordinals.
    @U8
    EnumName: AlertLevel;
    EnumVal{
        Warning => 0x01,
        Fatal => 0x02
    }
}

enum_builder! {
    /// The `HeartbeatMessageType` TLS protocol enum.  Values in this enum are taken
    /// from the various RFCs covering TLS, and are listed by IANA.
    /// The `Unknown` item is used when processing unrecognised ordinals.
    @U8
    EnumName: HeartbeatMessageType;
    EnumVal{
        Request => 0x01,
        Response => 0x02
    }
}

enum_builder! {
    /// The `ExtensionType` TLS protocol enum.  Values in this enum are taken
    /// from the various RFCs covering TLS, and are listed by IANA.
    /// The `Unknown` item is used when processing unrecognised ordinals.
    @U16
    EnumName: ExtensionType;
    EnumVal{
        ServerName => 0x0000,
        MaxFragmentLength => 0x0001,
        ClientCertificateUrl => 0x0002,
        TrustedCAKeys => 0x0003,
        TruncatedHMAC => 0x0004,
        StatusRequest => 0x0005,
        UserMapping => 0x0006,
        ClientAuthz => 0x0007,
        ServerAuthz => 0x0008,
        CertificateType => 0x0009,
        EllipticCurves => 0x000a,
        ECPointFormats => 0x000b,
        SRP => 0x000c,
        SignatureAlgorithms => 0x000d,
        UseSRTP => 0x000e,
        Heartbeat => 0x000f,
        ALProtocolNegotiation => 0x0010,
        SCT => 0x0012,
        Padding => 0x0015,
        ExtendedMasterSecret => 0x0017,
        CompressedCertAlgs => 0x001b,
        SessionTicket => 0x0023,
        PreSharedKey => 0x0029,
        EarlyData => 0x002a,
        SupportedVersions => 0x002b,
        Cookie => 0x002c,
        PSKKeyExchangeModes => 0x002d,
        TicketEarlyDataInfo => 0x002e,
        CertificateAuthorities => 0x002f,
        OIDFilters => 0x0030,
        PostHandshakeAuth => 0x0031,
        SignatureAlgorithmsCert => 0x0032,
        KeyShare => 0x0033,
        TransportParameters => 0x0039,
        NextProtocolNegotiation => 0x3374,
        ApplicationSettings => 0x4469,
        ChannelId => 0x754f,
        RenegotiationInfo => 0xff01,
        TransportParametersDraft => 0xffa5
    }
}

impl TlsListElement for ExtensionType {
    const SIZE_LEN: ListLength = ListLength::U16;
}

enum_builder! {
    /// The `ServerNameType` TLS protocol enum.  Values in this enum are taken
    /// from the various RFCs covering TLS, and are listed by IANA.
    /// The `Unknown` item is used when processing unrecognised ordinals.
    @U8
    EnumName: ServerNameType;
    EnumVal{
        HostName => 0x00
    }
}

enum_builder! {
    /// The `NamedCurve` TLS protocol enum.  Values in this enum are taken
    /// from the various RFCs covering TLS, and are listed by IANA.
    /// The `Unknown` item is used when processing unrecognised ordinals.
    @U16
    EnumName: NamedCurve;
    EnumVal{
        sect163k1 => 0x0001,
        sect163r1 => 0x0002,
        sect163r2 => 0x0003,
        sect193r1 => 0x0004,
        sect193r2 => 0x0005,
        sect233k1 => 0x0006,
        sect233r1 => 0x0007,
        sect239k1 => 0x0008,
        sect283k1 => 0x0009,
        sect283r1 => 0x000a,
        sect409k1 => 0x000b,
        sect409r1 => 0x000c,
        sect571k1 => 0x000d,
        sect571r1 => 0x000e,
        secp160k1 => 0x000f,
        secp160r1 => 0x0010,
        secp160r2 => 0x0011,
        secp192k1 => 0x0012,
        secp192r1 => 0x0013,
        secp224k1 => 0x0014,
        secp224r1 => 0x0015,
        secp256k1 => 0x0016,
        secp256r1 => 0x0017,
        secp384r1 => 0x0018,
        secp521r1 => 0x0019,
        brainpoolp256r1 => 0x001a,
        brainpoolp384r1 => 0x001b,
        brainpoolp512r1 => 0x001c,
        X25519 => 0x001d,
        X448 => 0x001e,
        arbitrary_explicit_prime_curves => 0xff01,
        arbitrary_explicit_char2_curves => 0xff02
    }
}

enum_builder! {
    /// The `NamedGroup` TLS protocol enum.  Values in this enum are taken
    /// from the various RFCs covering TLS, and are listed by IANA.
    /// The `Unknown` item is used when processing unrecognised ordinals.
    @U16
    EnumName: NamedGroup;
    EnumVal{
        secp256r1 => 0x0017,
        secp384r1 => 0x0018,
        secp521r1 => 0x0019,
        X25519 => 0x001d,
        X448 => 0x001e,
        FFDHE2048 => 0x0100,
        FFDHE3072 => 0x0101,
        FFDHE4096 => 0x0102,
        FFDHE6144 => 0x0103,
        FFDHE8192 => 0x0104
    }
}

enum_builder! {
    /// The `ECPointFormat` TLS protocol enum.  Values in this enum are taken
    /// from the various RFCs covering TLS, and are listed by IANA.
    /// The `Unknown` item is used when processing unrecognised ordinals.
    @U8
    EnumName: ECPointFormat;
    EnumVal{
        Uncompressed => 0x00,
        ANSIX962CompressedPrime => 0x01,
        ANSIX962CompressedChar2 => 0x02
    }
}

impl ECPointFormat {
    pub const SUPPORTED: [Self; 1] = [Self::Uncompressed];
}

enum_builder! {
    /// The `HeartbeatMode` TLS protocol enum.  Values in this enum are taken
    /// from the various RFCs covering TLS, and are listed by IANA.
    /// The `Unknown` item is used when processing unrecognised ordinals.
    @U8
    EnumName: HeartbeatMode;
    EnumVal{
        PeerAllowedToSend => 0x01,
        PeerNotAllowedToSend => 0x02
    }
}

enum_builder! {
    /// The `ECCurveType` TLS protocol enum.  Values in this enum are taken
    /// from the various RFCs covering TLS, and are listed by IANA.
    /// The `Unknown` item is used when processing unrecognised ordinals.
    @U8
    EnumName: ECCurveType;
    EnumVal{
        ExplicitPrime => 0x01,
        ExplicitChar2 => 0x02,
        NamedCurve => 0x03
    }
}

enum_builder! {
    /// The `PSKKeyExchangeMode` TLS protocol enum.  Values in this enum are taken
    /// from the various RFCs covering TLS, and are listed by IANA.
    /// The `Unknown` item is used when processing unrecognised ordinals.
    @U8
    EnumName: PSKKeyExchangeMode;
    EnumVal{
        PSK_KE => 0x00,
        PSK_DHE_KE => 0x01
    }
}

enum_builder! {
    /// The `KeyUpdateRequest` TLS protocol enum.  Values in this enum are taken
    /// from the various RFCs covering TLS, and are listed by IANA.
    /// The `Unknown` item is used when processing unrecognised ordinals.
    @U8
    EnumName: KeyUpdateRequest;
    EnumVal{
        UpdateNotRequested => 0x00,
        UpdateRequested => 0x01
    }
}

enum_builder! {
    /// The `CertificateStatusType` TLS protocol enum.  Values in this enum are taken
    /// from the various RFCs covering TLS, and are listed by IANA.
    /// The `Unknown` item is used when processing unrecognised ordinals.
    @U8
    EnumName: CertificateStatusType;
    EnumVal{
        OCSP => 0x01
    }
}

#[cfg(test)]
pub(crate) mod tests {
    //! These tests are intended to provide coverage and
    //! check panic-safety of relatively unused values.

    use super::*;
    use crate::msgs::codec::Codec;

    #[test]
    fn test_enums() {
        test_enum8::<HashAlgorithm>(HashAlgorithm::NONE, HashAlgorithm::SHA512);
        test_enum8::<ClientCertificateType>(
            ClientCertificateType::RSASign,
            ClientCertificateType::ECDSAFixedECDH,
        );
        test_enum8::<Compression>(Compression::Null, Compression::LSZ);
        test_enum8::<AlertLevel>(AlertLevel::Warning, AlertLevel::Fatal);
        test_enum8::<HeartbeatMessageType>(
            HeartbeatMessageType::Request,
            HeartbeatMessageType::Response,
        );
        test_enum16::<ExtensionType>(ExtensionType::ServerName, ExtensionType::RenegotiationInfo);
        test_enum8::<ServerNameType>(ServerNameType::HostName, ServerNameType::HostName);
        test_enum16::<NamedCurve>(
            NamedCurve::sect163k1,
            NamedCurve::arbitrary_explicit_char2_curves,
        );
        test_enum16::<NamedGroup>(NamedGroup::secp256r1, NamedGroup::FFDHE8192);
        test_enum8::<ECPointFormat>(
            ECPointFormat::Uncompressed,
            ECPointFormat::ANSIX962CompressedChar2,
        );
        test_enum8::<HeartbeatMode>(
            HeartbeatMode::PeerAllowedToSend,
            HeartbeatMode::PeerNotAllowedToSend,
        );
        test_enum8::<ECCurveType>(ECCurveType::ExplicitPrime, ECCurveType::NamedCurve);
        test_enum8::<PSKKeyExchangeMode>(
            PSKKeyExchangeMode::PSK_KE,
            PSKKeyExchangeMode::PSK_DHE_KE,
        );
        test_enum8::<KeyUpdateRequest>(
            KeyUpdateRequest::UpdateNotRequested,
            KeyUpdateRequest::UpdateRequested,
        );
        test_enum8::<CertificateStatusType>(
            CertificateStatusType::OCSP,
            CertificateStatusType::OCSP,
        );
    }

    pub(crate) fn test_enum8<T: Codec>(first: T, last: T) {
        let first_v = get8(&first);
        let last_v = get8(&last);

        for val in first_v..last_v + 1 {
            let mut buf = Vec::new();
            val.encode(&mut buf);
            assert_eq!(buf.len(), 1);

            let t = T::read_bytes(&buf).unwrap();
            assert_eq!(val, get8(&t));
        }
    }

    pub(crate) fn test_enum16<T: Codec>(first: T, last: T) {
        let first_v = get16(&first);
        let last_v = get16(&last);

        for val in first_v..last_v + 1 {
            let mut buf = Vec::new();
            val.encode(&mut buf);
            assert_eq!(buf.len(), 2);

            let t = T::read_bytes(&buf).unwrap();
            assert_eq!(val, get16(&t));
        }
    }

    fn get8<T: Codec>(enum_value: &T) -> u8 {
        let enc = enum_value.get_encoding();
        assert_eq!(enc.len(), 1);
        enc[0]
    }

    fn get16<T: Codec>(enum_value: &T) -> u16 {
        let enc = enum_value.get_encoding();
        assert_eq!(enc.len(), 2);
        (enc[0] as u16 >> 8) | (enc[1] as u16)
    }
}
