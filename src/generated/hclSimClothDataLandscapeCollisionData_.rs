use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclSimClothDataLandscapeCollisionData`
/// - version: `0`
/// - signature: `0x1d595840`
/// - size: `  0`(x86)/` 20`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclSimClothDataLandscapeCollisionData<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub __ptr: Option<Pointer<'a>>,
    /// # C++ Info
    /// - name: `landscapeRadius`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "landscapeRadius"))]
    #[cfg_attr(feature = "serde", serde(rename = "landscapeRadius"))]
    pub m_landscapeRadius: f32,
    /// # C++ Info
    /// - name: `enableStuckParticleDetection`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`  4`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "enableStuckParticleDetection")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "enableStuckParticleDetection"))]
    pub m_enableStuckParticleDetection: bool,
    /// # C++ Info
    /// - name: `stuckParticlesStretchFactorSq`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`  8`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "stuckParticlesStretchFactorSq")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "stuckParticlesStretchFactorSq"))]
    pub m_stuckParticlesStretchFactorSq: f32,
    /// # C++ Info
    /// - name: `pinchDetectionEnabled`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 12`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "pinchDetectionEnabled"))]
    #[cfg_attr(feature = "serde", serde(rename = "pinchDetectionEnabled"))]
    pub m_pinchDetectionEnabled: bool,
    /// # C++ Info
    /// - name: `pinchDetectionPriority`(ctype: `hkInt8`)
    /// - offset: `  0`(x86)/` 13`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "pinchDetectionPriority"))]
    #[cfg_attr(feature = "serde", serde(rename = "pinchDetectionPriority"))]
    pub m_pinchDetectionPriority: I8<'a>,
    /// # C++ Info
    /// - name: `pinchDetectionRadius`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "pinchDetectionRadius"))]
    #[cfg_attr(feature = "serde", serde(rename = "pinchDetectionRadius"))]
    pub m_pinchDetectionRadius: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclSimClothDataLandscapeCollisionData<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclSimClothDataLandscapeCollisionData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x1d595840)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hclSimClothDataLandscapeCollisionData<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x1d595840)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hclSimClothDataLandscapeCollisionData",
                    class_meta,
                    (0u64, 20u64),
                )?;
            serializer.serialize_field("landscapeRadius", &self.m_landscapeRadius)?;
            serializer
                .serialize_field(
                    "enableStuckParticleDetection",
                    &self.m_enableStuckParticleDetection,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field(
                    "stuckParticlesStretchFactorSq",
                    &self.m_stuckParticlesStretchFactorSq,
                )?;
            serializer
                .serialize_field(
                    "pinchDetectionEnabled",
                    &self.m_pinchDetectionEnabled,
                )?;
            serializer
                .serialize_field(
                    "pinchDetectionPriority",
                    &self.m_pinchDetectionPriority,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer
                .serialize_field("pinchDetectionRadius", &self.m_pinchDetectionRadius)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hclSimClothDataLandscapeCollisionData<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_landscapeRadius,
                m_enableStuckParticleDetection,
                m_stuckParticlesStretchFactorSq,
                m_pinchDetectionEnabled,
                m_pinchDetectionPriority,
                m_pinchDetectionRadius,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "field identifier")
                }
                /// Intended for use in XML.
                #[allow(clippy::match_single_binding)]
                #[allow(clippy::reversed_empty_ranges)]
                #[allow(clippy::single_match)]
                fn visit_key<__E>(
                    self,
                    __value: &str,
                ) -> core::result::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "landscapeRadius" => Ok(__Field::m_landscapeRadius),
                        "enableStuckParticleDetection" => {
                            Ok(__Field::m_enableStuckParticleDetection)
                        }
                        "stuckParticlesStretchFactorSq" => {
                            Ok(__Field::m_stuckParticlesStretchFactorSq)
                        }
                        "pinchDetectionEnabled" => Ok(__Field::m_pinchDetectionEnabled),
                        "pinchDetectionPriority" => Ok(__Field::m_pinchDetectionPriority),
                        "pinchDetectionRadius" => Ok(__Field::m_pinchDetectionRadius),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> core::result::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
                }
            }
            struct __hclSimClothDataLandscapeCollisionDataVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hclSimClothDataLandscapeCollisionData<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hclSimClothDataLandscapeCollisionDataVisitor<'de> {
                type Value = hclSimClothDataLandscapeCollisionData<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclSimClothDataLandscapeCollisionData",
                    )
                }
                fn visit_struct_for_bytes<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let __ptr = __A::class_ptr(&mut __map);
                    let mut m_landscapeRadius: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_enableStuckParticleDetection: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_stuckParticlesStretchFactorSq: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_pinchDetectionEnabled: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_pinchDetectionPriority: _serde::__private::Option<
                        I8<'de>,
                    > = _serde::__private::None;
                    let mut m_pinchDetectionRadius: _serde::__private::Option<f32> = _serde::__private::None;
                    for i in 0..6usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_landscapeRadius) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "landscapeRadius",
                                        ),
                                    );
                                }
                                m_landscapeRadius = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_enableStuckParticleDetection,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableStuckParticleDetection",
                                        ),
                                    );
                                }
                                m_enableStuckParticleDetection = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_stuckParticlesStretchFactorSq,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stuckParticlesStretchFactorSq",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 3usize)?;
                                m_stuckParticlesStretchFactorSq = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_pinchDetectionEnabled,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pinchDetectionEnabled",
                                        ),
                                    );
                                }
                                m_pinchDetectionEnabled = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_pinchDetectionPriority,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pinchDetectionPriority",
                                        ),
                                    );
                                }
                                m_pinchDetectionPriority = _serde::__private::Some(
                                    match __A::next_value::<I8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_pinchDetectionRadius,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pinchDetectionRadius",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 2usize)?;
                                m_pinchDetectionRadius = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {}
                        }
                    }
                    let m_landscapeRadius = match m_landscapeRadius {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "landscapeRadius",
                                ),
                            );
                        }
                    };
                    let m_enableStuckParticleDetection = match m_enableStuckParticleDetection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableStuckParticleDetection",
                                ),
                            );
                        }
                    };
                    let m_stuckParticlesStretchFactorSq = match m_stuckParticlesStretchFactorSq {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stuckParticlesStretchFactorSq",
                                ),
                            );
                        }
                    };
                    let m_pinchDetectionEnabled = match m_pinchDetectionEnabled {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pinchDetectionEnabled",
                                ),
                            );
                        }
                    };
                    let m_pinchDetectionPriority = match m_pinchDetectionPriority {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pinchDetectionPriority",
                                ),
                            );
                        }
                    };
                    let m_pinchDetectionRadius = match m_pinchDetectionRadius {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pinchDetectionRadius",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hclSimClothDataLandscapeCollisionData {
                        __ptr,
                        m_landscapeRadius,
                        m_enableStuckParticleDetection,
                        m_stuckParticlesStretchFactorSq,
                        m_pinchDetectionEnabled,
                        m_pinchDetectionPriority,
                        m_pinchDetectionRadius,
                    })
                }
                #[allow(clippy::manual_unwrap_or_default)]
                fn visit_struct<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut m_landscapeRadius: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_enableStuckParticleDetection: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_stuckParticlesStretchFactorSq: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_pinchDetectionEnabled: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_pinchDetectionPriority: _serde::__private::Option<
                        I8<'de>,
                    > = _serde::__private::None;
                    let mut m_pinchDetectionRadius: _serde::__private::Option<f32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_landscapeRadius => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_landscapeRadius) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "landscapeRadius",
                                        ),
                                    );
                                }
                                m_landscapeRadius = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_enableStuckParticleDetection => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_enableStuckParticleDetection,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableStuckParticleDetection",
                                        ),
                                    );
                                }
                                m_enableStuckParticleDetection = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_stuckParticlesStretchFactorSq => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_stuckParticlesStretchFactorSq,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stuckParticlesStretchFactorSq",
                                        ),
                                    );
                                }
                                m_stuckParticlesStretchFactorSq = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_pinchDetectionEnabled => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_pinchDetectionEnabled,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pinchDetectionEnabled",
                                        ),
                                    );
                                }
                                m_pinchDetectionEnabled = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_pinchDetectionPriority => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_pinchDetectionPriority,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pinchDetectionPriority",
                                        ),
                                    );
                                }
                                m_pinchDetectionPriority = _serde::__private::Some(
                                    match __A::next_value::<I8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_pinchDetectionRadius => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_pinchDetectionRadius,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pinchDetectionRadius",
                                        ),
                                    );
                                }
                                m_pinchDetectionRadius = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => __A::skip_value(&mut __map)?,
                        }
                    }
                    let m_landscapeRadius = match m_landscapeRadius {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "landscapeRadius",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_enableStuckParticleDetection = match m_enableStuckParticleDetection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableStuckParticleDetection",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_stuckParticlesStretchFactorSq = match m_stuckParticlesStretchFactorSq {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stuckParticlesStretchFactorSq",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_pinchDetectionEnabled = match m_pinchDetectionEnabled {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pinchDetectionEnabled",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_pinchDetectionPriority = match m_pinchDetectionPriority {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pinchDetectionPriority",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_pinchDetectionRadius = match m_pinchDetectionRadius {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pinchDetectionRadius",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hclSimClothDataLandscapeCollisionData {
                        __ptr: __ptr.clone(),
                        m_landscapeRadius,
                        m_enableStuckParticleDetection,
                        m_stuckParticlesStretchFactorSq,
                        m_pinchDetectionEnabled,
                        m_pinchDetectionPriority,
                        m_pinchDetectionRadius,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "landscapeRadius",
                "enableStuckParticleDetection",
                "stuckParticlesStretchFactorSq",
                "pinchDetectionEnabled",
                "pinchDetectionPriority",
                "pinchDetectionRadius",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclSimClothDataLandscapeCollisionData",
                FIELDS,
                __hclSimClothDataLandscapeCollisionDataVisitor {
                    marker: _serde::__private::PhantomData::<
                        hclSimClothDataLandscapeCollisionData,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
