use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkaPredictiveCompressedAnimationTrackCompressionParams`
/// - version: `0`
/// - signature: `0x739c5f1`
/// - size: `  0`(x86)/` 32`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaPredictiveCompressedAnimationTrackCompressionParams<'a> {
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
    /// - name: `staticTranslationTolerance`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "staticTranslationTolerance"))]
    #[cfg_attr(feature = "serde", serde(rename = "staticTranslationTolerance"))]
    pub m_staticTranslationTolerance: f32,
    /// # C++ Info
    /// - name: `staticRotationTolerance`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`  4`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "staticRotationTolerance"))]
    #[cfg_attr(feature = "serde", serde(rename = "staticRotationTolerance"))]
    pub m_staticRotationTolerance: f32,
    /// # C++ Info
    /// - name: `staticScaleTolerance`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`  8`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "staticScaleTolerance"))]
    #[cfg_attr(feature = "serde", serde(rename = "staticScaleTolerance"))]
    pub m_staticScaleTolerance: f32,
    /// # C++ Info
    /// - name: `staticFloatTolerance`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 12`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "staticFloatTolerance"))]
    #[cfg_attr(feature = "serde", serde(rename = "staticFloatTolerance"))]
    pub m_staticFloatTolerance: f32,
    /// # C++ Info
    /// - name: `dynamicTranslationTolerance`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "dynamicTranslationTolerance")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "dynamicTranslationTolerance"))]
    pub m_dynamicTranslationTolerance: f32,
    /// # C++ Info
    /// - name: `dynamicRotationTolerance`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 20`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "dynamicRotationTolerance"))]
    #[cfg_attr(feature = "serde", serde(rename = "dynamicRotationTolerance"))]
    pub m_dynamicRotationTolerance: f32,
    /// # C++ Info
    /// - name: `dynamicScaleTolerance`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 24`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "dynamicScaleTolerance"))]
    #[cfg_attr(feature = "serde", serde(rename = "dynamicScaleTolerance"))]
    pub m_dynamicScaleTolerance: f32,
    /// # C++ Info
    /// - name: `dynamicFloatTolerance`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 28`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "dynamicFloatTolerance"))]
    #[cfg_attr(feature = "serde", serde(rename = "dynamicFloatTolerance"))]
    pub m_dynamicFloatTolerance: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass
    for hkaPredictiveCompressedAnimationTrackCompressionParams<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaPredictiveCompressedAnimationTrackCompressionParams"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x739c5f1)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize
    for hkaPredictiveCompressedAnimationTrackCompressionParams<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x739c5f1)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkaPredictiveCompressedAnimationTrackCompressionParams",
                    class_meta,
                    (0u64, 32u64),
                )?;
            serializer
                .serialize_field(
                    "staticTranslationTolerance",
                    &self.m_staticTranslationTolerance,
                )?;
            serializer
                .serialize_field(
                    "staticRotationTolerance",
                    &self.m_staticRotationTolerance,
                )?;
            serializer
                .serialize_field("staticScaleTolerance", &self.m_staticScaleTolerance)?;
            serializer
                .serialize_field("staticFloatTolerance", &self.m_staticFloatTolerance)?;
            serializer
                .serialize_field(
                    "dynamicTranslationTolerance",
                    &self.m_dynamicTranslationTolerance,
                )?;
            serializer
                .serialize_field(
                    "dynamicRotationTolerance",
                    &self.m_dynamicRotationTolerance,
                )?;
            serializer
                .serialize_field(
                    "dynamicScaleTolerance",
                    &self.m_dynamicScaleTolerance,
                )?;
            serializer
                .serialize_field(
                    "dynamicFloatTolerance",
                    &self.m_dynamicFloatTolerance,
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
    impl<'de> _serde::Deserialize<'de>
    for hkaPredictiveCompressedAnimationTrackCompressionParams<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_staticTranslationTolerance,
                m_staticRotationTolerance,
                m_staticScaleTolerance,
                m_staticFloatTolerance,
                m_dynamicTranslationTolerance,
                m_dynamicRotationTolerance,
                m_dynamicScaleTolerance,
                m_dynamicFloatTolerance,
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
                        "staticTranslationTolerance" => {
                            Ok(__Field::m_staticTranslationTolerance)
                        }
                        "staticRotationTolerance" => {
                            Ok(__Field::m_staticRotationTolerance)
                        }
                        "staticScaleTolerance" => Ok(__Field::m_staticScaleTolerance),
                        "staticFloatTolerance" => Ok(__Field::m_staticFloatTolerance),
                        "dynamicTranslationTolerance" => {
                            Ok(__Field::m_dynamicTranslationTolerance)
                        }
                        "dynamicRotationTolerance" => {
                            Ok(__Field::m_dynamicRotationTolerance)
                        }
                        "dynamicScaleTolerance" => Ok(__Field::m_dynamicScaleTolerance),
                        "dynamicFloatTolerance" => Ok(__Field::m_dynamicFloatTolerance),
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
            struct __hkaPredictiveCompressedAnimationTrackCompressionParamsVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkaPredictiveCompressedAnimationTrackCompressionParams<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkaPredictiveCompressedAnimationTrackCompressionParamsVisitor<'de> {
                type Value = hkaPredictiveCompressedAnimationTrackCompressionParams<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkaPredictiveCompressedAnimationTrackCompressionParams",
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
                    let mut m_staticTranslationTolerance: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_staticRotationTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_staticScaleTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_staticFloatTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_dynamicTranslationTolerance: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_dynamicRotationTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_dynamicScaleTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_dynamicFloatTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    for i in 0..8usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_staticTranslationTolerance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "staticTranslationTolerance",
                                        ),
                                    );
                                }
                                m_staticTranslationTolerance = _serde::__private::Some(
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
                                    &m_staticRotationTolerance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "staticRotationTolerance",
                                        ),
                                    );
                                }
                                m_staticRotationTolerance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_staticScaleTolerance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "staticScaleTolerance",
                                        ),
                                    );
                                }
                                m_staticScaleTolerance = _serde::__private::Some(
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
                                    &m_staticFloatTolerance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "staticFloatTolerance",
                                        ),
                                    );
                                }
                                m_staticFloatTolerance = _serde::__private::Some(
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
                                    &m_dynamicTranslationTolerance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dynamicTranslationTolerance",
                                        ),
                                    );
                                }
                                m_dynamicTranslationTolerance = _serde::__private::Some(
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
                                    &m_dynamicRotationTolerance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dynamicRotationTolerance",
                                        ),
                                    );
                                }
                                m_dynamicRotationTolerance = _serde::__private::Some(
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
                                    &m_dynamicScaleTolerance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dynamicScaleTolerance",
                                        ),
                                    );
                                }
                                m_dynamicScaleTolerance = _serde::__private::Some(
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
                                    &m_dynamicFloatTolerance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dynamicFloatTolerance",
                                        ),
                                    );
                                }
                                m_dynamicFloatTolerance = _serde::__private::Some(
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
                    let m_staticTranslationTolerance = match m_staticTranslationTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticTranslationTolerance",
                                ),
                            );
                        }
                    };
                    let m_staticRotationTolerance = match m_staticRotationTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticRotationTolerance",
                                ),
                            );
                        }
                    };
                    let m_staticScaleTolerance = match m_staticScaleTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticScaleTolerance",
                                ),
                            );
                        }
                    };
                    let m_staticFloatTolerance = match m_staticFloatTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticFloatTolerance",
                                ),
                            );
                        }
                    };
                    let m_dynamicTranslationTolerance = match m_dynamicTranslationTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dynamicTranslationTolerance",
                                ),
                            );
                        }
                    };
                    let m_dynamicRotationTolerance = match m_dynamicRotationTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dynamicRotationTolerance",
                                ),
                            );
                        }
                    };
                    let m_dynamicScaleTolerance = match m_dynamicScaleTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dynamicScaleTolerance",
                                ),
                            );
                        }
                    };
                    let m_dynamicFloatTolerance = match m_dynamicFloatTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dynamicFloatTolerance",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkaPredictiveCompressedAnimationTrackCompressionParams {
                        __ptr,
                        m_staticTranslationTolerance,
                        m_staticRotationTolerance,
                        m_staticScaleTolerance,
                        m_staticFloatTolerance,
                        m_dynamicTranslationTolerance,
                        m_dynamicRotationTolerance,
                        m_dynamicScaleTolerance,
                        m_dynamicFloatTolerance,
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
                    let mut m_staticTranslationTolerance: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_staticRotationTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_staticScaleTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_staticFloatTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_dynamicTranslationTolerance: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_dynamicRotationTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_dynamicScaleTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_dynamicFloatTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_staticTranslationTolerance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_staticTranslationTolerance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "staticTranslationTolerance",
                                        ),
                                    );
                                }
                                m_staticTranslationTolerance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_staticRotationTolerance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_staticRotationTolerance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "staticRotationTolerance",
                                        ),
                                    );
                                }
                                m_staticRotationTolerance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_staticScaleTolerance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_staticScaleTolerance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "staticScaleTolerance",
                                        ),
                                    );
                                }
                                m_staticScaleTolerance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_staticFloatTolerance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_staticFloatTolerance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "staticFloatTolerance",
                                        ),
                                    );
                                }
                                m_staticFloatTolerance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_dynamicTranslationTolerance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_dynamicTranslationTolerance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dynamicTranslationTolerance",
                                        ),
                                    );
                                }
                                m_dynamicTranslationTolerance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_dynamicRotationTolerance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_dynamicRotationTolerance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dynamicRotationTolerance",
                                        ),
                                    );
                                }
                                m_dynamicRotationTolerance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_dynamicScaleTolerance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_dynamicScaleTolerance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dynamicScaleTolerance",
                                        ),
                                    );
                                }
                                m_dynamicScaleTolerance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_dynamicFloatTolerance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_dynamicFloatTolerance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dynamicFloatTolerance",
                                        ),
                                    );
                                }
                                m_dynamicFloatTolerance = _serde::__private::Some(
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
                    let m_staticTranslationTolerance = match m_staticTranslationTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticTranslationTolerance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_staticRotationTolerance = match m_staticRotationTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticRotationTolerance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_staticScaleTolerance = match m_staticScaleTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticScaleTolerance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_staticFloatTolerance = match m_staticFloatTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticFloatTolerance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_dynamicTranslationTolerance = match m_dynamicTranslationTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dynamicTranslationTolerance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_dynamicRotationTolerance = match m_dynamicRotationTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dynamicRotationTolerance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_dynamicScaleTolerance = match m_dynamicScaleTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dynamicScaleTolerance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_dynamicFloatTolerance = match m_dynamicFloatTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dynamicFloatTolerance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkaPredictiveCompressedAnimationTrackCompressionParams {
                        __ptr: __ptr.clone(),
                        m_staticTranslationTolerance,
                        m_staticRotationTolerance,
                        m_staticScaleTolerance,
                        m_staticFloatTolerance,
                        m_dynamicTranslationTolerance,
                        m_dynamicRotationTolerance,
                        m_dynamicScaleTolerance,
                        m_dynamicFloatTolerance,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "staticTranslationTolerance",
                "staticRotationTolerance",
                "staticScaleTolerance",
                "staticFloatTolerance",
                "dynamicTranslationTolerance",
                "dynamicRotationTolerance",
                "dynamicScaleTolerance",
                "dynamicFloatTolerance",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkaPredictiveCompressedAnimationTrackCompressionParams",
                FIELDS,
                __hkaPredictiveCompressedAnimationTrackCompressionParamsVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkaPredictiveCompressedAnimationTrackCompressionParams,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
