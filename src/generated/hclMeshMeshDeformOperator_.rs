use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclMeshMeshDeformOperator`
/// - version: `2`
/// - signature: `0xff4f25c3`
/// - size: `  0`(x86)/`104`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclMeshMeshDeformOperator<'a> {
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
    pub parent: hclOperator<'a>,
    /// # C++ Info
    /// - name: `inputTrianglesSubset`(ctype: `hkArray<hkUint16>`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "inputTrianglesSubset"))]
    #[cfg_attr(feature = "serde", serde(rename = "inputTrianglesSubset"))]
    pub m_inputTrianglesSubset: Vec<U16<'a>>,
    /// # C++ Info
    /// - name: `triangleVertexPairs`(ctype: `hkArray<struct hclMeshMeshDeformOperatorTriangleVertexPair>`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "triangleVertexPairs"))]
    #[cfg_attr(feature = "serde", serde(rename = "triangleVertexPairs"))]
    pub m_triangleVertexPairs: Vec<hclMeshMeshDeformOperatorTriangleVertexPair<'a>>,
    /// # C++ Info
    /// - name: `triangleVertexStartForVertex`(ctype: `hkArray<hkUint16>`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "triangleVertexStartForVertex")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "triangleVertexStartForVertex"))]
    pub m_triangleVertexStartForVertex: Vec<U16<'a>>,
    /// # C++ Info
    /// - name: `inputBufferIdx`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/` 80`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "inputBufferIdx"))]
    #[cfg_attr(feature = "serde", serde(rename = "inputBufferIdx"))]
    pub m_inputBufferIdx: U32<'a>,
    /// # C++ Info
    /// - name: `outputBufferIdx`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/` 84`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "outputBufferIdx"))]
    #[cfg_attr(feature = "serde", serde(rename = "outputBufferIdx"))]
    pub m_outputBufferIdx: U32<'a>,
    /// # C++ Info
    /// - name: `startVertex`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/` 88`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "startVertex"))]
    #[cfg_attr(feature = "serde", serde(rename = "startVertex"))]
    pub m_startVertex: U16<'a>,
    /// # C++ Info
    /// - name: `endVertex`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/` 90`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "endVertex"))]
    #[cfg_attr(feature = "serde", serde(rename = "endVertex"))]
    pub m_endVertex: U16<'a>,
    /// # C++ Info
    /// - name: `scaleNormalBehaviour`(ctype: `enum ScaleNormalBehaviour`)
    /// - offset: `  0`(x86)/` 92`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "scaleNormalBehaviour"))]
    #[cfg_attr(feature = "serde", serde(rename = "scaleNormalBehaviour"))]
    pub m_scaleNormalBehaviour: ScaleNormalBehaviour,
    /// # C++ Info
    /// - name: `deformNormals`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 96`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "deformNormals"))]
    #[cfg_attr(feature = "serde", serde(rename = "deformNormals"))]
    pub m_deformNormals: bool,
    /// # C++ Info
    /// - name: `partialDeform`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 97`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "partialDeform"))]
    #[cfg_attr(feature = "serde", serde(rename = "partialDeform"))]
    pub m_partialDeform: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclMeshMeshDeformOperator<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclMeshMeshDeformOperator"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xff4f25c3)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(
                self
                    .m_triangleVertexPairs
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v
        }
    }
    impl<'a> _serde::Serialize for hclMeshMeshDeformOperator<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xff4f25c3)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hclMeshMeshDeformOperator",
                    class_meta,
                    (0u64, 104u64),
                )?;
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
                    "inputTrianglesSubset",
                    &self.m_inputTrianglesSubset,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "triangleVertexPairs",
                    &self.m_triangleVertexPairs,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 48u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "triangleVertexStartForVertex",
                    &self.m_triangleVertexStartForVertex,
                    TypeSize::NonPtr,
                )?;
            serializer.serialize_field("inputBufferIdx", &self.m_inputBufferIdx)?;
            serializer.serialize_field("outputBufferIdx", &self.m_outputBufferIdx)?;
            serializer.serialize_field("startVertex", &self.m_startVertex)?;
            serializer.serialize_field("endVertex", &self.m_endVertex)?;
            serializer
                .serialize_field("scaleNormalBehaviour", &self.m_scaleNormalBehaviour)?;
            serializer.serialize_field("deformNormals", &self.m_deformNormals)?;
            serializer.serialize_field("partialDeform", &self.m_partialDeform)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hclMeshMeshDeformOperator<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_name,
                m_inputTrianglesSubset,
                m_triangleVertexPairs,
                m_triangleVertexStartForVertex,
                m_inputBufferIdx,
                m_outputBufferIdx,
                m_startVertex,
                m_endVertex,
                m_scaleNormalBehaviour,
                m_deformNormals,
                m_partialDeform,
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
                        "inputTrianglesSubset" => Ok(__Field::m_inputTrianglesSubset),
                        "triangleVertexPairs" => Ok(__Field::m_triangleVertexPairs),
                        "triangleVertexStartForVertex" => {
                            Ok(__Field::m_triangleVertexStartForVertex)
                        }
                        "inputBufferIdx" => Ok(__Field::m_inputBufferIdx),
                        "outputBufferIdx" => Ok(__Field::m_outputBufferIdx),
                        "startVertex" => Ok(__Field::m_startVertex),
                        "endVertex" => Ok(__Field::m_endVertex),
                        "scaleNormalBehaviour" => Ok(__Field::m_scaleNormalBehaviour),
                        "deformNormals" => Ok(__Field::m_deformNormals),
                        "partialDeform" => Ok(__Field::m_partialDeform),
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
            struct __hclMeshMeshDeformOperatorVisitor<'de> {
                marker: _serde::__private::PhantomData<hclMeshMeshDeformOperator<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hclMeshMeshDeformOperatorVisitor<'de> {
                type Value = hclMeshMeshDeformOperator<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclMeshMeshDeformOperator",
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
                    let mut m_inputTrianglesSubset: _serde::__private::Option<
                        Vec<U16<'de>>,
                    > = _serde::__private::None;
                    let mut m_triangleVertexPairs: _serde::__private::Option<
                        Vec<hclMeshMeshDeformOperatorTriangleVertexPair<'de>>,
                    > = _serde::__private::None;
                    let mut m_triangleVertexStartForVertex: _serde::__private::Option<
                        Vec<U16<'de>>,
                    > = _serde::__private::None;
                    let mut m_inputBufferIdx: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_outputBufferIdx: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_startVertex: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_endVertex: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_scaleNormalBehaviour: _serde::__private::Option<
                        ScaleNormalBehaviour,
                    > = _serde::__private::None;
                    let mut m_deformNormals: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_partialDeform: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..10usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_inputTrianglesSubset,
                                ) {
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
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_triangleVertexPairs,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "triangleVertexPairs",
                                        ),
                                    );
                                }
                                m_triangleVertexPairs = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclMeshMeshDeformOperatorTriangleVertexPair<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_triangleVertexStartForVertex,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "triangleVertexStartForVertex",
                                        ),
                                    );
                                }
                                m_triangleVertexStartForVertex = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_inputBufferIdx) {
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
                            4usize => {
                                if _serde::__private::Option::is_some(&m_outputBufferIdx) {
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
                            5usize => {
                                if _serde::__private::Option::is_some(&m_startVertex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "startVertex",
                                        ),
                                    );
                                }
                                m_startVertex = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_endVertex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "endVertex",
                                        ),
                                    );
                                }
                                m_endVertex = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_scaleNormalBehaviour,
                                ) {
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
                            8usize => {
                                if _serde::__private::Option::is_some(&m_deformNormals) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deformNormals",
                                        ),
                                    );
                                }
                                m_deformNormals = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_partialDeform) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "partialDeform",
                                        ),
                                    );
                                }
                                m_partialDeform = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
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
                    __A::pad(&mut __map, 0usize, 6usize)?;
                    let m_inputTrianglesSubset = match m_inputTrianglesSubset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "inputTrianglesSubset",
                                ),
                            );
                        }
                    };
                    let m_triangleVertexPairs = match m_triangleVertexPairs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "triangleVertexPairs",
                                ),
                            );
                        }
                    };
                    let m_triangleVertexStartForVertex = match m_triangleVertexStartForVertex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "triangleVertexStartForVertex",
                                ),
                            );
                        }
                    };
                    let m_inputBufferIdx = match m_inputBufferIdx {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "inputBufferIdx",
                                ),
                            );
                        }
                    };
                    let m_outputBufferIdx = match m_outputBufferIdx {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "outputBufferIdx",
                                ),
                            );
                        }
                    };
                    let m_startVertex = match m_startVertex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "startVertex",
                                ),
                            );
                        }
                    };
                    let m_endVertex = match m_endVertex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "endVertex",
                                ),
                            );
                        }
                    };
                    let m_scaleNormalBehaviour = match m_scaleNormalBehaviour {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "scaleNormalBehaviour",
                                ),
                            );
                        }
                    };
                    let m_deformNormals = match m_deformNormals {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deformNormals",
                                ),
                            );
                        }
                    };
                    let m_partialDeform = match m_partialDeform {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "partialDeform",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hclMeshMeshDeformOperator {
                        __ptr,
                        parent,
                        m_inputTrianglesSubset,
                        m_triangleVertexPairs,
                        m_triangleVertexStartForVertex,
                        m_inputBufferIdx,
                        m_outputBufferIdx,
                        m_startVertex,
                        m_endVertex,
                        m_scaleNormalBehaviour,
                        m_deformNormals,
                        m_partialDeform,
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
                    let mut m_inputTrianglesSubset: _serde::__private::Option<
                        Vec<U16<'de>>,
                    > = _serde::__private::None;
                    let mut m_triangleVertexPairs: _serde::__private::Option<
                        Vec<hclMeshMeshDeformOperatorTriangleVertexPair<'de>>,
                    > = _serde::__private::None;
                    let mut m_triangleVertexStartForVertex: _serde::__private::Option<
                        Vec<U16<'de>>,
                    > = _serde::__private::None;
                    let mut m_inputBufferIdx: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_outputBufferIdx: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_startVertex: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_endVertex: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_scaleNormalBehaviour: _serde::__private::Option<
                        ScaleNormalBehaviour,
                    > = _serde::__private::None;
                    let mut m_deformNormals: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_partialDeform: _serde::__private::Option<bool> = _serde::__private::None;
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
                            __Field::m_triangleVertexPairs => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_triangleVertexPairs,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "triangleVertexPairs",
                                        ),
                                    );
                                }
                                m_triangleVertexPairs = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclMeshMeshDeformOperatorTriangleVertexPair<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_triangleVertexStartForVertex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_triangleVertexStartForVertex,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "triangleVertexStartForVertex",
                                        ),
                                    );
                                }
                                m_triangleVertexStartForVertex = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
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
                            __Field::m_startVertex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_startVertex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "startVertex",
                                        ),
                                    );
                                }
                                m_startVertex = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_endVertex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_endVertex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "endVertex",
                                        ),
                                    );
                                }
                                m_endVertex = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
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
                            __Field::m_deformNormals => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_deformNormals) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deformNormals",
                                        ),
                                    );
                                }
                                m_deformNormals = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_partialDeform => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_partialDeform) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "partialDeform",
                                        ),
                                    );
                                }
                                m_partialDeform = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
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
                    let m_triangleVertexPairs = match m_triangleVertexPairs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "triangleVertexPairs",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_triangleVertexStartForVertex = match m_triangleVertexStartForVertex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "triangleVertexStartForVertex",
                                ),
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
                    let m_startVertex = match m_startVertex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "startVertex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_endVertex = match m_endVertex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "endVertex",
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
                    let m_deformNormals = match m_deformNormals {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deformNormals",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_partialDeform = match m_partialDeform {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "partialDeform",
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hclMeshMeshDeformOperator {
                        __ptr: __ptr.clone(),
                        parent,
                        m_inputTrianglesSubset,
                        m_triangleVertexPairs,
                        m_triangleVertexStartForVertex,
                        m_inputBufferIdx,
                        m_outputBufferIdx,
                        m_startVertex,
                        m_endVertex,
                        m_scaleNormalBehaviour,
                        m_deformNormals,
                        m_partialDeform,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "inputTrianglesSubset",
                "triangleVertexPairs",
                "triangleVertexStartForVertex",
                "inputBufferIdx",
                "outputBufferIdx",
                "startVertex",
                "endVertex",
                "scaleNormalBehaviour",
                "deformNormals",
                "partialDeform",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclMeshMeshDeformOperator",
                FIELDS,
                __hclMeshMeshDeformOperatorVisitor {
                    marker: _serde::__private::PhantomData::<hclMeshMeshDeformOperator>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
/// # C++ Info
/// - name: `ScaleNormalBehaviour`(ctype: `hkEnum<ScaleNormalBehaviour, hkUint32>`)
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum ScaleNormalBehaviour {
    #[default]
    SCALE_NORMAL_IGNORE = 0isize,
    SCALE_NORMAL_APPLY = 1isize,
    SCALE_NORMAL_INVERT = 2isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for ScaleNormalBehaviour {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::SCALE_NORMAL_IGNORE => {
                    __serializer.serialize_field("SCALE_NORMAL_IGNORE", &0u64)
                }
                Self::SCALE_NORMAL_APPLY => {
                    __serializer.serialize_field("SCALE_NORMAL_APPLY", &1u64)
                }
                Self::SCALE_NORMAL_INVERT => {
                    __serializer.serialize_field("SCALE_NORMAL_INVERT", &2u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u32()
                .ok_or(S::Error::custom("Failed enum ScaleNormalBehaviour to_u32"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for ScaleNormalBehaviour {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_uint32<__E>(
                    self,
                    __value: U32<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        U32::Number(0u32) => _serde::__private::Ok(__Field::__field0),
                        U32::Number(1u32) => _serde::__private::Ok(__Field::__field1),
                        U32::Number(2u32) => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint32(__value),
                                    &"value(u32) of variant is one of 0, 1, 2",
                                ),
                            )
                        }
                    }
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "0"
                                || v.eq_ignore_ascii_case("SCALE_NORMAL_IGNORE") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("SCALE_NORMAL_APPLY") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("SCALE_NORMAL_INVERT") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        _serde::de::ReadEnumSize::Uint32,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<ScaleNormalBehaviour>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ScaleNormalBehaviour;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum ScaleNormalBehaviour",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ScaleNormalBehaviour::SCALE_NORMAL_IGNORE,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ScaleNormalBehaviour::SCALE_NORMAL_APPLY,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ScaleNormalBehaviour::SCALE_NORMAL_INVERT,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "SCALE_NORMAL_IGNORE",
                "SCALE_NORMAL_APPLY",
                "SCALE_NORMAL_INVERT",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "ScaleNormalBehaviour",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<ScaleNormalBehaviour>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
