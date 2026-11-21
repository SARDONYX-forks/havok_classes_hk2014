use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclStandardLinkSetupObject`
/// - version: `1`
/// - signature: `0x7e0ea82a`
/// - size: `  0`(x86)/`120`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclStandardLinkSetupObject<'a> {
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
    pub parent: hclConstraintSetSetupObject<'a>,
    /// # C++ Info
    /// - name: `name`(ctype: `TYPE_STRINGPTR`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// - name: `simulationMesh`(ctype: `hclSimulationSetupMesh`)
    /// - offset: `  0`(x86)/` 24`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simulationMesh"))]
    #[cfg_attr(feature = "serde", serde(rename = "simulationMesh"))]
    pub m_simulationMesh: Pointer<'a>,
    /// # C++ Info
    /// - name: `vertexSelection`(ctype: `hclVertexSelectionInput`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "vertexSelection"))]
    #[cfg_attr(feature = "serde", serde(rename = "vertexSelection"))]
    pub m_vertexSelection: hclVertexSelectionInput<'a>,
    /// # C++ Info
    /// - name: `edgeSelection`(ctype: `hclEdgeSelectionInput`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "edgeSelection"))]
    #[cfg_attr(feature = "serde", serde(rename = "edgeSelection"))]
    pub m_edgeSelection: hclEdgeSelectionInput<'a>,
    /// # C++ Info
    /// - name: `ignoreHiddenEdges`(ctype: `TYPE_BOOL`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "ignoreHiddenEdges"))]
    #[cfg_attr(feature = "serde", serde(rename = "ignoreHiddenEdges"))]
    pub m_ignoreHiddenEdges: bool,
    /// # C++ Info
    /// - name: `stiffness`(ctype: `hclVertexFloatInput`)
    /// - offset: `  0`(x86)/` 72`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "stiffness"))]
    #[cfg_attr(feature = "serde", serde(rename = "stiffness"))]
    pub m_stiffness: hclVertexFloatInput<'a>,
    /// # C++ Info
    /// - name: `allowedCompression`(ctype: `hclVertexFloatInput`)
    /// - offset: `  0`(x86)/` 88`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "allowedCompression"))]
    #[cfg_attr(feature = "serde", serde(rename = "allowedCompression"))]
    pub m_allowedCompression: hclVertexFloatInput<'a>,
    /// # C++ Info
    /// - name: `allowedStretching`(ctype: `hclVertexFloatInput`)
    /// - offset: `  0`(x86)/`104`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "allowedStretching"))]
    #[cfg_attr(feature = "serde", serde(rename = "allowedStretching"))]
    pub m_allowedStretching: hclVertexFloatInput<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclStandardLinkSetupObject<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclStandardLinkSetupObject"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x7e0ea82a)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.m_simulationMesh);
            v.extend(self.m_vertexSelection.deps_indexes());
            v.extend(self.m_edgeSelection.deps_indexes());
            v.extend(self.m_stiffness.deps_indexes());
            v.extend(self.m_allowedCompression.deps_indexes());
            v.extend(self.m_allowedStretching.deps_indexes());
            v
        }
    }
    impl<'a> _serde::Serialize for hclStandardLinkSetupObject<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x7e0ea82a)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hclStandardLinkSetupObject",
                    class_meta,
                    (0u64, 120u64),
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndRefCount",
                    &self.parent.parent.m_memSizeAndRefCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("name", &self.m_name)?;
            serializer.serialize_field("simulationMesh", &self.m_simulationMesh)?;
            serializer.serialize_field("vertexSelection", &self.m_vertexSelection)?;
            serializer.serialize_field("edgeSelection", &self.m_edgeSelection)?;
            serializer.serialize_field("ignoreHiddenEdges", &self.m_ignoreHiddenEdges)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("stiffness", &self.m_stiffness)?;
            serializer
                .serialize_field("allowedCompression", &self.m_allowedCompression)?;
            serializer.serialize_field("allowedStretching", &self.m_allowedStretching)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hclStandardLinkSetupObject<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_name,
                m_simulationMesh,
                m_vertexSelection,
                m_edgeSelection,
                m_ignoreHiddenEdges,
                m_stiffness,
                m_allowedCompression,
                m_allowedStretching,
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
                        "simulationMesh" => Ok(__Field::m_simulationMesh),
                        "vertexSelection" => Ok(__Field::m_vertexSelection),
                        "edgeSelection" => Ok(__Field::m_edgeSelection),
                        "ignoreHiddenEdges" => Ok(__Field::m_ignoreHiddenEdges),
                        "stiffness" => Ok(__Field::m_stiffness),
                        "allowedCompression" => Ok(__Field::m_allowedCompression),
                        "allowedStretching" => Ok(__Field::m_allowedStretching),
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
            struct __hclStandardLinkSetupObjectVisitor<'de> {
                marker: _serde::__private::PhantomData<hclStandardLinkSetupObject<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hclStandardLinkSetupObjectVisitor<'de> {
                type Value = hclStandardLinkSetupObject<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclStandardLinkSetupObject",
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
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_simulationMesh: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_vertexSelection: _serde::__private::Option<
                        hclVertexSelectionInput<'de>,
                    > = _serde::__private::None;
                    let mut m_edgeSelection: _serde::__private::Option<
                        hclEdgeSelectionInput<'de>,
                    > = _serde::__private::None;
                    let mut m_ignoreHiddenEdges: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_stiffness: _serde::__private::Option<
                        hclVertexFloatInput<'de>,
                    > = _serde::__private::None;
                    let mut m_allowedCompression: _serde::__private::Option<
                        hclVertexFloatInput<'de>,
                    > = _serde::__private::None;
                    let mut m_allowedStretching: _serde::__private::Option<
                        hclVertexFloatInput<'de>,
                    > = _serde::__private::None;
                    for i in 0..8usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_name) {
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
                            1usize => {
                                if _serde::__private::Option::is_some(&m_simulationMesh) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simulationMesh",
                                        ),
                                    );
                                }
                                m_simulationMesh = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_vertexSelection) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertexSelection",
                                        ),
                                    );
                                }
                                m_vertexSelection = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclVertexSelectionInput<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_edgeSelection) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "edgeSelection",
                                        ),
                                    );
                                }
                                m_edgeSelection = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclEdgeSelectionInput<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_ignoreHiddenEdges,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ignoreHiddenEdges",
                                        ),
                                    );
                                }
                                m_ignoreHiddenEdges = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_stiffness) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stiffness",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 7usize)?;
                                m_stiffness = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclVertexFloatInput<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_allowedCompression,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "allowedCompression",
                                        ),
                                    );
                                }
                                m_allowedCompression = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclVertexFloatInput<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_allowedStretching,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "allowedStretching",
                                        ),
                                    );
                                }
                                m_allowedStretching = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclVertexFloatInput<'de>,
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
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                        }
                    };
                    let m_simulationMesh = match m_simulationMesh {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simulationMesh",
                                ),
                            );
                        }
                    };
                    let m_vertexSelection = match m_vertexSelection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vertexSelection",
                                ),
                            );
                        }
                    };
                    let m_edgeSelection = match m_edgeSelection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "edgeSelection",
                                ),
                            );
                        }
                    };
                    let m_ignoreHiddenEdges = match m_ignoreHiddenEdges {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ignoreHiddenEdges",
                                ),
                            );
                        }
                    };
                    let m_stiffness = match m_stiffness {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stiffness",
                                ),
                            );
                        }
                    };
                    let m_allowedCompression = match m_allowedCompression {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "allowedCompression",
                                ),
                            );
                        }
                    };
                    let m_allowedStretching = match m_allowedStretching {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "allowedStretching",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hclStandardLinkSetupObject {
                        __ptr,
                        parent,
                        m_name,
                        m_simulationMesh,
                        m_vertexSelection,
                        m_edgeSelection,
                        m_ignoreHiddenEdges,
                        m_stiffness,
                        m_allowedCompression,
                        m_allowedStretching,
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
                    let mut m_simulationMesh: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_vertexSelection: _serde::__private::Option<
                        hclVertexSelectionInput<'de>,
                    > = _serde::__private::None;
                    let mut m_edgeSelection: _serde::__private::Option<
                        hclEdgeSelectionInput<'de>,
                    > = _serde::__private::None;
                    let mut m_ignoreHiddenEdges: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_stiffness: _serde::__private::Option<
                        hclVertexFloatInput<'de>,
                    > = _serde::__private::None;
                    let mut m_allowedCompression: _serde::__private::Option<
                        hclVertexFloatInput<'de>,
                    > = _serde::__private::None;
                    let mut m_allowedStretching: _serde::__private::Option<
                        hclVertexFloatInput<'de>,
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
                            __Field::m_simulationMesh => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_simulationMesh) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simulationMesh",
                                        ),
                                    );
                                }
                                m_simulationMesh = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_vertexSelection => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_vertexSelection) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertexSelection",
                                        ),
                                    );
                                }
                                m_vertexSelection = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclVertexSelectionInput<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_edgeSelection => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_edgeSelection) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "edgeSelection",
                                        ),
                                    );
                                }
                                m_edgeSelection = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclEdgeSelectionInput<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_ignoreHiddenEdges => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_ignoreHiddenEdges,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ignoreHiddenEdges",
                                        ),
                                    );
                                }
                                m_ignoreHiddenEdges = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_stiffness => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_stiffness) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stiffness",
                                        ),
                                    );
                                }
                                m_stiffness = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclVertexFloatInput<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_allowedCompression => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_allowedCompression,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "allowedCompression",
                                        ),
                                    );
                                }
                                m_allowedCompression = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclVertexFloatInput<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_allowedStretching => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_allowedStretching,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "allowedStretching",
                                        ),
                                    );
                                }
                                m_allowedStretching = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclVertexFloatInput<'de>,
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
                    let m_simulationMesh = match m_simulationMesh {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simulationMesh",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_vertexSelection = match m_vertexSelection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vertexSelection",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_edgeSelection = match m_edgeSelection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "edgeSelection",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_ignoreHiddenEdges = match m_ignoreHiddenEdges {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ignoreHiddenEdges",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_stiffness = match m_stiffness {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stiffness",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_allowedCompression = match m_allowedCompression {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "allowedCompression",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_allowedStretching = match m_allowedStretching {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "allowedStretching",
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
                    let parent = hclConstraintSetSetupObject {
                        __ptr: __ptr.clone(),
                        parent,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hclStandardLinkSetupObject {
                        __ptr: __ptr.clone(),
                        parent,
                        m_name,
                        m_simulationMesh,
                        m_vertexSelection,
                        m_edgeSelection,
                        m_ignoreHiddenEdges,
                        m_stiffness,
                        m_allowedCompression,
                        m_allowedStretching,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "name",
                "simulationMesh",
                "vertexSelection",
                "edgeSelection",
                "ignoreHiddenEdges",
                "stiffness",
                "allowedCompression",
                "allowedStretching",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclStandardLinkSetupObject",
                FIELDS,
                __hclStandardLinkSetupObjectVisitor {
                    marker: _serde::__private::PhantomData::<hclStandardLinkSetupObject>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
