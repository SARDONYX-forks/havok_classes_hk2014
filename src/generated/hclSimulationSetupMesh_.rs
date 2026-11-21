use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclSimulationSetupMesh`
/// - version: `1`
/// - signature: `0xefda0ea4`
/// - size: `  0`(x86)/`208`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclSimulationSetupMesh<'a> {
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
    pub parent: hclSetupMesh<'a>,
    /// # C++ Info
    /// - name: `originalMesh`(ctype: `hclSetupMesh`)
    /// - offset: `  0`(x86)/` 24`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "originalMesh"))]
    #[cfg_attr(feature = "serde", serde(rename = "originalMesh"))]
    pub m_originalMesh: Pointer<'a>,
    /// # C++ Info
    /// - name: `mergeOptions`(ctype: `hclSimulationSetupMeshMapOptions`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/` 24`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "mergeOptions"))]
    #[cfg_attr(feature = "serde", serde(rename = "mergeOptions"))]
    pub m_mergeOptions: hclSimulationSetupMeshMapOptions<'a>,
    /// # C++ Info
    /// - name: `mergedMeshMap`(ctype: `TYPE_POINTER`)
    /// - offset: `  0`(x86)/` 56`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "mergedMeshMap"))]
    #[cfg_attr(feature = "serde", serde(rename = "mergedMeshMap"))]
    pub m_mergedMeshMap: Pointer<'a>,
    /// # C++ Info
    /// - name: `worldFromOriginalMesh`(ctype: `TYPE_MATRIX4`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/` 64`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "worldFromOriginalMesh"))]
    #[cfg_attr(feature = "serde", serde(rename = "worldFromOriginalMesh"))]
    pub m_worldFromOriginalMesh: Matrix4,
    /// # C++ Info
    /// - name: `worldFromOriginalMeshInvTranspose`(ctype: `TYPE_MATRIX4`)
    /// - offset: `  0`(x86)/`128`(x86_64)
    /// - type_size: `  0`(x86)/` 64`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "worldFromOriginalMeshInvTranspose")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "worldFromOriginalMeshInvTranspose"))]
    pub m_worldFromOriginalMeshInvTranspose: Matrix4,
    /// # C++ Info
    /// - name: `originalMeshSections`(ctype: `TYPE_POINTER`)
    /// - offset: `  0`(x86)/`192`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "originalMeshSections"))]
    #[cfg_attr(feature = "serde", serde(rename = "originalMeshSections"))]
    pub m_originalMeshSections: Pointer<'a>,
    /// # C++ Info
    /// - name: `haveWorldTransforms`(ctype: `TYPE_BOOL`)
    /// - offset: `  0`(x86)/`200`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "haveWorldTransforms"))]
    #[cfg_attr(feature = "serde", serde(rename = "haveWorldTransforms"))]
    pub m_haveWorldTransforms: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclSimulationSetupMesh<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclSimulationSetupMesh"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xefda0ea4)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.m_originalMesh);
            v.extend(self.m_mergeOptions.deps_indexes());
            v.push(&self.m_mergedMeshMap);
            v.push(&self.m_originalMeshSections);
            v
        }
    }
    impl<'a> _serde::Serialize for hclSimulationSetupMesh<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xefda0ea4)));
            let mut serializer = __serializer
                .serialize_struct("hclSimulationSetupMesh", class_meta, (0u64, 208u64))?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndRefCount",
                    &self.parent.parent.m_memSizeAndRefCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("originalMesh", &self.m_originalMesh)?;
            serializer.serialize_field("mergeOptions", &self.m_mergeOptions)?;
            serializer.skip_field("mergedMeshMap", &self.m_mergedMeshMap)?;
            serializer
                .skip_field("worldFromOriginalMesh", &self.m_worldFromOriginalMesh)?;
            serializer
                .skip_field(
                    "worldFromOriginalMeshInvTranspose",
                    &self.m_worldFromOriginalMeshInvTranspose,
                )?;
            serializer.skip_field("originalMeshSections", &self.m_originalMeshSections)?;
            serializer.skip_field("haveWorldTransforms", &self.m_haveWorldTransforms)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hclSimulationSetupMesh<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_originalMesh,
                m_mergeOptions,
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
                        "originalMesh" => Ok(__Field::m_originalMesh),
                        "mergeOptions" => Ok(__Field::m_mergeOptions),
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
            struct __hclSimulationSetupMeshVisitor<'de> {
                marker: _serde::__private::PhantomData<hclSimulationSetupMesh<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hclSimulationSetupMeshVisitor<'de> {
                type Value = hclSimulationSetupMesh<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclSimulationSetupMesh",
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
                    let mut m_originalMesh: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_mergeOptions: _serde::__private::Option<
                        hclSimulationSetupMeshMapOptions<'de>,
                    > = _serde::__private::None;
                    let mut m_mergedMeshMap: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_worldFromOriginalMesh: _serde::__private::Option<
                        Matrix4,
                    > = _serde::__private::None;
                    let mut m_worldFromOriginalMeshInvTranspose: _serde::__private::Option<
                        Matrix4,
                    > = _serde::__private::None;
                    let mut m_originalMeshSections: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_haveWorldTransforms: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..7usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_originalMesh) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "originalMesh",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 8usize)?;
                                m_originalMesh = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_mergeOptions) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "mergeOptions",
                                        ),
                                    );
                                }
                                m_mergeOptions = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclSimulationSetupMeshMapOptions<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_mergedMeshMap) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "mergedMeshMap",
                                        ),
                                    );
                                }
                                m_mergedMeshMap = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_worldFromOriginalMesh,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "worldFromOriginalMesh",
                                        ),
                                    );
                                }
                                m_worldFromOriginalMesh = _serde::__private::Some(
                                    match __A::next_value::<Matrix4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_worldFromOriginalMeshInvTranspose,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "worldFromOriginalMeshInvTranspose",
                                        ),
                                    );
                                }
                                m_worldFromOriginalMeshInvTranspose = _serde::__private::Some(
                                    match __A::next_value::<Matrix4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_originalMeshSections,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "originalMeshSections",
                                        ),
                                    );
                                }
                                m_originalMeshSections = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_haveWorldTransforms,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "haveWorldTransforms",
                                        ),
                                    );
                                }
                                m_haveWorldTransforms = _serde::__private::Some(
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
                    __A::pad(&mut __map, 0usize, 7usize)?;
                    let m_originalMesh = match m_originalMesh {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "originalMesh",
                                ),
                            );
                        }
                    };
                    let m_mergeOptions = match m_mergeOptions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "mergeOptions",
                                ),
                            );
                        }
                    };
                    let m_mergedMeshMap = match m_mergedMeshMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "mergedMeshMap",
                                ),
                            );
                        }
                    };
                    let m_worldFromOriginalMesh = match m_worldFromOriginalMesh {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "worldFromOriginalMesh",
                                ),
                            );
                        }
                    };
                    let m_worldFromOriginalMeshInvTranspose = match m_worldFromOriginalMeshInvTranspose {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "worldFromOriginalMeshInvTranspose",
                                ),
                            );
                        }
                    };
                    let m_originalMeshSections = match m_originalMeshSections {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "originalMeshSections",
                                ),
                            );
                        }
                    };
                    let m_haveWorldTransforms = match m_haveWorldTransforms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "haveWorldTransforms",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hclSimulationSetupMesh {
                        __ptr,
                        parent,
                        m_originalMesh,
                        m_mergeOptions,
                        m_mergedMeshMap,
                        m_worldFromOriginalMesh,
                        m_worldFromOriginalMeshInvTranspose,
                        m_originalMeshSections,
                        m_haveWorldTransforms,
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
                    let mut m_originalMesh: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_mergeOptions: _serde::__private::Option<
                        hclSimulationSetupMeshMapOptions<'de>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_originalMesh => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_originalMesh) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "originalMesh",
                                        ),
                                    );
                                }
                                m_originalMesh = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_mergeOptions => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_mergeOptions) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "mergeOptions",
                                        ),
                                    );
                                }
                                m_mergeOptions = _serde::__private::Some(
                                    match __A::next_value::<
                                        hclSimulationSetupMeshMapOptions<'de>,
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
                    let m_originalMesh = match m_originalMesh {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "originalMesh",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_mergeOptions = match m_mergeOptions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "mergeOptions",
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
                    let parent = hclSetupMesh {
                        __ptr: __ptr.clone(),
                        parent,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hclSimulationSetupMesh {
                        __ptr: __ptr.clone(),
                        parent,
                        m_originalMesh,
                        m_mergeOptions,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "originalMesh",
                "mergeOptions",
                "mergedMeshMap",
                "worldFromOriginalMesh",
                "worldFromOriginalMeshInvTranspose",
                "originalMeshSections",
                "haveWorldTransforms",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclSimulationSetupMesh",
                FIELDS,
                __hclSimulationSetupMeshVisitor {
                    marker: _serde::__private::PhantomData::<hclSimulationSetupMesh>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
