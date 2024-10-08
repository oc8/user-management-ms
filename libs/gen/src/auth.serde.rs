// @generated
impl serde::Serialize for GenerateMagicLinkRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.email.is_empty() {
            len += 1;
        }
        if !self.pkce_challenge.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("auth.GenerateMagicLinkRequest", len)?;
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        if !self.pkce_challenge.is_empty() {
            struct_ser.serialize_field("pkceChallenge", &self.pkce_challenge)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenerateMagicLinkRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "email",
            "pkce_challenge",
            "pkceChallenge",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Email,
            PkceChallenge,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "email" => Ok(GeneratedField::Email),
                            "pkceChallenge" | "pkce_challenge" => Ok(GeneratedField::PkceChallenge),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenerateMagicLinkRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct auth.GenerateMagicLinkRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenerateMagicLinkRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut email__ = None;
                let mut pkce_challenge__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PkceChallenge => {
                            if pkce_challenge__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pkceChallenge"));
                            }
                            pkce_challenge__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenerateMagicLinkRequest {
                    email: email__.unwrap_or_default(),
                    pkce_challenge: pkce_challenge__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("auth.GenerateMagicLinkRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GenerateMagicLinkResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("auth.GenerateMagicLinkResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenerateMagicLinkResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenerateMagicLinkResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct auth.GenerateMagicLinkResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenerateMagicLinkResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GenerateMagicLinkResponse {
                })
            }
        }
        deserializer.deserialize_struct("auth.GenerateMagicLinkResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GenerateOtpRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.email.is_empty() {
            len += 1;
        }
        if !self.pkce_challenge.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("auth.GenerateOTPRequest", len)?;
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        if !self.pkce_challenge.is_empty() {
            struct_ser.serialize_field("pkceChallenge", &self.pkce_challenge)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenerateOtpRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "email",
            "pkce_challenge",
            "pkceChallenge",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Email,
            PkceChallenge,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "email" => Ok(GeneratedField::Email),
                            "pkceChallenge" | "pkce_challenge" => Ok(GeneratedField::PkceChallenge),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenerateOtpRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct auth.GenerateOTPRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenerateOtpRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut email__ = None;
                let mut pkce_challenge__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PkceChallenge => {
                            if pkce_challenge__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pkceChallenge"));
                            }
                            pkce_challenge__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenerateOtpRequest {
                    email: email__.unwrap_or_default(),
                    pkce_challenge: pkce_challenge__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("auth.GenerateOTPRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GenerateOtpResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("auth.GenerateOTPResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenerateOtpResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenerateOtpResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct auth.GenerateOTPResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenerateOtpResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GenerateOtpResponse {
                })
            }
        }
        deserializer.deserialize_struct("auth.GenerateOTPResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LogoutRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.refresh_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("auth.LogoutRequest", len)?;
        if !self.refresh_token.is_empty() {
            struct_ser.serialize_field("refreshToken", &self.refresh_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LogoutRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "refresh_token",
            "refreshToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RefreshToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "refreshToken" | "refresh_token" => Ok(GeneratedField::RefreshToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LogoutRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct auth.LogoutRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LogoutRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut refresh_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RefreshToken => {
                            if refresh_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refreshToken"));
                            }
                            refresh_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LogoutRequest {
                    refresh_token: refresh_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("auth.LogoutRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LogoutResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("auth.LogoutResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LogoutResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LogoutResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct auth.LogoutResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LogoutResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(LogoutResponse {
                })
            }
        }
        deserializer.deserialize_struct("auth.LogoutResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RefreshTokenRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.refresh_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("auth.RefreshTokenRequest", len)?;
        if !self.refresh_token.is_empty() {
            struct_ser.serialize_field("refreshToken", &self.refresh_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RefreshTokenRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "refresh_token",
            "refreshToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RefreshToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "refreshToken" | "refresh_token" => Ok(GeneratedField::RefreshToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RefreshTokenRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct auth.RefreshTokenRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RefreshTokenRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut refresh_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RefreshToken => {
                            if refresh_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refreshToken"));
                            }
                            refresh_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RefreshTokenRequest {
                    refresh_token: refresh_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("auth.RefreshTokenRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RefreshTokenResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tokens.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("auth.RefreshTokenResponse", len)?;
        if let Some(v) = self.tokens.as_ref() {
            struct_ser.serialize_field("tokens", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RefreshTokenResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tokens",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tokens,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tokens" => Ok(GeneratedField::Tokens),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RefreshTokenResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct auth.RefreshTokenResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RefreshTokenResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tokens__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tokens => {
                            if tokens__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokens"));
                            }
                            tokens__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RefreshTokenResponse {
                    tokens: tokens__,
                })
            }
        }
        deserializer.deserialize_struct("auth.RefreshTokenResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegisterRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.email.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("auth.RegisterRequest", len)?;
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisterRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "email",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Email,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "email" => Ok(GeneratedField::Email),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegisterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct auth.RegisterRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegisterRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut email__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RegisterRequest {
                    email: email__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("auth.RegisterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegisterResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("auth.RegisterResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisterResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegisterResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct auth.RegisterResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegisterResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RegisterResponse {
                })
            }
        }
        deserializer.deserialize_struct("auth.RegisterResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Tokens {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.access_token.is_empty() {
            len += 1;
        }
        if !self.refresh_token.is_empty() {
            len += 1;
        }
        if self.expires_in != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("auth.Tokens", len)?;
        if !self.access_token.is_empty() {
            struct_ser.serialize_field("accessToken", &self.access_token)?;
        }
        if !self.refresh_token.is_empty() {
            struct_ser.serialize_field("refreshToken", &self.refresh_token)?;
        }
        if self.expires_in != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("expiresIn", ToString::to_string(&self.expires_in).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Tokens {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "access_token",
            "accessToken",
            "refresh_token",
            "refreshToken",
            "expires_in",
            "expiresIn",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AccessToken,
            RefreshToken,
            ExpiresIn,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "accessToken" | "access_token" => Ok(GeneratedField::AccessToken),
                            "refreshToken" | "refresh_token" => Ok(GeneratedField::RefreshToken),
                            "expiresIn" | "expires_in" => Ok(GeneratedField::ExpiresIn),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Tokens;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct auth.Tokens")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Tokens, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut access_token__ = None;
                let mut refresh_token__ = None;
                let mut expires_in__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AccessToken => {
                            if access_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessToken"));
                            }
                            access_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RefreshToken => {
                            if refresh_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refreshToken"));
                            }
                            refresh_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExpiresIn => {
                            if expires_in__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiresIn"));
                            }
                            expires_in__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Tokens {
                    access_token: access_token__.unwrap_or_default(),
                    refresh_token: refresh_token__.unwrap_or_default(),
                    expires_in: expires_in__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("auth.Tokens", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for User {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.email.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("auth.User", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for User {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "email",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Email,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "email" => Ok(GeneratedField::Email),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = User;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct auth.User")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<User, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut email__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(User {
                    id: id__.unwrap_or_default(),
                    email: email__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("auth.User", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidateMagicLinkRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.email.is_empty() {
            len += 1;
        }
        if !self.code.is_empty() {
            len += 1;
        }
        if !self.pkce_verifier.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("auth.ValidateMagicLinkRequest", len)?;
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.pkce_verifier.is_empty() {
            struct_ser.serialize_field("pkceVerifier", &self.pkce_verifier)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidateMagicLinkRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "email",
            "code",
            "pkce_verifier",
            "pkceVerifier",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Email,
            Code,
            PkceVerifier,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "email" => Ok(GeneratedField::Email),
                            "code" => Ok(GeneratedField::Code),
                            "pkceVerifier" | "pkce_verifier" => Ok(GeneratedField::PkceVerifier),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidateMagicLinkRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct auth.ValidateMagicLinkRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValidateMagicLinkRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut email__ = None;
                let mut code__ = None;
                let mut pkce_verifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PkceVerifier => {
                            if pkce_verifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pkceVerifier"));
                            }
                            pkce_verifier__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ValidateMagicLinkRequest {
                    email: email__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    pkce_verifier: pkce_verifier__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("auth.ValidateMagicLinkRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidateMagicLinkResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tokens.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("auth.ValidateMagicLinkResponse", len)?;
        if let Some(v) = self.tokens.as_ref() {
            struct_ser.serialize_field("tokens", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidateMagicLinkResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tokens",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tokens,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tokens" => Ok(GeneratedField::Tokens),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidateMagicLinkResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct auth.ValidateMagicLinkResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValidateMagicLinkResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tokens__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tokens => {
                            if tokens__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokens"));
                            }
                            tokens__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ValidateMagicLinkResponse {
                    tokens: tokens__,
                })
            }
        }
        deserializer.deserialize_struct("auth.ValidateMagicLinkResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidateOtpRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.email.is_empty() {
            len += 1;
        }
        if !self.otp.is_empty() {
            len += 1;
        }
        if !self.pkce_verifier.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("auth.ValidateOTPRequest", len)?;
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        if !self.otp.is_empty() {
            struct_ser.serialize_field("otp", &self.otp)?;
        }
        if !self.pkce_verifier.is_empty() {
            struct_ser.serialize_field("pkceVerifier", &self.pkce_verifier)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidateOtpRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "email",
            "otp",
            "pkce_verifier",
            "pkceVerifier",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Email,
            Otp,
            PkceVerifier,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "email" => Ok(GeneratedField::Email),
                            "otp" => Ok(GeneratedField::Otp),
                            "pkceVerifier" | "pkce_verifier" => Ok(GeneratedField::PkceVerifier),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidateOtpRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct auth.ValidateOTPRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValidateOtpRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut email__ = None;
                let mut otp__ = None;
                let mut pkce_verifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Otp => {
                            if otp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("otp"));
                            }
                            otp__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PkceVerifier => {
                            if pkce_verifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pkceVerifier"));
                            }
                            pkce_verifier__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ValidateOtpRequest {
                    email: email__.unwrap_or_default(),
                    otp: otp__.unwrap_or_default(),
                    pkce_verifier: pkce_verifier__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("auth.ValidateOTPRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidateOtpResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tokens.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("auth.ValidateOTPResponse", len)?;
        if let Some(v) = self.tokens.as_ref() {
            struct_ser.serialize_field("tokens", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidateOtpResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tokens",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tokens,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tokens" => Ok(GeneratedField::Tokens),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidateOtpResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct auth.ValidateOTPResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValidateOtpResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tokens__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tokens => {
                            if tokens__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokens"));
                            }
                            tokens__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ValidateOtpResponse {
                    tokens: tokens__,
                })
            }
        }
        deserializer.deserialize_struct("auth.ValidateOTPResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidateTokenRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.access_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("auth.ValidateTokenRequest", len)?;
        if !self.access_token.is_empty() {
            struct_ser.serialize_field("accessToken", &self.access_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidateTokenRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "access_token",
            "accessToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AccessToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "accessToken" | "access_token" => Ok(GeneratedField::AccessToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidateTokenRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct auth.ValidateTokenRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValidateTokenRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut access_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AccessToken => {
                            if access_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessToken"));
                            }
                            access_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ValidateTokenRequest {
                    access_token: access_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("auth.ValidateTokenRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidateTokenResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.valid {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("auth.ValidateTokenResponse", len)?;
        if self.valid {
            struct_ser.serialize_field("valid", &self.valid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidateTokenResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "valid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Valid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "valid" => Ok(GeneratedField::Valid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidateTokenResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct auth.ValidateTokenResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValidateTokenResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut valid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Valid => {
                            if valid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valid"));
                            }
                            valid__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ValidateTokenResponse {
                    valid: valid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("auth.ValidateTokenResponse", FIELDS, GeneratedVisitor)
    }
}
