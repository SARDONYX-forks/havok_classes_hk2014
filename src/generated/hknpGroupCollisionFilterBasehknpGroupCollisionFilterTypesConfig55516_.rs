use super::class_requires::*;
use super::*;
use super::hknpCollisionFilter_::Type;
/// # C++ Info
/// - name: `hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516`
/// - version: `0`
/// - signature: `0x2fef5ac0`
/// - size: `  0`(x86)/`176`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516<'a> {
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
    pub parent: hknpCollisionFilter<'a>,
    /// # C++ Info
    /// - name: `nextFreeSystemGroup`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    /// - flags: `ALIGN_16`
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "nextFreeSystemGroup"))]
    #[cfg_attr(feature = "serde", serde(rename = "nextFreeSystemGroup"))]
    pub m_nextFreeSystemGroup: I32<'a>,
    /// # C++ Info
    /// - name: `collisionLookupTable`(ctype: `hkUint32[32]`)
    /// - offset: `  0`(x86)/` 36`(x86_64)
    /// - type_size: `  0`(x86)/`128`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "collisionLookupTable"))]
    #[cfg_attr(feature = "serde", serde(rename = "collisionLookupTable"))]
    pub m_collisionLookupTable: [U32<'a>; 32usize],
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass
    for hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x2fef5ac0)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize
    for hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x2fef5ac0)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516",
                    class_meta,
                    (0u64, 176u64),
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndRefCount",
                    &self.parent.parent.m_memSizeAndRefCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .serialize_field("nextFreeSystemGroup", &self.m_nextFreeSystemGroup)?;
            serializer
                .serialize_fixed_array_field(
                    "collisionLookupTable",
                    self.m_collisionLookupTable.as_slice(),
                    TypeSize::NonPtr,
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
    impl<'de> _serde::Deserialize<'de>
    for hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_type,
                m_nextFreeSystemGroup,
                m_collisionLookupTable,
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
                        "type" => Ok(__Field::m_type),
                        "nextFreeSystemGroup" => Ok(__Field::m_nextFreeSystemGroup),
                        "collisionLookupTable" => Ok(__Field::m_collisionLookupTable),
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
            struct __hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516Visitor<
                'de,
            > {
                marker: _serde::__private::PhantomData<
                    hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516<
                        'de,
                    >,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516Visitor<
                'de,
            > {
                type Value = hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516<
                    'de,
                >;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516",
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
                    let mut m_nextFreeSystemGroup: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_collisionLookupTable: _serde::__private::Option<
                        [U32<'de>; 32usize],
                    > = _serde::__private::None;
                    for i in 0..2usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_nextFreeSystemGroup,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nextFreeSystemGroup",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 8usize)?;
                                m_nextFreeSystemGroup = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_collisionLookupTable,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collisionLookupTable",
                                        ),
                                    );
                                }
                                m_collisionLookupTable = _serde::__private::Some(
                                    match __A::next_value::<[U32<'de>; 32usize]>(&mut __map) {
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
                    let m_nextFreeSystemGroup = match m_nextFreeSystemGroup {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nextFreeSystemGroup",
                                ),
                            );
                        }
                    };
                    let m_collisionLookupTable = match m_collisionLookupTable {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collisionLookupTable",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516 {
                        __ptr,
                        parent,
                        m_nextFreeSystemGroup,
                        m_collisionLookupTable,
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
                    let mut m_type: _serde::__private::Option<Type> = _serde::__private::None;
                    let mut m_nextFreeSystemGroup: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_collisionLookupTable: _serde::__private::Option<
                        [U32<'de>; 32usize],
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_type => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_type) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                    );
                                }
                                m_type = _serde::__private::Some(
                                    match __A::next_value::<Type>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nextFreeSystemGroup => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nextFreeSystemGroup,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nextFreeSystemGroup",
                                        ),
                                    );
                                }
                                m_nextFreeSystemGroup = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_collisionLookupTable => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_collisionLookupTable,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collisionLookupTable",
                                        ),
                                    );
                                }
                                m_collisionLookupTable = _serde::__private::Some(
                                    match __A::next_value::<[U32<'de>; 32usize]>(&mut __map) {
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
                    let m_type = match m_type {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("type"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nextFreeSystemGroup = match m_nextFreeSystemGroup {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nextFreeSystemGroup",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_collisionLookupTable = match m_collisionLookupTable {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collisionLookupTable",
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
                    let parent = hknpCollisionFilter {
                        __ptr: __ptr.clone(),
                        parent,
                        m_type,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516 {
                        __ptr: __ptr.clone(),
                        parent,
                        m_nextFreeSystemGroup,
                        m_collisionLookupTable,
                    })
                }
            }
            const FIELDS: &[&str] = &["nextFreeSystemGroup", "collisionLookupTable"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516",
                FIELDS,
                __hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516Visitor {
                    marker: _serde::__private::PhantomData::<
                        hknpGroupCollisionFilterBasehknpGroupCollisionFilterTypesConfig55516,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
