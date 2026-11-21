use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkVariableTweakingHelper`
/// - version: `0`
/// - signature: `0xc9d21eef`
/// - size: `  0`(x86)/`136`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkVariableTweakingHelper<'a> {
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
    /// - name: `boolVariableInfo`(ctype: `hkArray<struct hkVariableTweakingHelperBoolVariableInfo>`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "boolVariableInfo"))]
    #[cfg_attr(feature = "serde", serde(rename = "boolVariableInfo"))]
    pub m_boolVariableInfo: Vec<hkVariableTweakingHelperBoolVariableInfo<'a>>,
    /// # C++ Info
    /// - name: `intVariableInfo`(ctype: `hkArray<struct hkVariableTweakingHelperIntVariableInfo>`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "intVariableInfo"))]
    #[cfg_attr(feature = "serde", serde(rename = "intVariableInfo"))]
    pub m_intVariableInfo: Vec<hkVariableTweakingHelperIntVariableInfo<'a>>,
    /// # C++ Info
    /// - name: `realVariableInfo`(ctype: `hkArray<struct hkVariableTweakingHelperRealVariableInfo>`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "realVariableInfo"))]
    #[cfg_attr(feature = "serde", serde(rename = "realVariableInfo"))]
    pub m_realVariableInfo: Vec<hkVariableTweakingHelperRealVariableInfo<'a>>,
    /// # C++ Info
    /// - name: `vector4VariableInfo`(ctype: `hkArray<struct hkVariableTweakingHelperVector4VariableInfo>`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "vector4VariableInfo"))]
    #[cfg_attr(feature = "serde", serde(rename = "vector4VariableInfo"))]
    pub m_vector4VariableInfo: Vec<hkVariableTweakingHelperVector4VariableInfo<'a>>,
    /// # C++ Info
    /// - name: `behavior`(ctype: `void*`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "behavior"))]
    #[cfg_attr(feature = "serde", serde(rename = "behavior"))]
    pub m_behavior: Pointer<'a>,
    /// # C++ Info
    /// - name: `boolIndices`(ctype: `hkArray<void>`)
    /// - offset: `  0`(x86)/` 72`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "boolIndices"))]
    #[cfg_attr(feature = "serde", serde(rename = "boolIndices"))]
    pub m_boolIndices: Vec<()>,
    /// # C++ Info
    /// - name: `intIndices`(ctype: `hkArray<void>`)
    /// - offset: `  0`(x86)/` 88`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "intIndices"))]
    #[cfg_attr(feature = "serde", serde(rename = "intIndices"))]
    pub m_intIndices: Vec<()>,
    /// # C++ Info
    /// - name: `realIndices`(ctype: `hkArray<void>`)
    /// - offset: `  0`(x86)/`104`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "realIndices"))]
    #[cfg_attr(feature = "serde", serde(rename = "realIndices"))]
    pub m_realIndices: Vec<()>,
    /// # C++ Info
    /// - name: `vector4Indices`(ctype: `hkArray<void>`)
    /// - offset: `  0`(x86)/`120`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "vector4Indices"))]
    #[cfg_attr(feature = "serde", serde(rename = "vector4Indices"))]
    pub m_vector4Indices: Vec<()>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkVariableTweakingHelper<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkVariableTweakingHelper"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xc9d21eef)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(
                self
                    .m_boolVariableInfo
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(
                self
                    .m_intVariableInfo
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(
                self
                    .m_realVariableInfo
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(
                self
                    .m_vector4VariableInfo
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.push(&self.m_behavior);
            v
        }
    }
    impl<'a> _serde::Serialize for hkVariableTweakingHelper<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xc9d21eef)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkVariableTweakingHelper",
                    class_meta,
                    (0u64, 136u64),
                )?;
            serializer
                .serialize_array_field(
                    "boolVariableInfo",
                    &self.m_boolVariableInfo,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 16u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "intVariableInfo",
                    &self.m_intVariableInfo,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 16u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "realVariableInfo",
                    &self.m_realVariableInfo,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 16u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "vector4VariableInfo",
                    &self.m_vector4VariableInfo,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 32u64,
                    },
                )?;
            serializer.skip_field("behavior", &self.m_behavior)?;
            serializer
                .skip_array_field("boolIndices", &self.m_boolIndices, TypeSize::NonPtr)?;
            serializer
                .skip_array_field("intIndices", &self.m_intIndices, TypeSize::NonPtr)?;
            serializer
                .skip_array_field("realIndices", &self.m_realIndices, TypeSize::NonPtr)?;
            serializer
                .skip_array_field(
                    "vector4Indices",
                    &self.m_vector4Indices,
                    TypeSize::NonPtr,
                )?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkVariableTweakingHelper<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_boolVariableInfo,
                m_intVariableInfo,
                m_realVariableInfo,
                m_vector4VariableInfo,
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
                        "boolVariableInfo" => Ok(__Field::m_boolVariableInfo),
                        "intVariableInfo" => Ok(__Field::m_intVariableInfo),
                        "realVariableInfo" => Ok(__Field::m_realVariableInfo),
                        "vector4VariableInfo" => Ok(__Field::m_vector4VariableInfo),
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
            struct __hkVariableTweakingHelperVisitor<'de> {
                marker: _serde::__private::PhantomData<hkVariableTweakingHelper<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkVariableTweakingHelperVisitor<'de> {
                type Value = hkVariableTweakingHelper<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkVariableTweakingHelper",
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
                    let mut m_boolVariableInfo: _serde::__private::Option<
                        Vec<hkVariableTweakingHelperBoolVariableInfo<'de>>,
                    > = _serde::__private::None;
                    let mut m_intVariableInfo: _serde::__private::Option<
                        Vec<hkVariableTweakingHelperIntVariableInfo<'de>>,
                    > = _serde::__private::None;
                    let mut m_realVariableInfo: _serde::__private::Option<
                        Vec<hkVariableTweakingHelperRealVariableInfo<'de>>,
                    > = _serde::__private::None;
                    let mut m_vector4VariableInfo: _serde::__private::Option<
                        Vec<hkVariableTweakingHelperVector4VariableInfo<'de>>,
                    > = _serde::__private::None;
                    let mut m_behavior: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_boolIndices: _serde::__private::Option<Vec<()>> = _serde::__private::None;
                    let mut m_intIndices: _serde::__private::Option<Vec<()>> = _serde::__private::None;
                    let mut m_realIndices: _serde::__private::Option<Vec<()>> = _serde::__private::None;
                    let mut m_vector4Indices: _serde::__private::Option<Vec<()>> = _serde::__private::None;
                    for i in 0..9usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_boolVariableInfo) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boolVariableInfo",
                                        ),
                                    );
                                }
                                m_boolVariableInfo = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkVariableTweakingHelperBoolVariableInfo<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_intVariableInfo) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intVariableInfo",
                                        ),
                                    );
                                }
                                m_intVariableInfo = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkVariableTweakingHelperIntVariableInfo<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_realVariableInfo) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "realVariableInfo",
                                        ),
                                    );
                                }
                                m_realVariableInfo = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkVariableTweakingHelperRealVariableInfo<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_vector4VariableInfo,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vector4VariableInfo",
                                        ),
                                    );
                                }
                                m_vector4VariableInfo = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkVariableTweakingHelperVector4VariableInfo<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_behavior) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "behavior",
                                        ),
                                    );
                                }
                                m_behavior = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_boolIndices) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boolIndices",
                                        ),
                                    );
                                }
                                m_boolIndices = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_intIndices) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intIndices",
                                        ),
                                    );
                                }
                                m_intIndices = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_realIndices) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "realIndices",
                                        ),
                                    );
                                }
                                m_realIndices = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_vector4Indices) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vector4Indices",
                                        ),
                                    );
                                }
                                m_vector4Indices = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
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
                    let m_boolVariableInfo = match m_boolVariableInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boolVariableInfo",
                                ),
                            );
                        }
                    };
                    let m_intVariableInfo = match m_intVariableInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intVariableInfo",
                                ),
                            );
                        }
                    };
                    let m_realVariableInfo = match m_realVariableInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "realVariableInfo",
                                ),
                            );
                        }
                    };
                    let m_vector4VariableInfo = match m_vector4VariableInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vector4VariableInfo",
                                ),
                            );
                        }
                    };
                    let m_behavior = match m_behavior {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("behavior"),
                            );
                        }
                    };
                    let m_boolIndices = match m_boolIndices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boolIndices",
                                ),
                            );
                        }
                    };
                    let m_intIndices = match m_intIndices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intIndices",
                                ),
                            );
                        }
                    };
                    let m_realIndices = match m_realIndices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "realIndices",
                                ),
                            );
                        }
                    };
                    let m_vector4Indices = match m_vector4Indices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vector4Indices",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkVariableTweakingHelper {
                        __ptr,
                        m_boolVariableInfo,
                        m_intVariableInfo,
                        m_realVariableInfo,
                        m_vector4VariableInfo,
                        m_behavior,
                        m_boolIndices,
                        m_intIndices,
                        m_realIndices,
                        m_vector4Indices,
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
                    let mut m_boolVariableInfo: _serde::__private::Option<
                        Vec<hkVariableTweakingHelperBoolVariableInfo<'de>>,
                    > = _serde::__private::None;
                    let mut m_intVariableInfo: _serde::__private::Option<
                        Vec<hkVariableTweakingHelperIntVariableInfo<'de>>,
                    > = _serde::__private::None;
                    let mut m_realVariableInfo: _serde::__private::Option<
                        Vec<hkVariableTweakingHelperRealVariableInfo<'de>>,
                    > = _serde::__private::None;
                    let mut m_vector4VariableInfo: _serde::__private::Option<
                        Vec<hkVariableTweakingHelperVector4VariableInfo<'de>>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_boolVariableInfo => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_boolVariableInfo) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boolVariableInfo",
                                        ),
                                    );
                                }
                                m_boolVariableInfo = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkVariableTweakingHelperBoolVariableInfo<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_intVariableInfo => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_intVariableInfo) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intVariableInfo",
                                        ),
                                    );
                                }
                                m_intVariableInfo = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkVariableTweakingHelperIntVariableInfo<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_realVariableInfo => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_realVariableInfo) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "realVariableInfo",
                                        ),
                                    );
                                }
                                m_realVariableInfo = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkVariableTweakingHelperRealVariableInfo<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_vector4VariableInfo => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_vector4VariableInfo,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vector4VariableInfo",
                                        ),
                                    );
                                }
                                m_vector4VariableInfo = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkVariableTweakingHelperVector4VariableInfo<'de>>,
                                    >(&mut __map) {
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
                    let m_boolVariableInfo = match m_boolVariableInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boolVariableInfo",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_intVariableInfo = match m_intVariableInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intVariableInfo",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_realVariableInfo = match m_realVariableInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "realVariableInfo",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_vector4VariableInfo = match m_vector4VariableInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vector4VariableInfo",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkVariableTweakingHelper {
                        __ptr: __ptr.clone(),
                        m_boolVariableInfo,
                        m_intVariableInfo,
                        m_realVariableInfo,
                        m_vector4VariableInfo,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "boolVariableInfo",
                "intVariableInfo",
                "realVariableInfo",
                "vector4VariableInfo",
                "behavior",
                "boolIndices",
                "intIndices",
                "realIndices",
                "vector4Indices",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkVariableTweakingHelper",
                FIELDS,
                __hkVariableTweakingHelperVisitor {
                    marker: _serde::__private::PhantomData::<hkVariableTweakingHelper>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
