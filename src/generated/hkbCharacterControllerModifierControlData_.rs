use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbCharacterControllerModifierControlData`
/// - version: `0`
/// - signature: `0xc4e809af`
/// - size: `  0`(x86)/` 16`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCharacterControllerModifierControlData<'a> {
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
    /// - name: `verticalGain`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "verticalGain"))]
    #[cfg_attr(feature = "serde", serde(rename = "verticalGain"))]
    pub m_verticalGain: f32,
    /// # C++ Info
    /// - name: `horizontalCatchUpGain`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`  4`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "horizontalCatchUpGain"))]
    #[cfg_attr(feature = "serde", serde(rename = "horizontalCatchUpGain"))]
    pub m_horizontalCatchUpGain: f32,
    /// # C++ Info
    /// - name: `maxVerticalSeparation`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`  8`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxVerticalSeparation"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxVerticalSeparation"))]
    pub m_maxVerticalSeparation: f32,
    /// # C++ Info
    /// - name: `maxHorizontalSeparation`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 12`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxHorizontalSeparation"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxHorizontalSeparation"))]
    pub m_maxHorizontalSeparation: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbCharacterControllerModifierControlData<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCharacterControllerModifierControlData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xc4e809af)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hkbCharacterControllerModifierControlData<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xc4e809af)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbCharacterControllerModifierControlData",
                    class_meta,
                    (0u64, 16u64),
                )?;
            serializer.serialize_field("verticalGain", &self.m_verticalGain)?;
            serializer
                .serialize_field(
                    "horizontalCatchUpGain",
                    &self.m_horizontalCatchUpGain,
                )?;
            serializer
                .serialize_field(
                    "maxVerticalSeparation",
                    &self.m_maxVerticalSeparation,
                )?;
            serializer
                .serialize_field(
                    "maxHorizontalSeparation",
                    &self.m_maxHorizontalSeparation,
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
    for hkbCharacterControllerModifierControlData<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_verticalGain,
                m_horizontalCatchUpGain,
                m_maxVerticalSeparation,
                m_maxHorizontalSeparation,
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
                        "verticalGain" => Ok(__Field::m_verticalGain),
                        "horizontalCatchUpGain" => Ok(__Field::m_horizontalCatchUpGain),
                        "maxVerticalSeparation" => Ok(__Field::m_maxVerticalSeparation),
                        "maxHorizontalSeparation" => {
                            Ok(__Field::m_maxHorizontalSeparation)
                        }
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
            struct __hkbCharacterControllerModifierControlDataVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkbCharacterControllerModifierControlData<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbCharacterControllerModifierControlDataVisitor<'de> {
                type Value = hkbCharacterControllerModifierControlData<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbCharacterControllerModifierControlData",
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
                    let mut m_verticalGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_horizontalCatchUpGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxVerticalSeparation: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxHorizontalSeparation: _serde::__private::Option<f32> = _serde::__private::None;
                    for i in 0..4usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_verticalGain) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "verticalGain",
                                        ),
                                    );
                                }
                                m_verticalGain = _serde::__private::Some(
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
                                    &m_horizontalCatchUpGain,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "horizontalCatchUpGain",
                                        ),
                                    );
                                }
                                m_horizontalCatchUpGain = _serde::__private::Some(
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
                                    &m_maxVerticalSeparation,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxVerticalSeparation",
                                        ),
                                    );
                                }
                                m_maxVerticalSeparation = _serde::__private::Some(
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
                                    &m_maxHorizontalSeparation,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxHorizontalSeparation",
                                        ),
                                    );
                                }
                                m_maxHorizontalSeparation = _serde::__private::Some(
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
                    let m_verticalGain = match m_verticalGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "verticalGain",
                                ),
                            );
                        }
                    };
                    let m_horizontalCatchUpGain = match m_horizontalCatchUpGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "horizontalCatchUpGain",
                                ),
                            );
                        }
                    };
                    let m_maxVerticalSeparation = match m_maxVerticalSeparation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxVerticalSeparation",
                                ),
                            );
                        }
                    };
                    let m_maxHorizontalSeparation = match m_maxHorizontalSeparation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxHorizontalSeparation",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbCharacterControllerModifierControlData {
                        __ptr,
                        m_verticalGain,
                        m_horizontalCatchUpGain,
                        m_maxVerticalSeparation,
                        m_maxHorizontalSeparation,
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
                    let mut m_verticalGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_horizontalCatchUpGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxVerticalSeparation: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxHorizontalSeparation: _serde::__private::Option<f32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_verticalGain => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_verticalGain) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "verticalGain",
                                        ),
                                    );
                                }
                                m_verticalGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_horizontalCatchUpGain => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_horizontalCatchUpGain,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "horizontalCatchUpGain",
                                        ),
                                    );
                                }
                                m_horizontalCatchUpGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxVerticalSeparation => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxVerticalSeparation,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxVerticalSeparation",
                                        ),
                                    );
                                }
                                m_maxVerticalSeparation = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxHorizontalSeparation => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxHorizontalSeparation,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxHorizontalSeparation",
                                        ),
                                    );
                                }
                                m_maxHorizontalSeparation = _serde::__private::Some(
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
                    let m_verticalGain = match m_verticalGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "verticalGain",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_horizontalCatchUpGain = match m_horizontalCatchUpGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "horizontalCatchUpGain",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxVerticalSeparation = match m_maxVerticalSeparation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxVerticalSeparation",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxHorizontalSeparation = match m_maxHorizontalSeparation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxHorizontalSeparation",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbCharacterControllerModifierControlData {
                        __ptr: __ptr.clone(),
                        m_verticalGain,
                        m_horizontalCatchUpGain,
                        m_maxVerticalSeparation,
                        m_maxHorizontalSeparation,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "verticalGain",
                "horizontalCatchUpGain",
                "maxVerticalSeparation",
                "maxHorizontalSeparation",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbCharacterControllerModifierControlData",
                FIELDS,
                __hkbCharacterControllerModifierControlDataVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbCharacterControllerModifierControlData,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
