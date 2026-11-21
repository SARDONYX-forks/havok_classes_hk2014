use super::class_requires::*;
use super::*;
use super::hknpShape_::FlagsEnum;
use super::hknpCollisionDispatchType_::Enum;
/// # C++ Info
/// - name: `hknpHeightFieldShape`
/// - version: `2`
/// - signature: `0xf196fcc7`
/// - size: `  0`(x86)/`240`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hknpHeightFieldShape<'a> {
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
    /// - name: `aabb`(ctype: `struct hkAabb`)
    /// - offset: `  0`(x86)/` 96`(x86_64)
    /// - type_size: `  0`(x86)/` 32`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "aabb"))]
    #[cfg_attr(feature = "serde", serde(rename = "aabb"))]
    pub m_aabb: hkAabb<'a>,
    /// # C++ Info
    /// - name: `floatToIntScale`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`128`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatToIntScale"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatToIntScale"))]
    pub m_floatToIntScale: Vector4,
    /// # C++ Info
    /// - name: `intToFloatScale`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`144`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "intToFloatScale"))]
    #[cfg_attr(feature = "serde", serde(rename = "intToFloatScale"))]
    pub m_intToFloatScale: Vector4,
    /// # C++ Info
    /// - name: `intSizeX`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`160`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "intSizeX"))]
    #[cfg_attr(feature = "serde", serde(rename = "intSizeX"))]
    pub m_intSizeX: I32<'a>,
    /// # C++ Info
    /// - name: `intSizeZ`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`164`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "intSizeZ"))]
    #[cfg_attr(feature = "serde", serde(rename = "intSizeZ"))]
    pub m_intSizeZ: I32<'a>,
    /// # C++ Info
    /// - name: `numBitsX`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`168`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "numBitsX"))]
    #[cfg_attr(feature = "serde", serde(rename = "numBitsX"))]
    pub m_numBitsX: I32<'a>,
    /// # C++ Info
    /// - name: `numBitsZ`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`172`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "numBitsZ"))]
    #[cfg_attr(feature = "serde", serde(rename = "numBitsZ"))]
    pub m_numBitsZ: I32<'a>,
    /// # C++ Info
    /// - name: `minMaxTree`(ctype: `struct hknpMinMaxQuadTree`)
    /// - offset: `  0`(x86)/`176`(x86_64)
    /// - type_size: `  0`(x86)/` 48`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "minMaxTree"))]
    #[cfg_attr(feature = "serde", serde(rename = "minMaxTree"))]
    pub m_minMaxTree: hknpMinMaxQuadTree<'a>,
    /// # C++ Info
    /// - name: `minMaxTreeCoarseness`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`224`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "minMaxTreeCoarseness"))]
    #[cfg_attr(feature = "serde", serde(rename = "minMaxTreeCoarseness"))]
    pub m_minMaxTreeCoarseness: I32<'a>,
    /// # C++ Info
    /// - name: `includeShapeKeyInSdfContacts`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`228`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "includeShapeKeyInSdfContacts")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "includeShapeKeyInSdfContacts"))]
    pub m_includeShapeKeyInSdfContacts: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hknpHeightFieldShape<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hknpHeightFieldShape"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf196fcc7)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.parent.parent.m_properties);
            v.extend(self.parent.m_edgeWeldingMap.deps_indexes());
            v.extend(self.m_aabb.deps_indexes());
            v.extend(self.m_minMaxTree.deps_indexes());
            v
        }
    }
    impl<'a> _serde::Serialize for hknpHeightFieldShape<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xf196fcc7)));
            let mut serializer = __serializer
                .serialize_struct("hknpHeightFieldShape", class_meta, (0u64, 240u64))?;
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
            serializer.serialize_field("aabb", &self.m_aabb)?;
            serializer.serialize_field("floatToIntScale", &self.m_floatToIntScale)?;
            serializer.serialize_field("intToFloatScale", &self.m_intToFloatScale)?;
            serializer.serialize_field("intSizeX", &self.m_intSizeX)?;
            serializer.serialize_field("intSizeZ", &self.m_intSizeZ)?;
            serializer.serialize_field("numBitsX", &self.m_numBitsX)?;
            serializer.serialize_field("numBitsZ", &self.m_numBitsZ)?;
            serializer.serialize_field("minMaxTree", &self.m_minMaxTree)?;
            serializer
                .serialize_field("minMaxTreeCoarseness", &self.m_minMaxTreeCoarseness)?;
            serializer
                .serialize_field(
                    "includeShapeKeyInSdfContacts",
                    &self.m_includeShapeKeyInSdfContacts,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 11usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hknpHeightFieldShape<'de> {
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
                m_aabb,
                m_floatToIntScale,
                m_intToFloatScale,
                m_intSizeX,
                m_intSizeZ,
                m_numBitsX,
                m_numBitsZ,
                m_minMaxTree,
                m_minMaxTreeCoarseness,
                m_includeShapeKeyInSdfContacts,
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
                        "aabb" => Ok(__Field::m_aabb),
                        "floatToIntScale" => Ok(__Field::m_floatToIntScale),
                        "intToFloatScale" => Ok(__Field::m_intToFloatScale),
                        "intSizeX" => Ok(__Field::m_intSizeX),
                        "intSizeZ" => Ok(__Field::m_intSizeZ),
                        "numBitsX" => Ok(__Field::m_numBitsX),
                        "numBitsZ" => Ok(__Field::m_numBitsZ),
                        "minMaxTree" => Ok(__Field::m_minMaxTree),
                        "minMaxTreeCoarseness" => Ok(__Field::m_minMaxTreeCoarseness),
                        "includeShapeKeyInSdfContacts" => {
                            Ok(__Field::m_includeShapeKeyInSdfContacts)
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
            struct __hknpHeightFieldShapeVisitor<'de> {
                marker: _serde::__private::PhantomData<hknpHeightFieldShape<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hknpHeightFieldShapeVisitor<'de> {
                type Value = hknpHeightFieldShape<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hknpHeightFieldShape",
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
                    let mut m_aabb: _serde::__private::Option<hkAabb> = _serde::__private::None;
                    let mut m_floatToIntScale: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_intToFloatScale: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_intSizeX: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_intSizeZ: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_numBitsX: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_numBitsZ: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_minMaxTree: _serde::__private::Option<
                        hknpMinMaxQuadTree<'de>,
                    > = _serde::__private::None;
                    let mut m_minMaxTreeCoarseness: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_includeShapeKeyInSdfContacts: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    for i in 0..10usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_aabb) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("aabb"),
                                    );
                                }
                                m_aabb = _serde::__private::Some(
                                    match __A::next_value::<hkAabb>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_floatToIntScale) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatToIntScale",
                                        ),
                                    );
                                }
                                m_floatToIntScale = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_intToFloatScale) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intToFloatScale",
                                        ),
                                    );
                                }
                                m_intToFloatScale = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_intSizeX) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intSizeX",
                                        ),
                                    );
                                }
                                m_intSizeX = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_intSizeZ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intSizeZ",
                                        ),
                                    );
                                }
                                m_intSizeZ = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_numBitsX) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numBitsX",
                                        ),
                                    );
                                }
                                m_numBitsX = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_numBitsZ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numBitsZ",
                                        ),
                                    );
                                }
                                m_numBitsZ = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_minMaxTree) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minMaxTree",
                                        ),
                                    );
                                }
                                m_minMaxTree = _serde::__private::Some(
                                    match __A::next_value::<
                                        hknpMinMaxQuadTree<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_minMaxTreeCoarseness,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minMaxTreeCoarseness",
                                        ),
                                    );
                                }
                                m_minMaxTreeCoarseness = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(
                                    &m_includeShapeKeyInSdfContacts,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "includeShapeKeyInSdfContacts",
                                        ),
                                    );
                                }
                                m_includeShapeKeyInSdfContacts = _serde::__private::Some(
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
                    __A::pad(&mut __map, 0usize, 11usize)?;
                    let m_aabb = match m_aabb {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("aabb"),
                            );
                        }
                    };
                    let m_floatToIntScale = match m_floatToIntScale {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatToIntScale",
                                ),
                            );
                        }
                    };
                    let m_intToFloatScale = match m_intToFloatScale {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intToFloatScale",
                                ),
                            );
                        }
                    };
                    let m_intSizeX = match m_intSizeX {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("intSizeX"),
                            );
                        }
                    };
                    let m_intSizeZ = match m_intSizeZ {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("intSizeZ"),
                            );
                        }
                    };
                    let m_numBitsX = match m_numBitsX {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("numBitsX"),
                            );
                        }
                    };
                    let m_numBitsZ = match m_numBitsZ {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("numBitsZ"),
                            );
                        }
                    };
                    let m_minMaxTree = match m_minMaxTree {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minMaxTree",
                                ),
                            );
                        }
                    };
                    let m_minMaxTreeCoarseness = match m_minMaxTreeCoarseness {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minMaxTreeCoarseness",
                                ),
                            );
                        }
                    };
                    let m_includeShapeKeyInSdfContacts = match m_includeShapeKeyInSdfContacts {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "includeShapeKeyInSdfContacts",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hknpHeightFieldShape {
                        __ptr,
                        parent,
                        m_aabb,
                        m_floatToIntScale,
                        m_intToFloatScale,
                        m_intSizeX,
                        m_intSizeZ,
                        m_numBitsX,
                        m_numBitsZ,
                        m_minMaxTree,
                        m_minMaxTreeCoarseness,
                        m_includeShapeKeyInSdfContacts,
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
                    let mut m_aabb: _serde::__private::Option<hkAabb> = _serde::__private::None;
                    let mut m_floatToIntScale: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_intToFloatScale: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_intSizeX: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_intSizeZ: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_numBitsX: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_numBitsZ: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_minMaxTree: _serde::__private::Option<
                        hknpMinMaxQuadTree<'de>,
                    > = _serde::__private::None;
                    let mut m_minMaxTreeCoarseness: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_includeShapeKeyInSdfContacts: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
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
                            __Field::m_aabb => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_aabb) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("aabb"),
                                    );
                                }
                                m_aabb = _serde::__private::Some(
                                    match __A::next_value::<hkAabb>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatToIntScale => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatToIntScale) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatToIntScale",
                                        ),
                                    );
                                }
                                m_floatToIntScale = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_intToFloatScale => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_intToFloatScale) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intToFloatScale",
                                        ),
                                    );
                                }
                                m_intToFloatScale = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_intSizeX => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_intSizeX) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intSizeX",
                                        ),
                                    );
                                }
                                m_intSizeX = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_intSizeZ => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_intSizeZ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intSizeZ",
                                        ),
                                    );
                                }
                                m_intSizeZ = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numBitsX => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_numBitsX) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numBitsX",
                                        ),
                                    );
                                }
                                m_numBitsX = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numBitsZ => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_numBitsZ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numBitsZ",
                                        ),
                                    );
                                }
                                m_numBitsZ = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_minMaxTree => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_minMaxTree) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minMaxTree",
                                        ),
                                    );
                                }
                                m_minMaxTree = _serde::__private::Some(
                                    match __A::next_value::<
                                        hknpMinMaxQuadTree<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_minMaxTreeCoarseness => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_minMaxTreeCoarseness,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minMaxTreeCoarseness",
                                        ),
                                    );
                                }
                                m_minMaxTreeCoarseness = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_includeShapeKeyInSdfContacts => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_includeShapeKeyInSdfContacts,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "includeShapeKeyInSdfContacts",
                                        ),
                                    );
                                }
                                m_includeShapeKeyInSdfContacts = _serde::__private::Some(
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
                    let m_aabb = match m_aabb {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("aabb"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatToIntScale = match m_floatToIntScale {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatToIntScale",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_intToFloatScale = match m_intToFloatScale {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intToFloatScale",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_intSizeX = match m_intSizeX {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("intSizeX"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_intSizeZ = match m_intSizeZ {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("intSizeZ"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numBitsX = match m_numBitsX {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("numBitsX"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numBitsZ = match m_numBitsZ {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("numBitsZ"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_minMaxTree = match m_minMaxTree {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minMaxTree",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_minMaxTreeCoarseness = match m_minMaxTreeCoarseness {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minMaxTreeCoarseness",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_includeShapeKeyInSdfContacts = match m_includeShapeKeyInSdfContacts {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "includeShapeKeyInSdfContacts",
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
                    _serde::__private::Ok(hknpHeightFieldShape {
                        __ptr: __ptr.clone(),
                        parent,
                        m_aabb,
                        m_floatToIntScale,
                        m_intToFloatScale,
                        m_intSizeX,
                        m_intSizeZ,
                        m_numBitsX,
                        m_numBitsZ,
                        m_minMaxTree,
                        m_minMaxTreeCoarseness,
                        m_includeShapeKeyInSdfContacts,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "aabb",
                "floatToIntScale",
                "intToFloatScale",
                "intSizeX",
                "intSizeZ",
                "numBitsX",
                "numBitsZ",
                "minMaxTree",
                "minMaxTreeCoarseness",
                "includeShapeKeyInSdfContacts",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hknpHeightFieldShape",
                FIELDS,
                __hknpHeightFieldShapeVisitor {
                    marker: _serde::__private::PhantomData::<hknpHeightFieldShape>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
