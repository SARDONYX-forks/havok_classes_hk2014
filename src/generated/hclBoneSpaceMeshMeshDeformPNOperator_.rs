use super::class_requires::*;
use super::*;
use super::hclBoneSpaceMeshMeshDeformOperator_::ScaleNormalBehaviour;
/// # C++ Info
/// - name: `hclBoneSpaceMeshMeshDeformPNOperator`
/// - version: `1`
/// - signature: `0x2a74c429`
/// - size: `  0`(x86)/`184`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclBoneSpaceMeshMeshDeformPNOperator<'a> {
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
    pub parent: hclBoneSpaceMeshMeshDeformOperator<'a>,
    /// # C++ Info
    /// - name: `localPNs`(ctype: `hkArray<struct hclBoneSpaceDeformerLocalBlockPN>`)
    /// - offset: `  0`(x86)/`152`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "localPNs"))]
    #[cfg_attr(feature = "serde", serde(rename = "localPNs"))]
    pub m_localPNs: Vec<hclBoneSpaceDeformerLocalBlockPN<'a>>,
    /// # C++ Info
    /// - name: `localUnpackedPNs`(ctype: `hkArray<struct hclBoneSpaceDeformerLocalBlockUnpackedPN>`)
    /// - offset: `  0`(x86)/`168`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "localUnpackedPNs"))]
    #[cfg_attr(feature = "serde", serde(rename = "localUnpackedPNs"))]
    pub m_localUnpackedPNs: Vec<hclBoneSpaceDeformerLocalBlockUnpackedPN<'a>>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclBoneSpaceMeshMeshDeformPNOperator<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclBoneSpaceMeshMeshDeformPNOperator"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x2a74c429)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(self.parent.m_boneSpaceDeformer.deps_indexes());
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
    impl<'a> _serde::Serialize for hclBoneSpaceMeshMeshDeformPNOperator<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x2a74c429)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hclBoneSpaceMeshMeshDeformPNOperator",
                    class_meta,
                    (0u64, 184u64),
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
            serializer.serialize_field("inputBufferIdx", &self.parent.m_inputBufferIdx)?;
            serializer
                .serialize_field("outputBufferIdx", &self.parent.m_outputBufferIdx)?;
            serializer
                .serialize_field(
                    "scaleNormalBehaviour",
                    &self.parent.m_scaleNormalBehaviour,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "inputTrianglesSubset",
                    &self.parent.m_inputTrianglesSubset,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_field("boneSpaceDeformer", &self.parent.m_boneSpaceDeformer)?;
            serializer
                .serialize_array_field(
                    "localPNs",
                    &self.m_localPNs,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 384u64,
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
    impl<'de> _serde::Deserialize<'de> for hclBoneSpaceMeshMeshDeformPNOperator<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_name,
                m_inputBufferIdx,
                m_outputBufferIdx,
                m_scaleNormalBehaviour,
                m_inputTrianglesSubset,
                m_boneSpaceDeformer,
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
                        "inputBufferIdx" => Ok(__Field::m_inputBufferIdx),
                        "outputBufferIdx" => Ok(__Field::m_outputBufferIdx),
                        "scaleNormalBehaviour" => Ok(__Field::m_scaleNormalBehaviour),
                        "inputTrianglesSubset" => Ok(__Field::m_inputTrianglesSubset),
                        "boneSpaceDeformer" => Ok(__Field::m_boneSpaceDeformer),
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
            struct __hclBoneSpaceMeshMeshDeformPNOperatorVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hclBoneSpaceMeshMeshDeformPNOperator<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hclBoneSpaceMeshMeshDeformPNOperatorVisitor<'de> {
                type Value = hclBoneSpaceMeshMeshDeformPNOperator<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclBoneSpaceMeshMeshDeformPNOperator",
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
                        Vec<hclBoneSpaceDeformerLocalBlockPN<'de>>,
                    > = _serde::__private::None;
                    let mut m_localUnpackedPNs: _serde::__private::Option<
                        Vec<hclBoneSpaceDeformerLocalBlockUnpackedPN>,
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
                                        Vec<hclBoneSpaceDeformerLocalBlockPN<'de>>,
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
                                        Vec<hclBoneSpaceDeformerLocalBlockUnpackedPN>,
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
                    _serde::__private::Ok(hclBoneSpaceMeshMeshDeformPNOperator {
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
                    let mut m_inputBufferIdx: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_outputBufferIdx: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_scaleNormalBehaviour: _serde::__private::Option<
                        ScaleNormalBehaviour,
                    > = _serde::__private::None;
                    let mut m_inputTrianglesSubset: _serde::__private::Option<
                        Vec<U16<'de>>,
                    > = _serde::__private::None;
                    let mut m_boneSpaceDeformer: _serde::__private::Option<
                        hclBoneSpaceDeformer<'de>,
                    > = _serde::__private::None;
                    let mut m_localPNs: _serde::__private::Option<
                        Vec<hclBoneSpaceDeformerLocalBlockPN<'de>>,
                    > = _serde::__private::None;
                    let mut m_localUnpackedPNs: _serde::__private::Option<
                        Vec<hclBoneSpaceDeformerLocalBlockUnpackedPN>,
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
                            __Field::m_inputBufferIdx => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_inputBufferIdx) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "inputBufferIdx",
                                        ),
                                    );
                                }
                                m_inputBufferIdx = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_outputBufferIdx => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_outputBufferIdx) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "outputBufferIdx",
                                        ),
                                    );
                                }
                                m_outputBufferIdx = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_scaleNormalBehaviour => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_scaleNormalBehaviour,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "scaleNormalBehaviour",
                                        ),
                                    );
                                }
                                m_scaleNormalBehaviour = _serde::__private::Some(
                                    match __A::next_value::<ScaleNormalBehaviour>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_inputTrianglesSubset => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_inputTrianglesSubset,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "inputTrianglesSubset",
                                        ),
                                    );
                                }
                                m_inputTrianglesSubset = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_boneSpaceDeformer => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_boneSpaceDeformer,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneSpaceDeformer",
                                        ),
                                    );
                                }
                                m_boneSpaceDeformer = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclBoneSpaceDeformer<'de>,
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
                                        Vec<hclBoneSpaceDeformerLocalBlockPN<'de>>,
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
                                        Vec<hclBoneSpaceDeformerLocalBlockUnpackedPN>,
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
                    let m_inputBufferIdx = match m_inputBufferIdx {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "inputBufferIdx",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_outputBufferIdx = match m_outputBufferIdx {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "outputBufferIdx",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_scaleNormalBehaviour = match m_scaleNormalBehaviour {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "scaleNormalBehaviour",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_inputTrianglesSubset = match m_inputTrianglesSubset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "inputTrianglesSubset",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_boneSpaceDeformer = match m_boneSpaceDeformer {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneSpaceDeformer",
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
                    let parent = hclBoneSpaceMeshMeshDeformOperator {
                        __ptr: __ptr.clone(),
                        parent,
                        m_inputBufferIdx,
                        m_outputBufferIdx,
                        m_scaleNormalBehaviour,
                        m_inputTrianglesSubset,
                        m_boneSpaceDeformer,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hclBoneSpaceMeshMeshDeformPNOperator {
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
                "hclBoneSpaceMeshMeshDeformPNOperator",
                FIELDS,
                __hclBoneSpaceMeshMeshDeformPNOperatorVisitor {
                    marker: _serde::__private::PhantomData::<
                        hclBoneSpaceMeshMeshDeformPNOperator,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
