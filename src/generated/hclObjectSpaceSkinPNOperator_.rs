use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclObjectSpaceSkinPNOperator`
/// - version: `1`
/// - signature: `0x84488b96`
/// - size: `  0`(x86)/`192`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclObjectSpaceSkinPNOperator<'a> {
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
    pub parent: hclObjectSpaceSkinOperator<'a>,
    /// # C++ Info
    /// - name: `localPNs`(ctype: `hkArray<struct hclObjectSpaceDeformerLocalBlockPN>`)
    /// - offset: `  0`(x86)/`160`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "localPNs"))]
    #[cfg_attr(feature = "serde", serde(rename = "localPNs"))]
    pub m_localPNs: Vec<hclObjectSpaceDeformerLocalBlockPN<'a>>,
    /// # C++ Info
    /// - name: `localUnpackedPNs`(ctype: `hkArray<struct hclObjectSpaceDeformerLocalBlockUnpackedPN>`)
    /// - offset: `  0`(x86)/`176`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "localUnpackedPNs"))]
    #[cfg_attr(feature = "serde", serde(rename = "localUnpackedPNs"))]
    pub m_localUnpackedPNs: Vec<hclObjectSpaceDeformerLocalBlockUnpackedPN<'a>>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclObjectSpaceSkinPNOperator<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclObjectSpaceSkinPNOperator"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x84488b96)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(self.parent.m_objectSpaceDeformer.deps_indexes());
            v.extend(
                self
                    .m_localPNs
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(
                self
                    .m_localUnpackedPNs
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v
        }
    }
    impl<'a> _serde::Serialize for hclObjectSpaceSkinPNOperator<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x84488b96)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hclObjectSpaceSkinPNOperator",
                    class_meta,
                    (0u64, 192u64),
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndRefCount",
                    &self.parent.parent.parent.m_memSizeAndRefCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("type", &self.parent.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "boneFromSkinMeshTransforms",
                    &self.parent.m_boneFromSkinMeshTransforms,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "transformSubset",
                    &self.parent.m_transformSubset,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_field("outputBufferIndex", &self.parent.m_outputBufferIndex)?;
            serializer
                .serialize_field("transformSetIndex", &self.parent.m_transformSetIndex)?;
            serializer
                .serialize_field(
                    "objectSpaceDeformer",
                    &self.parent.m_objectSpaceDeformer,
                )?;
            serializer
                .serialize_array_field(
                    "localPNs",
                    &self.m_localPNs,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 256u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "localUnpackedPNs",
                    &self.m_localUnpackedPNs,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 512u64,
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
    impl<'de> _serde::Deserialize<'de> for hclObjectSpaceSkinPNOperator<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_name,
                m_boneFromSkinMeshTransforms,
                m_transformSubset,
                m_outputBufferIndex,
                m_transformSetIndex,
                m_objectSpaceDeformer,
                m_localPNs,
                m_localUnpackedPNs,
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
                        "boneFromSkinMeshTransforms" => {
                            Ok(__Field::m_boneFromSkinMeshTransforms)
                        }
                        "transformSubset" => Ok(__Field::m_transformSubset),
                        "outputBufferIndex" => Ok(__Field::m_outputBufferIndex),
                        "transformSetIndex" => Ok(__Field::m_transformSetIndex),
                        "objectSpaceDeformer" => Ok(__Field::m_objectSpaceDeformer),
                        "localPNs" => Ok(__Field::m_localPNs),
                        "localUnpackedPNs" => Ok(__Field::m_localUnpackedPNs),
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
            struct __hclObjectSpaceSkinPNOperatorVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hclObjectSpaceSkinPNOperator<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hclObjectSpaceSkinPNOperatorVisitor<'de> {
                type Value = hclObjectSpaceSkinPNOperator<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclObjectSpaceSkinPNOperator",
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
                    let mut m_localPNs: _serde::__private::Option<
                        Vec<hclObjectSpaceDeformerLocalBlockPN<'de>>,
                    > = _serde::__private::None;
                    let mut m_localUnpackedPNs: _serde::__private::Option<
                        Vec<hclObjectSpaceDeformerLocalBlockUnpackedPN>,
                    > = _serde::__private::None;
                    for i in 0..2usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_localPNs) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "localPNs",
                                        ),
                                    );
                                }
                                m_localPNs = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclObjectSpaceDeformerLocalBlockPN<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_localUnpackedPNs) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "localUnpackedPNs",
                                        ),
                                    );
                                }
                                m_localUnpackedPNs = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclObjectSpaceDeformerLocalBlockUnpackedPN>,
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
                    let m_localPNs = match m_localPNs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("localPNs"),
                            );
                        }
                    };
                    let m_localUnpackedPNs = match m_localUnpackedPNs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "localUnpackedPNs",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hclObjectSpaceSkinPNOperator {
                        __ptr,
                        parent,
                        m_localPNs,
                        m_localUnpackedPNs,
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
                    let mut m_boneFromSkinMeshTransforms: _serde::__private::Option<
                        Vec<Matrix4>,
                    > = _serde::__private::None;
                    let mut m_transformSubset: _serde::__private::Option<
                        Vec<U16<'de>>,
                    > = _serde::__private::None;
                    let mut m_outputBufferIndex: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_transformSetIndex: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_objectSpaceDeformer: _serde::__private::Option<
                        hclObjectSpaceDeformer<'de>,
                    > = _serde::__private::None;
                    let mut m_localPNs: _serde::__private::Option<
                        Vec<hclObjectSpaceDeformerLocalBlockPN<'de>>,
                    > = _serde::__private::None;
                    let mut m_localUnpackedPNs: _serde::__private::Option<
                        Vec<hclObjectSpaceDeformerLocalBlockUnpackedPN>,
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
                            __Field::m_boneFromSkinMeshTransforms => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_boneFromSkinMeshTransforms,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneFromSkinMeshTransforms",
                                        ),
                                    );
                                }
                                m_boneFromSkinMeshTransforms = _serde::__private::Some(
                                    match __A::next_value::<Vec<Matrix4>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_transformSubset => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_transformSubset) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transformSubset",
                                        ),
                                    );
                                }
                                m_transformSubset = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_outputBufferIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_outputBufferIndex,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "outputBufferIndex",
                                        ),
                                    );
                                }
                                m_outputBufferIndex = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_transformSetIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_transformSetIndex,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transformSetIndex",
                                        ),
                                    );
                                }
                                m_transformSetIndex = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_objectSpaceDeformer => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_objectSpaceDeformer,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "objectSpaceDeformer",
                                        ),
                                    );
                                }
                                m_objectSpaceDeformer = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclObjectSpaceDeformer<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_localPNs => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_localPNs) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "localPNs",
                                        ),
                                    );
                                }
                                m_localPNs = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclObjectSpaceDeformerLocalBlockPN<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_localUnpackedPNs => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_localUnpackedPNs) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "localUnpackedPNs",
                                        ),
                                    );
                                }
                                m_localUnpackedPNs = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclObjectSpaceDeformerLocalBlockUnpackedPN>,
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
                    let m_boneFromSkinMeshTransforms = match m_boneFromSkinMeshTransforms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneFromSkinMeshTransforms",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_transformSubset = match m_transformSubset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transformSubset",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_outputBufferIndex = match m_outputBufferIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "outputBufferIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_transformSetIndex = match m_transformSetIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transformSetIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_objectSpaceDeformer = match m_objectSpaceDeformer {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "objectSpaceDeformer",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_localPNs = match m_localPNs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("localPNs"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_localUnpackedPNs = match m_localUnpackedPNs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "localUnpackedPNs",
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
                    let parent = hclOperator {
                        __ptr: __ptr.clone(),
                        parent,
                        m_name,
                        ..Default::default()
                    };
                    let parent = hclObjectSpaceSkinOperator {
                        __ptr: __ptr.clone(),
                        parent,
                        m_boneFromSkinMeshTransforms,
                        m_transformSubset,
                        m_outputBufferIndex,
                        m_transformSetIndex,
                        m_objectSpaceDeformer,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hclObjectSpaceSkinPNOperator {
                        __ptr: __ptr.clone(),
                        parent,
                        m_localPNs,
                        m_localUnpackedPNs,
                    })
                }
            }
            const FIELDS: &[&str] = &["localPNs", "localUnpackedPNs"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclObjectSpaceSkinPNOperator",
                FIELDS,
                __hclObjectSpaceSkinPNOperatorVisitor {
                    marker: _serde::__private::PhantomData::<
                        hclObjectSpaceSkinPNOperator,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
