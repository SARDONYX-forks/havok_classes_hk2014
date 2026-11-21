use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hknpMotionProperties`
/// - version: `3`
/// - signature: `0x5dee8641`
/// - size: `  0`(x86)/` 64`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hknpMotionProperties<'a> {
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
    /// - name: `isExclusive`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    /// - flags: `ALIGN_16`
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "isExclusive"))]
    #[cfg_attr(feature = "serde", serde(rename = "isExclusive"))]
    pub m_isExclusive: U32<'a>,
    /// # C++ Info
    /// - name: `flags`(ctype: `flags FlagsEnum`)
    /// - offset: `  0`(x86)/`  4`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "flags"))]
    #[cfg_attr(feature = "serde", serde(rename = "flags"))]
    pub m_flags: FlagsEnum,
    /// # C++ Info
    /// - name: `gravityFactor`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`  8`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "gravityFactor"))]
    #[cfg_attr(feature = "serde", serde(rename = "gravityFactor"))]
    pub m_gravityFactor: f32,
    /// # C++ Info
    /// - name: `timeFactor`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 12`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "timeFactor"))]
    #[cfg_attr(feature = "serde", serde(rename = "timeFactor"))]
    pub m_timeFactor: f32,
    /// # C++ Info
    /// - name: `maxLinearSpeed`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxLinearSpeed"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxLinearSpeed"))]
    pub m_maxLinearSpeed: f32,
    /// # C++ Info
    /// - name: `maxAngularSpeed`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 20`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxAngularSpeed"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxAngularSpeed"))]
    pub m_maxAngularSpeed: f32,
    /// # C++ Info
    /// - name: `linearDamping`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 24`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "linearDamping"))]
    #[cfg_attr(feature = "serde", serde(rename = "linearDamping"))]
    pub m_linearDamping: f32,
    /// # C++ Info
    /// - name: `angularDamping`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 28`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "angularDamping"))]
    #[cfg_attr(feature = "serde", serde(rename = "angularDamping"))]
    pub m_angularDamping: f32,
    /// # C++ Info
    /// - name: `solverStabilizationSpeedThreshold`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "solverStabilizationSpeedThreshold")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "solverStabilizationSpeedThreshold"))]
    pub m_solverStabilizationSpeedThreshold: f32,
    /// # C++ Info
    /// - name: `solverStabilizationSpeedReduction`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 36`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "solverStabilizationSpeedReduction")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "solverStabilizationSpeedReduction"))]
    pub m_solverStabilizationSpeedReduction: f32,
    /// # C++ Info
    /// - name: `maxDistSqrd`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 40`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxDistSqrd"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxDistSqrd"))]
    pub m_maxDistSqrd: f32,
    /// # C++ Info
    /// - name: `maxRotSqrd`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 44`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxRotSqrd"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxRotSqrd"))]
    pub m_maxRotSqrd: f32,
    /// # C++ Info
    /// - name: `invBlockSize`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "invBlockSize"))]
    #[cfg_attr(feature = "serde", serde(rename = "invBlockSize"))]
    pub m_invBlockSize: f32,
    /// # C++ Info
    /// - name: `pathingUpperThreshold`(ctype: `hkInt16`)
    /// - offset: `  0`(x86)/` 52`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "pathingUpperThreshold"))]
    #[cfg_attr(feature = "serde", serde(rename = "pathingUpperThreshold"))]
    pub m_pathingUpperThreshold: I16<'a>,
    /// # C++ Info
    /// - name: `pathingLowerThreshold`(ctype: `hkInt16`)
    /// - offset: `  0`(x86)/` 54`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "pathingLowerThreshold"))]
    #[cfg_attr(feature = "serde", serde(rename = "pathingLowerThreshold"))]
    pub m_pathingLowerThreshold: I16<'a>,
    /// # C++ Info
    /// - name: `numDeactivationFrequencyPasses`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/` 56`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "numDeactivationFrequencyPasses")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "numDeactivationFrequencyPasses"))]
    pub m_numDeactivationFrequencyPasses: U8<'a>,
    /// # C++ Info
    /// - name: `deactivationVelocityScaleSquare`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/` 57`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "deactivationVelocityScaleSquare")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "deactivationVelocityScaleSquare"))]
    pub m_deactivationVelocityScaleSquare: U8<'a>,
    /// # C++ Info
    /// - name: `minimumPathingVelocityScaleSquare`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/` 58`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "minimumPathingVelocityScaleSquare")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "minimumPathingVelocityScaleSquare"))]
    pub m_minimumPathingVelocityScaleSquare: U8<'a>,
    /// # C++ Info
    /// - name: `spikingVelocityScaleThresholdSquared`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/` 59`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "spikingVelocityScaleThresholdSquared")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(rename = "spikingVelocityScaleThresholdSquared")
    )]
    pub m_spikingVelocityScaleThresholdSquared: U8<'a>,
    /// # C++ Info
    /// - name: `minimumSpikingVelocityScaleSquared`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/` 60`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "minimumSpikingVelocityScaleSquared")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "minimumSpikingVelocityScaleSquared"))]
    pub m_minimumSpikingVelocityScaleSquared: U8<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hknpMotionProperties<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hknpMotionProperties"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x5dee8641)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hknpMotionProperties<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x5dee8641)));
            let mut serializer = __serializer
                .serialize_struct("hknpMotionProperties", class_meta, (0u64, 64u64))?;
            serializer.serialize_field("isExclusive", &self.m_isExclusive)?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer.serialize_field("gravityFactor", &self.m_gravityFactor)?;
            serializer.serialize_field("timeFactor", &self.m_timeFactor)?;
            serializer.serialize_field("maxLinearSpeed", &self.m_maxLinearSpeed)?;
            serializer.serialize_field("maxAngularSpeed", &self.m_maxAngularSpeed)?;
            serializer.serialize_field("linearDamping", &self.m_linearDamping)?;
            serializer.serialize_field("angularDamping", &self.m_angularDamping)?;
            serializer
                .serialize_field(
                    "solverStabilizationSpeedThreshold",
                    &self.m_solverStabilizationSpeedThreshold,
                )?;
            serializer
                .serialize_field(
                    "solverStabilizationSpeedReduction",
                    &self.m_solverStabilizationSpeedReduction,
                )?;
            serializer.serialize_field("maxDistSqrd", &self.m_maxDistSqrd)?;
            serializer.serialize_field("maxRotSqrd", &self.m_maxRotSqrd)?;
            serializer.serialize_field("invBlockSize", &self.m_invBlockSize)?;
            serializer
                .serialize_field(
                    "pathingUpperThreshold",
                    &self.m_pathingUpperThreshold,
                )?;
            serializer
                .serialize_field(
                    "pathingLowerThreshold",
                    &self.m_pathingLowerThreshold,
                )?;
            serializer
                .serialize_field(
                    "numDeactivationFrequencyPasses",
                    &self.m_numDeactivationFrequencyPasses,
                )?;
            serializer
                .serialize_field(
                    "deactivationVelocityScaleSquare",
                    &self.m_deactivationVelocityScaleSquare,
                )?;
            serializer
                .serialize_field(
                    "minimumPathingVelocityScaleSquare",
                    &self.m_minimumPathingVelocityScaleSquare,
                )?;
            serializer
                .serialize_field(
                    "spikingVelocityScaleThresholdSquared",
                    &self.m_spikingVelocityScaleThresholdSquared,
                )?;
            serializer
                .serialize_field(
                    "minimumSpikingVelocityScaleSquared",
                    &self.m_minimumSpikingVelocityScaleSquared,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hknpMotionProperties<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_isExclusive,
                m_flags,
                m_gravityFactor,
                m_timeFactor,
                m_maxLinearSpeed,
                m_maxAngularSpeed,
                m_linearDamping,
                m_angularDamping,
                m_solverStabilizationSpeedThreshold,
                m_solverStabilizationSpeedReduction,
                m_maxDistSqrd,
                m_maxRotSqrd,
                m_invBlockSize,
                m_pathingUpperThreshold,
                m_pathingLowerThreshold,
                m_numDeactivationFrequencyPasses,
                m_deactivationVelocityScaleSquare,
                m_minimumPathingVelocityScaleSquare,
                m_spikingVelocityScaleThresholdSquared,
                m_minimumSpikingVelocityScaleSquared,
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
                        "isExclusive" => Ok(__Field::m_isExclusive),
                        "flags" => Ok(__Field::m_flags),
                        "gravityFactor" => Ok(__Field::m_gravityFactor),
                        "timeFactor" => Ok(__Field::m_timeFactor),
                        "maxLinearSpeed" => Ok(__Field::m_maxLinearSpeed),
                        "maxAngularSpeed" => Ok(__Field::m_maxAngularSpeed),
                        "linearDamping" => Ok(__Field::m_linearDamping),
                        "angularDamping" => Ok(__Field::m_angularDamping),
                        "solverStabilizationSpeedThreshold" => {
                            Ok(__Field::m_solverStabilizationSpeedThreshold)
                        }
                        "solverStabilizationSpeedReduction" => {
                            Ok(__Field::m_solverStabilizationSpeedReduction)
                        }
                        "maxDistSqrd" => Ok(__Field::m_maxDistSqrd),
                        "maxRotSqrd" => Ok(__Field::m_maxRotSqrd),
                        "invBlockSize" => Ok(__Field::m_invBlockSize),
                        "pathingUpperThreshold" => Ok(__Field::m_pathingUpperThreshold),
                        "pathingLowerThreshold" => Ok(__Field::m_pathingLowerThreshold),
                        "numDeactivationFrequencyPasses" => {
                            Ok(__Field::m_numDeactivationFrequencyPasses)
                        }
                        "deactivationVelocityScaleSquare" => {
                            Ok(__Field::m_deactivationVelocityScaleSquare)
                        }
                        "minimumPathingVelocityScaleSquare" => {
                            Ok(__Field::m_minimumPathingVelocityScaleSquare)
                        }
                        "spikingVelocityScaleThresholdSquared" => {
                            Ok(__Field::m_spikingVelocityScaleThresholdSquared)
                        }
                        "minimumSpikingVelocityScaleSquared" => {
                            Ok(__Field::m_minimumSpikingVelocityScaleSquared)
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
            struct __hknpMotionPropertiesVisitor<'de> {
                marker: _serde::__private::PhantomData<hknpMotionProperties<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hknpMotionPropertiesVisitor<'de> {
                type Value = hknpMotionProperties<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hknpMotionProperties",
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
                    let mut m_isExclusive: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_flags: _serde::__private::Option<FlagsEnum> = _serde::__private::None;
                    let mut m_gravityFactor: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_timeFactor: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxLinearSpeed: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxAngularSpeed: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_linearDamping: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_angularDamping: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_solverStabilizationSpeedThreshold: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_solverStabilizationSpeedReduction: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_maxDistSqrd: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxRotSqrd: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_invBlockSize: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_pathingUpperThreshold: _serde::__private::Option<
                        I16<'de>,
                    > = _serde::__private::None;
                    let mut m_pathingLowerThreshold: _serde::__private::Option<
                        I16<'de>,
                    > = _serde::__private::None;
                    let mut m_numDeactivationFrequencyPasses: _serde::__private::Option<
                        U8<'de>,
                    > = _serde::__private::None;
                    let mut m_deactivationVelocityScaleSquare: _serde::__private::Option<
                        U8<'de>,
                    > = _serde::__private::None;
                    let mut m_minimumPathingVelocityScaleSquare: _serde::__private::Option<
                        U8<'de>,
                    > = _serde::__private::None;
                    let mut m_spikingVelocityScaleThresholdSquared: _serde::__private::Option<
                        U8<'de>,
                    > = _serde::__private::None;
                    let mut m_minimumSpikingVelocityScaleSquared: _serde::__private::Option<
                        U8<'de>,
                    > = _serde::__private::None;
                    for i in 0..20usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_isExclusive) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isExclusive",
                                        ),
                                    );
                                }
                                m_isExclusive = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_flags) {
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
                            2usize => {
                                if _serde::__private::Option::is_some(&m_gravityFactor) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "gravityFactor",
                                        ),
                                    );
                                }
                                m_gravityFactor = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_timeFactor) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "timeFactor",
                                        ),
                                    );
                                }
                                m_timeFactor = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_maxLinearSpeed) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxLinearSpeed",
                                        ),
                                    );
                                }
                                m_maxLinearSpeed = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_maxAngularSpeed) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxAngularSpeed",
                                        ),
                                    );
                                }
                                m_maxAngularSpeed = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_linearDamping) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "linearDamping",
                                        ),
                                    );
                                }
                                m_linearDamping = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_angularDamping) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "angularDamping",
                                        ),
                                    );
                                }
                                m_angularDamping = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_solverStabilizationSpeedThreshold,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "solverStabilizationSpeedThreshold",
                                        ),
                                    );
                                }
                                m_solverStabilizationSpeedThreshold = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(
                                    &m_solverStabilizationSpeedReduction,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "solverStabilizationSpeedReduction",
                                        ),
                                    );
                                }
                                m_solverStabilizationSpeedReduction = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_maxDistSqrd) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxDistSqrd",
                                        ),
                                    );
                                }
                                m_maxDistSqrd = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_maxRotSqrd) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxRotSqrd",
                                        ),
                                    );
                                }
                                m_maxRotSqrd = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_invBlockSize) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invBlockSize",
                                        ),
                                    );
                                }
                                m_invBlockSize = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(
                                    &m_pathingUpperThreshold,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pathingUpperThreshold",
                                        ),
                                    );
                                }
                                m_pathingUpperThreshold = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(
                                    &m_pathingLowerThreshold,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pathingLowerThreshold",
                                        ),
                                    );
                                }
                                m_pathingLowerThreshold = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(
                                    &m_numDeactivationFrequencyPasses,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numDeactivationFrequencyPasses",
                                        ),
                                    );
                                }
                                m_numDeactivationFrequencyPasses = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
                                if _serde::__private::Option::is_some(
                                    &m_deactivationVelocityScaleSquare,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deactivationVelocityScaleSquare",
                                        ),
                                    );
                                }
                                m_deactivationVelocityScaleSquare = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            17usize => {
                                if _serde::__private::Option::is_some(
                                    &m_minimumPathingVelocityScaleSquare,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minimumPathingVelocityScaleSquare",
                                        ),
                                    );
                                }
                                m_minimumPathingVelocityScaleSquare = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            18usize => {
                                if _serde::__private::Option::is_some(
                                    &m_spikingVelocityScaleThresholdSquared,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "spikingVelocityScaleThresholdSquared",
                                        ),
                                    );
                                }
                                m_spikingVelocityScaleThresholdSquared = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            19usize => {
                                if _serde::__private::Option::is_some(
                                    &m_minimumSpikingVelocityScaleSquared,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minimumSpikingVelocityScaleSquared",
                                        ),
                                    );
                                }
                                m_minimumSpikingVelocityScaleSquared = _serde::__private::Some(
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
                    __A::pad(&mut __map, 0usize, 3usize)?;
                    let m_isExclusive = match m_isExclusive {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isExclusive",
                                ),
                            );
                        }
                    };
                    let m_flags = match m_flags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("flags"),
                            );
                        }
                    };
                    let m_gravityFactor = match m_gravityFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "gravityFactor",
                                ),
                            );
                        }
                    };
                    let m_timeFactor = match m_timeFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "timeFactor",
                                ),
                            );
                        }
                    };
                    let m_maxLinearSpeed = match m_maxLinearSpeed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxLinearSpeed",
                                ),
                            );
                        }
                    };
                    let m_maxAngularSpeed = match m_maxAngularSpeed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxAngularSpeed",
                                ),
                            );
                        }
                    };
                    let m_linearDamping = match m_linearDamping {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "linearDamping",
                                ),
                            );
                        }
                    };
                    let m_angularDamping = match m_angularDamping {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "angularDamping",
                                ),
                            );
                        }
                    };
                    let m_solverStabilizationSpeedThreshold = match m_solverStabilizationSpeedThreshold {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "solverStabilizationSpeedThreshold",
                                ),
                            );
                        }
                    };
                    let m_solverStabilizationSpeedReduction = match m_solverStabilizationSpeedReduction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "solverStabilizationSpeedReduction",
                                ),
                            );
                        }
                    };
                    let m_maxDistSqrd = match m_maxDistSqrd {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxDistSqrd",
                                ),
                            );
                        }
                    };
                    let m_maxRotSqrd = match m_maxRotSqrd {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxRotSqrd",
                                ),
                            );
                        }
                    };
                    let m_invBlockSize = match m_invBlockSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "invBlockSize",
                                ),
                            );
                        }
                    };
                    let m_pathingUpperThreshold = match m_pathingUpperThreshold {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pathingUpperThreshold",
                                ),
                            );
                        }
                    };
                    let m_pathingLowerThreshold = match m_pathingLowerThreshold {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pathingLowerThreshold",
                                ),
                            );
                        }
                    };
                    let m_numDeactivationFrequencyPasses = match m_numDeactivationFrequencyPasses {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numDeactivationFrequencyPasses",
                                ),
                            );
                        }
                    };
                    let m_deactivationVelocityScaleSquare = match m_deactivationVelocityScaleSquare {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deactivationVelocityScaleSquare",
                                ),
                            );
                        }
                    };
                    let m_minimumPathingVelocityScaleSquare = match m_minimumPathingVelocityScaleSquare {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minimumPathingVelocityScaleSquare",
                                ),
                            );
                        }
                    };
                    let m_spikingVelocityScaleThresholdSquared = match m_spikingVelocityScaleThresholdSquared {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "spikingVelocityScaleThresholdSquared",
                                ),
                            );
                        }
                    };
                    let m_minimumSpikingVelocityScaleSquared = match m_minimumSpikingVelocityScaleSquared {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minimumSpikingVelocityScaleSquared",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hknpMotionProperties {
                        __ptr,
                        m_isExclusive,
                        m_flags,
                        m_gravityFactor,
                        m_timeFactor,
                        m_maxLinearSpeed,
                        m_maxAngularSpeed,
                        m_linearDamping,
                        m_angularDamping,
                        m_solverStabilizationSpeedThreshold,
                        m_solverStabilizationSpeedReduction,
                        m_maxDistSqrd,
                        m_maxRotSqrd,
                        m_invBlockSize,
                        m_pathingUpperThreshold,
                        m_pathingLowerThreshold,
                        m_numDeactivationFrequencyPasses,
                        m_deactivationVelocityScaleSquare,
                        m_minimumPathingVelocityScaleSquare,
                        m_spikingVelocityScaleThresholdSquared,
                        m_minimumSpikingVelocityScaleSquared,
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
                    let mut m_isExclusive: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_flags: _serde::__private::Option<FlagsEnum> = _serde::__private::None;
                    let mut m_gravityFactor: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_timeFactor: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxLinearSpeed: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxAngularSpeed: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_linearDamping: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_angularDamping: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_solverStabilizationSpeedThreshold: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_solverStabilizationSpeedReduction: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_maxDistSqrd: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxRotSqrd: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_invBlockSize: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_pathingUpperThreshold: _serde::__private::Option<
                        I16<'de>,
                    > = _serde::__private::None;
                    let mut m_pathingLowerThreshold: _serde::__private::Option<
                        I16<'de>,
                    > = _serde::__private::None;
                    let mut m_numDeactivationFrequencyPasses: _serde::__private::Option<
                        U8<'de>,
                    > = _serde::__private::None;
                    let mut m_deactivationVelocityScaleSquare: _serde::__private::Option<
                        U8<'de>,
                    > = _serde::__private::None;
                    let mut m_minimumPathingVelocityScaleSquare: _serde::__private::Option<
                        U8<'de>,
                    > = _serde::__private::None;
                    let mut m_spikingVelocityScaleThresholdSquared: _serde::__private::Option<
                        U8<'de>,
                    > = _serde::__private::None;
                    let mut m_minimumSpikingVelocityScaleSquared: _serde::__private::Option<
                        U8<'de>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_isExclusive => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_isExclusive) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isExclusive",
                                        ),
                                    );
                                }
                                m_isExclusive = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
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
                            __Field::m_gravityFactor => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_gravityFactor) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "gravityFactor",
                                        ),
                                    );
                                }
                                m_gravityFactor = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_timeFactor => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_timeFactor) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "timeFactor",
                                        ),
                                    );
                                }
                                m_timeFactor = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxLinearSpeed => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_maxLinearSpeed) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxLinearSpeed",
                                        ),
                                    );
                                }
                                m_maxLinearSpeed = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxAngularSpeed => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_maxAngularSpeed) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxAngularSpeed",
                                        ),
                                    );
                                }
                                m_maxAngularSpeed = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_linearDamping => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_linearDamping) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "linearDamping",
                                        ),
                                    );
                                }
                                m_linearDamping = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_angularDamping => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_angularDamping) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "angularDamping",
                                        ),
                                    );
                                }
                                m_angularDamping = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_solverStabilizationSpeedThreshold => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_solverStabilizationSpeedThreshold,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "solverStabilizationSpeedThreshold",
                                        ),
                                    );
                                }
                                m_solverStabilizationSpeedThreshold = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_solverStabilizationSpeedReduction => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_solverStabilizationSpeedReduction,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "solverStabilizationSpeedReduction",
                                        ),
                                    );
                                }
                                m_solverStabilizationSpeedReduction = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxDistSqrd => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_maxDistSqrd) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxDistSqrd",
                                        ),
                                    );
                                }
                                m_maxDistSqrd = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxRotSqrd => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_maxRotSqrd) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxRotSqrd",
                                        ),
                                    );
                                }
                                m_maxRotSqrd = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_invBlockSize => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_invBlockSize) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invBlockSize",
                                        ),
                                    );
                                }
                                m_invBlockSize = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_pathingUpperThreshold => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_pathingUpperThreshold,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pathingUpperThreshold",
                                        ),
                                    );
                                }
                                m_pathingUpperThreshold = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_pathingLowerThreshold => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_pathingLowerThreshold,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pathingLowerThreshold",
                                        ),
                                    );
                                }
                                m_pathingLowerThreshold = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numDeactivationFrequencyPasses => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_numDeactivationFrequencyPasses,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numDeactivationFrequencyPasses",
                                        ),
                                    );
                                }
                                m_numDeactivationFrequencyPasses = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_deactivationVelocityScaleSquare => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_deactivationVelocityScaleSquare,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deactivationVelocityScaleSquare",
                                        ),
                                    );
                                }
                                m_deactivationVelocityScaleSquare = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_minimumPathingVelocityScaleSquare => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_minimumPathingVelocityScaleSquare,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minimumPathingVelocityScaleSquare",
                                        ),
                                    );
                                }
                                m_minimumPathingVelocityScaleSquare = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_spikingVelocityScaleThresholdSquared => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_spikingVelocityScaleThresholdSquared,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "spikingVelocityScaleThresholdSquared",
                                        ),
                                    );
                                }
                                m_spikingVelocityScaleThresholdSquared = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_minimumSpikingVelocityScaleSquared => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_minimumSpikingVelocityScaleSquared,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "minimumSpikingVelocityScaleSquared",
                                        ),
                                    );
                                }
                                m_minimumSpikingVelocityScaleSquared = _serde::__private::Some(
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
                    let m_isExclusive = match m_isExclusive {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isExclusive",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
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
                    let m_gravityFactor = match m_gravityFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "gravityFactor",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_timeFactor = match m_timeFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "timeFactor",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxLinearSpeed = match m_maxLinearSpeed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxLinearSpeed",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxAngularSpeed = match m_maxAngularSpeed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxAngularSpeed",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_linearDamping = match m_linearDamping {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "linearDamping",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_angularDamping = match m_angularDamping {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "angularDamping",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_solverStabilizationSpeedThreshold = match m_solverStabilizationSpeedThreshold {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "solverStabilizationSpeedThreshold",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_solverStabilizationSpeedReduction = match m_solverStabilizationSpeedReduction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "solverStabilizationSpeedReduction",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxDistSqrd = match m_maxDistSqrd {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxDistSqrd",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxRotSqrd = match m_maxRotSqrd {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxRotSqrd",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_invBlockSize = match m_invBlockSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "invBlockSize",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_pathingUpperThreshold = match m_pathingUpperThreshold {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pathingUpperThreshold",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_pathingLowerThreshold = match m_pathingLowerThreshold {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pathingLowerThreshold",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numDeactivationFrequencyPasses = match m_numDeactivationFrequencyPasses {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numDeactivationFrequencyPasses",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_deactivationVelocityScaleSquare = match m_deactivationVelocityScaleSquare {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deactivationVelocityScaleSquare",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_minimumPathingVelocityScaleSquare = match m_minimumPathingVelocityScaleSquare {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minimumPathingVelocityScaleSquare",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_spikingVelocityScaleThresholdSquared = match m_spikingVelocityScaleThresholdSquared {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "spikingVelocityScaleThresholdSquared",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_minimumSpikingVelocityScaleSquared = match m_minimumSpikingVelocityScaleSquared {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "minimumSpikingVelocityScaleSquared",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hknpMotionProperties {
                        __ptr: __ptr.clone(),
                        m_isExclusive,
                        m_flags,
                        m_gravityFactor,
                        m_timeFactor,
                        m_maxLinearSpeed,
                        m_maxAngularSpeed,
                        m_linearDamping,
                        m_angularDamping,
                        m_solverStabilizationSpeedThreshold,
                        m_solverStabilizationSpeedReduction,
                        m_maxDistSqrd,
                        m_maxRotSqrd,
                        m_invBlockSize,
                        m_pathingUpperThreshold,
                        m_pathingLowerThreshold,
                        m_numDeactivationFrequencyPasses,
                        m_deactivationVelocityScaleSquare,
                        m_minimumPathingVelocityScaleSquare,
                        m_spikingVelocityScaleThresholdSquared,
                        m_minimumSpikingVelocityScaleSquared,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "isExclusive",
                "flags",
                "gravityFactor",
                "timeFactor",
                "maxLinearSpeed",
                "maxAngularSpeed",
                "linearDamping",
                "angularDamping",
                "solverStabilizationSpeedThreshold",
                "solverStabilizationSpeedReduction",
                "maxDistSqrd",
                "maxRotSqrd",
                "invBlockSize",
                "pathingUpperThreshold",
                "pathingLowerThreshold",
                "numDeactivationFrequencyPasses",
                "deactivationVelocityScaleSquare",
                "minimumPathingVelocityScaleSquare",
                "spikingVelocityScaleThresholdSquared",
                "minimumSpikingVelocityScaleSquared",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hknpMotionProperties",
                FIELDS,
                __hknpMotionPropertiesVisitor {
                    marker: _serde::__private::PhantomData::<hknpMotionProperties>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[havok_types_derive::impl_flags_methods]
bitflags::bitflags! {
    #[doc = r" Bit flags"] #[doc = r""] #[doc = r" # C++ Info"] #[doc =
    " - name: `FlagsEnum`(ctype: `hkFlags<FlagsEnum, hkUint32>`)"]
    #[allow(non_upper_case_globals, non_snake_case)] #[cfg_attr(feature = "serde",
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]
    #[repr(transparent)] #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct
    FlagsEnum : u32 { #[doc = "2"] const NEVER_REBUILD_MASS_PROPERTIES = 2u32; #[doc =
    "536870912"] const ENABLE_GRAVITY_MODIFICATION = 536870912u32; #[doc = "1073741824"]
    const ENABLE_TIME_FACTOR = 1073741824u32; #[doc = "-536870912"] const FLAGS_MASK =
    3758096384u32; #[doc = "66060288"] const AUTO_FLAGS_MASK = 66060288u32; }
}
#[cfg(feature = "json_schema")]
const _: () = {
    use schemars::{SchemaGenerator, Schema, JsonSchema, json_schema};
    use std::borrow::Cow;
    impl JsonSchema for FlagsEnum {
        fn schema_name() -> Cow<'static, str> {
            "FlagsEnum".into()
        }
        fn schema_id() -> Cow<'static, str> {
            concat!(module_path!(), "::", "FlagsEnum").into()
        }
        fn json_schema(_generate: &mut SchemaGenerator) -> Schema {
            json_schema!(
                { "description" :
                "Bitflags field. Specific flags: NEVER_REBUILD_MASS_PROPERTIES: 2, ENABLE_GRAVITY_MODIFICATION: 536870912, ENABLE_TIME_FACTOR: 1073741824, FLAGS_MASK: -536870912, AUTO_FLAGS_MASK: 66060288. Additional unspecified bits may be set.(e.g.: BIT_FLAG|BIT_FLAG2|4)",
                "type" : "string", }
            )
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for FlagsEnum {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            if self.is_empty() {
                __serializer.serialize_bits(&self.bits())?;
                __serializer.serialize_empty_bit()?;
                return __serializer.end();
            }
            for flag in self.iter() {
                match flag {
                    Self::NEVER_REBUILD_MASS_PROPERTIES => {
                        __serializer
                            .serialize_field(
                                "NEVER_REBUILD_MASS_PROPERTIES",
                                &Self::NEVER_REBUILD_MASS_PROPERTIES,
                            )
                    }
                    Self::ENABLE_GRAVITY_MODIFICATION => {
                        __serializer
                            .serialize_field(
                                "ENABLE_GRAVITY_MODIFICATION",
                                &Self::ENABLE_GRAVITY_MODIFICATION,
                            )
                    }
                    Self::ENABLE_TIME_FACTOR => {
                        __serializer
                            .serialize_field(
                                "ENABLE_TIME_FACTOR",
                                &Self::ENABLE_TIME_FACTOR,
                            )
                    }
                    Self::FLAGS_MASK => {
                        __serializer.serialize_field("FLAGS_MASK", &Self::FLAGS_MASK)
                    }
                    Self::AUTO_FLAGS_MASK => {
                        __serializer
                            .serialize_field("AUTO_FLAGS_MASK", &Self::AUTO_FLAGS_MASK)
                    }
                    remain => {
                        __serializer
                            .serialize_field(&remain.bits().to_string(), &remain.bits())
                    }
                }?;
            }
            __serializer.serialize_bits(&self.bits())?;
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
    impl<'de> _serde::Deserialize<'de> for FlagsEnum {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<FlagsEnum>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = FlagsEnum;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct FlagsEnum(flags)",
                    )
                }
                #[inline]
                fn visit_uint32<__E>(
                    self,
                    __value: U32<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        U32::Number(__value) => {
                            Ok(FlagsEnum::from_bits_retain(__value as _))
                        }
                        _ => {
                            Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint32(__value as _),
                                    &"FlagsEnum(U32) Number",
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
                    match <FlagsEnum as core::str::FromStr>::from_str(
                        __value.into_inner().unwrap().as_ref(),
                    ) {
                        Ok(flags) => Ok(flags),
                        Err(err) => Err(_serde::de::Error::custom(err)),
                    }
                }
            }
            _serde::Deserializer::deserialize_flags(
                __deserializer,
                _serde::de::ReadEnumSize::Uint32,
                __Visitor {
                    marker: _serde::__private::PhantomData::<FlagsEnum>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
