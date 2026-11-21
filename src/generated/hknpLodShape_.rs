use super::class_requires::*;
use super::*;
use super::hknpShape_::FlagsEnum;
use super::hknpCollisionDispatchType_::Enum;
/// # C++ Info
/// - name: `hknpLodShape`
/// - version: `1`
/// - signature: `0xca634210`
/// - size: `  0`(x86)/`320`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hknpLodShape<'a> {
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
    pub parent: hknpCompositeShape<'a>,
    /// # C++ Info
    /// - name: `numLevelsOfDetail`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/` 96`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "numLevelsOfDetail"))]
    #[cfg_attr(feature = "serde", serde(rename = "numLevelsOfDetail"))]
    pub m_numLevelsOfDetail: I32<'a>,
    /// # C++ Info
    /// - name: `infos`(ctype: `struct hknpLodShapeLevelOfDetailInfo[8]`)
    /// - offset: `  0`(x86)/`100`(x86_64)
    /// - type_size: `  0`(x86)/` 64`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "infos"))]
    #[cfg_attr(feature = "serde", serde(rename = "infos"))]
    pub m_infos: [hknpLodShapeLevelOfDetailInfo<'a>; 8usize],
    /// # C++ Info
    /// - name: `shapes`(ctype: `struct hknpShape*`)
    /// - offset: `  0`(x86)/`168`(x86_64)
    /// - type_size: `  0`(x86)/` 64`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "shapes"))]
    #[cfg_attr(feature = "serde", serde(rename = "shapes"))]
    pub m_shapes: [Pointer<'a>; 8usize],
    /// # C++ Info
    /// - name: `shapesMemorySizes`(ctype: `hkUint32[8]`)
    /// - offset: `  0`(x86)/`232`(x86_64)
    /// - type_size: `  0`(x86)/` 32`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "shapesMemorySizes"))]
    #[cfg_attr(feature = "serde", serde(rename = "shapesMemorySizes"))]
    pub m_shapesMemorySizes: [U32<'a>; 8usize],
    /// # C++ Info
    /// - name: `indexCurrentShapeOnSpu`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`264`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "indexCurrentShapeOnSpu"))]
    #[cfg_attr(feature = "serde", serde(rename = "indexCurrentShapeOnSpu"))]
    pub m_indexCurrentShapeOnSpu: I32<'a>,
    /// # C++ Info
    /// - name: `currentShapePpuAddress`(ctype: `struct hknpShape*`)
    /// - offset: `  0`(x86)/`272`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "currentShapePpuAddress"))]
    #[cfg_attr(feature = "serde", serde(rename = "currentShapePpuAddress"))]
    pub m_currentShapePpuAddress: Pointer<'a>,
    /// # C++ Info
    /// - name: `maximumAabb`(ctype: `struct hkAabb`)
    /// - offset: `  0`(x86)/`288`(x86_64)
    /// - type_size: `  0`(x86)/` 32`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "maximumAabb"))]
    #[cfg_attr(feature = "serde", serde(rename = "maximumAabb"))]
    pub m_maximumAabb: hkAabb<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hknpLodShape<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hknpLodShape"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xca634210)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.parent.parent.m_properties);
            v.extend(self.parent.m_edgeWeldingMap.deps_indexes());
            v.extend(
                self
                    .m_infos
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(self.m_shapes.iter());
            v.push(&self.m_currentShapePpuAddress);
            v.extend(self.m_maximumAabb.deps_indexes());
            v
        }
    }
    impl<'a> _serde::Serialize for hknpLodShape<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xca634210)));
            let mut serializer = __serializer
                .serialize_struct("hknpLodShape", class_meta, (0u64, 320u64))?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndRefCount",
                    &self.parent.parent.parent.m_memSizeAndRefCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("flags", &self.parent.parent.m_flags)?;
            serializer
                .serialize_field(
                    "numShapeKeyBits",
                    &self.parent.parent.m_numShapeKeyBits,
                )?;
            serializer
                .serialize_field("dispatchType", &self.parent.parent.m_dispatchType)?;
            serializer
                .serialize_field("convexRadius", &self.parent.parent.m_convexRadius)?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer.serialize_field("properties", &self.parent.parent.m_properties)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("edgeWeldingMap", &self.parent.m_edgeWeldingMap)?;
            serializer
                .serialize_field("shapeTagCodecInfo", &self.parent.m_shapeTagCodecInfo)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("numLevelsOfDetail", &self.m_numLevelsOfDetail)?;
            serializer
                .serialize_fixed_array_field(
                    "infos",
                    self.m_infos.as_slice(),
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 8u64,
                    },
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_fixed_array_field(
                    "shapes",
                    self.m_shapes.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "shapesMemorySizes",
                    self.m_shapesMemorySizes.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_field(
                    "indexCurrentShapeOnSpu",
                    &self.m_indexCurrentShapeOnSpu,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "currentShapePpuAddress",
                    &self.m_currentShapePpuAddress,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("maximumAabb", &self.m_maximumAabb)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hknpLodShape<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_flags,
                m_numShapeKeyBits,
                m_dispatchType,
                m_convexRadius,
                m_userData,
                m_properties,
                m_edgeWeldingMap,
                m_shapeTagCodecInfo,
                m_numLevelsOfDetail,
                m_infos,
                m_shapes,
                m_shapesMemorySizes,
                m_indexCurrentShapeOnSpu,
                m_currentShapePpuAddress,
                m_maximumAabb,
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
                        "flags" => Ok(__Field::m_flags),
                        "numShapeKeyBits" => Ok(__Field::m_numShapeKeyBits),
                        "dispatchType" => Ok(__Field::m_dispatchType),
                        "convexRadius" => Ok(__Field::m_convexRadius),
                        "userData" => Ok(__Field::m_userData),
                        "properties" => Ok(__Field::m_properties),
                        "edgeWeldingMap" => Ok(__Field::m_edgeWeldingMap),
                        "shapeTagCodecInfo" => Ok(__Field::m_shapeTagCodecInfo),
                        "numLevelsOfDetail" => Ok(__Field::m_numLevelsOfDetail),
                        "infos" => Ok(__Field::m_infos),
                        "shapes" => Ok(__Field::m_shapes),
                        "shapesMemorySizes" => Ok(__Field::m_shapesMemorySizes),
                        "indexCurrentShapeOnSpu" => Ok(__Field::m_indexCurrentShapeOnSpu),
                        "currentShapePpuAddress" => Ok(__Field::m_currentShapePpuAddress),
                        "maximumAabb" => Ok(__Field::m_maximumAabb),
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
            struct __hknpLodShapeVisitor<'de> {
                marker: _serde::__private::PhantomData<hknpLodShape<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hknpLodShapeVisitor<'de> {
                type Value = hknpLodShape<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hknpLodShape")
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
                    let mut m_numLevelsOfDetail: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_infos: _serde::__private::Option<
                        [hknpLodShapeLevelOfDetailInfo; 8usize],
                    > = _serde::__private::None;
                    let mut m_shapes: _serde::__private::Option<
                        [Pointer<'de>; 8usize],
                    > = _serde::__private::None;
                    let mut m_shapesMemorySizes: _serde::__private::Option<
                        [U32<'de>; 8usize],
                    > = _serde::__private::None;
                    let mut m_indexCurrentShapeOnSpu: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_currentShapePpuAddress: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_maximumAabb: _serde::__private::Option<hkAabb> = _serde::__private::None;
                    for i in 0..7usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_numLevelsOfDetail,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numLevelsOfDetail",
                                        ),
                                    );
                                }
                                m_numLevelsOfDetail = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_infos) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("infos"),
                                    );
                                }
                                m_infos = _serde::__private::Some(
                                    match __A::next_value::<
                                        [hknpLodShapeLevelOfDetailInfo; 8usize],
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_shapes) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("shapes"),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_shapes = _serde::__private::Some(
                                    match __A::next_value::<
                                        [Pointer<'de>; 8usize],
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
                                    &m_shapesMemorySizes,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "shapesMemorySizes",
                                        ),
                                    );
                                }
                                m_shapesMemorySizes = _serde::__private::Some(
                                    match __A::next_value::<[U32<'de>; 8usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_indexCurrentShapeOnSpu,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "indexCurrentShapeOnSpu",
                                        ),
                                    );
                                }
                                m_indexCurrentShapeOnSpu = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_currentShapePpuAddress,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentShapePpuAddress",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_currentShapePpuAddress = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_maximumAabb) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maximumAabb",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 8usize)?;
                                m_maximumAabb = _serde::__private::Some(
                                    match __A::next_value::<hkAabb>(&mut __map) {
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
                    let m_numLevelsOfDetail = match m_numLevelsOfDetail {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numLevelsOfDetail",
                                ),
                            );
                        }
                    };
                    let m_infos = match m_infos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("infos"),
                            );
                        }
                    };
                    let m_shapes = match m_shapes {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("shapes"),
                            );
                        }
                    };
                    let m_shapesMemorySizes = match m_shapesMemorySizes {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "shapesMemorySizes",
                                ),
                            );
                        }
                    };
                    let m_indexCurrentShapeOnSpu = match m_indexCurrentShapeOnSpu {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "indexCurrentShapeOnSpu",
                                ),
                            );
                        }
                    };
                    let m_currentShapePpuAddress = match m_currentShapePpuAddress {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentShapePpuAddress",
                                ),
                            );
                        }
                    };
                    let m_maximumAabb = match m_maximumAabb {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maximumAabb",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hknpLodShape {
                        __ptr,
                        parent,
                        m_numLevelsOfDetail,
                        m_infos,
                        m_shapes,
                        m_shapesMemorySizes,
                        m_indexCurrentShapeOnSpu,
                        m_currentShapePpuAddress,
                        m_maximumAabb,
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
                    let mut m_flags: _serde::__private::Option<FlagsEnum> = _serde::__private::None;
                    let mut m_numShapeKeyBits: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_dispatchType: _serde::__private::Option<Enum> = _serde::__private::None;
                    let mut m_convexRadius: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_userData: _serde::__private::Option<U64<'de>> = _serde::__private::None;
                    let mut m_properties: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_edgeWeldingMap: _serde::__private::Option<
                        hknpSparseCompactMapunsignedshort<'de>,
                    > = _serde::__private::None;
                    let mut m_shapeTagCodecInfo: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_numLevelsOfDetail: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_infos: _serde::__private::Option<
                        [hknpLodShapeLevelOfDetailInfo; 8usize],
                    > = _serde::__private::None;
                    let mut m_shapes: _serde::__private::Option<
                        [Pointer<'de>; 8usize],
                    > = _serde::__private::None;
                    let mut m_shapesMemorySizes: _serde::__private::Option<
                        [U32<'de>; 8usize],
                    > = _serde::__private::None;
                    let mut m_indexCurrentShapeOnSpu: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_currentShapePpuAddress: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_maximumAabb: _serde::__private::Option<hkAabb> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_flags => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_flags) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("flags"),
                                    );
                                }
                                m_flags = _serde::__private::Some(
                                    match __A::next_value::<FlagsEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numShapeKeyBits => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_numShapeKeyBits) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numShapeKeyBits",
                                        ),
                                    );
                                }
                                m_numShapeKeyBits = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_dispatchType => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_dispatchType) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dispatchType",
                                        ),
                                    );
                                }
                                m_dispatchType = _serde::__private::Some(
                                    match __A::next_value::<Enum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_convexRadius => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_convexRadius) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "convexRadius",
                                        ),
                                    );
                                }
                                m_convexRadius = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_userData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_userData) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userData",
                                        ),
                                    );
                                }
                                m_userData = _serde::__private::Some(
                                    match __A::next_value::<U64<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_properties => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_properties) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "properties",
                                        ),
                                    );
                                }
                                m_properties = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_edgeWeldingMap => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_edgeWeldingMap) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "edgeWeldingMap",
                                        ),
                                    );
                                }
                                m_edgeWeldingMap = _serde::__private::Some(
                                    match __A::next_value::<
                                        hknpSparseCompactMapunsignedshort<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_shapeTagCodecInfo => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_shapeTagCodecInfo,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "shapeTagCodecInfo",
                                        ),
                                    );
                                }
                                m_shapeTagCodecInfo = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numLevelsOfDetail => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_numLevelsOfDetail,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numLevelsOfDetail",
                                        ),
                                    );
                                }
                                m_numLevelsOfDetail = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_infos => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_infos) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("infos"),
                                    );
                                }
                                m_infos = _serde::__private::Some(
                                    match __A::next_value::<
                                        [hknpLodShapeLevelOfDetailInfo; 8usize],
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_shapes => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_shapes) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("shapes"),
                                    );
                                }
                                m_shapes = _serde::__private::Some(
                                    match __A::next_value::<
                                        [Pointer<'de>; 8usize],
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_shapesMemorySizes => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_shapesMemorySizes,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "shapesMemorySizes",
                                        ),
                                    );
                                }
                                m_shapesMemorySizes = _serde::__private::Some(
                                    match __A::next_value::<[U32<'de>; 8usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_indexCurrentShapeOnSpu => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_indexCurrentShapeOnSpu,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "indexCurrentShapeOnSpu",
                                        ),
                                    );
                                }
                                m_indexCurrentShapeOnSpu = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_currentShapePpuAddress => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_currentShapePpuAddress,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentShapePpuAddress",
                                        ),
                                    );
                                }
                                m_currentShapePpuAddress = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maximumAabb => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_maximumAabb) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maximumAabb",
                                        ),
                                    );
                                }
                                m_maximumAabb = _serde::__private::Some(
                                    match __A::next_value::<hkAabb>(&mut __map) {
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
                    let m_flags = match m_flags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("flags"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numShapeKeyBits = match m_numShapeKeyBits {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numShapeKeyBits",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_dispatchType = match m_dispatchType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dispatchType",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_convexRadius = match m_convexRadius {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "convexRadius",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_userData = match m_userData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("userData"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_properties = match m_properties {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "properties",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_edgeWeldingMap = match m_edgeWeldingMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "edgeWeldingMap",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_shapeTagCodecInfo = match m_shapeTagCodecInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "shapeTagCodecInfo",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numLevelsOfDetail = match m_numLevelsOfDetail {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numLevelsOfDetail",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_infos = match m_infos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("infos"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_shapes = match m_shapes {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("shapes"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_shapesMemorySizes = match m_shapesMemorySizes {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "shapesMemorySizes",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_indexCurrentShapeOnSpu = match m_indexCurrentShapeOnSpu {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "indexCurrentShapeOnSpu",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_currentShapePpuAddress = match m_currentShapePpuAddress {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentShapePpuAddress",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maximumAabb = match m_maximumAabb {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maximumAabb",
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
                    let parent = hknpShape {
                        __ptr: __ptr.clone(),
                        parent,
                        m_flags,
                        m_numShapeKeyBits,
                        m_dispatchType,
                        m_convexRadius,
                        m_userData,
                        m_properties,
                    };
                    let parent = hknpCompositeShape {
                        __ptr: __ptr.clone(),
                        parent,
                        m_edgeWeldingMap,
                        m_shapeTagCodecInfo,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hknpLodShape {
                        __ptr: __ptr.clone(),
                        parent,
                        m_numLevelsOfDetail,
                        m_infos,
                        m_shapes,
                        m_shapesMemorySizes,
                        m_indexCurrentShapeOnSpu,
                        m_currentShapePpuAddress,
                        m_maximumAabb,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "numLevelsOfDetail",
                "infos",
                "shapes",
                "shapesMemorySizes",
                "indexCurrentShapeOnSpu",
                "currentShapePpuAddress",
                "maximumAabb",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hknpLodShape",
                FIELDS,
                __hknpLodShapeVisitor {
                    marker: _serde::__private::PhantomData::<hknpLodShape>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
