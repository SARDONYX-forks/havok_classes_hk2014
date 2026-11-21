use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkxMeshSection`
/// - version: `5`
/// - signature: `0x54533691`
/// - size: `  0`(x86)/`112`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxMeshSection<'a> {
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
    /// - name: `vertexBuffer`(ctype: `struct hkxVertexBuffer*`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "vertexBuffer"))]
    #[cfg_attr(feature = "serde", serde(rename = "vertexBuffer"))]
    pub m_vertexBuffer: Pointer<'a>,
    /// # C++ Info
    /// - name: `indexBuffers`(ctype: `hkArray<hkxIndexBuffer*>`)
    /// - offset: `  0`(x86)/` 24`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "indexBuffers"))]
    #[cfg_attr(feature = "serde", serde(rename = "indexBuffers"))]
    pub m_indexBuffers: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `material`(ctype: `struct hkxMaterial*`)
    /// - offset: `  0`(x86)/` 40`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "material"))]
    #[cfg_attr(feature = "serde", serde(rename = "material"))]
    pub m_material: Pointer<'a>,
    /// # C++ Info
    /// - name: `userChannels`(ctype: `hkArray<hkReferencedObject*>`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "userChannels"))]
    #[cfg_attr(feature = "serde", serde(rename = "userChannels"))]
    pub m_userChannels: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `vertexAnimations`(ctype: `hkArray<hkxVertexAnimation*>`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "vertexAnimations"))]
    #[cfg_attr(feature = "serde", serde(rename = "vertexAnimations"))]
    pub m_vertexAnimations: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `linearKeyFrameHints`(ctype: `hkArray<hkReal>`)
    /// - offset: `  0`(x86)/` 80`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "linearKeyFrameHints"))]
    #[cfg_attr(feature = "serde", serde(rename = "linearKeyFrameHints"))]
    pub m_linearKeyFrameHints: Vec<f32>,
    /// # C++ Info
    /// - name: `boneMatrixMap`(ctype: `hkArray<struct hkMeshBoneIndexMapping>`)
    /// - offset: `  0`(x86)/` 96`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "boneMatrixMap"))]
    #[cfg_attr(feature = "serde", serde(rename = "boneMatrixMap"))]
    pub m_boneMatrixMap: Vec<hkMeshBoneIndexMapping<'a>>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkxMeshSection<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxMeshSection"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x54533691)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.m_vertexBuffer);
            v.extend(self.m_indexBuffers.iter());
            v.push(&self.m_material);
            v.extend(self.m_userChannels.iter());
            v.extend(self.m_vertexAnimations.iter());
            v.extend(
                self
                    .m_boneMatrixMap
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v
        }
    }
    impl<'a> _serde::Serialize for hkxMeshSection<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x54533691)));
            let mut serializer = __serializer
                .serialize_struct("hkxMeshSection", class_meta, (0u64, 112u64))?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndRefCount", &self.parent.m_memSizeAndRefCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("vertexBuffer", &self.m_vertexBuffer)?;
            serializer
                .serialize_array_field(
                    "indexBuffers",
                    &self.m_indexBuffers,
                    TypeSize::NonPtr,
                )?;
            serializer.serialize_field("material", &self.m_material)?;
            serializer
                .serialize_array_field(
                    "userChannels",
                    &self.m_userChannels,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "vertexAnimations",
                    &self.m_vertexAnimations,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "linearKeyFrameHints",
                    &self.m_linearKeyFrameHints,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "boneMatrixMap",
                    &self.m_boneMatrixMap,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 16u64,
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
    impl<'de> _serde::Deserialize<'de> for hkxMeshSection<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_vertexBuffer,
                m_indexBuffers,
                m_material,
                m_userChannels,
                m_vertexAnimations,
                m_linearKeyFrameHints,
                m_boneMatrixMap,
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
                        "vertexBuffer" => Ok(__Field::m_vertexBuffer),
                        "indexBuffers" => Ok(__Field::m_indexBuffers),
                        "material" => Ok(__Field::m_material),
                        "userChannels" => Ok(__Field::m_userChannels),
                        "vertexAnimations" => Ok(__Field::m_vertexAnimations),
                        "linearKeyFrameHints" => Ok(__Field::m_linearKeyFrameHints),
                        "boneMatrixMap" => Ok(__Field::m_boneMatrixMap),
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
            struct __hkxMeshSectionVisitor<'de> {
                marker: _serde::__private::PhantomData<hkxMeshSection<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkxMeshSectionVisitor<'de> {
                type Value = hkxMeshSection<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hkxMeshSection")
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
                    let mut m_vertexBuffer: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_indexBuffers: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_material: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_userChannels: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_vertexAnimations: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_linearKeyFrameHints: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_boneMatrixMap: _serde::__private::Option<
                        Vec<hkMeshBoneIndexMapping<'de>>,
                    > = _serde::__private::None;
                    for i in 0..7usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_vertexBuffer) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertexBuffer",
                                        ),
                                    );
                                }
                                m_vertexBuffer = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_indexBuffers) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "indexBuffers",
                                        ),
                                    );
                                }
                                m_indexBuffers = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_material) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "material",
                                        ),
                                    );
                                }
                                m_material = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_userChannels) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userChannels",
                                        ),
                                    );
                                }
                                m_userChannels = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_vertexAnimations) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertexAnimations",
                                        ),
                                    );
                                }
                                m_vertexAnimations = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_linearKeyFrameHints,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "linearKeyFrameHints",
                                        ),
                                    );
                                }
                                m_linearKeyFrameHints = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_boneMatrixMap) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneMatrixMap",
                                        ),
                                    );
                                }
                                m_boneMatrixMap = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkMeshBoneIndexMapping<'de>>,
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
                    let m_vertexBuffer = match m_vertexBuffer {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vertexBuffer",
                                ),
                            );
                        }
                    };
                    let m_indexBuffers = match m_indexBuffers {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "indexBuffers",
                                ),
                            );
                        }
                    };
                    let m_material = match m_material {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("material"),
                            );
                        }
                    };
                    let m_userChannels = match m_userChannels {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "userChannels",
                                ),
                            );
                        }
                    };
                    let m_vertexAnimations = match m_vertexAnimations {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vertexAnimations",
                                ),
                            );
                        }
                    };
                    let m_linearKeyFrameHints = match m_linearKeyFrameHints {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "linearKeyFrameHints",
                                ),
                            );
                        }
                    };
                    let m_boneMatrixMap = match m_boneMatrixMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneMatrixMap",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkxMeshSection {
                        __ptr,
                        parent,
                        m_vertexBuffer,
                        m_indexBuffers,
                        m_material,
                        m_userChannels,
                        m_vertexAnimations,
                        m_linearKeyFrameHints,
                        m_boneMatrixMap,
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
                    let mut m_vertexBuffer: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_indexBuffers: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_material: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_userChannels: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_vertexAnimations: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_linearKeyFrameHints: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_boneMatrixMap: _serde::__private::Option<
                        Vec<hkMeshBoneIndexMapping<'de>>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_vertexBuffer => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_vertexBuffer) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertexBuffer",
                                        ),
                                    );
                                }
                                m_vertexBuffer = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_indexBuffers => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_indexBuffers) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "indexBuffers",
                                        ),
                                    );
                                }
                                m_indexBuffers = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_material => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_material) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "material",
                                        ),
                                    );
                                }
                                m_material = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_userChannels => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_userChannels) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userChannels",
                                        ),
                                    );
                                }
                                m_userChannels = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_vertexAnimations => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_vertexAnimations) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertexAnimations",
                                        ),
                                    );
                                }
                                m_vertexAnimations = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_linearKeyFrameHints => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_linearKeyFrameHints,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "linearKeyFrameHints",
                                        ),
                                    );
                                }
                                m_linearKeyFrameHints = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_boneMatrixMap => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_boneMatrixMap) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneMatrixMap",
                                        ),
                                    );
                                }
                                m_boneMatrixMap = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkMeshBoneIndexMapping<'de>>,
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
                    let m_vertexBuffer = match m_vertexBuffer {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vertexBuffer",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_indexBuffers = match m_indexBuffers {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "indexBuffers",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_material = match m_material {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("material"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_userChannels = match m_userChannels {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "userChannels",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_vertexAnimations = match m_vertexAnimations {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vertexAnimations",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_linearKeyFrameHints = match m_linearKeyFrameHints {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "linearKeyFrameHints",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_boneMatrixMap = match m_boneMatrixMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneMatrixMap",
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
                    _serde::__private::Ok(hkxMeshSection {
                        __ptr: __ptr.clone(),
                        parent,
                        m_vertexBuffer,
                        m_indexBuffers,
                        m_material,
                        m_userChannels,
                        m_vertexAnimations,
                        m_linearKeyFrameHints,
                        m_boneMatrixMap,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "vertexBuffer",
                "indexBuffers",
                "material",
                "userChannels",
                "vertexAnimations",
                "linearKeyFrameHints",
                "boneMatrixMap",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkxMeshSection",
                FIELDS,
                __hkxMeshSectionVisitor {
                    marker: _serde::__private::PhantomData::<hkxMeshSection>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
