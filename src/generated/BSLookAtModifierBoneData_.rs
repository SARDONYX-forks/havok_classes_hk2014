use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `BSLookAtModifierBoneData`
/// - version: `2`
/// - signature: `0xf3b4d9be`
/// - size: `  0`(x86)/`528`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSLookAtModifierBoneData<'a> {
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
    /// - name: `index`(ctype: `hkInt16`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "index"))]
    #[cfg_attr(feature = "serde", serde(rename = "index"))]
    pub m_index: I16<'a>,
    /// # C++ Info
    /// - name: `fwdAxisLS`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "fwdAxisLS"))]
    #[cfg_attr(feature = "serde", serde(rename = "fwdAxisLS"))]
    pub m_fwdAxisLS: Vector4,
    /// # C++ Info
    /// - name: `upAxisLS`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "upAxisLS"))]
    #[cfg_attr(feature = "serde", serde(rename = "upAxisLS"))]
    pub m_upAxisLS: Vector4,
    /// # C++ Info
    /// - name: `specifyAxesByColor`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "specifyAxesByColor"))]
    #[cfg_attr(feature = "serde", serde(rename = "specifyAxesByColor"))]
    pub m_specifyAxesByColor: bool,
    /// # C++ Info
    /// - name: `forwardAxisColor`(ctype: `enum AXIS_COLOR`)
    /// - offset: `  0`(x86)/` 49`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "forwardAxisColor"))]
    #[cfg_attr(feature = "serde", serde(rename = "forwardAxisColor"))]
    pub m_forwardAxisColor: AXIS_COLOR,
    /// # C++ Info
    /// - name: `upAxisColor`(ctype: `enum AXIS_COLOR`)
    /// - offset: `  0`(x86)/` 50`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "upAxisColor"))]
    #[cfg_attr(feature = "serde", serde(rename = "upAxisColor"))]
    pub m_upAxisColor: AXIS_COLOR,
    /// # C++ Info
    /// - name: `limitAngleDegrees`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 52`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "limitAngleDegrees"))]
    #[cfg_attr(feature = "serde", serde(rename = "limitAngleDegrees"))]
    pub m_limitAngleDegrees: f32,
    /// # C++ Info
    /// - name: `limitAngleDegreesLeft`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 56`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "limitAngleDegreesLeft"))]
    #[cfg_attr(feature = "serde", serde(rename = "limitAngleDegreesLeft"))]
    pub m_limitAngleDegreesLeft: f32,
    /// # C++ Info
    /// - name: `limitAngleDegreesRight`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 60`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "limitAngleDegreesRight"))]
    #[cfg_attr(feature = "serde", serde(rename = "limitAngleDegreesRight"))]
    pub m_limitAngleDegreesRight: f32,
    /// # C++ Info
    /// - name: `limitAngleDegreesUp`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "limitAngleDegreesUp"))]
    #[cfg_attr(feature = "serde", serde(rename = "limitAngleDegreesUp"))]
    pub m_limitAngleDegreesUp: f32,
    /// # C++ Info
    /// - name: `limitAngleDegreesDown`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 68`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "limitAngleDegreesDown"))]
    #[cfg_attr(feature = "serde", serde(rename = "limitAngleDegreesDown"))]
    pub m_limitAngleDegreesDown: f32,
    /// # C++ Info
    /// - name: `onGain`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 72`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "onGain"))]
    #[cfg_attr(feature = "serde", serde(rename = "onGain"))]
    pub m_onGain: f32,
    /// # C++ Info
    /// - name: `offGain`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 76`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "offGain"))]
    #[cfg_attr(feature = "serde", serde(rename = "offGain"))]
    pub m_offGain: f32,
    /// # C++ Info
    /// - name: `onLeadIn`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 80`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "onLeadIn"))]
    #[cfg_attr(feature = "serde", serde(rename = "onLeadIn"))]
    pub m_onLeadIn: f32,
    /// # C++ Info
    /// - name: `offLeadIn`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 84`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "offLeadIn"))]
    #[cfg_attr(feature = "serde", serde(rename = "offLeadIn"))]
    pub m_offLeadIn: f32,
    /// # C++ Info
    /// - name: `rotateAboutAxes`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 88`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "rotateAboutAxes"))]
    #[cfg_attr(feature = "serde", serde(rename = "rotateAboutAxes"))]
    pub m_rotateAboutAxes: bool,
    /// # C++ Info
    /// - name: `enabled`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 89`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "enabled"))]
    #[cfg_attr(feature = "serde", serde(rename = "enabled"))]
    pub m_enabled: bool,
    /// # C++ Info
    /// - name: `drawDebugInfo`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 90`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "drawDebugInfo"))]
    #[cfg_attr(feature = "serde", serde(rename = "drawDebugInfo"))]
    pub m_drawDebugInfo: bool,
    /// # C++ Info
    /// - name: `axisFwdLS`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/` 96`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "axisFwdLS"))]
    #[cfg_attr(feature = "serde", serde(rename = "axisFwdLS"))]
    pub m_axisFwdLS: Vector4,
    /// # C++ Info
    /// - name: `axisUpLS`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`112`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "axisUpLS"))]
    #[cfg_attr(feature = "serde", serde(rename = "axisUpLS"))]
    pub m_axisUpLS: Vector4,
    /// # C++ Info
    /// - name: `axisSideLS`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`128`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "axisSideLS"))]
    #[cfg_attr(feature = "serde", serde(rename = "axisSideLS"))]
    pub m_axisSideLS: Vector4,
    /// # C++ Info
    /// - name: `currentFwdAxisLS`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`144`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "currentFwdAxisLS"))]
    #[cfg_attr(feature = "serde", serde(rename = "currentFwdAxisLS"))]
    pub m_currentFwdAxisLS: Vector4,
    /// # C++ Info
    /// - name: `currentRotMS`(ctype: `hkQuaternion`)
    /// - offset: `  0`(x86)/`160`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "currentRotMS"))]
    #[cfg_attr(feature = "serde", serde(rename = "currentRotMS"))]
    pub m_currentRotMS: Quaternion,
    /// # C++ Info
    /// - name: `currentGain`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`176`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "currentGain"))]
    #[cfg_attr(feature = "serde", serde(rename = "currentGain"))]
    pub m_currentGain: f32,
    /// # C++ Info
    /// - name: `defaultRefTransformMS`(ctype: `hkQsTransform`)
    /// - offset: `  0`(x86)/`192`(x86_64)
    /// - type_size: `  0`(x86)/` 48`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "defaultRefTransformMS"))]
    #[cfg_attr(feature = "serde", serde(rename = "defaultRefTransformMS"))]
    pub m_defaultRefTransformMS: QsTransform,
    /// # C++ Info
    /// - name: `defaultRefTransformLS`(ctype: `hkQsTransform`)
    /// - offset: `  0`(x86)/`240`(x86_64)
    /// - type_size: `  0`(x86)/` 48`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "defaultRefTransformLS"))]
    #[cfg_attr(feature = "serde", serde(rename = "defaultRefTransformLS"))]
    pub m_defaultRefTransformLS: QsTransform,
    /// # C++ Info
    /// - name: `defaultOffsetFwd`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`288`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "defaultOffsetFwd"))]
    #[cfg_attr(feature = "serde", serde(rename = "defaultOffsetFwd"))]
    pub m_defaultOffsetFwd: f32,
    /// # C++ Info
    /// - name: `defaultOffsetVert`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`292`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "defaultOffsetVert"))]
    #[cfg_attr(feature = "serde", serde(rename = "defaultOffsetVert"))]
    pub m_defaultOffsetVert: f32,
    /// # C++ Info
    /// - name: `customRefTransformMS`(ctype: `hkQsTransform`)
    /// - offset: `  0`(x86)/`304`(x86_64)
    /// - type_size: `  0`(x86)/` 48`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "customRefTransformMS"))]
    #[cfg_attr(feature = "serde", serde(rename = "customRefTransformMS"))]
    pub m_customRefTransformMS: QsTransform,
    /// # C++ Info
    /// - name: `customRefTransformLS`(ctype: `hkQsTransform`)
    /// - offset: `  0`(x86)/`352`(x86_64)
    /// - type_size: `  0`(x86)/` 48`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "customRefTransformLS"))]
    #[cfg_attr(feature = "serde", serde(rename = "customRefTransformLS"))]
    pub m_customRefTransformLS: QsTransform,
    /// # C++ Info
    /// - name: `customOffsetFwd`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`400`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "customOffsetFwd"))]
    #[cfg_attr(feature = "serde", serde(rename = "customOffsetFwd"))]
    pub m_customOffsetFwd: f32,
    /// # C++ Info
    /// - name: `customOffsetVert`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`404`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "customOffsetVert"))]
    #[cfg_attr(feature = "serde", serde(rename = "customOffsetVert"))]
    pub m_customOffsetVert: f32,
    /// # C++ Info
    /// - name: `currentRefTransformMS`(ctype: `hkQsTransform`)
    /// - offset: `  0`(x86)/`416`(x86_64)
    /// - type_size: `  0`(x86)/` 48`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "currentRefTransformMS"))]
    #[cfg_attr(feature = "serde", serde(rename = "currentRefTransformMS"))]
    pub m_currentRefTransformMS: QsTransform,
    /// # C++ Info
    /// - name: `currentRefTransformLS`(ctype: `hkQsTransform`)
    /// - offset: `  0`(x86)/`464`(x86_64)
    /// - type_size: `  0`(x86)/` 48`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "currentRefTransformLS"))]
    #[cfg_attr(feature = "serde", serde(rename = "currentRefTransformLS"))]
    pub m_currentRefTransformLS: QsTransform,
    /// # C++ Info
    /// - name: `currentOffsetFwd`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`512`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "currentOffsetFwd"))]
    #[cfg_attr(feature = "serde", serde(rename = "currentOffsetFwd"))]
    pub m_currentOffsetFwd: f32,
    /// # C++ Info
    /// - name: `currentOffsetVert`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`516`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "currentOffsetVert"))]
    #[cfg_attr(feature = "serde", serde(rename = "currentOffsetVert"))]
    pub m_currentOffsetVert: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSLookAtModifierBoneData<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSLookAtModifierBoneData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf3b4d9be)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for BSLookAtModifierBoneData<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xf3b4d9be)));
            let mut serializer = __serializer
                .serialize_struct(
                    "BSLookAtModifierBoneData",
                    class_meta,
                    (0u64, 528u64),
                )?;
            serializer.serialize_field("index", &self.m_index)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer.serialize_field("fwdAxisLS", &self.m_fwdAxisLS)?;
            serializer.serialize_field("upAxisLS", &self.m_upAxisLS)?;
            serializer
                .serialize_field("specifyAxesByColor", &self.m_specifyAxesByColor)?;
            serializer.serialize_field("forwardAxisColor", &self.m_forwardAxisColor)?;
            serializer.serialize_field("upAxisColor", &self.m_upAxisColor)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("limitAngleDegrees", &self.m_limitAngleDegrees)?;
            serializer
                .serialize_field(
                    "limitAngleDegreesLeft",
                    &self.m_limitAngleDegreesLeft,
                )?;
            serializer
                .serialize_field(
                    "limitAngleDegreesRight",
                    &self.m_limitAngleDegreesRight,
                )?;
            serializer
                .serialize_field("limitAngleDegreesUp", &self.m_limitAngleDegreesUp)?;
            serializer
                .serialize_field(
                    "limitAngleDegreesDown",
                    &self.m_limitAngleDegreesDown,
                )?;
            serializer.serialize_field("onGain", &self.m_onGain)?;
            serializer.serialize_field("offGain", &self.m_offGain)?;
            serializer.serialize_field("onLeadIn", &self.m_onLeadIn)?;
            serializer.serialize_field("offLeadIn", &self.m_offLeadIn)?;
            serializer.serialize_field("rotateAboutAxes", &self.m_rotateAboutAxes)?;
            serializer.serialize_field("enabled", &self.m_enabled)?;
            serializer.serialize_field("drawDebugInfo", &self.m_drawDebugInfo)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer.skip_field("axisFwdLS", &self.m_axisFwdLS)?;
            serializer.skip_field("axisUpLS", &self.m_axisUpLS)?;
            serializer.skip_field("axisSideLS", &self.m_axisSideLS)?;
            serializer.skip_field("currentFwdAxisLS", &self.m_currentFwdAxisLS)?;
            serializer.skip_field("currentRotMS", &self.m_currentRotMS)?;
            serializer.skip_field("currentGain", &self.m_currentGain)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer
                .skip_field("defaultRefTransformMS", &self.m_defaultRefTransformMS)?;
            serializer
                .skip_field("defaultRefTransformLS", &self.m_defaultRefTransformLS)?;
            serializer.skip_field("defaultOffsetFwd", &self.m_defaultOffsetFwd)?;
            serializer.skip_field("defaultOffsetVert", &self.m_defaultOffsetVert)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("customRefTransformMS", &self.m_customRefTransformMS)?;
            serializer.skip_field("customRefTransformLS", &self.m_customRefTransformLS)?;
            serializer.skip_field("customOffsetFwd", &self.m_customOffsetFwd)?;
            serializer.skip_field("customOffsetVert", &self.m_customOffsetVert)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("currentRefTransformMS", &self.m_currentRefTransformMS)?;
            serializer
                .skip_field("currentRefTransformLS", &self.m_currentRefTransformLS)?;
            serializer.skip_field("currentOffsetFwd", &self.m_currentOffsetFwd)?;
            serializer.skip_field("currentOffsetVert", &self.m_currentOffsetVert)?;
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
    impl<'de> _serde::Deserialize<'de> for BSLookAtModifierBoneData<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_index,
                m_fwdAxisLS,
                m_upAxisLS,
                m_specifyAxesByColor,
                m_forwardAxisColor,
                m_upAxisColor,
                m_limitAngleDegrees,
                m_limitAngleDegreesLeft,
                m_limitAngleDegreesRight,
                m_limitAngleDegreesUp,
                m_limitAngleDegreesDown,
                m_onGain,
                m_offGain,
                m_onLeadIn,
                m_offLeadIn,
                m_rotateAboutAxes,
                m_enabled,
                m_drawDebugInfo,
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
                        "index" => Ok(__Field::m_index),
                        "fwdAxisLS" => Ok(__Field::m_fwdAxisLS),
                        "upAxisLS" => Ok(__Field::m_upAxisLS),
                        "specifyAxesByColor" => Ok(__Field::m_specifyAxesByColor),
                        "forwardAxisColor" => Ok(__Field::m_forwardAxisColor),
                        "upAxisColor" => Ok(__Field::m_upAxisColor),
                        "limitAngleDegrees" => Ok(__Field::m_limitAngleDegrees),
                        "limitAngleDegreesLeft" => Ok(__Field::m_limitAngleDegreesLeft),
                        "limitAngleDegreesRight" => Ok(__Field::m_limitAngleDegreesRight),
                        "limitAngleDegreesUp" => Ok(__Field::m_limitAngleDegreesUp),
                        "limitAngleDegreesDown" => Ok(__Field::m_limitAngleDegreesDown),
                        "onGain" => Ok(__Field::m_onGain),
                        "offGain" => Ok(__Field::m_offGain),
                        "onLeadIn" => Ok(__Field::m_onLeadIn),
                        "offLeadIn" => Ok(__Field::m_offLeadIn),
                        "rotateAboutAxes" => Ok(__Field::m_rotateAboutAxes),
                        "enabled" => Ok(__Field::m_enabled),
                        "drawDebugInfo" => Ok(__Field::m_drawDebugInfo),
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
            struct __BSLookAtModifierBoneDataVisitor<'de> {
                marker: _serde::__private::PhantomData<BSLookAtModifierBoneData<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __BSLookAtModifierBoneDataVisitor<'de> {
                type Value = BSLookAtModifierBoneData<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct BSLookAtModifierBoneData",
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
                    let mut m_index: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_fwdAxisLS: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_upAxisLS: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_specifyAxesByColor: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_forwardAxisColor: _serde::__private::Option<AXIS_COLOR> = _serde::__private::None;
                    let mut m_upAxisColor: _serde::__private::Option<AXIS_COLOR> = _serde::__private::None;
                    let mut m_limitAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_limitAngleDegreesLeft: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_limitAngleDegreesRight: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_limitAngleDegreesUp: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_limitAngleDegreesDown: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_onGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_offGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_onLeadIn: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_offLeadIn: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_rotateAboutAxes: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_enabled: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_drawDebugInfo: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_axisFwdLS: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_axisUpLS: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_axisSideLS: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_currentFwdAxisLS: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_currentRotMS: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_currentGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_defaultRefTransformMS: _serde::__private::Option<
                        QsTransform,
                    > = _serde::__private::None;
                    let mut m_defaultRefTransformLS: _serde::__private::Option<
                        QsTransform,
                    > = _serde::__private::None;
                    let mut m_defaultOffsetFwd: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_defaultOffsetVert: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_customRefTransformMS: _serde::__private::Option<
                        QsTransform,
                    > = _serde::__private::None;
                    let mut m_customRefTransformLS: _serde::__private::Option<
                        QsTransform,
                    > = _serde::__private::None;
                    let mut m_customOffsetFwd: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_customOffsetVert: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_currentRefTransformMS: _serde::__private::Option<
                        QsTransform,
                    > = _serde::__private::None;
                    let mut m_currentRefTransformLS: _serde::__private::Option<
                        QsTransform,
                    > = _serde::__private::None;
                    let mut m_currentOffsetFwd: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_currentOffsetVert: _serde::__private::Option<f32> = _serde::__private::None;
                    for i in 0..36usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_index) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("index"),
                                    );
                                }
                                m_index = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_fwdAxisLS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fwdAxisLS",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 14usize)?;
                                m_fwdAxisLS = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_upAxisLS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "upAxisLS",
                                        ),
                                    );
                                }
                                m_upAxisLS = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_specifyAxesByColor,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "specifyAxesByColor",
                                        ),
                                    );
                                }
                                m_specifyAxesByColor = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_forwardAxisColor) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "forwardAxisColor",
                                        ),
                                    );
                                }
                                m_forwardAxisColor = _serde::__private::Some(
                                    match __A::next_value::<AXIS_COLOR>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_upAxisColor) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "upAxisColor",
                                        ),
                                    );
                                }
                                m_upAxisColor = _serde::__private::Some(
                                    match __A::next_value::<AXIS_COLOR>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_limitAngleDegrees,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "limitAngleDegrees",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 1usize)?;
                                m_limitAngleDegrees = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_limitAngleDegreesLeft,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "limitAngleDegreesLeft",
                                        ),
                                    );
                                }
                                m_limitAngleDegreesLeft = _serde::__private::Some(
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
                                    &m_limitAngleDegreesRight,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "limitAngleDegreesRight",
                                        ),
                                    );
                                }
                                m_limitAngleDegreesRight = _serde::__private::Some(
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
                                    &m_limitAngleDegreesUp,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "limitAngleDegreesUp",
                                        ),
                                    );
                                }
                                m_limitAngleDegreesUp = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(
                                    &m_limitAngleDegreesDown,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "limitAngleDegreesDown",
                                        ),
                                    );
                                }
                                m_limitAngleDegreesDown = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_onGain) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("onGain"),
                                    );
                                }
                                m_onGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_offGain) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "offGain",
                                        ),
                                    );
                                }
                                m_offGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(&m_onLeadIn) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "onLeadIn",
                                        ),
                                    );
                                }
                                m_onLeadIn = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(&m_offLeadIn) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "offLeadIn",
                                        ),
                                    );
                                }
                                m_offLeadIn = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(&m_rotateAboutAxes) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rotateAboutAxes",
                                        ),
                                    );
                                }
                                m_rotateAboutAxes = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
                                if _serde::__private::Option::is_some(&m_enabled) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enabled",
                                        ),
                                    );
                                }
                                m_enabled = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            17usize => {
                                if _serde::__private::Option::is_some(&m_drawDebugInfo) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "drawDebugInfo",
                                        ),
                                    );
                                }
                                m_drawDebugInfo = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            18usize => {
                                if _serde::__private::Option::is_some(&m_axisFwdLS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "axisFwdLS",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 5usize)?;
                                m_axisFwdLS = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            19usize => {
                                if _serde::__private::Option::is_some(&m_axisUpLS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "axisUpLS",
                                        ),
                                    );
                                }
                                m_axisUpLS = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            20usize => {
                                if _serde::__private::Option::is_some(&m_axisSideLS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "axisSideLS",
                                        ),
                                    );
                                }
                                m_axisSideLS = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            21usize => {
                                if _serde::__private::Option::is_some(&m_currentFwdAxisLS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentFwdAxisLS",
                                        ),
                                    );
                                }
                                m_currentFwdAxisLS = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            22usize => {
                                if _serde::__private::Option::is_some(&m_currentRotMS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentRotMS",
                                        ),
                                    );
                                }
                                m_currentRotMS = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            23usize => {
                                if _serde::__private::Option::is_some(&m_currentGain) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentGain",
                                        ),
                                    );
                                }
                                m_currentGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            24usize => {
                                if _serde::__private::Option::is_some(
                                    &m_defaultRefTransformMS,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "defaultRefTransformMS",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 12usize)?;
                                m_defaultRefTransformMS = _serde::__private::Some(
                                    match __A::next_value::<QsTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            25usize => {
                                if _serde::__private::Option::is_some(
                                    &m_defaultRefTransformLS,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "defaultRefTransformLS",
                                        ),
                                    );
                                }
                                m_defaultRefTransformLS = _serde::__private::Some(
                                    match __A::next_value::<QsTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            26usize => {
                                if _serde::__private::Option::is_some(&m_defaultOffsetFwd) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "defaultOffsetFwd",
                                        ),
                                    );
                                }
                                m_defaultOffsetFwd = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            27usize => {
                                if _serde::__private::Option::is_some(
                                    &m_defaultOffsetVert,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "defaultOffsetVert",
                                        ),
                                    );
                                }
                                m_defaultOffsetVert = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            28usize => {
                                if _serde::__private::Option::is_some(
                                    &m_customRefTransformMS,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "customRefTransformMS",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 8usize)?;
                                m_customRefTransformMS = _serde::__private::Some(
                                    match __A::next_value::<QsTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            29usize => {
                                if _serde::__private::Option::is_some(
                                    &m_customRefTransformLS,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "customRefTransformLS",
                                        ),
                                    );
                                }
                                m_customRefTransformLS = _serde::__private::Some(
                                    match __A::next_value::<QsTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            30usize => {
                                if _serde::__private::Option::is_some(&m_customOffsetFwd) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "customOffsetFwd",
                                        ),
                                    );
                                }
                                m_customOffsetFwd = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            31usize => {
                                if _serde::__private::Option::is_some(&m_customOffsetVert) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "customOffsetVert",
                                        ),
                                    );
                                }
                                m_customOffsetVert = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            32usize => {
                                if _serde::__private::Option::is_some(
                                    &m_currentRefTransformMS,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentRefTransformMS",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 8usize)?;
                                m_currentRefTransformMS = _serde::__private::Some(
                                    match __A::next_value::<QsTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            33usize => {
                                if _serde::__private::Option::is_some(
                                    &m_currentRefTransformLS,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentRefTransformLS",
                                        ),
                                    );
                                }
                                m_currentRefTransformLS = _serde::__private::Some(
                                    match __A::next_value::<QsTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            34usize => {
                                if _serde::__private::Option::is_some(&m_currentOffsetFwd) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentOffsetFwd",
                                        ),
                                    );
                                }
                                m_currentOffsetFwd = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            35usize => {
                                if _serde::__private::Option::is_some(
                                    &m_currentOffsetVert,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentOffsetVert",
                                        ),
                                    );
                                }
                                m_currentOffsetVert = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
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
                    let m_index = match m_index {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("index"),
                            );
                        }
                    };
                    let m_fwdAxisLS = match m_fwdAxisLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fwdAxisLS",
                                ),
                            );
                        }
                    };
                    let m_upAxisLS = match m_upAxisLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("upAxisLS"),
                            );
                        }
                    };
                    let m_specifyAxesByColor = match m_specifyAxesByColor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "specifyAxesByColor",
                                ),
                            );
                        }
                    };
                    let m_forwardAxisColor = match m_forwardAxisColor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "forwardAxisColor",
                                ),
                            );
                        }
                    };
                    let m_upAxisColor = match m_upAxisColor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "upAxisColor",
                                ),
                            );
                        }
                    };
                    let m_limitAngleDegrees = match m_limitAngleDegrees {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitAngleDegrees",
                                ),
                            );
                        }
                    };
                    let m_limitAngleDegreesLeft = match m_limitAngleDegreesLeft {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitAngleDegreesLeft",
                                ),
                            );
                        }
                    };
                    let m_limitAngleDegreesRight = match m_limitAngleDegreesRight {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitAngleDegreesRight",
                                ),
                            );
                        }
                    };
                    let m_limitAngleDegreesUp = match m_limitAngleDegreesUp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitAngleDegreesUp",
                                ),
                            );
                        }
                    };
                    let m_limitAngleDegreesDown = match m_limitAngleDegreesDown {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitAngleDegreesDown",
                                ),
                            );
                        }
                    };
                    let m_onGain = match m_onGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("onGain"),
                            );
                        }
                    };
                    let m_offGain = match m_offGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("offGain"),
                            );
                        }
                    };
                    let m_onLeadIn = match m_onLeadIn {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("onLeadIn"),
                            );
                        }
                    };
                    let m_offLeadIn = match m_offLeadIn {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "offLeadIn",
                                ),
                            );
                        }
                    };
                    let m_rotateAboutAxes = match m_rotateAboutAxes {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rotateAboutAxes",
                                ),
                            );
                        }
                    };
                    let m_enabled = match m_enabled {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("enabled"),
                            );
                        }
                    };
                    let m_drawDebugInfo = match m_drawDebugInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "drawDebugInfo",
                                ),
                            );
                        }
                    };
                    let m_axisFwdLS = match m_axisFwdLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "axisFwdLS",
                                ),
                            );
                        }
                    };
                    let m_axisUpLS = match m_axisUpLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("axisUpLS"),
                            );
                        }
                    };
                    let m_axisSideLS = match m_axisSideLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "axisSideLS",
                                ),
                            );
                        }
                    };
                    let m_currentFwdAxisLS = match m_currentFwdAxisLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentFwdAxisLS",
                                ),
                            );
                        }
                    };
                    let m_currentRotMS = match m_currentRotMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentRotMS",
                                ),
                            );
                        }
                    };
                    let m_currentGain = match m_currentGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentGain",
                                ),
                            );
                        }
                    };
                    let m_defaultRefTransformMS = match m_defaultRefTransformMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "defaultRefTransformMS",
                                ),
                            );
                        }
                    };
                    let m_defaultRefTransformLS = match m_defaultRefTransformLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "defaultRefTransformLS",
                                ),
                            );
                        }
                    };
                    let m_defaultOffsetFwd = match m_defaultOffsetFwd {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "defaultOffsetFwd",
                                ),
                            );
                        }
                    };
                    let m_defaultOffsetVert = match m_defaultOffsetVert {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "defaultOffsetVert",
                                ),
                            );
                        }
                    };
                    let m_customRefTransformMS = match m_customRefTransformMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "customRefTransformMS",
                                ),
                            );
                        }
                    };
                    let m_customRefTransformLS = match m_customRefTransformLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "customRefTransformLS",
                                ),
                            );
                        }
                    };
                    let m_customOffsetFwd = match m_customOffsetFwd {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "customOffsetFwd",
                                ),
                            );
                        }
                    };
                    let m_customOffsetVert = match m_customOffsetVert {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "customOffsetVert",
                                ),
                            );
                        }
                    };
                    let m_currentRefTransformMS = match m_currentRefTransformMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentRefTransformMS",
                                ),
                            );
                        }
                    };
                    let m_currentRefTransformLS = match m_currentRefTransformLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentRefTransformLS",
                                ),
                            );
                        }
                    };
                    let m_currentOffsetFwd = match m_currentOffsetFwd {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentOffsetFwd",
                                ),
                            );
                        }
                    };
                    let m_currentOffsetVert = match m_currentOffsetVert {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentOffsetVert",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(BSLookAtModifierBoneData {
                        __ptr,
                        m_index,
                        m_fwdAxisLS,
                        m_upAxisLS,
                        m_specifyAxesByColor,
                        m_forwardAxisColor,
                        m_upAxisColor,
                        m_limitAngleDegrees,
                        m_limitAngleDegreesLeft,
                        m_limitAngleDegreesRight,
                        m_limitAngleDegreesUp,
                        m_limitAngleDegreesDown,
                        m_onGain,
                        m_offGain,
                        m_onLeadIn,
                        m_offLeadIn,
                        m_rotateAboutAxes,
                        m_enabled,
                        m_drawDebugInfo,
                        m_axisFwdLS,
                        m_axisUpLS,
                        m_axisSideLS,
                        m_currentFwdAxisLS,
                        m_currentRotMS,
                        m_currentGain,
                        m_defaultRefTransformMS,
                        m_defaultRefTransformLS,
                        m_defaultOffsetFwd,
                        m_defaultOffsetVert,
                        m_customRefTransformMS,
                        m_customRefTransformLS,
                        m_customOffsetFwd,
                        m_customOffsetVert,
                        m_currentRefTransformMS,
                        m_currentRefTransformLS,
                        m_currentOffsetFwd,
                        m_currentOffsetVert,
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
                    let mut m_index: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_fwdAxisLS: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_upAxisLS: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_specifyAxesByColor: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_forwardAxisColor: _serde::__private::Option<AXIS_COLOR> = _serde::__private::None;
                    let mut m_upAxisColor: _serde::__private::Option<AXIS_COLOR> = _serde::__private::None;
                    let mut m_limitAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_limitAngleDegreesLeft: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_limitAngleDegreesRight: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_limitAngleDegreesUp: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_limitAngleDegreesDown: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_onGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_offGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_onLeadIn: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_offLeadIn: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_rotateAboutAxes: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_enabled: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_drawDebugInfo: _serde::__private::Option<bool> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_index => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_index) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("index"),
                                    );
                                }
                                m_index = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_fwdAxisLS => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_fwdAxisLS) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fwdAxisLS",
                                        ),
                                    );
                                }
                                m_fwdAxisLS = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_upAxisLS => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_upAxisLS) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "upAxisLS",
                                        ),
                                    );
                                }
                                m_upAxisLS = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_specifyAxesByColor => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_specifyAxesByColor,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "specifyAxesByColor",
                                        ),
                                    );
                                }
                                m_specifyAxesByColor = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_forwardAxisColor => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_forwardAxisColor) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "forwardAxisColor",
                                        ),
                                    );
                                }
                                m_forwardAxisColor = _serde::__private::Some(
                                    match __A::next_value::<AXIS_COLOR>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_upAxisColor => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_upAxisColor) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "upAxisColor",
                                        ),
                                    );
                                }
                                m_upAxisColor = _serde::__private::Some(
                                    match __A::next_value::<AXIS_COLOR>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_limitAngleDegrees => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_limitAngleDegrees,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "limitAngleDegrees",
                                        ),
                                    );
                                }
                                m_limitAngleDegrees = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_limitAngleDegreesLeft => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_limitAngleDegreesLeft,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "limitAngleDegreesLeft",
                                        ),
                                    );
                                }
                                m_limitAngleDegreesLeft = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_limitAngleDegreesRight => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_limitAngleDegreesRight,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "limitAngleDegreesRight",
                                        ),
                                    );
                                }
                                m_limitAngleDegreesRight = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_limitAngleDegreesUp => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_limitAngleDegreesUp,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "limitAngleDegreesUp",
                                        ),
                                    );
                                }
                                m_limitAngleDegreesUp = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_limitAngleDegreesDown => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_limitAngleDegreesDown,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "limitAngleDegreesDown",
                                        ),
                                    );
                                }
                                m_limitAngleDegreesDown = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_onGain => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_onGain) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("onGain"),
                                    );
                                }
                                m_onGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_offGain => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_offGain) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "offGain",
                                        ),
                                    );
                                }
                                m_offGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_onLeadIn => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_onLeadIn) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "onLeadIn",
                                        ),
                                    );
                                }
                                m_onLeadIn = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_offLeadIn => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_offLeadIn) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "offLeadIn",
                                        ),
                                    );
                                }
                                m_offLeadIn = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_rotateAboutAxes => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_rotateAboutAxes) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rotateAboutAxes",
                                        ),
                                    );
                                }
                                m_rotateAboutAxes = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_enabled => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_enabled) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enabled",
                                        ),
                                    );
                                }
                                m_enabled = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_drawDebugInfo => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_drawDebugInfo) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "drawDebugInfo",
                                        ),
                                    );
                                }
                                m_drawDebugInfo = _serde::__private::Some(
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
                    let m_index = match m_index {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("index"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_fwdAxisLS = match m_fwdAxisLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fwdAxisLS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_upAxisLS = match m_upAxisLS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("upAxisLS"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_specifyAxesByColor = match m_specifyAxesByColor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "specifyAxesByColor",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_forwardAxisColor = match m_forwardAxisColor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "forwardAxisColor",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_upAxisColor = match m_upAxisColor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "upAxisColor",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_limitAngleDegrees = match m_limitAngleDegrees {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitAngleDegrees",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_limitAngleDegreesLeft = match m_limitAngleDegreesLeft {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitAngleDegreesLeft",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_limitAngleDegreesRight = match m_limitAngleDegreesRight {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitAngleDegreesRight",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_limitAngleDegreesUp = match m_limitAngleDegreesUp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitAngleDegreesUp",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_limitAngleDegreesDown = match m_limitAngleDegreesDown {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitAngleDegreesDown",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_onGain = match m_onGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("onGain"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_offGain = match m_offGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("offGain"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_onLeadIn = match m_onLeadIn {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("onLeadIn"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_offLeadIn = match m_offLeadIn {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "offLeadIn",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_rotateAboutAxes = match m_rotateAboutAxes {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rotateAboutAxes",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_enabled = match m_enabled {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("enabled"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_drawDebugInfo = match m_drawDebugInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "drawDebugInfo",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(BSLookAtModifierBoneData {
                        __ptr: __ptr.clone(),
                        m_index,
                        m_fwdAxisLS,
                        m_upAxisLS,
                        m_specifyAxesByColor,
                        m_forwardAxisColor,
                        m_upAxisColor,
                        m_limitAngleDegrees,
                        m_limitAngleDegreesLeft,
                        m_limitAngleDegreesRight,
                        m_limitAngleDegreesUp,
                        m_limitAngleDegreesDown,
                        m_onGain,
                        m_offGain,
                        m_onLeadIn,
                        m_offLeadIn,
                        m_rotateAboutAxes,
                        m_enabled,
                        m_drawDebugInfo,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "index",
                "fwdAxisLS",
                "upAxisLS",
                "specifyAxesByColor",
                "forwardAxisColor",
                "upAxisColor",
                "limitAngleDegrees",
                "limitAngleDegreesLeft",
                "limitAngleDegreesRight",
                "limitAngleDegreesUp",
                "limitAngleDegreesDown",
                "onGain",
                "offGain",
                "onLeadIn",
                "offLeadIn",
                "rotateAboutAxes",
                "enabled",
                "drawDebugInfo",
                "axisFwdLS",
                "axisUpLS",
                "axisSideLS",
                "currentFwdAxisLS",
                "currentRotMS",
                "currentGain",
                "defaultRefTransformMS",
                "defaultRefTransformLS",
                "defaultOffsetFwd",
                "defaultOffsetVert",
                "customRefTransformMS",
                "customRefTransformLS",
                "customOffsetFwd",
                "customOffsetVert",
                "currentRefTransformMS",
                "currentRefTransformLS",
                "currentOffsetFwd",
                "currentOffsetVert",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "BSLookAtModifierBoneData",
                FIELDS,
                __BSLookAtModifierBoneDataVisitor {
                    marker: _serde::__private::PhantomData::<BSLookAtModifierBoneData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
/// # C++ Info
/// - name: `AXIS_COLOR`(ctype: `hkEnum<AXIS_COLOR, hkInt8>`)
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
pub enum AXIS_COLOR {
    #[default]
    Red = 0isize,
    Green = 1isize,
    Blue = 2isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for AXIS_COLOR {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::Red => __serializer.serialize_field("Red", &0u64),
                Self::Green => __serializer.serialize_field("Green", &1u64),
                Self::Blue => __serializer.serialize_field("Blue", &2u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum AXIS_COLOR to_i8"))?;
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
    impl<'de> _serde::Deserialize<'de> for AXIS_COLOR {
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
                fn visit_int8<__E>(
                    self,
                    __value: I8<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        I8::Number(0i8) => _serde::__private::Ok(__Field::__field0),
                        I8::Number(1i8) => _serde::__private::Ok(__Field::__field1),
                        I8::Number(2i8) => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1, 2",
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
                            v if v == "0" || v.eq_ignore_ascii_case("Red") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("Green") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2" || v.eq_ignore_ascii_case("Blue") => {
                                _serde::__private::Ok(__Field::__field2)
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
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<AXIS_COLOR>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = AXIS_COLOR;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum AXIS_COLOR",
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
                            _serde::__private::Ok(AXIS_COLOR::Red)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AXIS_COLOR::Green)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AXIS_COLOR::Blue)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &["Red", "Green", "Blue"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "AXIS_COLOR",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<AXIS_COLOR>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
