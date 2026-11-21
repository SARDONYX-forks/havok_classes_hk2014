use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclSimClothSetupObjectTransferMotionSetupData`
/// - version: `1`
/// - signature: `0xdb341cb1`
/// - size: `  0`(x86)/` 56`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclSimClothSetupObjectTransferMotionSetupData<'a> {
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
    /// - name: `transferMotionTransformSetSetup`(ctype: `hclTransformSetSetupObject`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "transferMotionTransformSetSetup")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "transferMotionTransformSetSetup"))]
    pub m_transferMotionTransformSetSetup: Pointer<'a>,
    /// # C++ Info
    /// - name: `transferMotionTransformName`(ctype: `TYPE_STRINGPTR`)
    /// - offset: `  0`(x86)/`  8`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "transferMotionTransformName")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "transferMotionTransformName"))]
    pub m_transferMotionTransformName: StringPtr<'a>,
    /// # C++ Info
    /// - name: `transferTranslationMotion`(ctype: `TYPE_BOOL`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "transferTranslationMotion"))]
    #[cfg_attr(feature = "serde", serde(rename = "transferTranslationMotion"))]
    pub m_transferTranslationMotion: bool,
    /// # C++ Info
    /// - name: `minTranslationSpeed`(ctype: `TYPE_REAL`)
    /// - offset: `  0`(x86)/` 20`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "minTranslationSpeed"))]
    #[cfg_attr(feature = "serde", serde(rename = "minTranslationSpeed"))]
    pub m_minTranslationSpeed: f32,
    /// # C++ Info
    /// - name: `maxTranslationSpeed`(ctype: `TYPE_REAL`)
    /// - offset: `  0`(x86)/` 24`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxTranslationSpeed"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxTranslationSpeed"))]
    pub m_maxTranslationSpeed: f32,
    /// # C++ Info
    /// - name: `minTranslationBlend`(ctype: `TYPE_REAL`)
    /// - offset: `  0`(x86)/` 28`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "minTranslationBlend"))]
    #[cfg_attr(feature = "serde", serde(rename = "minTranslationBlend"))]
    pub m_minTranslationBlend: f32,
    /// # C++ Info
    /// - name: `maxTranslationBlend`(ctype: `TYPE_REAL`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxTranslationBlend"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxTranslationBlend"))]
    pub m_maxTranslationBlend: f32,
    /// # C++ Info
    /// - name: `transferRotationMotion`(ctype: `TYPE_BOOL`)
    /// - offset: `  0`(x86)/` 36`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "transferRotationMotion"))]
    #[cfg_attr(feature = "serde", serde(rename = "transferRotationMotion"))]
    pub m_transferRotationMotion: bool,
    /// # C++ Info
    /// - name: `minRotationSpeed`(ctype: `TYPE_REAL`)
    /// - offset: `  0`(x86)/` 40`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "minRotationSpeed"))]
    #[cfg_attr(feature = "serde", serde(rename = "minRotationSpeed"))]
    pub m_minRotationSpeed: f32,
    /// # C++ Info
    /// - name: `maxRotationSpeed`(ctype: `TYPE_REAL`)
    /// - offset: `  0`(x86)/` 44`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxRotationSpeed"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxRotationSpeed"))]
    pub m_maxRotationSpeed: f32,
    /// # C++ Info
    /// - name: `minRotationBlend`(ctype: `TYPE_REAL`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "minRotationBlend"))]
    #[cfg_attr(feature = "serde", serde(rename = "minRotationBlend"))]
    pub m_minRotationBlend: f32,
    /// # C++ Info
    /// - name: `maxRotationBlend`(ctype: `TYPE_REAL`)
    /// - offset: `  0`(x86)/` 52`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxRotationBlend"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxRotationBlend"))]
    pub m_maxRotationBlend: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclSimClothSetupObjectTransferMotionSetupData<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclSimClothSetupObjectTransferMotionSetupData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xdb341cb1)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.m_transferMotionTransformSetSetup);
            v
        }
    }
    impl<'a> _serde::Serialize for hclSimClothSetupObjectTransferMotionSetupData<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xdb341cb1)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hclSimClothSetupObjectTransferMotionSetupData",
                    class_meta,
                    (0u64, 56u64),
                )?;
            serializer
                .serialize_field(
                    "transferMotionTransformSetSetup",
                    &self.m_transferMotionTransformSetSetup,
                )?;
            serializer
                .serialize_field(
                    "transferMotionTransformName",
                    &self.m_transferMotionTransformName,
                )?;
            serializer
                .serialize_field(
                    "transferTranslationMotion",
                    &self.m_transferTranslationMotion,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field("minTranslationSpeed", &self.m_minTranslationSpeed)?;
            serializer
                .serialize_field("maxTranslationSpeed", &self.m_maxTranslationSpeed)?;
            serializer
                .serialize_field("minTranslationBlend", &self.m_minTranslationBlend)?;
            serializer
                .serialize_field("maxTranslationBlend", &self.m_maxTranslationBlend)?;
            serializer
                .serialize_field(
                    "transferRotationMotion",
                    &self.m_transferRotationMotion,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("minRotationSpeed", &self.m_minRotationSpeed)?;
            serializer.serialize_field("maxRotationSpeed", &self.m_maxRotationSpeed)?;
            serializer.serialize_field("minRotationBlend", &self.m_minRotationBlend)?;
            serializer.serialize_field("maxRotationBlend", &self.m_maxRotationBlend)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de>
    for hclSimClothSetupObjectTransferMotionSetupData<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_transferMotionTransformSetSetup,
                m_transferMotionTransformName,
                m_transferTranslationMotion,
                m_minTranslationSpeed,
                m_maxTranslationSpeed,
                m_minTranslationBlend,
                m_maxTranslationBlend,
                m_transferRotationMotion,
                m_minRotationSpeed,
                m_maxRotationSpeed,
                m_minRotationBlend,
                m_maxRotationBlend,
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
                        "transferMotionTransformSetSetup" => {
                            Ok(__Field::m_transferMotionTransformSetSetup)
                        }
                        "transferMotionTransformName" => {
                            Ok(__Field::m_transferMotionTransformName)
                        }
                        "transferTranslationMotion" => {
                            Ok(__Field::m_transferTranslationMotion)
                        }
                        "minTranslationSpeed" => Ok(__Field::m_minTranslationSpeed),
                        "maxTranslationSpeed" => Ok(__Field::m_maxTranslationSpeed),
                        "minTranslationBlend" => Ok(__Field::m_minTranslationBlend),
                        "maxTranslationBlend" => Ok(__Field::m_maxTranslationBlend),
                        "transferRotationMotion" => Ok(__Field::m_transferRotationMotion),
                        "minRotationSpeed" => Ok(__Field::m_minRotationSpeed),
                        "maxRotationSpeed" => Ok(__Field::m_maxRotationSpeed),
                        "minRotationBlend" => Ok(__Field::m_minRotationBlend),
                        "maxRotationBlend" => Ok(__Field::m_maxRotationBlend),
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
            struct __hclSimClothSetupObjectTransferMotionSetupDataVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hclSimClothSetupObjectTransferMotionSetupData<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hclSimClothSetupObjectTransferMotionSetupDataVisitor<'de> {
                type Value = hclSimClothSetupObjectTransferMotionSetupData<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclSimClothSetupObjectTransferMotionSetupData",
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
                    let mut m_transferMotionTransformSetSetup: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_transferMotionTransformName: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
                    let mut m_transferTranslationMotion: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_minTranslationSpeed: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxTranslationSpeed: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_minTranslationBlend: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxTranslationBlend: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_transferRotationMotion: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_minRotationSpeed: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxRotationSpeed: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_minRotationBlend: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxRotationBlend: _serde::__private::Option<f32> = _serde::__private::None;
                    for i in 0..12usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_transferMotionTransformSetSetup,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transferMotionTransformSetSetup",
                                        ),
                                    );
                                }
                                m_transferMotionTransformSetSetup = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_transferMotionTransformName,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transferMotionTransformName",
                                        ),
                                    );
                                }
                                m_transferMotionTransformName = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_transferTranslationMotion,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transferTranslationMotion",
                                        ),
                                    );
                                }
                                m_transferTranslationMotion = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_minTranslationSpeed,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minTranslationSpeed",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 3usize)?;
                                m_minTranslationSpeed = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxTranslationSpeed,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxTranslationSpeed",
                                        ),
                                    );
                                }
                                m_maxTranslationSpeed = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_minTranslationBlend,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minTranslationBlend",
                                        ),
                                    );
                                }
                                m_minTranslationBlend = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxTranslationBlend,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxTranslationBlend",
                                        ),
                                    );
                                }
                                m_maxTranslationBlend = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_transferRotationMotion,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transferRotationMotion",
                                        ),
                                    );
                                }
                                m_transferRotationMotion = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_minRotationSpeed) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minRotationSpeed",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 3usize)?;
                                m_minRotationSpeed = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_maxRotationSpeed) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxRotationSpeed",
                                        ),
                                    );
                                }
                                m_maxRotationSpeed = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_minRotationBlend) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minRotationBlend",
                                        ),
                                    );
                                }
                                m_minRotationBlend = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_maxRotationBlend) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxRotationBlend",
                                        ),
                                    );
                                }
                                m_maxRotationBlend = _serde::__private::Some(
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
                    let m_transferMotionTransformSetSetup = match m_transferMotionTransformSetSetup {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transferMotionTransformSetSetup",
                                ),
                            );
                        }
                    };
                    let m_transferMotionTransformName = match m_transferMotionTransformName {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transferMotionTransformName",
                                ),
                            );
                        }
                    };
                    let m_transferTranslationMotion = match m_transferTranslationMotion {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transferTranslationMotion",
                                ),
                            );
                        }
                    };
                    let m_minTranslationSpeed = match m_minTranslationSpeed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minTranslationSpeed",
                                ),
                            );
                        }
                    };
                    let m_maxTranslationSpeed = match m_maxTranslationSpeed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxTranslationSpeed",
                                ),
                            );
                        }
                    };
                    let m_minTranslationBlend = match m_minTranslationBlend {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minTranslationBlend",
                                ),
                            );
                        }
                    };
                    let m_maxTranslationBlend = match m_maxTranslationBlend {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxTranslationBlend",
                                ),
                            );
                        }
                    };
                    let m_transferRotationMotion = match m_transferRotationMotion {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transferRotationMotion",
                                ),
                            );
                        }
                    };
                    let m_minRotationSpeed = match m_minRotationSpeed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minRotationSpeed",
                                ),
                            );
                        }
                    };
                    let m_maxRotationSpeed = match m_maxRotationSpeed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxRotationSpeed",
                                ),
                            );
                        }
                    };
                    let m_minRotationBlend = match m_minRotationBlend {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minRotationBlend",
                                ),
                            );
                        }
                    };
                    let m_maxRotationBlend = match m_maxRotationBlend {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxRotationBlend",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hclSimClothSetupObjectTransferMotionSetupData {
                        __ptr,
                        m_transferMotionTransformSetSetup,
                        m_transferMotionTransformName,
                        m_transferTranslationMotion,
                        m_minTranslationSpeed,
                        m_maxTranslationSpeed,
                        m_minTranslationBlend,
                        m_maxTranslationBlend,
                        m_transferRotationMotion,
                        m_minRotationSpeed,
                        m_maxRotationSpeed,
                        m_minRotationBlend,
                        m_maxRotationBlend,
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
                    let mut m_transferMotionTransformSetSetup: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_transferMotionTransformName: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
                    let mut m_transferTranslationMotion: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_minTranslationSpeed: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxTranslationSpeed: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_minTranslationBlend: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxTranslationBlend: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_transferRotationMotion: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_minRotationSpeed: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxRotationSpeed: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_minRotationBlend: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxRotationBlend: _serde::__private::Option<f32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_transferMotionTransformSetSetup => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_transferMotionTransformSetSetup,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transferMotionTransformSetSetup",
                                        ),
                                    );
                                }
                                m_transferMotionTransformSetSetup = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_transferMotionTransformName => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_transferMotionTransformName,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transferMotionTransformName",
                                        ),
                                    );
                                }
                                m_transferMotionTransformName = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_transferTranslationMotion => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_transferTranslationMotion,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transferTranslationMotion",
                                        ),
                                    );
                                }
                                m_transferTranslationMotion = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_minTranslationSpeed => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_minTranslationSpeed,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minTranslationSpeed",
                                        ),
                                    );
                                }
                                m_minTranslationSpeed = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxTranslationSpeed => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxTranslationSpeed,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxTranslationSpeed",
                                        ),
                                    );
                                }
                                m_maxTranslationSpeed = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_minTranslationBlend => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_minTranslationBlend,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minTranslationBlend",
                                        ),
                                    );
                                }
                                m_minTranslationBlend = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxTranslationBlend => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxTranslationBlend,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxTranslationBlend",
                                        ),
                                    );
                                }
                                m_maxTranslationBlend = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_transferRotationMotion => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_transferRotationMotion,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transferRotationMotion",
                                        ),
                                    );
                                }
                                m_transferRotationMotion = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_minRotationSpeed => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_minRotationSpeed) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minRotationSpeed",
                                        ),
                                    );
                                }
                                m_minRotationSpeed = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxRotationSpeed => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_maxRotationSpeed) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxRotationSpeed",
                                        ),
                                    );
                                }
                                m_maxRotationSpeed = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_minRotationBlend => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_minRotationBlend) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minRotationBlend",
                                        ),
                                    );
                                }
                                m_minRotationBlend = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxRotationBlend => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_maxRotationBlend) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxRotationBlend",
                                        ),
                                    );
                                }
                                m_maxRotationBlend = _serde::__private::Some(
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
                    let m_transferMotionTransformSetSetup = match m_transferMotionTransformSetSetup {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transferMotionTransformSetSetup",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_transferMotionTransformName = match m_transferMotionTransformName {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transferMotionTransformName",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_transferTranslationMotion = match m_transferTranslationMotion {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transferTranslationMotion",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_minTranslationSpeed = match m_minTranslationSpeed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minTranslationSpeed",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxTranslationSpeed = match m_maxTranslationSpeed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxTranslationSpeed",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_minTranslationBlend = match m_minTranslationBlend {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minTranslationBlend",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxTranslationBlend = match m_maxTranslationBlend {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxTranslationBlend",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_transferRotationMotion = match m_transferRotationMotion {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transferRotationMotion",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_minRotationSpeed = match m_minRotationSpeed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minRotationSpeed",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxRotationSpeed = match m_maxRotationSpeed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxRotationSpeed",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_minRotationBlend = match m_minRotationBlend {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minRotationBlend",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxRotationBlend = match m_maxRotationBlend {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxRotationBlend",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hclSimClothSetupObjectTransferMotionSetupData {
                        __ptr: __ptr.clone(),
                        m_transferMotionTransformSetSetup,
                        m_transferMotionTransformName,
                        m_transferTranslationMotion,
                        m_minTranslationSpeed,
                        m_maxTranslationSpeed,
                        m_minTranslationBlend,
                        m_maxTranslationBlend,
                        m_transferRotationMotion,
                        m_minRotationSpeed,
                        m_maxRotationSpeed,
                        m_minRotationBlend,
                        m_maxRotationBlend,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "transferMotionTransformSetSetup",
                "transferMotionTransformName",
                "transferTranslationMotion",
                "minTranslationSpeed",
                "maxTranslationSpeed",
                "minTranslationBlend",
                "maxTranslationBlend",
                "transferRotationMotion",
                "minRotationSpeed",
                "maxRotationSpeed",
                "minRotationBlend",
                "maxRotationBlend",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclSimClothSetupObjectTransferMotionSetupData",
                FIELDS,
                __hclSimClothSetupObjectTransferMotionSetupDataVisitor {
                    marker: _serde::__private::PhantomData::<
                        hclSimClothSetupObjectTransferMotionSetupData,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
