use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbnpRagdollInterface`
/// - version: `0`
/// - signature: `0x2abf53d6`
/// - size: `  0`(x86)/` 96`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbnpRagdollInterface<'a> {
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
    pub parent: hkbRagdollInterface<'a>,
    /// # C++ Info
    /// - name: `physicsInterface`(ctype: `void*`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "physicsInterface"))]
    #[cfg_attr(feature = "serde", serde(rename = "physicsInterface"))]
    pub m_physicsInterface: Pointer<'a>,
    /// # C++ Info
    /// - name: `ragdoll`(ctype: `void*`)
    /// - offset: `  0`(x86)/` 24`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "ragdoll"))]
    #[cfg_attr(feature = "serde", serde(rename = "ragdoll"))]
    pub m_ragdoll: Pointer<'a>,
    /// # C++ Info
    /// - name: `scaledObjects`(ctype: `hkArray<void*>`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "scaledObjects"))]
    #[cfg_attr(feature = "serde", serde(rename = "scaledObjects"))]
    pub m_scaledObjects: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `rigidBodyHandles`(ctype: `hkArray<void*>`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "rigidBodyHandles"))]
    #[cfg_attr(feature = "serde", serde(rename = "rigidBodyHandles"))]
    pub m_rigidBodyHandles: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `addedToWorld`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 88`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "addedToWorld"))]
    #[cfg_attr(feature = "serde", serde(rename = "addedToWorld"))]
    pub m_addedToWorld: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbnpRagdollInterface<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbnpRagdollInterface"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x2abf53d6)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.m_physicsInterface);
            v.push(&self.m_ragdoll);
            v.extend(self.m_scaledObjects.iter());
            v.extend(self.m_rigidBodyHandles.iter());
            v
        }
    }
    impl<'a> _serde::Serialize for hkbnpRagdollInterface<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x2abf53d6)));
            let mut serializer = __serializer
                .serialize_struct("hkbnpRagdollInterface", class_meta, (0u64, 96u64))?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndRefCount",
                    &self.parent.parent.m_memSizeAndRefCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("physicsInterface", &self.m_physicsInterface)?;
            serializer.skip_field("ragdoll", &self.m_ragdoll)?;
            serializer
                .skip_array_field(
                    "scaledObjects",
                    &self.m_scaledObjects,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_array_field(
                    "rigidBodyHandles",
                    &self.m_rigidBodyHandles,
                    TypeSize::NonPtr,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 24usize].as_slice())?;
            serializer.skip_field("addedToWorld", &self.m_addedToWorld)?;
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
    impl<'de> _serde::Deserialize<'de> for hkbnpRagdollInterface<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
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
            struct __hkbnpRagdollInterfaceVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbnpRagdollInterface<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkbnpRagdollInterfaceVisitor<'de> {
                type Value = hkbnpRagdollInterface<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbnpRagdollInterface",
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
                    let mut m_physicsInterface: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_ragdoll: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_scaledObjects: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_rigidBodyHandles: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_addedToWorld: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..5usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_physicsInterface) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "physicsInterface",
                                        ),
                                    );
                                }
                                m_physicsInterface = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_ragdoll) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ragdoll",
                                        ),
                                    );
                                }
                                m_ragdoll = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_scaledObjects) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "scaledObjects",
                                        ),
                                    );
                                }
                                m_scaledObjects = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_rigidBodyHandles) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rigidBodyHandles",
                                        ),
                                    );
                                }
                                m_rigidBodyHandles = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_addedToWorld) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "addedToWorld",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 24usize)?;
                                m_addedToWorld = _serde::__private::Some(
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
                    let m_physicsInterface = match m_physicsInterface {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "physicsInterface",
                                ),
                            );
                        }
                    };
                    let m_ragdoll = match m_ragdoll {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("ragdoll"),
                            );
                        }
                    };
                    let m_scaledObjects = match m_scaledObjects {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "scaledObjects",
                                ),
                            );
                        }
                    };
                    let m_rigidBodyHandles = match m_rigidBodyHandles {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rigidBodyHandles",
                                ),
                            );
                        }
                    };
                    let m_addedToWorld = match m_addedToWorld {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "addedToWorld",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbnpRagdollInterface {
                        __ptr,
                        parent,
                        m_physicsInterface,
                        m_ragdoll,
                        m_scaledObjects,
                        m_rigidBodyHandles,
                        m_addedToWorld,
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
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            _ => __A::skip_value(&mut __map)?,
                        }
                    }
                    let __ptr = None;
                    let parent = hkBaseObject {
                        __ptr: __ptr.clone(),
                    };
                    let parent = hkReferencedObject {
                        __ptr: __ptr.clone(),
                        parent,
                        ..Default::default()
                    };
                    let parent = hkbRagdollInterface {
                        __ptr: __ptr.clone(),
                        parent,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbnpRagdollInterface {
                        __ptr: __ptr.clone(),
                        parent,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "physicsInterface",
                "ragdoll",
                "scaledObjects",
                "rigidBodyHandles",
                "addedToWorld",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbnpRagdollInterface",
                FIELDS,
                __hkbnpRagdollInterfaceVisitor {
                    marker: _serde::__private::PhantomData::<hkbnpRagdollInterface>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
