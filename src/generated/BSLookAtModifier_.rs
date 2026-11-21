use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `BSLookAtModifier`
/// - version: `5`
/// - signature: `0x5901a37d`
/// - size: `  0`(x86)/`816`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSLookAtModifier<'a> {
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
    pub parent: hkbModifier<'a>,
    /// # C++ Info
    /// - name: `additive`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 88`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "additive"))]
    #[cfg_attr(feature = "serde", serde(rename = "additive"))]
    pub m_additive: bool,
    /// # C++ Info
    /// - name: `clampAdditive`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 89`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "clampAdditive"))]
    #[cfg_attr(feature = "serde", serde(rename = "clampAdditive"))]
    pub m_clampAdditive: bool,
    /// # C++ Info
    /// - name: `target`(ctype: `enum TARGET`)
    /// - offset: `  0`(x86)/` 90`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "target"))]
    #[cfg_attr(feature = "serde", serde(rename = "target"))]
    pub m_target: TARGET,
    /// # C++ Info
    /// - name: `HeadTrackingOn1`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 91`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "HeadTrackingOn1"))]
    #[cfg_attr(feature = "serde", serde(rename = "HeadTrackingOn1"))]
    pub m_HeadTrackingOn1: bool,
    /// # C++ Info
    /// - name: `HeadTrackingOn2`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 92`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "HeadTrackingOn2"))]
    #[cfg_attr(feature = "serde", serde(rename = "HeadTrackingOn2"))]
    pub m_HeadTrackingOn2: bool,
    /// # C++ Info
    /// - name: `characterType`(ctype: `enum CHARACTER_TYPE`)
    /// - offset: `  0`(x86)/` 93`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "characterType"))]
    #[cfg_attr(feature = "serde", serde(rename = "characterType"))]
    pub m_characterType: CHARACTER_TYPE,
    /// # C++ Info
    /// - name: `rootBone`(ctype: `struct BSLookAtModifierBoneData`)
    /// - offset: `  0`(x86)/` 96`(x86_64)
    /// - type_size: `  0`(x86)/`528`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "rootBone"))]
    #[cfg_attr(feature = "serde", serde(rename = "rootBone"))]
    pub m_rootBone: BSLookAtModifierBoneData<'a>,
    /// # C++ Info
    /// - name: `bones`(ctype: `hkArray<struct BSLookAtModifierBoneData>`)
    /// - offset: `  0`(x86)/`624`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "bones"))]
    #[cfg_attr(feature = "serde", serde(rename = "bones"))]
    pub m_bones: Vec<BSLookAtModifierBoneData<'a>>,
    /// # C++ Info
    /// - name: `eyeBones`(ctype: `hkArray<struct BSLookAtModifierBoneData>`)
    /// - offset: `  0`(x86)/`640`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "eyeBones"))]
    #[cfg_attr(feature = "serde", serde(rename = "eyeBones"))]
    pub m_eyeBones: Vec<BSLookAtModifierBoneData<'a>>,
    /// # C++ Info
    /// - name: `useIndividualLimitAngles`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`656`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "useIndividualLimitAngles"))]
    #[cfg_attr(feature = "serde", serde(rename = "useIndividualLimitAngles"))]
    pub m_useIndividualLimitAngles: bool,
    /// # C++ Info
    /// - name: `limitAngleThresholdDegrees`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`660`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "limitAngleThresholdDegrees"))]
    #[cfg_attr(feature = "serde", serde(rename = "limitAngleThresholdDegrees"))]
    pub m_limitAngleThresholdDegrees: f32,
    /// # C++ Info
    /// - name: `continueLookOutsideOfLimit`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`664`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "continueLookOutsideOfLimit"))]
    #[cfg_attr(feature = "serde", serde(rename = "continueLookOutsideOfLimit"))]
    pub m_continueLookOutsideOfLimit: bool,
    /// # C++ Info
    /// - name: `onGain`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`668`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "onGain"))]
    #[cfg_attr(feature = "serde", serde(rename = "onGain"))]
    pub m_onGain: f32,
    /// # C++ Info
    /// - name: `offGain`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`672`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "offGain"))]
    #[cfg_attr(feature = "serde", serde(rename = "offGain"))]
    pub m_offGain: f32,
    /// # C++ Info
    /// - name: `onLeadIn`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`676`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "onLeadIn"))]
    #[cfg_attr(feature = "serde", serde(rename = "onLeadIn"))]
    pub m_onLeadIn: f32,
    /// # C++ Info
    /// - name: `offLeadIn`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`680`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "offLeadIn"))]
    #[cfg_attr(feature = "serde", serde(rename = "offLeadIn"))]
    pub m_offLeadIn: f32,
    /// # C++ Info
    /// - name: `useBoneGains`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`684`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "useBoneGains"))]
    #[cfg_attr(feature = "serde", serde(rename = "useBoneGains"))]
    pub m_useBoneGains: bool,
    /// # C++ Info
    /// - name: `refPoseBlendGain`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`688`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "refPoseBlendGain"))]
    #[cfg_attr(feature = "serde", serde(rename = "refPoseBlendGain"))]
    pub m_refPoseBlendGain: f32,
    /// # C++ Info
    /// - name: `restoreDefaultRefPosEventId`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`692`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "restoreDefaultRefPosEventId")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "restoreDefaultRefPosEventId"))]
    pub m_restoreDefaultRefPosEventId: I32<'a>,
    /// # C++ Info
    /// - name: `targetLocation`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`704`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "targetLocation"))]
    #[cfg_attr(feature = "serde", serde(rename = "targetLocation"))]
    pub m_targetLocation: Vector4,
    /// # C++ Info
    /// - name: `targetOutsideLimits`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`720`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "targetOutsideLimits"))]
    #[cfg_attr(feature = "serde", serde(rename = "targetOutsideLimits"))]
    pub m_targetOutsideLimits: bool,
    /// # C++ Info
    /// - name: `targetOutOfLimitEvent`(ctype: `struct hkbEventProperty`)
    /// - offset: `  0`(x86)/`728`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "targetOutOfLimitEvent"))]
    #[cfg_attr(feature = "serde", serde(rename = "targetOutOfLimitEvent"))]
    pub m_targetOutOfLimitEvent: hkbEventProperty<'a>,
    /// # C++ Info
    /// - name: `snapToTargetEventId`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`744`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "snapToTargetEventId"))]
    #[cfg_attr(feature = "serde", serde(rename = "snapToTargetEventId"))]
    pub m_snapToTargetEventId: I32<'a>,
    /// # C++ Info
    /// - name: `lookActive`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`748`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "lookActive"))]
    #[cfg_attr(feature = "serde", serde(rename = "lookActive"))]
    pub m_lookActive: bool,
    /// # C++ Info
    /// - name: `modificationActive`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`749`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "modificationActive"))]
    #[cfg_attr(feature = "serde", serde(rename = "modificationActive"))]
    pub m_modificationActive: bool,
    /// # C++ Info
    /// - name: `lookAtCameraX`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`752`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "lookAtCameraX"))]
    #[cfg_attr(feature = "serde", serde(rename = "lookAtCameraX"))]
    pub m_lookAtCameraX: f32,
    /// # C++ Info
    /// - name: `lookAtCameraY`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`756`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "lookAtCameraY"))]
    #[cfg_attr(feature = "serde", serde(rename = "lookAtCameraY"))]
    pub m_lookAtCameraY: f32,
    /// # C++ Info
    /// - name: `lookAtCameraZ`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`760`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "lookAtCameraZ"))]
    #[cfg_attr(feature = "serde", serde(rename = "lookAtCameraZ"))]
    pub m_lookAtCameraZ: f32,
    /// # C++ Info
    /// - name: `drawDebugInfo`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`764`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "drawDebugInfo"))]
    #[cfg_attr(feature = "serde", serde(rename = "drawDebugInfo"))]
    pub m_drawDebugInfo: bool,
    /// # C++ Info
    /// - name: `prevLookAtTarget`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`765`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "prevLookAtTarget"))]
    #[cfg_attr(feature = "serde", serde(rename = "prevLookAtTarget"))]
    pub m_prevLookAtTarget: bool,
    /// # C++ Info
    /// - name: `targetOutsideLimitsPrev`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`766`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "targetOutsideLimitsPrev"))]
    #[cfg_attr(feature = "serde", serde(rename = "targetOutsideLimitsPrev"))]
    pub m_targetOutsideLimitsPrev: bool,
    /// # C++ Info
    /// - name: `buseCustomRefPose`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`767`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "buseCustomRefPose"))]
    #[cfg_attr(feature = "serde", serde(rename = "buseCustomRefPose"))]
    pub m_buseCustomRefPose: bool,
    /// # C++ Info
    /// - name: `refPoseGain`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`768`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "refPoseGain"))]
    #[cfg_attr(feature = "serde", serde(rename = "refPoseGain"))]
    pub m_refPoseGain: f32,
    /// # C++ Info
    /// - name: `snapToTarget`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`772`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "snapToTarget"))]
    #[cfg_attr(feature = "serde", serde(rename = "snapToTarget"))]
    pub m_snapToTarget: bool,
    /// # C++ Info
    /// - name: `timeStep`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`776`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "timeStep"))]
    #[cfg_attr(feature = "serde", serde(rename = "timeStep"))]
    pub m_timeStep: f32,
    /// # C++ Info
    /// - name: `curLookAtTarget`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`780`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "curLookAtTarget"))]
    #[cfg_attr(feature = "serde", serde(rename = "curLookAtTarget"))]
    pub m_curLookAtTarget: bool,
    /// # C++ Info
    /// - name: `curTargetLocation`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`784`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "curTargetLocation"))]
    #[cfg_attr(feature = "serde", serde(rename = "curTargetLocation"))]
    pub m_curTargetLocation: Vector4,
    /// # C++ Info
    /// - name: `ballBonesValid`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`800`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "ballBonesValid"))]
    #[cfg_attr(feature = "serde", serde(rename = "ballBonesValid"))]
    pub m_ballBonesValid: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSLookAtModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSLookAtModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x5901a37d)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.parent.parent.parent.m_variableBindingSet);
            v.extend(self.parent.parent.parent.m_cachedBindables.iter());
            v.push(&self.parent.parent.m_nodeInfo);
            v.extend(self.m_rootBone.deps_indexes());
            v.extend(
                self
                    .m_bones
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(
                self
                    .m_eyeBones
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(self.m_targetOutOfLimitEvent.deps_indexes());
            v
        }
    }
    impl<'a> _serde::Serialize for BSLookAtModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x5901a37d)));
            let mut serializer = __serializer
                .serialize_struct("BSLookAtModifier", class_meta, (0u64, 816u64))?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndRefCount",
                    &self.parent.parent.parent.parent.m_memSizeAndRefCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer
                .skip_field(
                    "hasEnableChanged",
                    &self.parent.parent.parent.m_hasEnableChanged,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer.serialize_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer.skip_field("type", &self.parent.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("nodeInfo", &self.parent.parent.m_nodeInfo)?;
            serializer.serialize_field("enable", &self.parent.m_enable)?;
            serializer
                .skip_fixed_array_field(
                    "padModifier",
                    self.parent.m_padModifier.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("additive", &self.m_additive)?;
            serializer.serialize_field("clampAdditive", &self.m_clampAdditive)?;
            serializer.serialize_field("target", &self.m_target)?;
            serializer.serialize_field("HeadTrackingOn1", &self.m_HeadTrackingOn1)?;
            serializer.serialize_field("HeadTrackingOn2", &self.m_HeadTrackingOn2)?;
            serializer.serialize_field("characterType", &self.m_characterType)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("rootBone", &self.m_rootBone)?;
            serializer
                .serialize_array_field(
                    "bones",
                    &self.m_bones,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 528u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "eyeBones",
                    &self.m_eyeBones,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 528u64,
                    },
                )?;
            serializer
                .serialize_field(
                    "useIndividualLimitAngles",
                    &self.m_useIndividualLimitAngles,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field(
                    "limitAngleThresholdDegrees",
                    &self.m_limitAngleThresholdDegrees,
                )?;
            serializer
                .serialize_field(
                    "continueLookOutsideOfLimit",
                    &self.m_continueLookOutsideOfLimit,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("onGain", &self.m_onGain)?;
            serializer.serialize_field("offGain", &self.m_offGain)?;
            serializer.serialize_field("onLeadIn", &self.m_onLeadIn)?;
            serializer.serialize_field("offLeadIn", &self.m_offLeadIn)?;
            serializer.serialize_field("useBoneGains", &self.m_useBoneGains)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("refPoseBlendGain", &self.m_refPoseBlendGain)?;
            serializer
                .serialize_field(
                    "restoreDefaultRefPosEventId",
                    &self.m_restoreDefaultRefPosEventId,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("targetLocation", &self.m_targetLocation)?;
            serializer
                .serialize_field("targetOutsideLimits", &self.m_targetOutsideLimits)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_field(
                    "targetOutOfLimitEvent",
                    &self.m_targetOutOfLimitEvent,
                )?;
            serializer
                .serialize_field("snapToTargetEventId", &self.m_snapToTargetEventId)?;
            serializer.serialize_field("lookActive", &self.m_lookActive)?;
            serializer
                .serialize_field("modificationActive", &self.m_modificationActive)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("lookAtCameraX", &self.m_lookAtCameraX)?;
            serializer.serialize_field("lookAtCameraY", &self.m_lookAtCameraY)?;
            serializer.serialize_field("lookAtCameraZ", &self.m_lookAtCameraZ)?;
            serializer.serialize_field("drawDebugInfo", &self.m_drawDebugInfo)?;
            serializer.skip_field("prevLookAtTarget", &self.m_prevLookAtTarget)?;
            serializer
                .skip_field("targetOutsideLimitsPrev", &self.m_targetOutsideLimitsPrev)?;
            serializer.skip_field("buseCustomRefPose", &self.m_buseCustomRefPose)?;
            serializer.skip_field("refPoseGain", &self.m_refPoseGain)?;
            serializer.skip_field("snapToTarget", &self.m_snapToTarget)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.skip_field("timeStep", &self.m_timeStep)?;
            serializer.skip_field("curLookAtTarget", &self.m_curLookAtTarget)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.skip_field("curTargetLocation", &self.m_curTargetLocation)?;
            serializer.skip_field("ballBonesValid", &self.m_ballBonesValid)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 15usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BSLookAtModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_variableBindingSet,
                m_userData,
                m_name,
                m_enable,
                m_additive,
                m_clampAdditive,
                m_target,
                m_HeadTrackingOn1,
                m_HeadTrackingOn2,
                m_characterType,
                m_rootBone,
                m_bones,
                m_eyeBones,
                m_useIndividualLimitAngles,
                m_limitAngleThresholdDegrees,
                m_continueLookOutsideOfLimit,
                m_onGain,
                m_offGain,
                m_onLeadIn,
                m_offLeadIn,
                m_useBoneGains,
                m_refPoseBlendGain,
                m_restoreDefaultRefPosEventId,
                m_targetLocation,
                m_targetOutsideLimits,
                m_targetOutOfLimitEvent,
                m_snapToTargetEventId,
                m_lookActive,
                m_modificationActive,
                m_lookAtCameraX,
                m_lookAtCameraY,
                m_lookAtCameraZ,
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
                        "variableBindingSet" => Ok(__Field::m_variableBindingSet),
                        "userData" => Ok(__Field::m_userData),
                        "name" => Ok(__Field::m_name),
                        "enable" => Ok(__Field::m_enable),
                        "additive" => Ok(__Field::m_additive),
                        "clampAdditive" => Ok(__Field::m_clampAdditive),
                        "target" => Ok(__Field::m_target),
                        "HeadTrackingOn1" => Ok(__Field::m_HeadTrackingOn1),
                        "HeadTrackingOn2" => Ok(__Field::m_HeadTrackingOn2),
                        "characterType" => Ok(__Field::m_characterType),
                        "rootBone" => Ok(__Field::m_rootBone),
                        "bones" => Ok(__Field::m_bones),
                        "eyeBones" => Ok(__Field::m_eyeBones),
                        "useIndividualLimitAngles" => {
                            Ok(__Field::m_useIndividualLimitAngles)
                        }
                        "limitAngleThresholdDegrees" => {
                            Ok(__Field::m_limitAngleThresholdDegrees)
                        }
                        "continueLookOutsideOfLimit" => {
                            Ok(__Field::m_continueLookOutsideOfLimit)
                        }
                        "onGain" => Ok(__Field::m_onGain),
                        "offGain" => Ok(__Field::m_offGain),
                        "onLeadIn" => Ok(__Field::m_onLeadIn),
                        "offLeadIn" => Ok(__Field::m_offLeadIn),
                        "useBoneGains" => Ok(__Field::m_useBoneGains),
                        "refPoseBlendGain" => Ok(__Field::m_refPoseBlendGain),
                        "restoreDefaultRefPosEventId" => {
                            Ok(__Field::m_restoreDefaultRefPosEventId)
                        }
                        "targetLocation" => Ok(__Field::m_targetLocation),
                        "targetOutsideLimits" => Ok(__Field::m_targetOutsideLimits),
                        "targetOutOfLimitEvent" => Ok(__Field::m_targetOutOfLimitEvent),
                        "snapToTargetEventId" => Ok(__Field::m_snapToTargetEventId),
                        "lookActive" => Ok(__Field::m_lookActive),
                        "modificationActive" => Ok(__Field::m_modificationActive),
                        "lookAtCameraX" => Ok(__Field::m_lookAtCameraX),
                        "lookAtCameraY" => Ok(__Field::m_lookAtCameraY),
                        "lookAtCameraZ" => Ok(__Field::m_lookAtCameraZ),
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
            struct __BSLookAtModifierVisitor<'de> {
                marker: _serde::__private::PhantomData<BSLookAtModifier<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __BSLookAtModifierVisitor<'de> {
                type Value = BSLookAtModifier<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct BSLookAtModifier",
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
                    let mut m_additive: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_clampAdditive: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_target: _serde::__private::Option<TARGET> = _serde::__private::None;
                    let mut m_HeadTrackingOn1: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_HeadTrackingOn2: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_characterType: _serde::__private::Option<CHARACTER_TYPE> = _serde::__private::None;
                    let mut m_rootBone: _serde::__private::Option<
                        BSLookAtModifierBoneData<'de>,
                    > = _serde::__private::None;
                    let mut m_bones: _serde::__private::Option<
                        Vec<BSLookAtModifierBoneData<'de>>,
                    > = _serde::__private::None;
                    let mut m_eyeBones: _serde::__private::Option<
                        Vec<BSLookAtModifierBoneData<'de>>,
                    > = _serde::__private::None;
                    let mut m_useIndividualLimitAngles: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_limitAngleThresholdDegrees: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_continueLookOutsideOfLimit: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_onGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_offGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_onLeadIn: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_offLeadIn: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_useBoneGains: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_refPoseBlendGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_restoreDefaultRefPosEventId: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_targetLocation: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_targetOutsideLimits: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_targetOutOfLimitEvent: _serde::__private::Option<
                        hkbEventProperty<'de>,
                    > = _serde::__private::None;
                    let mut m_snapToTargetEventId: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_lookActive: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_modificationActive: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_lookAtCameraX: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_lookAtCameraY: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_lookAtCameraZ: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_drawDebugInfo: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_prevLookAtTarget: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_targetOutsideLimitsPrev: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_buseCustomRefPose: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_refPoseGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_snapToTarget: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_timeStep: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_curLookAtTarget: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_curTargetLocation: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_ballBonesValid: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..38usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_additive) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "additive",
                                        ),
                                    );
                                }
                                m_additive = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_clampAdditive) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "clampAdditive",
                                        ),
                                    );
                                }
                                m_clampAdditive = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_target) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("target"),
                                    );
                                }
                                m_target = _serde::__private::Some(
                                    match __A::next_value::<TARGET>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_HeadTrackingOn1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "HeadTrackingOn1",
                                        ),
                                    );
                                }
                                m_HeadTrackingOn1 = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_HeadTrackingOn2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "HeadTrackingOn2",
                                        ),
                                    );
                                }
                                m_HeadTrackingOn2 = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_characterType) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterType",
                                        ),
                                    );
                                }
                                m_characterType = _serde::__private::Some(
                                    match __A::next_value::<CHARACTER_TYPE>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_rootBone) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rootBone",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 2usize)?;
                                m_rootBone = _serde::__private::Some(
                                    match __A::next_value::<
                                        BSLookAtModifierBoneData<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_bones) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("bones"),
                                    );
                                }
                                m_bones = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<BSLookAtModifierBoneData<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_eyeBones) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eyeBones",
                                        ),
                                    );
                                }
                                m_eyeBones = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<BSLookAtModifierBoneData<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(
                                    &m_useIndividualLimitAngles,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useIndividualLimitAngles",
                                        ),
                                    );
                                }
                                m_useIndividualLimitAngles = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(
                                    &m_limitAngleThresholdDegrees,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "limitAngleThresholdDegrees",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 3usize)?;
                                m_limitAngleThresholdDegrees = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(
                                    &m_continueLookOutsideOfLimit,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "continueLookOutsideOfLimit",
                                        ),
                                    );
                                }
                                m_continueLookOutsideOfLimit = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_onGain) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("onGain"),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 3usize)?;
                                m_onGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
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
                            14usize => {
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
                            15usize => {
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
                            16usize => {
                                if _serde::__private::Option::is_some(&m_useBoneGains) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useBoneGains",
                                        ),
                                    );
                                }
                                m_useBoneGains = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            17usize => {
                                if _serde::__private::Option::is_some(&m_refPoseBlendGain) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "refPoseBlendGain",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 3usize)?;
                                m_refPoseBlendGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            18usize => {
                                if _serde::__private::Option::is_some(
                                    &m_restoreDefaultRefPosEventId,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "restoreDefaultRefPosEventId",
                                        ),
                                    );
                                }
                                m_restoreDefaultRefPosEventId = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            19usize => {
                                if _serde::__private::Option::is_some(&m_targetLocation) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "targetLocation",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 8usize)?;
                                m_targetLocation = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            20usize => {
                                if _serde::__private::Option::is_some(
                                    &m_targetOutsideLimits,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "targetOutsideLimits",
                                        ),
                                    );
                                }
                                m_targetOutsideLimits = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            21usize => {
                                if _serde::__private::Option::is_some(
                                    &m_targetOutOfLimitEvent,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "targetOutOfLimitEvent",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 7usize)?;
                                m_targetOutOfLimitEvent = _serde::__private::Some(
                                    match __A::next_value::<hkbEventProperty<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            22usize => {
                                if _serde::__private::Option::is_some(
                                    &m_snapToTargetEventId,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "snapToTargetEventId",
                                        ),
                                    );
                                }
                                m_snapToTargetEventId = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            23usize => {
                                if _serde::__private::Option::is_some(&m_lookActive) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lookActive",
                                        ),
                                    );
                                }
                                m_lookActive = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            24usize => {
                                if _serde::__private::Option::is_some(
                                    &m_modificationActive,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "modificationActive",
                                        ),
                                    );
                                }
                                m_modificationActive = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            25usize => {
                                if _serde::__private::Option::is_some(&m_lookAtCameraX) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lookAtCameraX",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 2usize)?;
                                m_lookAtCameraX = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            26usize => {
                                if _serde::__private::Option::is_some(&m_lookAtCameraY) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lookAtCameraY",
                                        ),
                                    );
                                }
                                m_lookAtCameraY = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            27usize => {
                                if _serde::__private::Option::is_some(&m_lookAtCameraZ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lookAtCameraZ",
                                        ),
                                    );
                                }
                                m_lookAtCameraZ = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            28usize => {
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
                            29usize => {
                                if _serde::__private::Option::is_some(&m_prevLookAtTarget) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "prevLookAtTarget",
                                        ),
                                    );
                                }
                                m_prevLookAtTarget = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            30usize => {
                                if _serde::__private::Option::is_some(
                                    &m_targetOutsideLimitsPrev,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "targetOutsideLimitsPrev",
                                        ),
                                    );
                                }
                                m_targetOutsideLimitsPrev = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            31usize => {
                                if _serde::__private::Option::is_some(
                                    &m_buseCustomRefPose,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "buseCustomRefPose",
                                        ),
                                    );
                                }
                                m_buseCustomRefPose = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            32usize => {
                                if _serde::__private::Option::is_some(&m_refPoseGain) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "refPoseGain",
                                        ),
                                    );
                                }
                                m_refPoseGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            33usize => {
                                if _serde::__private::Option::is_some(&m_snapToTarget) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "snapToTarget",
                                        ),
                                    );
                                }
                                m_snapToTarget = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            34usize => {
                                if _serde::__private::Option::is_some(&m_timeStep) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "timeStep",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 3usize)?;
                                m_timeStep = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            35usize => {
                                if _serde::__private::Option::is_some(&m_curLookAtTarget) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "curLookAtTarget",
                                        ),
                                    );
                                }
                                m_curLookAtTarget = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            36usize => {
                                if _serde::__private::Option::is_some(
                                    &m_curTargetLocation,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "curTargetLocation",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 3usize)?;
                                m_curTargetLocation = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            37usize => {
                                if _serde::__private::Option::is_some(&m_ballBonesValid) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ballBonesValid",
                                        ),
                                    );
                                }
                                m_ballBonesValid = _serde::__private::Some(
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
                    __A::pad(&mut __map, 0usize, 15usize)?;
                    let m_additive = match m_additive {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("additive"),
                            );
                        }
                    };
                    let m_clampAdditive = match m_clampAdditive {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "clampAdditive",
                                ),
                            );
                        }
                    };
                    let m_target = match m_target {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("target"),
                            );
                        }
                    };
                    let m_HeadTrackingOn1 = match m_HeadTrackingOn1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "HeadTrackingOn1",
                                ),
                            );
                        }
                    };
                    let m_HeadTrackingOn2 = match m_HeadTrackingOn2 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "HeadTrackingOn2",
                                ),
                            );
                        }
                    };
                    let m_characterType = match m_characterType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterType",
                                ),
                            );
                        }
                    };
                    let m_rootBone = match m_rootBone {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("rootBone"),
                            );
                        }
                    };
                    let m_bones = match m_bones {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("bones"),
                            );
                        }
                    };
                    let m_eyeBones = match m_eyeBones {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("eyeBones"),
                            );
                        }
                    };
                    let m_useIndividualLimitAngles = match m_useIndividualLimitAngles {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useIndividualLimitAngles",
                                ),
                            );
                        }
                    };
                    let m_limitAngleThresholdDegrees = match m_limitAngleThresholdDegrees {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitAngleThresholdDegrees",
                                ),
                            );
                        }
                    };
                    let m_continueLookOutsideOfLimit = match m_continueLookOutsideOfLimit {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "continueLookOutsideOfLimit",
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
                    let m_useBoneGains = match m_useBoneGains {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useBoneGains",
                                ),
                            );
                        }
                    };
                    let m_refPoseBlendGain = match m_refPoseBlendGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "refPoseBlendGain",
                                ),
                            );
                        }
                    };
                    let m_restoreDefaultRefPosEventId = match m_restoreDefaultRefPosEventId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "restoreDefaultRefPosEventId",
                                ),
                            );
                        }
                    };
                    let m_targetLocation = match m_targetLocation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "targetLocation",
                                ),
                            );
                        }
                    };
                    let m_targetOutsideLimits = match m_targetOutsideLimits {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "targetOutsideLimits",
                                ),
                            );
                        }
                    };
                    let m_targetOutOfLimitEvent = match m_targetOutOfLimitEvent {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "targetOutOfLimitEvent",
                                ),
                            );
                        }
                    };
                    let m_snapToTargetEventId = match m_snapToTargetEventId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "snapToTargetEventId",
                                ),
                            );
                        }
                    };
                    let m_lookActive = match m_lookActive {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lookActive",
                                ),
                            );
                        }
                    };
                    let m_modificationActive = match m_modificationActive {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "modificationActive",
                                ),
                            );
                        }
                    };
                    let m_lookAtCameraX = match m_lookAtCameraX {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lookAtCameraX",
                                ),
                            );
                        }
                    };
                    let m_lookAtCameraY = match m_lookAtCameraY {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lookAtCameraY",
                                ),
                            );
                        }
                    };
                    let m_lookAtCameraZ = match m_lookAtCameraZ {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lookAtCameraZ",
                                ),
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
                    let m_prevLookAtTarget = match m_prevLookAtTarget {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "prevLookAtTarget",
                                ),
                            );
                        }
                    };
                    let m_targetOutsideLimitsPrev = match m_targetOutsideLimitsPrev {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "targetOutsideLimitsPrev",
                                ),
                            );
                        }
                    };
                    let m_buseCustomRefPose = match m_buseCustomRefPose {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "buseCustomRefPose",
                                ),
                            );
                        }
                    };
                    let m_refPoseGain = match m_refPoseGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "refPoseGain",
                                ),
                            );
                        }
                    };
                    let m_snapToTarget = match m_snapToTarget {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "snapToTarget",
                                ),
                            );
                        }
                    };
                    let m_timeStep = match m_timeStep {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("timeStep"),
                            );
                        }
                    };
                    let m_curLookAtTarget = match m_curLookAtTarget {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "curLookAtTarget",
                                ),
                            );
                        }
                    };
                    let m_curTargetLocation = match m_curTargetLocation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "curTargetLocation",
                                ),
                            );
                        }
                    };
                    let m_ballBonesValid = match m_ballBonesValid {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ballBonesValid",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(BSLookAtModifier {
                        __ptr,
                        parent,
                        m_additive,
                        m_clampAdditive,
                        m_target,
                        m_HeadTrackingOn1,
                        m_HeadTrackingOn2,
                        m_characterType,
                        m_rootBone,
                        m_bones,
                        m_eyeBones,
                        m_useIndividualLimitAngles,
                        m_limitAngleThresholdDegrees,
                        m_continueLookOutsideOfLimit,
                        m_onGain,
                        m_offGain,
                        m_onLeadIn,
                        m_offLeadIn,
                        m_useBoneGains,
                        m_refPoseBlendGain,
                        m_restoreDefaultRefPosEventId,
                        m_targetLocation,
                        m_targetOutsideLimits,
                        m_targetOutOfLimitEvent,
                        m_snapToTargetEventId,
                        m_lookActive,
                        m_modificationActive,
                        m_lookAtCameraX,
                        m_lookAtCameraY,
                        m_lookAtCameraZ,
                        m_drawDebugInfo,
                        m_prevLookAtTarget,
                        m_targetOutsideLimitsPrev,
                        m_buseCustomRefPose,
                        m_refPoseGain,
                        m_snapToTarget,
                        m_timeStep,
                        m_curLookAtTarget,
                        m_curTargetLocation,
                        m_ballBonesValid,
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
                    let mut m_variableBindingSet: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_userData: _serde::__private::Option<Ulong> = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_enable: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_additive: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_clampAdditive: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_target: _serde::__private::Option<TARGET> = _serde::__private::None;
                    let mut m_HeadTrackingOn1: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_HeadTrackingOn2: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_characterType: _serde::__private::Option<CHARACTER_TYPE> = _serde::__private::None;
                    let mut m_rootBone: _serde::__private::Option<
                        BSLookAtModifierBoneData<'de>,
                    > = _serde::__private::None;
                    let mut m_bones: _serde::__private::Option<
                        Vec<BSLookAtModifierBoneData<'de>>,
                    > = _serde::__private::None;
                    let mut m_eyeBones: _serde::__private::Option<
                        Vec<BSLookAtModifierBoneData<'de>>,
                    > = _serde::__private::None;
                    let mut m_useIndividualLimitAngles: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_limitAngleThresholdDegrees: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_continueLookOutsideOfLimit: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_onGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_offGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_onLeadIn: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_offLeadIn: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_useBoneGains: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_refPoseBlendGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_restoreDefaultRefPosEventId: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_targetLocation: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_targetOutsideLimits: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_targetOutOfLimitEvent: _serde::__private::Option<
                        hkbEventProperty<'de>,
                    > = _serde::__private::None;
                    let mut m_snapToTargetEventId: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_lookActive: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_modificationActive: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_lookAtCameraX: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_lookAtCameraY: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_lookAtCameraZ: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_drawDebugInfo: _serde::__private::Option<bool> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_variableBindingSet => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_variableBindingSet,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "variableBindingSet",
                                        ),
                                    );
                                }
                                m_variableBindingSet = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
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
                                    match __A::next_value::<Ulong>(&mut __map) {
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
                            __Field::m_enable => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_enable) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("enable"),
                                    );
                                }
                                m_enable = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_additive => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_additive) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "additive",
                                        ),
                                    );
                                }
                                m_additive = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_clampAdditive => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_clampAdditive) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "clampAdditive",
                                        ),
                                    );
                                }
                                m_clampAdditive = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_target => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_target) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("target"),
                                    );
                                }
                                m_target = _serde::__private::Some(
                                    match __A::next_value::<TARGET>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_HeadTrackingOn1 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_HeadTrackingOn1) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "HeadTrackingOn1",
                                        ),
                                    );
                                }
                                m_HeadTrackingOn1 = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_HeadTrackingOn2 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_HeadTrackingOn2) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "HeadTrackingOn2",
                                        ),
                                    );
                                }
                                m_HeadTrackingOn2 = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_characterType => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_characterType) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterType",
                                        ),
                                    );
                                }
                                m_characterType = _serde::__private::Some(
                                    match __A::next_value::<CHARACTER_TYPE>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_rootBone => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_rootBone) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rootBone",
                                        ),
                                    );
                                }
                                m_rootBone = _serde::__private::Some(
                                    match __A::next_value::<
                                        BSLookAtModifierBoneData<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bones => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bones) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("bones"),
                                    );
                                }
                                m_bones = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<BSLookAtModifierBoneData<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_eyeBones => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_eyeBones) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eyeBones",
                                        ),
                                    );
                                }
                                m_eyeBones = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<BSLookAtModifierBoneData<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_useIndividualLimitAngles => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_useIndividualLimitAngles,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useIndividualLimitAngles",
                                        ),
                                    );
                                }
                                m_useIndividualLimitAngles = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_limitAngleThresholdDegrees => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_limitAngleThresholdDegrees,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "limitAngleThresholdDegrees",
                                        ),
                                    );
                                }
                                m_limitAngleThresholdDegrees = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_continueLookOutsideOfLimit => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_continueLookOutsideOfLimit,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "continueLookOutsideOfLimit",
                                        ),
                                    );
                                }
                                m_continueLookOutsideOfLimit = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
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
                            __Field::m_useBoneGains => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_useBoneGains) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useBoneGains",
                                        ),
                                    );
                                }
                                m_useBoneGains = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_refPoseBlendGain => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_refPoseBlendGain) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "refPoseBlendGain",
                                        ),
                                    );
                                }
                                m_refPoseBlendGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_restoreDefaultRefPosEventId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_restoreDefaultRefPosEventId,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "restoreDefaultRefPosEventId",
                                        ),
                                    );
                                }
                                m_restoreDefaultRefPosEventId = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_targetLocation => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_targetLocation) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "targetLocation",
                                        ),
                                    );
                                }
                                m_targetLocation = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_targetOutsideLimits => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_targetOutsideLimits,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "targetOutsideLimits",
                                        ),
                                    );
                                }
                                m_targetOutsideLimits = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_targetOutOfLimitEvent => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_targetOutOfLimitEvent,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "targetOutOfLimitEvent",
                                        ),
                                    );
                                }
                                m_targetOutOfLimitEvent = _serde::__private::Some(
                                    match __A::next_value::<hkbEventProperty<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_snapToTargetEventId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_snapToTargetEventId,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "snapToTargetEventId",
                                        ),
                                    );
                                }
                                m_snapToTargetEventId = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_lookActive => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_lookActive) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lookActive",
                                        ),
                                    );
                                }
                                m_lookActive = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_modificationActive => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_modificationActive,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "modificationActive",
                                        ),
                                    );
                                }
                                m_modificationActive = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_lookAtCameraX => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_lookAtCameraX) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lookAtCameraX",
                                        ),
                                    );
                                }
                                m_lookAtCameraX = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_lookAtCameraY => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_lookAtCameraY) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lookAtCameraY",
                                        ),
                                    );
                                }
                                m_lookAtCameraY = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_lookAtCameraZ => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_lookAtCameraZ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lookAtCameraZ",
                                        ),
                                    );
                                }
                                m_lookAtCameraZ = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
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
                    let m_variableBindingSet = match m_variableBindingSet {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "variableBindingSet",
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
                    let m_enable = match m_enable {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("enable"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_additive = match m_additive {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("additive"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_clampAdditive = match m_clampAdditive {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "clampAdditive",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_target = match m_target {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("target"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_HeadTrackingOn1 = match m_HeadTrackingOn1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "HeadTrackingOn1",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_HeadTrackingOn2 = match m_HeadTrackingOn2 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "HeadTrackingOn2",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_characterType = match m_characterType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterType",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_rootBone = match m_rootBone {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("rootBone"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bones = match m_bones {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("bones"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_eyeBones = match m_eyeBones {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("eyeBones"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_useIndividualLimitAngles = match m_useIndividualLimitAngles {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useIndividualLimitAngles",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_limitAngleThresholdDegrees = match m_limitAngleThresholdDegrees {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitAngleThresholdDegrees",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_continueLookOutsideOfLimit = match m_continueLookOutsideOfLimit {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "continueLookOutsideOfLimit",
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
                    let m_useBoneGains = match m_useBoneGains {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useBoneGains",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_refPoseBlendGain = match m_refPoseBlendGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "refPoseBlendGain",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_restoreDefaultRefPosEventId = match m_restoreDefaultRefPosEventId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "restoreDefaultRefPosEventId",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_targetLocation = match m_targetLocation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "targetLocation",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_targetOutsideLimits = match m_targetOutsideLimits {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "targetOutsideLimits",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_targetOutOfLimitEvent = match m_targetOutOfLimitEvent {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "targetOutOfLimitEvent",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_snapToTargetEventId = match m_snapToTargetEventId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "snapToTargetEventId",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_lookActive = match m_lookActive {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lookActive",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_modificationActive = match m_modificationActive {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "modificationActive",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_lookAtCameraX = match m_lookAtCameraX {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lookAtCameraX",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_lookAtCameraY = match m_lookAtCameraY {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lookAtCameraY",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_lookAtCameraZ = match m_lookAtCameraZ {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lookAtCameraZ",
                                ),
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
                    let __ptr = None;
                    let parent = hkBaseObject {
                        __ptr: __ptr.clone(),
                    };
                    let parent = hkReferencedObject {
                        __ptr: __ptr.clone(),
                        parent,
                        ..Default::default()
                    };
                    let parent = hkbBindable {
                        __ptr: __ptr.clone(),
                        parent,
                        m_variableBindingSet,
                        ..Default::default()
                    };
                    let parent = hkbNode {
                        __ptr: __ptr.clone(),
                        parent,
                        m_userData,
                        m_name,
                        ..Default::default()
                    };
                    let parent = hkbModifier {
                        __ptr: __ptr.clone(),
                        parent,
                        m_enable,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(BSLookAtModifier {
                        __ptr: __ptr.clone(),
                        parent,
                        m_additive,
                        m_clampAdditive,
                        m_target,
                        m_HeadTrackingOn1,
                        m_HeadTrackingOn2,
                        m_characterType,
                        m_rootBone,
                        m_bones,
                        m_eyeBones,
                        m_useIndividualLimitAngles,
                        m_limitAngleThresholdDegrees,
                        m_continueLookOutsideOfLimit,
                        m_onGain,
                        m_offGain,
                        m_onLeadIn,
                        m_offLeadIn,
                        m_useBoneGains,
                        m_refPoseBlendGain,
                        m_restoreDefaultRefPosEventId,
                        m_targetLocation,
                        m_targetOutsideLimits,
                        m_targetOutOfLimitEvent,
                        m_snapToTargetEventId,
                        m_lookActive,
                        m_modificationActive,
                        m_lookAtCameraX,
                        m_lookAtCameraY,
                        m_lookAtCameraZ,
                        m_drawDebugInfo,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "additive",
                "clampAdditive",
                "target",
                "HeadTrackingOn1",
                "HeadTrackingOn2",
                "characterType",
                "rootBone",
                "bones",
                "eyeBones",
                "useIndividualLimitAngles",
                "limitAngleThresholdDegrees",
                "continueLookOutsideOfLimit",
                "onGain",
                "offGain",
                "onLeadIn",
                "offLeadIn",
                "useBoneGains",
                "refPoseBlendGain",
                "restoreDefaultRefPosEventId",
                "targetLocation",
                "targetOutsideLimits",
                "targetOutOfLimitEvent",
                "snapToTargetEventId",
                "lookActive",
                "modificationActive",
                "lookAtCameraX",
                "lookAtCameraY",
                "lookAtCameraZ",
                "drawDebugInfo",
                "prevLookAtTarget",
                "targetOutsideLimitsPrev",
                "buseCustomRefPose",
                "refPoseGain",
                "snapToTarget",
                "timeStep",
                "curLookAtTarget",
                "curTargetLocation",
                "ballBonesValid",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "BSLookAtModifier",
                FIELDS,
                __BSLookAtModifierVisitor {
                    marker: _serde::__private::PhantomData::<BSLookAtModifier>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
/// # C++ Info
/// - name: `TARGET`(ctype: `hkEnum<TARGET, hkInt8>`)
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
pub enum TARGET {
    #[default]
    Default = 0isize,
    AppSupplied = 1isize,
    Camera = 2isize,
}
/// # C++ Info
/// - name: `CHARACTER_TYPE`(ctype: `hkEnum<CHARACTER_TYPE, hkInt8>`)
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
pub enum CHARACTER_TYPE {
    #[default]
    Generic = 0isize,
    Biped = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for TARGET {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::Default => __serializer.serialize_field("Default", &0u64),
                Self::AppSupplied => __serializer.serialize_field("AppSupplied", &1u64),
                Self::Camera => __serializer.serialize_field("Camera", &2u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self.to_i8().ok_or(S::Error::custom("Failed enum TARGET to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for CHARACTER_TYPE {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::Generic => __serializer.serialize_field("Generic", &0u64),
                Self::Biped => __serializer.serialize_field("Biped", &1u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum CHARACTER_TYPE to_i8"))?;
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
    impl<'de> _serde::Deserialize<'de> for TARGET {
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
                            v if v == "0" || v.eq_ignore_ascii_case("Default") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("AppSupplied") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2" || v.eq_ignore_ascii_case("Camera") => {
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
                marker: _serde::__private::PhantomData<TARGET>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TARGET;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "enum TARGET")
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
                            _serde::__private::Ok(TARGET::Default)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TARGET::AppSupplied)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TARGET::Camera)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "Default",
                "AppSupplied",
                "Camera",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "TARGET",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<TARGET>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for CHARACTER_TYPE {
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
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1",
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
                            v if v == "0" || v.eq_ignore_ascii_case("Generic") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("Biped") => {
                                _serde::__private::Ok(__Field::__field1)
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
                marker: _serde::__private::PhantomData<CHARACTER_TYPE>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = CHARACTER_TYPE;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum CHARACTER_TYPE",
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
                            _serde::__private::Ok(CHARACTER_TYPE::Generic)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(CHARACTER_TYPE::Biped)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &["Generic", "Biped"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "CHARACTER_TYPE",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<CHARACTER_TYPE>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
