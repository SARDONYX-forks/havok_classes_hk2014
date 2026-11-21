use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclRuntimeConversionInfo`
/// - version: `0`
/// - signature: `0xe36985b8`
/// - size: `  0`(x86)/` 42`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclRuntimeConversionInfo<'a> {
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
    /// # C++ Info
    /// - name: `slotConversions`(ctype: `struct hclRuntimeConversionInfoSlotConversion[4]`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/` 28`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "slotConversions"))]
    #[cfg_attr(feature = "serde", serde(rename = "slotConversions"))]
    pub m_slotConversions: [hclRuntimeConversionInfoSlotConversion<'a>; 4usize],
    /// # C++ Info
    /// - name: `elementConversions`(ctype: `struct hclRuntimeConversionInfoElementConversion[4]`)
    /// - offset: `  0`(x86)/` 28`(x86_64)
    /// - type_size: `  0`(x86)/` 12`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "elementConversions"))]
    #[cfg_attr(feature = "serde", serde(rename = "elementConversions"))]
    pub m_elementConversions: [hclRuntimeConversionInfoElementConversion<'a>; 4usize],
    /// # C++ Info
    /// - name: `numSlotsConverted`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/` 40`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "numSlotsConverted"))]
    #[cfg_attr(feature = "serde", serde(rename = "numSlotsConverted"))]
    pub m_numSlotsConverted: U8<'a>,
    /// # C++ Info
    /// - name: `numElementsConverted`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/` 41`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "numElementsConverted"))]
    #[cfg_attr(feature = "serde", serde(rename = "numElementsConverted"))]
    pub m_numElementsConverted: U8<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclRuntimeConversionInfo<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclRuntimeConversionInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xe36985b8)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(
                self
                    .m_slotConversions
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(
                self
                    .m_elementConversions
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v
        }
    }
    impl<'a> _serde::Serialize for hclRuntimeConversionInfo<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xe36985b8)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hclRuntimeConversionInfo",
                    class_meta,
                    (0u64, 42u64),
                )?;
            serializer
                .serialize_fixed_array_field(
                    "slotConversions",
                    self.m_slotConversions.as_slice(),
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 7u64,
                    },
                )?;
            serializer
                .serialize_fixed_array_field(
                    "elementConversions",
                    self.m_elementConversions.as_slice(),
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 3u64,
                    },
                )?;
            serializer.serialize_field("numSlotsConverted", &self.m_numSlotsConverted)?;
            serializer
                .serialize_field("numElementsConverted", &self.m_numElementsConverted)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hclRuntimeConversionInfo<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_slotConversions,
                m_elementConversions,
                m_numSlotsConverted,
                m_numElementsConverted,
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
                        "slotConversions" => Ok(__Field::m_slotConversions),
                        "elementConversions" => Ok(__Field::m_elementConversions),
                        "numSlotsConverted" => Ok(__Field::m_numSlotsConverted),
                        "numElementsConverted" => Ok(__Field::m_numElementsConverted),
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
            struct __hclRuntimeConversionInfoVisitor<'de> {
                marker: _serde::__private::PhantomData<hclRuntimeConversionInfo<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hclRuntimeConversionInfoVisitor<'de> {
                type Value = hclRuntimeConversionInfo<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclRuntimeConversionInfo",
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
                    let mut m_slotConversions: _serde::__private::Option<
                        [hclRuntimeConversionInfoSlotConversion<'de>; 4usize],
                    > = _serde::__private::None;
                    let mut m_elementConversions: _serde::__private::Option<
                        [hclRuntimeConversionInfoElementConversion<'de>; 4usize],
                    > = _serde::__private::None;
                    let mut m_numSlotsConverted: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_numElementsConverted: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    for i in 0..4usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_slotConversions) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "slotConversions",
                                        ),
                                    );
                                }
                                m_slotConversions = _serde::__private::Some(
                                    match __A::next_value::<
                                        [hclRuntimeConversionInfoSlotConversion<'de>; 4usize],
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_elementConversions,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "elementConversions",
                                        ),
                                    );
                                }
                                m_elementConversions = _serde::__private::Some(
                                    match __A::next_value::<
                                        [hclRuntimeConversionInfoElementConversion<'de>; 4usize],
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_numSlotsConverted,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numSlotsConverted",
                                        ),
                                    );
                                }
                                m_numSlotsConverted = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_numElementsConverted,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numElementsConverted",
                                        ),
                                    );
                                }
                                m_numElementsConverted = _serde::__private::Some(
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
                    let m_slotConversions = match m_slotConversions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "slotConversions",
                                ),
                            );
                        }
                    };
                    let m_elementConversions = match m_elementConversions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "elementConversions",
                                ),
                            );
                        }
                    };
                    let m_numSlotsConverted = match m_numSlotsConverted {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numSlotsConverted",
                                ),
                            );
                        }
                    };
                    let m_numElementsConverted = match m_numElementsConverted {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numElementsConverted",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hclRuntimeConversionInfo {
                        __ptr,
                        m_slotConversions,
                        m_elementConversions,
                        m_numSlotsConverted,
                        m_numElementsConverted,
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
                    let mut m_slotConversions: _serde::__private::Option<
                        [hclRuntimeConversionInfoSlotConversion<'de>; 4usize],
                    > = _serde::__private::None;
                    let mut m_elementConversions: _serde::__private::Option<
                        [hclRuntimeConversionInfoElementConversion<'de>; 4usize],
                    > = _serde::__private::None;
                    let mut m_numSlotsConverted: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_numElementsConverted: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_slotConversions => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_slotConversions) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "slotConversions",
                                        ),
                                    );
                                }
                                m_slotConversions = _serde::__private::Some(
                                    match __A::next_value::<
                                        [hclRuntimeConversionInfoSlotConversion<'de>; 4usize],
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_elementConversions => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_elementConversions,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "elementConversions",
                                        ),
                                    );
                                }
                                m_elementConversions = _serde::__private::Some(
                                    match __A::next_value::<
                                        [hclRuntimeConversionInfoElementConversion<'de>; 4usize],
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numSlotsConverted => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_numSlotsConverted,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numSlotsConverted",
                                        ),
                                    );
                                }
                                m_numSlotsConverted = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numElementsConverted => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_numElementsConverted,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numElementsConverted",
                                        ),
                                    );
                                }
                                m_numElementsConverted = _serde::__private::Some(
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
                    let m_slotConversions = match m_slotConversions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "slotConversions",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_elementConversions = match m_elementConversions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "elementConversions",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numSlotsConverted = match m_numSlotsConverted {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numSlotsConverted",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numElementsConverted = match m_numElementsConverted {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numElementsConverted",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hclRuntimeConversionInfo {
                        __ptr: __ptr.clone(),
                        m_slotConversions,
                        m_elementConversions,
                        m_numSlotsConverted,
                        m_numElementsConverted,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "slotConversions",
                "elementConversions",
                "numSlotsConverted",
                "numElementsConverted",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclRuntimeConversionInfo",
                FIELDS,
                __hclRuntimeConversionInfoVisitor {
                    marker: _serde::__private::PhantomData::<hclRuntimeConversionInfo>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
/// # C++ Info
/// - name: `VectorConversion`(ctype: `hkEnum<VectorConversion, hkUint8>`)
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum VectorConversion {
    #[default]
    VC_FLOAT4 = 0isize,
    VC_FLOAT3 = 1isize,
    VC_BYTE4 = 2isize,
    VC_SHORT3 = 3isize,
    VC_HFLOAT3 = 4isize,
    VC_CUSTOM_A = 20isize,
    VC_CUSTOM_B = 21isize,
    VC_CUSTOM_C = 22isize,
    VC_CUSTOM_D = 23isize,
    VC_CUSTOM_E = 24isize,
    VC_NONE = 250isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for VectorConversion {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::VC_FLOAT4 => __serializer.serialize_field("VC_FLOAT4", &0u64),
                Self::VC_FLOAT3 => __serializer.serialize_field("VC_FLOAT3", &1u64),
                Self::VC_BYTE4 => __serializer.serialize_field("VC_BYTE4", &2u64),
                Self::VC_SHORT3 => __serializer.serialize_field("VC_SHORT3", &3u64),
                Self::VC_HFLOAT3 => __serializer.serialize_field("VC_HFLOAT3", &4u64),
                Self::VC_CUSTOM_A => __serializer.serialize_field("VC_CUSTOM_A", &20u64),
                Self::VC_CUSTOM_B => __serializer.serialize_field("VC_CUSTOM_B", &21u64),
                Self::VC_CUSTOM_C => __serializer.serialize_field("VC_CUSTOM_C", &22u64),
                Self::VC_CUSTOM_D => __serializer.serialize_field("VC_CUSTOM_D", &23u64),
                Self::VC_CUSTOM_E => __serializer.serialize_field("VC_CUSTOM_E", &24u64),
                Self::VC_NONE => __serializer.serialize_field("VC_NONE", &250u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum VectorConversion to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for VectorConversion {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
                __field6,
                __field7,
                __field8,
                __field9,
                __field10,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_uint8<__E>(
                    self,
                    __value: U8<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        U8::Number(0u8) => _serde::__private::Ok(__Field::__field0),
                        U8::Number(1u8) => _serde::__private::Ok(__Field::__field1),
                        U8::Number(2u8) => _serde::__private::Ok(__Field::__field2),
                        U8::Number(3u8) => _serde::__private::Ok(__Field::__field3),
                        U8::Number(4u8) => _serde::__private::Ok(__Field::__field4),
                        U8::Number(20u8) => _serde::__private::Ok(__Field::__field5),
                        U8::Number(21u8) => _serde::__private::Ok(__Field::__field6),
                        U8::Number(22u8) => _serde::__private::Ok(__Field::__field7),
                        U8::Number(23u8) => _serde::__private::Ok(__Field::__field8),
                        U8::Number(24u8) => _serde::__private::Ok(__Field::__field9),
                        U8::Number(250u8) => _serde::__private::Ok(__Field::__field10),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 0, 1, 2, 3, 4, 20, 21, 22, 23, 24, 250",
                                ),
                            )
                        }
                    }
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "0" || v.eq_ignore_ascii_case("VC_FLOAT4") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("VC_FLOAT3") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2" || v.eq_ignore_ascii_case("VC_BYTE4") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3" || v.eq_ignore_ascii_case("VC_SHORT3") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4" || v.eq_ignore_ascii_case("VC_HFLOAT3") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "20" || v.eq_ignore_ascii_case("VC_CUSTOM_A") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "21" || v.eq_ignore_ascii_case("VC_CUSTOM_B") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "22" || v.eq_ignore_ascii_case("VC_CUSTOM_C") => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            v if v == "23" || v.eq_ignore_ascii_case("VC_CUSTOM_D") => {
                                _serde::__private::Ok(__Field::__field8)
                            }
                            v if v == "24" || v.eq_ignore_ascii_case("VC_CUSTOM_E") => {
                                _serde::__private::Ok(__Field::__field9)
                            }
                            v if v == "250" || v.eq_ignore_ascii_case("VC_NONE") => {
                                _serde::__private::Ok(__Field::__field10)
                            }
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        _serde::de::ReadEnumSize::Uint8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<VectorConversion>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = VectorConversion;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum VectorConversion",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(VectorConversion::VC_FLOAT4)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(VectorConversion::VC_FLOAT3)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(VectorConversion::VC_BYTE4)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(VectorConversion::VC_SHORT3)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(VectorConversion::VC_HFLOAT3)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(VectorConversion::VC_CUSTOM_A)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(VectorConversion::VC_CUSTOM_B)
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(VectorConversion::VC_CUSTOM_C)
                        }
                        (__Field::__field8, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(VectorConversion::VC_CUSTOM_D)
                        }
                        (__Field::__field9, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(VectorConversion::VC_CUSTOM_E)
                        }
                        (__Field::__field10, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(VectorConversion::VC_NONE)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "VC_FLOAT4",
                "VC_FLOAT3",
                "VC_BYTE4",
                "VC_SHORT3",
                "VC_HFLOAT3",
                "VC_CUSTOM_A",
                "VC_CUSTOM_B",
                "VC_CUSTOM_C",
                "VC_CUSTOM_D",
                "VC_CUSTOM_E",
                "VC_NONE",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "VectorConversion",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<VectorConversion>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
