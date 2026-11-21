use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbnpPhysicsInterface`
/// - version: `0`
/// - signature: `0xe96e7679`
/// - size: `  0`(x86)/`160`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbnpPhysicsInterface<'a> {
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
    pub parent: hkbPhysicsInterface<'a>,
    /// # C++ Info
    /// - name: `world`(ctype: `void*`)
    /// - offset: `  0`(x86)/` 24`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "world"))]
    #[cfg_attr(feature = "serde", serde(rename = "world"))]
    pub m_world: Pointer<'a>,
    /// # C++ Info
    /// - name: `taskQueue`(ctype: `void*`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "taskQueue"))]
    #[cfg_attr(feature = "serde", serde(rename = "taskQueue"))]
    pub m_taskQueue: Pointer<'a>,
    /// # C++ Info
    /// - name: `threadPool`(ctype: `void*`)
    /// - offset: `  0`(x86)/` 40`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "threadPool"))]
    #[cfg_attr(feature = "serde", serde(rename = "threadPool"))]
    pub m_threadPool: Pointer<'a>,
    /// # C++ Info
    /// - name: `stepLocalAllocator`(ctype: `void*`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "stepLocalAllocator"))]
    #[cfg_attr(feature = "serde", serde(rename = "stepLocalAllocator"))]
    pub m_stepLocalAllocator: Pointer<'a>,
    /// # C++ Info
    /// - name: `multiThreadCheck`(ctype: `struct hkMultiThreadCheck`)
    /// - offset: `  0`(x86)/` 56`(x86_64)
    /// - type_size: `  0`(x86)/` 12`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "multiThreadCheck"))]
    #[cfg_attr(feature = "serde", serde(rename = "multiThreadCheck"))]
    pub m_multiThreadCheck: hkMultiThreadCheck<'a>,
    /// # C++ Info
    /// - name: `criticalSection`(ctype: `void*`)
    /// - offset: `  0`(x86)/` 72`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "criticalSection"))]
    #[cfg_attr(feature = "serde", serde(rename = "criticalSection"))]
    pub m_criticalSection: Pointer<'a>,
    /// # C++ Info
    /// - name: `newRbHandles`(ctype: `hkArray<void>`)
    /// - offset: `  0`(x86)/` 80`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "newRbHandles"))]
    #[cfg_attr(feature = "serde", serde(rename = "newRbHandles"))]
    pub m_newRbHandles: Vec<()>,
    /// # C++ Info
    /// - name: `newConstraintHandles`(ctype: `hkArray<void>`)
    /// - offset: `  0`(x86)/` 96`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "newConstraintHandles"))]
    #[cfg_attr(feature = "serde", serde(rename = "newConstraintHandles"))]
    pub m_newConstraintHandles: Vec<()>,
    /// # C++ Info
    /// - name: `referencedRbHandles`(ctype: `hkArray<void>`)
    /// - offset: `  0`(x86)/`112`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "referencedRbHandles"))]
    #[cfg_attr(feature = "serde", serde(rename = "referencedRbHandles"))]
    pub m_referencedRbHandles: Vec<()>,
    /// # C++ Info
    /// - name: `referencedConstraintHandles`(ctype: `hkArray<void>`)
    /// - offset: `  0`(x86)/`128`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "referencedConstraintHandles")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "referencedConstraintHandles"))]
    pub m_referencedConstraintHandles: Vec<()>,
    /// # C++ Info
    /// - name: `referencedBodiesLocalFrames`(ctype: `hkArray<void*>`)
    /// - offset: `  0`(x86)/`144`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "referencedBodiesLocalFrames")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "referencedBodiesLocalFrames"))]
    pub m_referencedBodiesLocalFrames: Vec<Pointer<'a>>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbnpPhysicsInterface<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbnpPhysicsInterface"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xe96e7679)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.m_world);
            v.push(&self.m_taskQueue);
            v.push(&self.m_threadPool);
            v.push(&self.m_stepLocalAllocator);
            v.extend(self.m_multiThreadCheck.deps_indexes());
            v.push(&self.m_criticalSection);
            v.extend(self.m_referencedBodiesLocalFrames.iter());
            v
        }
    }
    impl<'a> _serde::Serialize for hkbnpPhysicsInterface<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xe96e7679)));
            let mut serializer = __serializer
                .serialize_struct("hkbnpPhysicsInterface", class_meta, (0u64, 160u64))?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndRefCount",
                    &self.parent.parent.m_memSizeAndRefCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("world", &self.m_world)?;
            serializer.skip_field("taskQueue", &self.m_taskQueue)?;
            serializer.skip_field("threadPool", &self.m_threadPool)?;
            serializer.skip_field("stepLocalAllocator", &self.m_stepLocalAllocator)?;
            serializer.skip_field("multiThreadCheck", &self.m_multiThreadCheck)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("criticalSection", &self.m_criticalSection)?;
            serializer
                .skip_array_field(
                    "newRbHandles",
                    &self.m_newRbHandles,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_array_field(
                    "newConstraintHandles",
                    &self.m_newConstraintHandles,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_array_field(
                    "referencedRbHandles",
                    &self.m_referencedRbHandles,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_array_field(
                    "referencedConstraintHandles",
                    &self.m_referencedConstraintHandles,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_array_field(
                    "referencedBodiesLocalFrames",
                    &self.m_referencedBodiesLocalFrames,
                    TypeSize::NonPtr,
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
    impl<'de> _serde::Deserialize<'de> for hkbnpPhysicsInterface<'de> {
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
            struct __hkbnpPhysicsInterfaceVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbnpPhysicsInterface<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkbnpPhysicsInterfaceVisitor<'de> {
                type Value = hkbnpPhysicsInterface<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbnpPhysicsInterface",
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
                    let mut m_world: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_taskQueue: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_threadPool: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_stepLocalAllocator: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_multiThreadCheck: _serde::__private::Option<
                        hkMultiThreadCheck<'de>,
                    > = _serde::__private::None;
                    let mut m_criticalSection: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_newRbHandles: _serde::__private::Option<Vec<()>> = _serde::__private::None;
                    let mut m_newConstraintHandles: _serde::__private::Option<Vec<()>> = _serde::__private::None;
                    let mut m_referencedRbHandles: _serde::__private::Option<Vec<()>> = _serde::__private::None;
                    let mut m_referencedConstraintHandles: _serde::__private::Option<
                        Vec<()>,
                    > = _serde::__private::None;
                    let mut m_referencedBodiesLocalFrames: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    for i in 0..11usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_world) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("world"),
                                    );
                                }
                                m_world = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_taskQueue) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "taskQueue",
                                        ),
                                    );
                                }
                                m_taskQueue = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_threadPool) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "threadPool",
                                        ),
                                    );
                                }
                                m_threadPool = _serde::__private::Some(
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
                                    &m_stepLocalAllocator,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stepLocalAllocator",
                                        ),
                                    );
                                }
                                m_stepLocalAllocator = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_multiThreadCheck) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "multiThreadCheck",
                                        ),
                                    );
                                }
                                m_multiThreadCheck = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkMultiThreadCheck<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_criticalSection) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "criticalSection",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_criticalSection = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_newRbHandles) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "newRbHandles",
                                        ),
                                    );
                                }
                                m_newRbHandles = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_newConstraintHandles,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "newConstraintHandles",
                                        ),
                                    );
                                }
                                m_newConstraintHandles = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_referencedRbHandles,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "referencedRbHandles",
                                        ),
                                    );
                                }
                                m_referencedRbHandles = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(
                                    &m_referencedConstraintHandles,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "referencedConstraintHandles",
                                        ),
                                    );
                                }
                                m_referencedConstraintHandles = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(
                                    &m_referencedBodiesLocalFrames,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "referencedBodiesLocalFrames",
                                        ),
                                    );
                                }
                                m_referencedBodiesLocalFrames = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
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
                    let m_world = match m_world {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("world"),
                            );
                        }
                    };
                    let m_taskQueue = match m_taskQueue {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "taskQueue",
                                ),
                            );
                        }
                    };
                    let m_threadPool = match m_threadPool {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "threadPool",
                                ),
                            );
                        }
                    };
                    let m_stepLocalAllocator = match m_stepLocalAllocator {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stepLocalAllocator",
                                ),
                            );
                        }
                    };
                    let m_multiThreadCheck = match m_multiThreadCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "multiThreadCheck",
                                ),
                            );
                        }
                    };
                    let m_criticalSection = match m_criticalSection {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "criticalSection",
                                ),
                            );
                        }
                    };
                    let m_newRbHandles = match m_newRbHandles {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "newRbHandles",
                                ),
                            );
                        }
                    };
                    let m_newConstraintHandles = match m_newConstraintHandles {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "newConstraintHandles",
                                ),
                            );
                        }
                    };
                    let m_referencedRbHandles = match m_referencedRbHandles {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "referencedRbHandles",
                                ),
                            );
                        }
                    };
                    let m_referencedConstraintHandles = match m_referencedConstraintHandles {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "referencedConstraintHandles",
                                ),
                            );
                        }
                    };
                    let m_referencedBodiesLocalFrames = match m_referencedBodiesLocalFrames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "referencedBodiesLocalFrames",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbnpPhysicsInterface {
                        __ptr,
                        parent,
                        m_world,
                        m_taskQueue,
                        m_threadPool,
                        m_stepLocalAllocator,
                        m_multiThreadCheck,
                        m_criticalSection,
                        m_newRbHandles,
                        m_newConstraintHandles,
                        m_referencedRbHandles,
                        m_referencedConstraintHandles,
                        m_referencedBodiesLocalFrames,
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
                    let parent = hkbPhysicsInterface {
                        __ptr: __ptr.clone(),
                        parent,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbnpPhysicsInterface {
                        __ptr: __ptr.clone(),
                        parent,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "world",
                "taskQueue",
                "threadPool",
                "stepLocalAllocator",
                "multiThreadCheck",
                "criticalSection",
                "newRbHandles",
                "newConstraintHandles",
                "referencedRbHandles",
                "referencedConstraintHandles",
                "referencedBodiesLocalFrames",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbnpPhysicsInterface",
                FIELDS,
                __hkbnpPhysicsInterfaceVisitor {
                    marker: _serde::__private::PhantomData::<hkbnpPhysicsInterface>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
