use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclUpdateAllVertexFramesOperator`
/// - version: `3`
/// - signature: `0x9f7b4db7`
/// - size: `  0`(x86)/`144`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclUpdateAllVertexFramesOperator<'a> {
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
    /// - name: `vertToNormalID`(ctype: `hkArray<hkUint16>`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "vertToNormalID"))]
    #[cfg_attr(feature = "serde", serde(rename = "vertToNormalID"))]
    pub m_vertToNormalID: Vec<U16<'a>>,
    /// # C++ Info
    /// - name: `triangleFlips`(ctype: `hkArray<hkUint8>`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "triangleFlips"))]
    #[cfg_attr(feature = "serde", serde(rename = "triangleFlips"))]
    pub m_triangleFlips: Vec<U8<'a>>,
    /// # C++ Info
    /// - name: `referenceVertices`(ctype: `hkArray<hkUint16>`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "referenceVertices"))]
    #[cfg_attr(feature = "serde", serde(rename = "referenceVertices"))]
    pub m_referenceVertices: Vec<U16<'a>>,
    /// # C++ Info
    /// - name: `tangentEdgeCosAngle`(ctype: `hkArray<hkReal>`)
    /// - offset: `  0`(x86)/` 80`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "tangentEdgeCosAngle"))]
    #[cfg_attr(feature = "serde", serde(rename = "tangentEdgeCosAngle"))]
    pub m_tangentEdgeCosAngle: Vec<f32>,
    /// # C++ Info
    /// - name: `tangentEdgeSinAngle`(ctype: `hkArray<hkReal>`)
    /// - offset: `  0`(x86)/` 96`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "tangentEdgeSinAngle"))]
    #[cfg_attr(feature = "serde", serde(rename = "tangentEdgeSinAngle"))]
    pub m_tangentEdgeSinAngle: Vec<f32>,
    /// # C++ Info
    /// - name: `biTangentFlip`(ctype: `hkArray<hkReal>`)
    /// - offset: `  0`(x86)/`112`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "biTangentFlip"))]
    #[cfg_attr(feature = "serde", serde(rename = "biTangentFlip"))]
    pub m_biTangentFlip: Vec<f32>,
    /// # C++ Info
    /// - name: `bufferIdx`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/`128`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "bufferIdx"))]
    #[cfg_attr(feature = "serde", serde(rename = "bufferIdx"))]
    pub m_bufferIdx: U32<'a>,
    /// # C++ Info
    /// - name: `numUniqueNormalIDs`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/`132`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "numUniqueNormalIDs"))]
    #[cfg_attr(feature = "serde", serde(rename = "numUniqueNormalIDs"))]
    pub m_numUniqueNormalIDs: U32<'a>,
    /// # C++ Info
    /// - name: `updateNormals`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`136`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "updateNormals"))]
    #[cfg_attr(feature = "serde", serde(rename = "updateNormals"))]
    pub m_updateNormals: bool,
    /// # C++ Info
    /// - name: `updateTangents`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`137`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "updateTangents"))]
    #[cfg_attr(feature = "serde", serde(rename = "updateTangents"))]
    pub m_updateTangents: bool,
    /// # C++ Info
    /// - name: `updateBiTangents`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`138`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "updateBiTangents"))]
    #[cfg_attr(feature = "serde", serde(rename = "updateBiTangents"))]
    pub m_updateBiTangents: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclUpdateAllVertexFramesOperator<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclUpdateAllVertexFramesOperator"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x9f7b4db7)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hclUpdateAllVertexFramesOperator<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x9f7b4db7)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hclUpdateAllVertexFramesOperator",
                    class_meta,
                    (0u64, 144u64),
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
                    "vertToNormalID",
                    &self.m_vertToNormalID,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "triangleFlips",
                    &self.m_triangleFlips,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "referenceVertices",
                    &self.m_referenceVertices,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "tangentEdgeCosAngle",
                    &self.m_tangentEdgeCosAngle,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "tangentEdgeSinAngle",
                    &self.m_tangentEdgeSinAngle,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "biTangentFlip",
                    &self.m_biTangentFlip,
                    TypeSize::NonPtr,
                )?;
            serializer.serialize_field("bufferIdx", &self.m_bufferIdx)?;
            serializer
                .serialize_field("numUniqueNormalIDs", &self.m_numUniqueNormalIDs)?;
            serializer.serialize_field("updateNormals", &self.m_updateNormals)?;
            serializer.serialize_field("updateTangents", &self.m_updateTangents)?;
            serializer.serialize_field("updateBiTangents", &self.m_updateBiTangents)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hclUpdateAllVertexFramesOperator<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_name,
                m_vertToNormalID,
                m_triangleFlips,
                m_referenceVertices,
                m_tangentEdgeCosAngle,
                m_tangentEdgeSinAngle,
                m_biTangentFlip,
                m_bufferIdx,
                m_numUniqueNormalIDs,
                m_updateNormals,
                m_updateTangents,
                m_updateBiTangents,
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
                        "vertToNormalID" => Ok(__Field::m_vertToNormalID),
                        "triangleFlips" => Ok(__Field::m_triangleFlips),
                        "referenceVertices" => Ok(__Field::m_referenceVertices),
                        "tangentEdgeCosAngle" => Ok(__Field::m_tangentEdgeCosAngle),
                        "tangentEdgeSinAngle" => Ok(__Field::m_tangentEdgeSinAngle),
                        "biTangentFlip" => Ok(__Field::m_biTangentFlip),
                        "bufferIdx" => Ok(__Field::m_bufferIdx),
                        "numUniqueNormalIDs" => Ok(__Field::m_numUniqueNormalIDs),
                        "updateNormals" => Ok(__Field::m_updateNormals),
                        "updateTangents" => Ok(__Field::m_updateTangents),
                        "updateBiTangents" => Ok(__Field::m_updateBiTangents),
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
            struct __hclUpdateAllVertexFramesOperatorVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hclUpdateAllVertexFramesOperator<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hclUpdateAllVertexFramesOperatorVisitor<'de> {
                type Value = hclUpdateAllVertexFramesOperator<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclUpdateAllVertexFramesOperator",
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
                    let mut m_vertToNormalID: _serde::__private::Option<Vec<U16<'de>>> = _serde::__private::None;
                    let mut m_triangleFlips: _serde::__private::Option<Vec<U8<'de>>> = _serde::__private::None;
                    let mut m_referenceVertices: _serde::__private::Option<
                        Vec<U16<'de>>,
                    > = _serde::__private::None;
                    let mut m_tangentEdgeCosAngle: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_tangentEdgeSinAngle: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_biTangentFlip: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_bufferIdx: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_numUniqueNormalIDs: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_updateNormals: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_updateTangents: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_updateBiTangents: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..11usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_vertToNormalID) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertToNormalID",
                                        ),
                                    );
                                }
                                m_vertToNormalID = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_triangleFlips) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "triangleFlips",
                                        ),
                                    );
                                }
                                m_triangleFlips = _serde::__private::Some(
                                    match __A::next_value::<Vec<U8<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_referenceVertices,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "referenceVertices",
                                        ),
                                    );
                                }
                                m_referenceVertices = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_tangentEdgeCosAngle,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "tangentEdgeCosAngle",
                                        ),
                                    );
                                }
                                m_tangentEdgeCosAngle = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_tangentEdgeSinAngle,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "tangentEdgeSinAngle",
                                        ),
                                    );
                                }
                                m_tangentEdgeSinAngle = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_biTangentFlip) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "biTangentFlip",
                                        ),
                                    );
                                }
                                m_biTangentFlip = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_bufferIdx) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bufferIdx",
                                        ),
                                    );
                                }
                                m_bufferIdx = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_numUniqueNormalIDs,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numUniqueNormalIDs",
                                        ),
                                    );
                                }
                                m_numUniqueNormalIDs = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_updateNormals) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "updateNormals",
                                        ),
                                    );
                                }
                                m_updateNormals = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_updateTangents) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "updateTangents",
                                        ),
                                    );
                                }
                                m_updateTangents = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_updateBiTangents) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "updateBiTangents",
                                        ),
                                    );
                                }
                                m_updateBiTangents = _serde::__private::Some(
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
                    __A::pad(&mut __map, 0usize, 5usize)?;
                    let m_vertToNormalID = match m_vertToNormalID {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vertToNormalID",
                                ),
                            );
                        }
                    };
                    let m_triangleFlips = match m_triangleFlips {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "triangleFlips",
                                ),
                            );
                        }
                    };
                    let m_referenceVertices = match m_referenceVertices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "referenceVertices",
                                ),
                            );
                        }
                    };
                    let m_tangentEdgeCosAngle = match m_tangentEdgeCosAngle {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "tangentEdgeCosAngle",
                                ),
                            );
                        }
                    };
                    let m_tangentEdgeSinAngle = match m_tangentEdgeSinAngle {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "tangentEdgeSinAngle",
                                ),
                            );
                        }
                    };
                    let m_biTangentFlip = match m_biTangentFlip {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "biTangentFlip",
                                ),
                            );
                        }
                    };
                    let m_bufferIdx = match m_bufferIdx {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bufferIdx",
                                ),
                            );
                        }
                    };
                    let m_numUniqueNormalIDs = match m_numUniqueNormalIDs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numUniqueNormalIDs",
                                ),
                            );
                        }
                    };
                    let m_updateNormals = match m_updateNormals {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "updateNormals",
                                ),
                            );
                        }
                    };
                    let m_updateTangents = match m_updateTangents {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "updateTangents",
                                ),
                            );
                        }
                    };
                    let m_updateBiTangents = match m_updateBiTangents {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "updateBiTangents",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hclUpdateAllVertexFramesOperator {
                        __ptr,
                        parent,
                        m_vertToNormalID,
                        m_triangleFlips,
                        m_referenceVertices,
                        m_tangentEdgeCosAngle,
                        m_tangentEdgeSinAngle,
                        m_biTangentFlip,
                        m_bufferIdx,
                        m_numUniqueNormalIDs,
                        m_updateNormals,
                        m_updateTangents,
                        m_updateBiTangents,
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
                    let mut m_vertToNormalID: _serde::__private::Option<Vec<U16<'de>>> = _serde::__private::None;
                    let mut m_triangleFlips: _serde::__private::Option<Vec<U8<'de>>> = _serde::__private::None;
                    let mut m_referenceVertices: _serde::__private::Option<
                        Vec<U16<'de>>,
                    > = _serde::__private::None;
                    let mut m_tangentEdgeCosAngle: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_tangentEdgeSinAngle: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_biTangentFlip: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_bufferIdx: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_numUniqueNormalIDs: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_updateNormals: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_updateTangents: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_updateBiTangents: _serde::__private::Option<bool> = _serde::__private::None;
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
                            __Field::m_vertToNormalID => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_vertToNormalID) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertToNormalID",
                                        ),
                                    );
                                }
                                m_vertToNormalID = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_triangleFlips => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_triangleFlips) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "triangleFlips",
                                        ),
                                    );
                                }
                                m_triangleFlips = _serde::__private::Some(
                                    match __A::next_value::<Vec<U8<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_referenceVertices => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_referenceVertices,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "referenceVertices",
                                        ),
                                    );
                                }
                                m_referenceVertices = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_tangentEdgeCosAngle => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_tangentEdgeCosAngle,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "tangentEdgeCosAngle",
                                        ),
                                    );
                                }
                                m_tangentEdgeCosAngle = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_tangentEdgeSinAngle => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_tangentEdgeSinAngle,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "tangentEdgeSinAngle",
                                        ),
                                    );
                                }
                                m_tangentEdgeSinAngle = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_biTangentFlip => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_biTangentFlip) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "biTangentFlip",
                                        ),
                                    );
                                }
                                m_biTangentFlip = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bufferIdx => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bufferIdx) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bufferIdx",
                                        ),
                                    );
                                }
                                m_bufferIdx = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numUniqueNormalIDs => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_numUniqueNormalIDs,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numUniqueNormalIDs",
                                        ),
                                    );
                                }
                                m_numUniqueNormalIDs = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_updateNormals => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_updateNormals) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "updateNormals",
                                        ),
                                    );
                                }
                                m_updateNormals = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_updateTangents => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_updateTangents) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "updateTangents",
                                        ),
                                    );
                                }
                                m_updateTangents = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_updateBiTangents => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_updateBiTangents) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "updateBiTangents",
                                        ),
                                    );
                                }
                                m_updateBiTangents = _serde::__private::Some(
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
                    let m_vertToNormalID = match m_vertToNormalID {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vertToNormalID",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_triangleFlips = match m_triangleFlips {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "triangleFlips",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_referenceVertices = match m_referenceVertices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "referenceVertices",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_tangentEdgeCosAngle = match m_tangentEdgeCosAngle {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "tangentEdgeCosAngle",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_tangentEdgeSinAngle = match m_tangentEdgeSinAngle {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "tangentEdgeSinAngle",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_biTangentFlip = match m_biTangentFlip {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "biTangentFlip",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bufferIdx = match m_bufferIdx {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bufferIdx",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numUniqueNormalIDs = match m_numUniqueNormalIDs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numUniqueNormalIDs",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_updateNormals = match m_updateNormals {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "updateNormals",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_updateTangents = match m_updateTangents {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "updateTangents",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_updateBiTangents = match m_updateBiTangents {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "updateBiTangents",
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
                    _serde::__private::Ok(hclUpdateAllVertexFramesOperator {
                        __ptr: __ptr.clone(),
                        parent,
                        m_vertToNormalID,
                        m_triangleFlips,
                        m_referenceVertices,
                        m_tangentEdgeCosAngle,
                        m_tangentEdgeSinAngle,
                        m_biTangentFlip,
                        m_bufferIdx,
                        m_numUniqueNormalIDs,
                        m_updateNormals,
                        m_updateTangents,
                        m_updateBiTangents,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "vertToNormalID",
                "triangleFlips",
                "referenceVertices",
                "tangentEdgeCosAngle",
                "tangentEdgeSinAngle",
                "biTangentFlip",
                "bufferIdx",
                "numUniqueNormalIDs",
                "updateNormals",
                "updateTangents",
                "updateBiTangents",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclUpdateAllVertexFramesOperator",
                FIELDS,
                __hclUpdateAllVertexFramesOperatorVisitor {
                    marker: _serde::__private::PhantomData::<
                        hclUpdateAllVertexFramesOperator,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
