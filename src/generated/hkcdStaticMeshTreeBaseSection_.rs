use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkcdStaticMeshTreeBaseSection`
/// - version: `3`
/// - signature: `0xfd01ccd8`
/// - size: `  0`(x86)/` 96`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkcdStaticMeshTreeBaseSection<'a> {
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
    pub parent: hkcdStaticTreeTreehkcdStaticTreeDynamicStorage4<'a>,
    /// # C++ Info
    /// - name: `codecParms`(ctype: `hkReal[6]`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/` 24`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "codecParms"))]
    #[cfg_attr(feature = "serde", serde(rename = "codecParms"))]
    pub m_codecParms: [f32; 6usize],
    /// # C++ Info
    /// - name: `firstPackedVertex`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/` 72`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "firstPackedVertex"))]
    #[cfg_attr(feature = "serde", serde(rename = "firstPackedVertex"))]
    pub m_firstPackedVertex: U32<'a>,
    /// # C++ Info
    /// - name: `sharedVertices`(ctype: `struct hkcdStaticMeshTreeBaseSectionSharedVertices`)
    /// - offset: `  0`(x86)/` 76`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "sharedVertices"))]
    #[cfg_attr(feature = "serde", serde(rename = "sharedVertices"))]
    pub m_sharedVertices: hkcdStaticMeshTreeBaseSectionSharedVertices<'a>,
    /// # C++ Info
    /// - name: `primitives`(ctype: `struct hkcdStaticMeshTreeBaseSectionPrimitives`)
    /// - offset: `  0`(x86)/` 80`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "primitives"))]
    #[cfg_attr(feature = "serde", serde(rename = "primitives"))]
    pub m_primitives: hkcdStaticMeshTreeBaseSectionPrimitives<'a>,
    /// # C++ Info
    /// - name: `dataRuns`(ctype: `struct hkcdStaticMeshTreeBaseSectionDataRuns`)
    /// - offset: `  0`(x86)/` 84`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "dataRuns"))]
    #[cfg_attr(feature = "serde", serde(rename = "dataRuns"))]
    pub m_dataRuns: hkcdStaticMeshTreeBaseSectionDataRuns<'a>,
    /// # C++ Info
    /// - name: `numPackedVertices`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/` 88`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "numPackedVertices"))]
    #[cfg_attr(feature = "serde", serde(rename = "numPackedVertices"))]
    pub m_numPackedVertices: U8<'a>,
    /// # C++ Info
    /// - name: `numSharedIndices`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/` 89`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "numSharedIndices"))]
    #[cfg_attr(feature = "serde", serde(rename = "numSharedIndices"))]
    pub m_numSharedIndices: U8<'a>,
    /// # C++ Info
    /// - name: `leafIndex`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/` 90`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "leafIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "leafIndex"))]
    pub m_leafIndex: U16<'a>,
    /// # C++ Info
    /// - name: `page`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/` 92`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "page"))]
    #[cfg_attr(feature = "serde", serde(rename = "page"))]
    pub m_page: U8<'a>,
    /// # C++ Info
    /// - name: `flags`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/` 93`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "flags"))]
    #[cfg_attr(feature = "serde", serde(rename = "flags"))]
    pub m_flags: U8<'a>,
    /// # C++ Info
    /// - name: `layerData`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/` 94`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "layerData"))]
    #[cfg_attr(feature = "serde", serde(rename = "layerData"))]
    pub m_layerData: U8<'a>,
    /// # C++ Info
    /// - name: `unusedData`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/` 95`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "unusedData"))]
    #[cfg_attr(feature = "serde", serde(rename = "unusedData"))]
    pub m_unusedData: U8<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkcdStaticMeshTreeBaseSection<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkcdStaticMeshTreeBaseSection"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xfd01ccd8)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(
                self
                    .parent
                    .parent
                    .parent
                    .m_nodes
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(self.parent.m_domain.deps_indexes());
            v.extend(self.m_sharedVertices.deps_indexes());
            v.extend(self.m_primitives.deps_indexes());
            v.extend(self.m_dataRuns.deps_indexes());
            v
        }
    }
    impl<'a> _serde::Serialize for hkcdStaticMeshTreeBaseSection<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xfd01ccd8)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkcdStaticMeshTreeBaseSection",
                    class_meta,
                    (0u64, 96u64),
                )?;
            serializer
                .serialize_array_field(
                    "nodes",
                    &self.parent.parent.parent.m_nodes,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 4u64,
                    },
                )?;
            serializer.serialize_field("domain", &self.parent.m_domain)?;
            serializer
                .serialize_fixed_array_field(
                    "codecParms",
                    self.m_codecParms.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer.serialize_field("firstPackedVertex", &self.m_firstPackedVertex)?;
            serializer.serialize_field("sharedVertices", &self.m_sharedVertices)?;
            serializer.serialize_field("primitives", &self.m_primitives)?;
            serializer.serialize_field("dataRuns", &self.m_dataRuns)?;
            serializer.serialize_field("numPackedVertices", &self.m_numPackedVertices)?;
            serializer.serialize_field("numSharedIndices", &self.m_numSharedIndices)?;
            serializer.serialize_field("leafIndex", &self.m_leafIndex)?;
            serializer.serialize_field("page", &self.m_page)?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer.serialize_field("layerData", &self.m_layerData)?;
            serializer.serialize_field("unusedData", &self.m_unusedData)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkcdStaticMeshTreeBaseSection<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_nodes,
                m_domain,
                m_codecParms,
                m_firstPackedVertex,
                m_sharedVertices,
                m_primitives,
                m_dataRuns,
                m_numPackedVertices,
                m_numSharedIndices,
                m_leafIndex,
                m_page,
                m_flags,
                m_layerData,
                m_unusedData,
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
                        "nodes" => Ok(__Field::m_nodes),
                        "domain" => Ok(__Field::m_domain),
                        "codecParms" => Ok(__Field::m_codecParms),
                        "firstPackedVertex" => Ok(__Field::m_firstPackedVertex),
                        "sharedVertices" => Ok(__Field::m_sharedVertices),
                        "primitives" => Ok(__Field::m_primitives),
                        "dataRuns" => Ok(__Field::m_dataRuns),
                        "numPackedVertices" => Ok(__Field::m_numPackedVertices),
                        "numSharedIndices" => Ok(__Field::m_numSharedIndices),
                        "leafIndex" => Ok(__Field::m_leafIndex),
                        "page" => Ok(__Field::m_page),
                        "flags" => Ok(__Field::m_flags),
                        "layerData" => Ok(__Field::m_layerData),
                        "unusedData" => Ok(__Field::m_unusedData),
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
            struct __hkcdStaticMeshTreeBaseSectionVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkcdStaticMeshTreeBaseSection<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkcdStaticMeshTreeBaseSectionVisitor<'de> {
                type Value = hkcdStaticMeshTreeBaseSection<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkcdStaticMeshTreeBaseSection",
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
                    let mut m_codecParms: _serde::__private::Option<[f32; 6usize]> = _serde::__private::None;
                    let mut m_firstPackedVertex: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_sharedVertices: _serde::__private::Option<
                        hkcdStaticMeshTreeBaseSectionSharedVertices<'de>,
                    > = _serde::__private::None;
                    let mut m_primitives: _serde::__private::Option<
                        hkcdStaticMeshTreeBaseSectionPrimitives<'de>,
                    > = _serde::__private::None;
                    let mut m_dataRuns: _serde::__private::Option<
                        hkcdStaticMeshTreeBaseSectionDataRuns<'de>,
                    > = _serde::__private::None;
                    let mut m_numPackedVertices: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_numSharedIndices: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_leafIndex: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_page: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_flags: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_layerData: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_unusedData: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    for i in 0..12usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_codecParms) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "codecParms",
                                        ),
                                    );
                                }
                                m_codecParms = _serde::__private::Some(
                                    match __A::next_value::<[f32; 6usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_firstPackedVertex,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "firstPackedVertex",
                                        ),
                                    );
                                }
                                m_firstPackedVertex = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_sharedVertices) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sharedVertices",
                                        ),
                                    );
                                }
                                m_sharedVertices = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkcdStaticMeshTreeBaseSectionSharedVertices<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_primitives) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "primitives",
                                        ),
                                    );
                                }
                                m_primitives = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkcdStaticMeshTreeBaseSectionPrimitives<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_dataRuns) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dataRuns",
                                        ),
                                    );
                                }
                                m_dataRuns = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkcdStaticMeshTreeBaseSectionDataRuns<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_numPackedVertices,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numPackedVertices",
                                        ),
                                    );
                                }
                                m_numPackedVertices = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_numSharedIndices) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numSharedIndices",
                                        ),
                                    );
                                }
                                m_numSharedIndices = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_leafIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "leafIndex",
                                        ),
                                    );
                                }
                                m_leafIndex = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_page) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("page"),
                                    );
                                }
                                m_page = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_flags) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("flags"),
                                    );
                                }
                                m_flags = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_layerData) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "layerData",
                                        ),
                                    );
                                }
                                m_layerData = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_unusedData) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "unusedData",
                                        ),
                                    );
                                }
                                m_unusedData = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
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
                    let m_codecParms = match m_codecParms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "codecParms",
                                ),
                            );
                        }
                    };
                    let m_firstPackedVertex = match m_firstPackedVertex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "firstPackedVertex",
                                ),
                            );
                        }
                    };
                    let m_sharedVertices = match m_sharedVertices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sharedVertices",
                                ),
                            );
                        }
                    };
                    let m_primitives = match m_primitives {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "primitives",
                                ),
                            );
                        }
                    };
                    let m_dataRuns = match m_dataRuns {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("dataRuns"),
                            );
                        }
                    };
                    let m_numPackedVertices = match m_numPackedVertices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numPackedVertices",
                                ),
                            );
                        }
                    };
                    let m_numSharedIndices = match m_numSharedIndices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numSharedIndices",
                                ),
                            );
                        }
                    };
                    let m_leafIndex = match m_leafIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "leafIndex",
                                ),
                            );
                        }
                    };
                    let m_page = match m_page {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("page"),
                            );
                        }
                    };
                    let m_flags = match m_flags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("flags"),
                            );
                        }
                    };
                    let m_layerData = match m_layerData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "layerData",
                                ),
                            );
                        }
                    };
                    let m_unusedData = match m_unusedData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "unusedData",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkcdStaticMeshTreeBaseSection {
                        __ptr,
                        parent,
                        m_codecParms,
                        m_firstPackedVertex,
                        m_sharedVertices,
                        m_primitives,
                        m_dataRuns,
                        m_numPackedVertices,
                        m_numSharedIndices,
                        m_leafIndex,
                        m_page,
                        m_flags,
                        m_layerData,
                        m_unusedData,
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
                    let mut m_nodes: _serde::__private::Option<
                        Vec<hkcdStaticTreeCodec3Axis4<'de>>,
                    > = _serde::__private::None;
                    let mut m_domain: _serde::__private::Option<hkAabb> = _serde::__private::None;
                    let mut m_codecParms: _serde::__private::Option<[f32; 6usize]> = _serde::__private::None;
                    let mut m_firstPackedVertex: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_sharedVertices: _serde::__private::Option<
                        hkcdStaticMeshTreeBaseSectionSharedVertices<'de>,
                    > = _serde::__private::None;
                    let mut m_primitives: _serde::__private::Option<
                        hkcdStaticMeshTreeBaseSectionPrimitives<'de>,
                    > = _serde::__private::None;
                    let mut m_dataRuns: _serde::__private::Option<
                        hkcdStaticMeshTreeBaseSectionDataRuns<'de>,
                    > = _serde::__private::None;
                    let mut m_numPackedVertices: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_numSharedIndices: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_leafIndex: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_page: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_flags: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_layerData: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_unusedData: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_nodes => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_nodes) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("nodes"),
                                    );
                                }
                                m_nodes = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkcdStaticTreeCodec3Axis4<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_domain => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_domain) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("domain"),
                                    );
                                }
                                m_domain = _serde::__private::Some(
                                    match __A::next_value::<hkAabb>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_codecParms => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_codecParms) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "codecParms",
                                        ),
                                    );
                                }
                                m_codecParms = _serde::__private::Some(
                                    match __A::next_value::<[f32; 6usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_firstPackedVertex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_firstPackedVertex,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "firstPackedVertex",
                                        ),
                                    );
                                }
                                m_firstPackedVertex = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_sharedVertices => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_sharedVertices) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sharedVertices",
                                        ),
                                    );
                                }
                                m_sharedVertices = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkcdStaticMeshTreeBaseSectionSharedVertices<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_primitives => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_primitives) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "primitives",
                                        ),
                                    );
                                }
                                m_primitives = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkcdStaticMeshTreeBaseSectionPrimitives<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_dataRuns => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_dataRuns) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dataRuns",
                                        ),
                                    );
                                }
                                m_dataRuns = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkcdStaticMeshTreeBaseSectionDataRuns<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numPackedVertices => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_numPackedVertices,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numPackedVertices",
                                        ),
                                    );
                                }
                                m_numPackedVertices = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numSharedIndices => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_numSharedIndices) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numSharedIndices",
                                        ),
                                    );
                                }
                                m_numSharedIndices = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_leafIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_leafIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "leafIndex",
                                        ),
                                    );
                                }
                                m_leafIndex = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_page => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_page) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("page"),
                                    );
                                }
                                m_page = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_flags => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_flags) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("flags"),
                                    );
                                }
                                m_flags = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_layerData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_layerData) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "layerData",
                                        ),
                                    );
                                }
                                m_layerData = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_unusedData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_unusedData) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "unusedData",
                                        ),
                                    );
                                }
                                m_unusedData = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
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
                    let m_nodes = match m_nodes {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("nodes"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_domain = match m_domain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("domain"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_codecParms = match m_codecParms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "codecParms",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_firstPackedVertex = match m_firstPackedVertex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "firstPackedVertex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_sharedVertices = match m_sharedVertices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sharedVertices",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_primitives = match m_primitives {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "primitives",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_dataRuns = match m_dataRuns {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("dataRuns"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numPackedVertices = match m_numPackedVertices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numPackedVertices",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numSharedIndices = match m_numSharedIndices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numSharedIndices",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_leafIndex = match m_leafIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "leafIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_page = match m_page {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("page"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_flags = match m_flags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("flags"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_layerData = match m_layerData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "layerData",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_unusedData = match m_unusedData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "unusedData",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkcdStaticTreeDynamicStoragehkcdStaticTreeCodec3Axis4 {
                        __ptr: __ptr.clone(),
                        m_nodes,
                    };
                    let parent = hkcdStaticTreeDynamicStorage4 {
                        __ptr: __ptr.clone(),
                        parent,
                    };
                    let parent = hkcdStaticTreeTreehkcdStaticTreeDynamicStorage4 {
                        __ptr: __ptr.clone(),
                        parent,
                        m_domain,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkcdStaticMeshTreeBaseSection {
                        __ptr: __ptr.clone(),
                        parent,
                        m_codecParms,
                        m_firstPackedVertex,
                        m_sharedVertices,
                        m_primitives,
                        m_dataRuns,
                        m_numPackedVertices,
                        m_numSharedIndices,
                        m_leafIndex,
                        m_page,
                        m_flags,
                        m_layerData,
                        m_unusedData,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "codecParms",
                "firstPackedVertex",
                "sharedVertices",
                "primitives",
                "dataRuns",
                "numPackedVertices",
                "numSharedIndices",
                "leafIndex",
                "page",
                "flags",
                "layerData",
                "unusedData",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkcdStaticMeshTreeBaseSection",
                FIELDS,
                __hkcdStaticMeshTreeBaseSectionVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkcdStaticMeshTreeBaseSection,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
