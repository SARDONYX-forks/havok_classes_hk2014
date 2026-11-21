use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkcdStaticPvs`
/// - version: `0`
/// - signature: `0xb1dc4e96`
/// - size: `  0`(x86)/`112`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkcdStaticPvs<'a> {
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
    /// - name: `cells`(ctype: `struct hkcdStaticTreeTreehkcdStaticTreeDynamicStorage6`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/` 48`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "cells"))]
    #[cfg_attr(feature = "serde", serde(rename = "cells"))]
    pub m_cells: hkcdStaticTreeTreehkcdStaticTreeDynamicStorage6<'a>,
    /// # C++ Info
    /// - name: `bytesPerCells`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "bytesPerCells"))]
    #[cfg_attr(feature = "serde", serde(rename = "bytesPerCells"))]
    pub m_bytesPerCells: I32<'a>,
    /// # C++ Info
    /// - name: `cellsPerBlock`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/` 52`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "cellsPerBlock"))]
    #[cfg_attr(feature = "serde", serde(rename = "cellsPerBlock"))]
    pub m_cellsPerBlock: I32<'a>,
    /// # C++ Info
    /// - name: `pvs`(ctype: `hkArray<hkUint8>`)
    /// - offset: `  0`(x86)/` 56`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "pvs"))]
    #[cfg_attr(feature = "serde", serde(rename = "pvs"))]
    pub m_pvs: Vec<U8<'a>>,
    /// # C++ Info
    /// - name: `map`(ctype: `hkArray<hkUint16>`)
    /// - offset: `  0`(x86)/` 72`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "map"))]
    #[cfg_attr(feature = "serde", serde(rename = "map"))]
    pub m_map: Vec<U16<'a>>,
    /// # C++ Info
    /// - name: `blocks`(ctype: `hkArray<struct hkcdStaticPvsBlockHeader>`)
    /// - offset: `  0`(x86)/` 88`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "blocks"))]
    #[cfg_attr(feature = "serde", serde(rename = "blocks"))]
    pub m_blocks: Vec<hkcdStaticPvsBlockHeader<'a>>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkcdStaticPvs<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkcdStaticPvs"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xb1dc4e96)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(self.m_cells.deps_indexes());
            v.extend(
                self
                    .m_blocks
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v
        }
    }
    impl<'a> _serde::Serialize for hkcdStaticPvs<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xb1dc4e96)));
            let mut serializer = __serializer
                .serialize_struct("hkcdStaticPvs", class_meta, (0u64, 112u64))?;
            serializer.serialize_field("cells", &self.m_cells)?;
            serializer.serialize_field("bytesPerCells", &self.m_bytesPerCells)?;
            serializer.serialize_field("cellsPerBlock", &self.m_cellsPerBlock)?;
            serializer.serialize_array_field("pvs", &self.m_pvs, TypeSize::NonPtr)?;
            serializer.serialize_array_field("map", &self.m_map, TypeSize::NonPtr)?;
            serializer
                .serialize_array_field(
                    "blocks",
                    &self.m_blocks,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 8u64,
                    },
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkcdStaticPvs<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_cells,
                m_bytesPerCells,
                m_cellsPerBlock,
                m_pvs,
                m_map,
                m_blocks,
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
                        "cells" => Ok(__Field::m_cells),
                        "bytesPerCells" => Ok(__Field::m_bytesPerCells),
                        "cellsPerBlock" => Ok(__Field::m_cellsPerBlock),
                        "pvs" => Ok(__Field::m_pvs),
                        "map" => Ok(__Field::m_map),
                        "blocks" => Ok(__Field::m_blocks),
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
            struct __hkcdStaticPvsVisitor<'de> {
                marker: _serde::__private::PhantomData<hkcdStaticPvs<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkcdStaticPvsVisitor<'de> {
                type Value = hkcdStaticPvs<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hkcdStaticPvs")
                }
                fn visit_struct_for_bytes<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let __ptr = __A::class_ptr(&mut __map);
                    let mut m_cells: _serde::__private::Option<
                        hkcdStaticTreeTreehkcdStaticTreeDynamicStorage6<'de>,
                    > = _serde::__private::None;
                    let mut m_bytesPerCells: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_cellsPerBlock: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_pvs: _serde::__private::Option<Vec<U8<'de>>> = _serde::__private::None;
                    let mut m_map: _serde::__private::Option<Vec<U16<'de>>> = _serde::__private::None;
                    let mut m_blocks: _serde::__private::Option<
                        Vec<hkcdStaticPvsBlockHeader<'de>>,
                    > = _serde::__private::None;
                    for i in 0..6usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_cells) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("cells"),
                                    );
                                }
                                m_cells = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkcdStaticTreeTreehkcdStaticTreeDynamicStorage6<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_bytesPerCells) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bytesPerCells",
                                        ),
                                    );
                                }
                                m_bytesPerCells = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_cellsPerBlock) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "cellsPerBlock",
                                        ),
                                    );
                                }
                                m_cellsPerBlock = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_pvs) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("pvs"),
                                    );
                                }
                                m_pvs = _serde::__private::Some(
                                    match __A::next_value::<Vec<U8<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_map) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("map"),
                                    );
                                }
                                m_map = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_blocks) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("blocks"),
                                    );
                                }
                                m_blocks = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkcdStaticPvsBlockHeader<'de>>,
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
                    __A::pad(&mut __map, 0usize, 8usize)?;
                    let m_cells = match m_cells {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("cells"),
                            );
                        }
                    };
                    let m_bytesPerCells = match m_bytesPerCells {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bytesPerCells",
                                ),
                            );
                        }
                    };
                    let m_cellsPerBlock = match m_cellsPerBlock {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "cellsPerBlock",
                                ),
                            );
                        }
                    };
                    let m_pvs = match m_pvs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("pvs"),
                            );
                        }
                    };
                    let m_map = match m_map {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("map"),
                            );
                        }
                    };
                    let m_blocks = match m_blocks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("blocks"),
                            );
                        }
                    };
                    _serde::__private::Ok(hkcdStaticPvs {
                        __ptr,
                        m_cells,
                        m_bytesPerCells,
                        m_cellsPerBlock,
                        m_pvs,
                        m_map,
                        m_blocks,
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
                    let mut m_cells: _serde::__private::Option<
                        hkcdStaticTreeTreehkcdStaticTreeDynamicStorage6<'de>,
                    > = _serde::__private::None;
                    let mut m_bytesPerCells: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_cellsPerBlock: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_pvs: _serde::__private::Option<Vec<U8<'de>>> = _serde::__private::None;
                    let mut m_map: _serde::__private::Option<Vec<U16<'de>>> = _serde::__private::None;
                    let mut m_blocks: _serde::__private::Option<
                        Vec<hkcdStaticPvsBlockHeader<'de>>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_cells => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_cells) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("cells"),
                                    );
                                }
                                m_cells = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkcdStaticTreeTreehkcdStaticTreeDynamicStorage6<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bytesPerCells => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bytesPerCells) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bytesPerCells",
                                        ),
                                    );
                                }
                                m_bytesPerCells = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_cellsPerBlock => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_cellsPerBlock) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "cellsPerBlock",
                                        ),
                                    );
                                }
                                m_cellsPerBlock = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_pvs => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_pvs) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("pvs"),
                                    );
                                }
                                m_pvs = _serde::__private::Some(
                                    match __A::next_value::<Vec<U8<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_map => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_map) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("map"),
                                    );
                                }
                                m_map = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_blocks => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_blocks) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("blocks"),
                                    );
                                }
                                m_blocks = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkcdStaticPvsBlockHeader<'de>>,
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
                    let m_cells = match m_cells {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("cells"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bytesPerCells = match m_bytesPerCells {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bytesPerCells",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_cellsPerBlock = match m_cellsPerBlock {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "cellsPerBlock",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_pvs = match m_pvs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("pvs"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_map = match m_map {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("map"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_blocks = match m_blocks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("blocks"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkcdStaticPvs {
                        __ptr: __ptr.clone(),
                        m_cells,
                        m_bytesPerCells,
                        m_cellsPerBlock,
                        m_pvs,
                        m_map,
                        m_blocks,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "cells",
                "bytesPerCells",
                "cellsPerBlock",
                "pvs",
                "map",
                "blocks",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkcdStaticPvs",
                FIELDS,
                __hkcdStaticPvsVisitor {
                    marker: _serde::__private::PhantomData::<hkcdStaticPvs>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
