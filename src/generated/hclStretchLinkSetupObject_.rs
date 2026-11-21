use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclStretchLinkSetupObject`
/// - version: `1`
/// - signature: `0x18d53465`
/// - size: `  0`(x86)/`128`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclStretchLinkSetupObject<'a> {
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
    /// - name: `movableParticlesSelection`(ctype: `hclVertexSelectionInput`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "movableParticlesSelection"))]
    #[cfg_attr(feature = "serde", serde(rename = "movableParticlesSelection"))]
    pub m_movableParticlesSelection: hclVertexSelectionInput<'a>,
    /// # C++ Info
    /// - name: `fixedParticlesSelection`(ctype: `hclVertexSelectionInput`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "fixedParticlesSelection"))]
    #[cfg_attr(feature = "serde", serde(rename = "fixedParticlesSelection"))]
    pub m_fixedParticlesSelection: hclVertexSelectionInput<'a>,
    /// # C++ Info
    /// - name: `rigidFactor`(ctype: `hclVertexFloatInput`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "rigidFactor"))]
    #[cfg_attr(feature = "serde", serde(rename = "rigidFactor"))]
    pub m_rigidFactor: hclVertexFloatInput<'a>,
    /// # C++ Info
    /// - name: `stiffness`(ctype: `hclVertexFloatInput`)
    /// - offset: `  0`(x86)/` 80`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "stiffness"))]
    #[cfg_attr(feature = "serde", serde(rename = "stiffness"))]
    pub m_stiffness: hclVertexFloatInput<'a>,
    /// # C++ Info
    /// - name: `stretchDirection`(ctype: `TYPE_VECTOR4`)
    /// - offset: `  0`(x86)/` 96`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "stretchDirection"))]
    #[cfg_attr(feature = "serde", serde(rename = "stretchDirection"))]
    pub m_stretchDirection: Vector4,
    /// # C++ Info
    /// - name: `useStretchDirection`(ctype: `TYPE_BOOL`)
    /// - offset: `  0`(x86)/`112`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "useStretchDirection"))]
    #[cfg_attr(feature = "serde", serde(rename = "useStretchDirection"))]
    pub m_useStretchDirection: bool,
    /// # C++ Info
    /// - name: `useMeshTopology`(ctype: `TYPE_BOOL`)
    /// - offset: `  0`(x86)/`113`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "useMeshTopology"))]
    #[cfg_attr(feature = "serde", serde(rename = "useMeshTopology"))]
    pub m_useMeshTopology: bool,
    /// # C++ Info
    /// - name: `allowDynamicLinks`(ctype: `TYPE_BOOL`)
    /// - offset: `  0`(x86)/`114`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "allowDynamicLinks"))]
    #[cfg_attr(feature = "serde", serde(rename = "allowDynamicLinks"))]
    pub m_allowDynamicLinks: bool,
    /// # C++ Info
    /// - name: `useTopologicalStretchDistance`(ctype: `TYPE_BOOL`)
    /// - offset: `  0`(x86)/`115`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "useTopologicalStretchDistance")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "useTopologicalStretchDistance"))]
    pub m_useTopologicalStretchDistance: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclStretchLinkSetupObject<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclStretchLinkSetupObject"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x18d53465)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.m_simulationMesh);
            v.extend(self.m_movableParticlesSelection.deps_indexes());
            v.extend(self.m_fixedParticlesSelection.deps_indexes());
            v.extend(self.m_rigidFactor.deps_indexes());
            v.extend(self.m_stiffness.deps_indexes());
            v
        }
    }
    impl<'a> _serde::Serialize for hclStretchLinkSetupObject<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x18d53465)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hclStretchLinkSetupObject",
                    class_meta,
                    (0u64, 128u64),
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
            serializer
                .serialize_field(
                    "movableParticlesSelection",
                    &self.m_movableParticlesSelection,
                )?;
            serializer
                .serialize_field(
                    "fixedParticlesSelection",
                    &self.m_fixedParticlesSelection,
                )?;
            serializer.serialize_field("rigidFactor", &self.m_rigidFactor)?;
            serializer.serialize_field("stiffness", &self.m_stiffness)?;
            serializer.serialize_field("stretchDirection", &self.m_stretchDirection)?;
            serializer
                .serialize_field("useStretchDirection", &self.m_useStretchDirection)?;
            serializer.serialize_field("useMeshTopology", &self.m_useMeshTopology)?;
            serializer.serialize_field("allowDynamicLinks", &self.m_allowDynamicLinks)?;
            serializer
                .serialize_field(
                    "useTopologicalStretchDistance",
                    &self.m_useTopologicalStretchDistance,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hclStretchLinkSetupObject<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_name,
                m_simulationMesh,
                m_movableParticlesSelection,
                m_fixedParticlesSelection,
                m_rigidFactor,
                m_stiffness,
                m_stretchDirection,
                m_useStretchDirection,
                m_useMeshTopology,
                m_allowDynamicLinks,
                m_useTopologicalStretchDistance,
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
                        "movableParticlesSelection" => {
                            Ok(__Field::m_movableParticlesSelection)
                        }
                        "fixedParticlesSelection" => {
                            Ok(__Field::m_fixedParticlesSelection)
                        }
                        "rigidFactor" => Ok(__Field::m_rigidFactor),
                        "stiffness" => Ok(__Field::m_stiffness),
                        "stretchDirection" => Ok(__Field::m_stretchDirection),
                        "useStretchDirection" => Ok(__Field::m_useStretchDirection),
                        "useMeshTopology" => Ok(__Field::m_useMeshTopology),
                        "allowDynamicLinks" => Ok(__Field::m_allowDynamicLinks),
                        "useTopologicalStretchDistance" => {
                            Ok(__Field::m_useTopologicalStretchDistance)
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
            struct __hclStretchLinkSetupObjectVisitor<'de> {
                marker: _serde::__private::PhantomData<hclStretchLinkSetupObject<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hclStretchLinkSetupObjectVisitor<'de> {
                type Value = hclStretchLinkSetupObject<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclStretchLinkSetupObject",
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
                    let mut m_movableParticlesSelection: _serde::__private::Option<
                        hclVertexSelectionInput<'de>,
                    > = _serde::__private::None;
                    let mut m_fixedParticlesSelection: _serde::__private::Option<
                        hclVertexSelectionInput<'de>,
                    > = _serde::__private::None;
                    let mut m_rigidFactor: _serde::__private::Option<
                        hclVertexFloatInput<'de>,
                    > = _serde::__private::None;
                    let mut m_stiffness: _serde::__private::Option<
                        hclVertexFloatInput<'de>,
                    > = _serde::__private::None;
                    let mut m_stretchDirection: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_useStretchDirection: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_useMeshTopology: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_allowDynamicLinks: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_useTopologicalStretchDistance: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    for i in 0..11usize {
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
                                if _serde::__private::Option::is_some(
                                    &m_movableParticlesSelection,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "movableParticlesSelection",
                                        ),
                                    );
                                }
                                m_movableParticlesSelection = _serde::__private::Some(
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
                                if _serde::__private::Option::is_some(
                                    &m_fixedParticlesSelection,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fixedParticlesSelection",
                                        ),
                                    );
                                }
                                m_fixedParticlesSelection = _serde::__private::Some(
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
                            4usize => {
                                if _serde::__private::Option::is_some(&m_rigidFactor) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rigidFactor",
                                        ),
                                    );
                                }
                                m_rigidFactor = _serde::__private::Some(
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
                            5usize => {
                                if _serde::__private::Option::is_some(&m_stiffness) {
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
                            6usize => {
                                if _serde::__private::Option::is_some(&m_stretchDirection) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stretchDirection",
                                        ),
                                    );
                                }
                                m_stretchDirection = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_useStretchDirection,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useStretchDirection",
                                        ),
                                    );
                                }
                                m_useStretchDirection = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_useMeshTopology) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useMeshTopology",
                                        ),
                                    );
                                }
                                m_useMeshTopology = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(
                                    &m_allowDynamicLinks,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "allowDynamicLinks",
                                        ),
                                    );
                                }
                                m_allowDynamicLinks = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(
                                    &m_useTopologicalStretchDistance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useTopologicalStretchDistance",
                                        ),
                                    );
                                }
                                m_useTopologicalStretchDistance = _serde::__private::Some(
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
                    __A::pad(&mut __map, 0usize, 12usize)?;
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
                    let m_movableParticlesSelection = match m_movableParticlesSelection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "movableParticlesSelection",
                                ),
                            );
                        }
                    };
                    let m_fixedParticlesSelection = match m_fixedParticlesSelection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fixedParticlesSelection",
                                ),
                            );
                        }
                    };
                    let m_rigidFactor = match m_rigidFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rigidFactor",
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
                    let m_stretchDirection = match m_stretchDirection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stretchDirection",
                                ),
                            );
                        }
                    };
                    let m_useStretchDirection = match m_useStretchDirection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useStretchDirection",
                                ),
                            );
                        }
                    };
                    let m_useMeshTopology = match m_useMeshTopology {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useMeshTopology",
                                ),
                            );
                        }
                    };
                    let m_allowDynamicLinks = match m_allowDynamicLinks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "allowDynamicLinks",
                                ),
                            );
                        }
                    };
                    let m_useTopologicalStretchDistance = match m_useTopologicalStretchDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useTopologicalStretchDistance",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hclStretchLinkSetupObject {
                        __ptr,
                        parent,
                        m_name,
                        m_simulationMesh,
                        m_movableParticlesSelection,
                        m_fixedParticlesSelection,
                        m_rigidFactor,
                        m_stiffness,
                        m_stretchDirection,
                        m_useStretchDirection,
                        m_useMeshTopology,
                        m_allowDynamicLinks,
                        m_useTopologicalStretchDistance,
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
                    let mut m_movableParticlesSelection: _serde::__private::Option<
                        hclVertexSelectionInput<'de>,
                    > = _serde::__private::None;
                    let mut m_fixedParticlesSelection: _serde::__private::Option<
                        hclVertexSelectionInput<'de>,
                    > = _serde::__private::None;
                    let mut m_rigidFactor: _serde::__private::Option<
                        hclVertexFloatInput<'de>,
                    > = _serde::__private::None;
                    let mut m_stiffness: _serde::__private::Option<
                        hclVertexFloatInput<'de>,
                    > = _serde::__private::None;
                    let mut m_stretchDirection: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_useStretchDirection: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_useMeshTopology: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_allowDynamicLinks: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_useTopologicalStretchDistance: _serde::__private::Option<
                        bool,
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
                            __Field::m_movableParticlesSelection => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_movableParticlesSelection,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "movableParticlesSelection",
                                        ),
                                    );
                                }
                                m_movableParticlesSelection = _serde::__private::Some(
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
                            __Field::m_fixedParticlesSelection => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_fixedParticlesSelection,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fixedParticlesSelection",
                                        ),
                                    );
                                }
                                m_fixedParticlesSelection = _serde::__private::Some(
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
                            __Field::m_rigidFactor => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_rigidFactor) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rigidFactor",
                                        ),
                                    );
                                }
                                m_rigidFactor = _serde::__private::Some(
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
                            __Field::m_stretchDirection => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_stretchDirection) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stretchDirection",
                                        ),
                                    );
                                }
                                m_stretchDirection = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_useStretchDirection => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_useStretchDirection,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useStretchDirection",
                                        ),
                                    );
                                }
                                m_useStretchDirection = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_useMeshTopology => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_useMeshTopology) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useMeshTopology",
                                        ),
                                    );
                                }
                                m_useMeshTopology = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_allowDynamicLinks => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_allowDynamicLinks,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "allowDynamicLinks",
                                        ),
                                    );
                                }
                                m_allowDynamicLinks = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_useTopologicalStretchDistance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_useTopologicalStretchDistance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useTopologicalStretchDistance",
                                        ),
                                    );
                                }
                                m_useTopologicalStretchDistance = _serde::__private::Some(
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
                    let m_movableParticlesSelection = match m_movableParticlesSelection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "movableParticlesSelection",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_fixedParticlesSelection = match m_fixedParticlesSelection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fixedParticlesSelection",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_rigidFactor = match m_rigidFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rigidFactor",
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
                    let m_stretchDirection = match m_stretchDirection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stretchDirection",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_useStretchDirection = match m_useStretchDirection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useStretchDirection",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_useMeshTopology = match m_useMeshTopology {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useMeshTopology",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_allowDynamicLinks = match m_allowDynamicLinks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "allowDynamicLinks",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_useTopologicalStretchDistance = match m_useTopologicalStretchDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useTopologicalStretchDistance",
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
                    _serde::__private::Ok(hclStretchLinkSetupObject {
                        __ptr: __ptr.clone(),
                        parent,
                        m_name,
                        m_simulationMesh,
                        m_movableParticlesSelection,
                        m_fixedParticlesSelection,
                        m_rigidFactor,
                        m_stiffness,
                        m_stretchDirection,
                        m_useStretchDirection,
                        m_useMeshTopology,
                        m_allowDynamicLinks,
                        m_useTopologicalStretchDistance,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "name",
                "simulationMesh",
                "movableParticlesSelection",
                "fixedParticlesSelection",
                "rigidFactor",
                "stiffness",
                "stretchDirection",
                "useStretchDirection",
                "useMeshTopology",
                "allowDynamicLinks",
                "useTopologicalStretchDistance",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclStretchLinkSetupObject",
                FIELDS,
                __hclStretchLinkSetupObjectVisitor {
                    marker: _serde::__private::PhantomData::<hclStretchLinkSetupObject>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
