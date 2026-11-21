use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hknpPhysicsSystemData`
/// - version: `0`
/// - signature: `0xb857718b`
/// - size: `  0`(x86)/`120`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hknpPhysicsSystemData<'a> {
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
    /// - name: `materials`(ctype: `hkArray<struct hknpMaterial>`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "materials"))]
    #[cfg_attr(feature = "serde", serde(rename = "materials"))]
    pub m_materials: Vec<hknpMaterial<'a>>,
    /// # C++ Info
    /// - name: `motionProperties`(ctype: `hkArray<struct hknpMotionProperties>`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "motionProperties"))]
    #[cfg_attr(feature = "serde", serde(rename = "motionProperties"))]
    pub m_motionProperties: Vec<hknpMotionProperties<'a>>,
    /// # C++ Info
    /// - name: `motionCinfos`(ctype: `hkArray<struct hknpMotionCinfo>`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "motionCinfos"))]
    #[cfg_attr(feature = "serde", serde(rename = "motionCinfos"))]
    pub m_motionCinfos: Vec<hknpMotionCinfo<'a>>,
    /// # C++ Info
    /// - name: `bodyCinfos`(ctype: `hkArray<struct hknpBodyCinfo>`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "bodyCinfos"))]
    #[cfg_attr(feature = "serde", serde(rename = "bodyCinfos"))]
    pub m_bodyCinfos: Vec<hknpBodyCinfo<'a>>,
    /// # C++ Info
    /// - name: `constraintCinfos`(ctype: `hkArray<struct hknpConstraintCinfo>`)
    /// - offset: `  0`(x86)/` 80`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "constraintCinfos"))]
    #[cfg_attr(feature = "serde", serde(rename = "constraintCinfos"))]
    pub m_constraintCinfos: Vec<hknpConstraintCinfo<'a>>,
    /// # C++ Info
    /// - name: `referencedObjects`(ctype: `hkArray<hkReferencedObject*>`)
    /// - offset: `  0`(x86)/` 96`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "referencedObjects"))]
    #[cfg_attr(feature = "serde", serde(rename = "referencedObjects"))]
    pub m_referencedObjects: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `name`(ctype: `hkStringPtr`)
    /// - offset: `  0`(x86)/`112`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    pub m_name: StringPtr<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hknpPhysicsSystemData<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hknpPhysicsSystemData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xb857718b)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(
                self
                    .m_materials
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(
                self
                    .m_motionProperties
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(
                self
                    .m_motionCinfos
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(
                self
                    .m_bodyCinfos
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(
                self
                    .m_constraintCinfos
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(self.m_referencedObjects.iter());
            v
        }
    }
    impl<'a> _serde::Serialize for hknpPhysicsSystemData<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xb857718b)));
            let mut serializer = __serializer
                .serialize_struct("hknpPhysicsSystemData", class_meta, (0u64, 120u64))?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndRefCount", &self.parent.m_memSizeAndRefCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "materials",
                    &self.m_materials,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 80u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "motionProperties",
                    &self.m_motionProperties,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 64u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "motionCinfos",
                    &self.m_motionCinfos,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 112u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "bodyCinfos",
                    &self.m_bodyCinfos,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 96u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "constraintCinfos",
                    &self.m_constraintCinfos,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 24u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "referencedObjects",
                    &self.m_referencedObjects,
                    TypeSize::NonPtr,
                )?;
            serializer.serialize_field("name", &self.m_name)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hknpPhysicsSystemData<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_materials,
                m_motionProperties,
                m_motionCinfos,
                m_bodyCinfos,
                m_constraintCinfos,
                m_referencedObjects,
                m_name,
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
                        "materials" => Ok(__Field::m_materials),
                        "motionProperties" => Ok(__Field::m_motionProperties),
                        "motionCinfos" => Ok(__Field::m_motionCinfos),
                        "bodyCinfos" => Ok(__Field::m_bodyCinfos),
                        "constraintCinfos" => Ok(__Field::m_constraintCinfos),
                        "referencedObjects" => Ok(__Field::m_referencedObjects),
                        "name" => Ok(__Field::m_name),
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
            struct __hknpPhysicsSystemDataVisitor<'de> {
                marker: _serde::__private::PhantomData<hknpPhysicsSystemData<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hknpPhysicsSystemDataVisitor<'de> {
                type Value = hknpPhysicsSystemData<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hknpPhysicsSystemData",
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
                    let mut m_materials: _serde::__private::Option<
                        Vec<hknpMaterial<'de>>,
                    > = _serde::__private::None;
                    let mut m_motionProperties: _serde::__private::Option<
                        Vec<hknpMotionProperties<'de>>,
                    > = _serde::__private::None;
                    let mut m_motionCinfos: _serde::__private::Option<
                        Vec<hknpMotionCinfo<'de>>,
                    > = _serde::__private::None;
                    let mut m_bodyCinfos: _serde::__private::Option<
                        Vec<hknpBodyCinfo<'de>>,
                    > = _serde::__private::None;
                    let mut m_constraintCinfos: _serde::__private::Option<
                        Vec<hknpConstraintCinfo<'de>>,
                    > = _serde::__private::None;
                    let mut m_referencedObjects: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    for i in 0..7usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_materials) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "materials",
                                        ),
                                    );
                                }
                                m_materials = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hknpMaterial<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_motionProperties) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "motionProperties",
                                        ),
                                    );
                                }
                                m_motionProperties = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hknpMotionProperties<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_motionCinfos) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "motionCinfos",
                                        ),
                                    );
                                }
                                m_motionCinfos = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hknpMotionCinfo<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_bodyCinfos) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bodyCinfos",
                                        ),
                                    );
                                }
                                m_bodyCinfos = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hknpBodyCinfo<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_constraintCinfos) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "constraintCinfos",
                                        ),
                                    );
                                }
                                m_constraintCinfos = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hknpConstraintCinfo<'de>>,
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
                                    &m_referencedObjects,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "referencedObjects",
                                        ),
                                    );
                                }
                                m_referencedObjects = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
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
                            _ => {}
                        }
                    }
                    let m_materials = match m_materials {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "materials",
                                ),
                            );
                        }
                    };
                    let m_motionProperties = match m_motionProperties {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "motionProperties",
                                ),
                            );
                        }
                    };
                    let m_motionCinfos = match m_motionCinfos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "motionCinfos",
                                ),
                            );
                        }
                    };
                    let m_bodyCinfos = match m_bodyCinfos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bodyCinfos",
                                ),
                            );
                        }
                    };
                    let m_constraintCinfos = match m_constraintCinfos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "constraintCinfos",
                                ),
                            );
                        }
                    };
                    let m_referencedObjects = match m_referencedObjects {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "referencedObjects",
                                ),
                            );
                        }
                    };
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                        }
                    };
                    _serde::__private::Ok(hknpPhysicsSystemData {
                        __ptr,
                        parent,
                        m_materials,
                        m_motionProperties,
                        m_motionCinfos,
                        m_bodyCinfos,
                        m_constraintCinfos,
                        m_referencedObjects,
                        m_name,
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
                    let mut m_materials: _serde::__private::Option<
                        Vec<hknpMaterial<'de>>,
                    > = _serde::__private::None;
                    let mut m_motionProperties: _serde::__private::Option<
                        Vec<hknpMotionProperties<'de>>,
                    > = _serde::__private::None;
                    let mut m_motionCinfos: _serde::__private::Option<
                        Vec<hknpMotionCinfo<'de>>,
                    > = _serde::__private::None;
                    let mut m_bodyCinfos: _serde::__private::Option<
                        Vec<hknpBodyCinfo<'de>>,
                    > = _serde::__private::None;
                    let mut m_constraintCinfos: _serde::__private::Option<
                        Vec<hknpConstraintCinfo<'de>>,
                    > = _serde::__private::None;
                    let mut m_referencedObjects: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_materials => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_materials) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "materials",
                                        ),
                                    );
                                }
                                m_materials = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hknpMaterial<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_motionProperties => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_motionProperties) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "motionProperties",
                                        ),
                                    );
                                }
                                m_motionProperties = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hknpMotionProperties<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_motionCinfos => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_motionCinfos) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "motionCinfos",
                                        ),
                                    );
                                }
                                m_motionCinfos = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hknpMotionCinfo<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bodyCinfos => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bodyCinfos) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bodyCinfos",
                                        ),
                                    );
                                }
                                m_bodyCinfos = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hknpBodyCinfo<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_constraintCinfos => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_constraintCinfos) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "constraintCinfos",
                                        ),
                                    );
                                }
                                m_constraintCinfos = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hknpConstraintCinfo<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_referencedObjects => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_referencedObjects,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "referencedObjects",
                                        ),
                                    );
                                }
                                m_referencedObjects = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
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
                            _ => __A::skip_value(&mut __map)?,
                        }
                    }
                    let m_materials = match m_materials {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "materials",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_motionProperties = match m_motionProperties {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "motionProperties",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_motionCinfos = match m_motionCinfos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "motionCinfos",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bodyCinfos = match m_bodyCinfos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bodyCinfos",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_constraintCinfos = match m_constraintCinfos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "constraintCinfos",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_referencedObjects = match m_referencedObjects {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "referencedObjects",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
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
                    _serde::__private::Ok(hknpPhysicsSystemData {
                        __ptr: __ptr.clone(),
                        parent,
                        m_materials,
                        m_motionProperties,
                        m_motionCinfos,
                        m_bodyCinfos,
                        m_constraintCinfos,
                        m_referencedObjects,
                        m_name,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "materials",
                "motionProperties",
                "motionCinfos",
                "bodyCinfos",
                "constraintCinfos",
                "referencedObjects",
                "name",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hknpPhysicsSystemData",
                FIELDS,
                __hknpPhysicsSystemDataVisitor {
                    marker: _serde::__private::PhantomData::<hknpPhysicsSystemData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
