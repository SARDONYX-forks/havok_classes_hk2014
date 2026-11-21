use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbCameraVariablesChangedCommand`
/// - version: `0`
/// - signature: `0x32f87051`
/// - size: `  0`(x86)/` 80`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCameraVariablesChangedCommand<'a> {
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
    /// Alternative to C++ class inheritance.
    #[cfg_attr(feature = "json_schema", schemars(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub parent: hkReferencedObject<'a>,
    /// # C++ Info
    /// - name: `cameraVariableFloatNames`(ctype: `hkArray<hkStringPtr>`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "cameraVariableFloatNames"))]
    #[cfg_attr(feature = "serde", serde(rename = "cameraVariableFloatNames"))]
    pub m_cameraVariableFloatNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// - name: `cameraFloatValues`(ctype: `hkArray<hkReal>`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "cameraFloatValues"))]
    #[cfg_attr(feature = "serde", serde(rename = "cameraFloatValues"))]
    pub m_cameraFloatValues: Vec<f32>,
    /// # C++ Info
    /// - name: `cameraVariableVectorNames`(ctype: `hkArray<hkStringPtr>`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "cameraVariableVectorNames"))]
    #[cfg_attr(feature = "serde", serde(rename = "cameraVariableVectorNames"))]
    pub m_cameraVariableVectorNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// - name: `cameraVectorValues`(ctype: `hkArray<hkVector4>`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "cameraVectorValues"))]
    #[cfg_attr(feature = "serde", serde(rename = "cameraVectorValues"))]
    pub m_cameraVectorValues: Vec<Vector4>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbCameraVariablesChangedCommand<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCameraVariablesChangedCommand"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x32f87051)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hkbCameraVariablesChangedCommand<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x32f87051)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbCameraVariablesChangedCommand",
                    class_meta,
                    (0u64, 80u64),
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndRefCount", &self.parent.m_memSizeAndRefCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cameraVariableFloatNames",
                    &self.m_cameraVariableFloatNames,
                    TypeSize::String,
                )?;
            serializer
                .serialize_array_field(
                    "cameraFloatValues",
                    &self.m_cameraFloatValues,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "cameraVariableVectorNames",
                    &self.m_cameraVariableVectorNames,
                    TypeSize::String,
                )?;
            serializer
                .serialize_array_field(
                    "cameraVectorValues",
                    &self.m_cameraVectorValues,
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
    impl<'de> _serde::Deserialize<'de> for hkbCameraVariablesChangedCommand<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_cameraVariableFloatNames,
                m_cameraFloatValues,
                m_cameraVariableVectorNames,
                m_cameraVectorValues,
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
                        "cameraVariableFloatNames" => {
                            Ok(__Field::m_cameraVariableFloatNames)
                        }
                        "cameraFloatValues" => Ok(__Field::m_cameraFloatValues),
                        "cameraVariableVectorNames" => {
                            Ok(__Field::m_cameraVariableVectorNames)
                        }
                        "cameraVectorValues" => Ok(__Field::m_cameraVectorValues),
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
            struct __hkbCameraVariablesChangedCommandVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkbCameraVariablesChangedCommand<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbCameraVariablesChangedCommandVisitor<'de> {
                type Value = hkbCameraVariablesChangedCommand<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbCameraVariablesChangedCommand",
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
                    let parent = __A::parent_value(&mut __map)?;
                    let mut m_cameraVariableFloatNames: _serde::__private::Option<
                        Vec<StringPtr<'de>>,
                    > = _serde::__private::None;
                    let mut m_cameraFloatValues: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_cameraVariableVectorNames: _serde::__private::Option<
                        Vec<StringPtr<'de>>,
                    > = _serde::__private::None;
                    let mut m_cameraVectorValues: _serde::__private::Option<
                        Vec<Vector4>,
                    > = _serde::__private::None;
                    for i in 0..4usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_cameraVariableFloatNames,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "cameraVariableFloatNames",
                                        ),
                                    );
                                }
                                m_cameraVariableFloatNames = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_cameraFloatValues,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "cameraFloatValues",
                                        ),
                                    );
                                }
                                m_cameraFloatValues = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_cameraVariableVectorNames,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "cameraVariableVectorNames",
                                        ),
                                    );
                                }
                                m_cameraVariableVectorNames = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_cameraVectorValues,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "cameraVectorValues",
                                        ),
                                    );
                                }
                                m_cameraVectorValues = _serde::__private::Some(
                                    match __A::next_value::<Vec<Vector4>>(&mut __map) {
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
                    let m_cameraVariableFloatNames = match m_cameraVariableFloatNames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "cameraVariableFloatNames",
                                ),
                            );
                        }
                    };
                    let m_cameraFloatValues = match m_cameraFloatValues {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "cameraFloatValues",
                                ),
                            );
                        }
                    };
                    let m_cameraVariableVectorNames = match m_cameraVariableVectorNames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "cameraVariableVectorNames",
                                ),
                            );
                        }
                    };
                    let m_cameraVectorValues = match m_cameraVectorValues {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "cameraVectorValues",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbCameraVariablesChangedCommand {
                        __ptr,
                        parent,
                        m_cameraVariableFloatNames,
                        m_cameraFloatValues,
                        m_cameraVariableVectorNames,
                        m_cameraVectorValues,
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
                    let mut m_cameraVariableFloatNames: _serde::__private::Option<
                        Vec<StringPtr<'de>>,
                    > = _serde::__private::None;
                    let mut m_cameraFloatValues: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_cameraVariableVectorNames: _serde::__private::Option<
                        Vec<StringPtr<'de>>,
                    > = _serde::__private::None;
                    let mut m_cameraVectorValues: _serde::__private::Option<
                        Vec<Vector4>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_cameraVariableFloatNames => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_cameraVariableFloatNames,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "cameraVariableFloatNames",
                                        ),
                                    );
                                }
                                m_cameraVariableFloatNames = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_cameraFloatValues => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_cameraFloatValues,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "cameraFloatValues",
                                        ),
                                    );
                                }
                                m_cameraFloatValues = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_cameraVariableVectorNames => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_cameraVariableVectorNames,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "cameraVariableVectorNames",
                                        ),
                                    );
                                }
                                m_cameraVariableVectorNames = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_cameraVectorValues => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_cameraVectorValues,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "cameraVectorValues",
                                        ),
                                    );
                                }
                                m_cameraVectorValues = _serde::__private::Some(
                                    match __A::next_value::<Vec<Vector4>>(&mut __map) {
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
                    let m_cameraVariableFloatNames = match m_cameraVariableFloatNames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "cameraVariableFloatNames",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_cameraFloatValues = match m_cameraFloatValues {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "cameraFloatValues",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_cameraVariableVectorNames = match m_cameraVariableVectorNames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "cameraVariableVectorNames",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_cameraVectorValues = match m_cameraVectorValues {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "cameraVectorValues",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkBaseObject {
                        __ptr: __ptr.clone(),
                    };
                    let parent = hkReferencedObject {
                        __ptr: __ptr.clone(),
                        parent,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbCameraVariablesChangedCommand {
                        __ptr: __ptr.clone(),
                        parent,
                        m_cameraVariableFloatNames,
                        m_cameraFloatValues,
                        m_cameraVariableVectorNames,
                        m_cameraVectorValues,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "cameraVariableFloatNames",
                "cameraFloatValues",
                "cameraVariableVectorNames",
                "cameraVectorValues",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbCameraVariablesChangedCommand",
                FIELDS,
                __hkbCameraVariablesChangedCommandVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbCameraVariablesChangedCommand,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
