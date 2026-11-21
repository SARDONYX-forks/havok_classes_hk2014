use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclVolumeConstraintMx`
/// - version: `0`
/// - signature: `0x37582f2`
/// - size: `  0`(x86)/` 96`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclVolumeConstraintMx<'a> {
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
    pub parent: hclConstraintSet<'a>,
    /// # C++ Info
    /// - name: `frameBatchDatas`(ctype: `hkArray<struct hclVolumeConstraintMxFrameBatchData>`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "frameBatchDatas"))]
    #[cfg_attr(feature = "serde", serde(rename = "frameBatchDatas"))]
    pub m_frameBatchDatas: Vec<hclVolumeConstraintMxFrameBatchData<'a>>,
    /// # C++ Info
    /// - name: `frameSingleDatas`(ctype: `hkArray<struct hclVolumeConstraintMxFrameSingleData>`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "frameSingleDatas"))]
    #[cfg_attr(feature = "serde", serde(rename = "frameSingleDatas"))]
    pub m_frameSingleDatas: Vec<hclVolumeConstraintMxFrameSingleData<'a>>,
    /// # C++ Info
    /// - name: `applyBatchDatas`(ctype: `hkArray<struct hclVolumeConstraintMxApplyBatchData>`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "applyBatchDatas"))]
    #[cfg_attr(feature = "serde", serde(rename = "applyBatchDatas"))]
    pub m_applyBatchDatas: Vec<hclVolumeConstraintMxApplyBatchData<'a>>,
    /// # C++ Info
    /// - name: `applySingleDatas`(ctype: `hkArray<struct hclVolumeConstraintMxApplySingleData>`)
    /// - offset: `  0`(x86)/` 80`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "applySingleDatas"))]
    #[cfg_attr(feature = "serde", serde(rename = "applySingleDatas"))]
    pub m_applySingleDatas: Vec<hclVolumeConstraintMxApplySingleData<'a>>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclVolumeConstraintMx<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclVolumeConstraintMx"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x37582f2)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(
                self
                    .m_frameBatchDatas
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(
                self
                    .m_frameSingleDatas
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(
                self
                    .m_applyBatchDatas
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(
                self
                    .m_applySingleDatas
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v
        }
    }
    impl<'a> _serde::Serialize for hclVolumeConstraintMx<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x37582f2)));
            let mut serializer = __serializer
                .serialize_struct("hclVolumeConstraintMx", class_meta, (0u64, 96u64))?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndRefCount",
                    &self.parent.parent.m_memSizeAndRefCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("name", &self.parent.m_name)?;
            serializer.skip_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "frameBatchDatas",
                    &self.m_frameBatchDatas,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 352u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "frameSingleDatas",
                    &self.m_frameSingleDatas,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 32u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "applyBatchDatas",
                    &self.m_applyBatchDatas,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 352u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "applySingleDatas",
                    &self.m_applySingleDatas,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 32u64,
                    },
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
    impl<'de> _serde::Deserialize<'de> for hclVolumeConstraintMx<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_name,
                m_frameBatchDatas,
                m_frameSingleDatas,
                m_applyBatchDatas,
                m_applySingleDatas,
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
                        "name" => Ok(__Field::m_name),
                        "frameBatchDatas" => Ok(__Field::m_frameBatchDatas),
                        "frameSingleDatas" => Ok(__Field::m_frameSingleDatas),
                        "applyBatchDatas" => Ok(__Field::m_applyBatchDatas),
                        "applySingleDatas" => Ok(__Field::m_applySingleDatas),
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
            struct __hclVolumeConstraintMxVisitor<'de> {
                marker: _serde::__private::PhantomData<hclVolumeConstraintMx<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hclVolumeConstraintMxVisitor<'de> {
                type Value = hclVolumeConstraintMx<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclVolumeConstraintMx",
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
                    let mut m_frameBatchDatas: _serde::__private::Option<
                        Vec<hclVolumeConstraintMxFrameBatchData<'de>>,
                    > = _serde::__private::None;
                    let mut m_frameSingleDatas: _serde::__private::Option<
                        Vec<hclVolumeConstraintMxFrameSingleData<'de>>,
                    > = _serde::__private::None;
                    let mut m_applyBatchDatas: _serde::__private::Option<
                        Vec<hclVolumeConstraintMxApplyBatchData<'de>>,
                    > = _serde::__private::None;
                    let mut m_applySingleDatas: _serde::__private::Option<
                        Vec<hclVolumeConstraintMxApplySingleData<'de>>,
                    > = _serde::__private::None;
                    for i in 0..4usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_frameBatchDatas) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "frameBatchDatas",
                                        ),
                                    );
                                }
                                m_frameBatchDatas = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclVolumeConstraintMxFrameBatchData<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_frameSingleDatas) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "frameSingleDatas",
                                        ),
                                    );
                                }
                                m_frameSingleDatas = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclVolumeConstraintMxFrameSingleData<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_applyBatchDatas) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "applyBatchDatas",
                                        ),
                                    );
                                }
                                m_applyBatchDatas = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclVolumeConstraintMxApplyBatchData<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_applySingleDatas) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "applySingleDatas",
                                        ),
                                    );
                                }
                                m_applySingleDatas = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclVolumeConstraintMxApplySingleData<'de>>,
                                    >(&mut __map) {
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
                    let m_frameBatchDatas = match m_frameBatchDatas {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "frameBatchDatas",
                                ),
                            );
                        }
                    };
                    let m_frameSingleDatas = match m_frameSingleDatas {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "frameSingleDatas",
                                ),
                            );
                        }
                    };
                    let m_applyBatchDatas = match m_applyBatchDatas {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "applyBatchDatas",
                                ),
                            );
                        }
                    };
                    let m_applySingleDatas = match m_applySingleDatas {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "applySingleDatas",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hclVolumeConstraintMx {
                        __ptr,
                        parent,
                        m_frameBatchDatas,
                        m_frameSingleDatas,
                        m_applyBatchDatas,
                        m_applySingleDatas,
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
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_frameBatchDatas: _serde::__private::Option<
                        Vec<hclVolumeConstraintMxFrameBatchData<'de>>,
                    > = _serde::__private::None;
                    let mut m_frameSingleDatas: _serde::__private::Option<
                        Vec<hclVolumeConstraintMxFrameSingleData<'de>>,
                    > = _serde::__private::None;
                    let mut m_applyBatchDatas: _serde::__private::Option<
                        Vec<hclVolumeConstraintMxApplyBatchData<'de>>,
                    > = _serde::__private::None;
                    let mut m_applySingleDatas: _serde::__private::Option<
                        Vec<hclVolumeConstraintMxApplySingleData<'de>>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_name => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_name) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                m_name = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_frameBatchDatas => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_frameBatchDatas) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "frameBatchDatas",
                                        ),
                                    );
                                }
                                m_frameBatchDatas = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclVolumeConstraintMxFrameBatchData<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_frameSingleDatas => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_frameSingleDatas) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "frameSingleDatas",
                                        ),
                                    );
                                }
                                m_frameSingleDatas = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclVolumeConstraintMxFrameSingleData<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_applyBatchDatas => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_applyBatchDatas) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "applyBatchDatas",
                                        ),
                                    );
                                }
                                m_applyBatchDatas = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclVolumeConstraintMxApplyBatchData<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_applySingleDatas => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_applySingleDatas) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "applySingleDatas",
                                        ),
                                    );
                                }
                                m_applySingleDatas = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclVolumeConstraintMxApplySingleData<'de>>,
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
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_frameBatchDatas = match m_frameBatchDatas {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "frameBatchDatas",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_frameSingleDatas = match m_frameSingleDatas {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "frameSingleDatas",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_applyBatchDatas = match m_applyBatchDatas {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "applyBatchDatas",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_applySingleDatas = match m_applySingleDatas {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "applySingleDatas",
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
                    let parent = hclConstraintSet {
                        __ptr: __ptr.clone(),
                        parent,
                        m_name,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hclVolumeConstraintMx {
                        __ptr: __ptr.clone(),
                        parent,
                        m_frameBatchDatas,
                        m_frameSingleDatas,
                        m_applyBatchDatas,
                        m_applySingleDatas,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "frameBatchDatas",
                "frameSingleDatas",
                "applyBatchDatas",
                "applySingleDatas",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclVolumeConstraintMx",
                FIELDS,
                __hclVolumeConstraintMxVisitor {
                    marker: _serde::__private::PhantomData::<hclVolumeConstraintMx>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
