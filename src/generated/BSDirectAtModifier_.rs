use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `BSDirectAtModifier`
/// - version: `3`
/// - signature: `0xcda56038`
/// - size: `  0`(x86)/`432`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSDirectAtModifier<'a> {
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
    /// - name: `directAtOn`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 88`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "directAtOn"))]
    #[cfg_attr(feature = "serde", serde(rename = "directAtOn"))]
    pub m_directAtOn: bool,
    /// # C++ Info
    /// - name: `target`(ctype: `enum TARGET`)
    /// - offset: `  0`(x86)/` 89`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "target"))]
    #[cfg_attr(feature = "serde", serde(rename = "target"))]
    pub m_target: TARGET,
    /// # C++ Info
    /// - name: `context`(ctype: `enum CONTEXT`)
    /// - offset: `  0`(x86)/` 90`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "context"))]
    #[cfg_attr(feature = "serde", serde(rename = "context"))]
    pub m_context: CONTEXT,
    /// # C++ Info
    /// - name: `sourceBoneIndex`(ctype: `hkInt16`)
    /// - offset: `  0`(x86)/` 92`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "sourceBoneIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "sourceBoneIndex"))]
    pub m_sourceBoneIndex: I16<'a>,
    /// # C++ Info
    /// - name: `offsetsCharacterRelative`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 94`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "offsetsCharacterRelative"))]
    #[cfg_attr(feature = "serde", serde(rename = "offsetsCharacterRelative"))]
    pub m_offsetsCharacterRelative: bool,
    /// # C++ Info
    /// - name: `startBoneIndex`(ctype: `hkInt16`)
    /// - offset: `  0`(x86)/` 96`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "startBoneIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "startBoneIndex"))]
    pub m_startBoneIndex: I16<'a>,
    /// # C++ Info
    /// - name: `endBoneIndex`(ctype: `hkInt16`)
    /// - offset: `  0`(x86)/` 98`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "endBoneIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "endBoneIndex"))]
    pub m_endBoneIndex: I16<'a>,
    /// # C++ Info
    /// - name: `limitHeadingDegreesCCW`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`100`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "limitHeadingDegreesCCW"))]
    #[cfg_attr(feature = "serde", serde(rename = "limitHeadingDegreesCCW"))]
    pub m_limitHeadingDegreesCCW: f32,
    /// # C++ Info
    /// - name: `limitHeadingDegreesCW`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`104`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "limitHeadingDegreesCW"))]
    #[cfg_attr(feature = "serde", serde(rename = "limitHeadingDegreesCW"))]
    pub m_limitHeadingDegreesCW: f32,
    /// # C++ Info
    /// - name: `limitPitchDegreesUp`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`108`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "limitPitchDegreesUp"))]
    #[cfg_attr(feature = "serde", serde(rename = "limitPitchDegreesUp"))]
    pub m_limitPitchDegreesUp: f32,
    /// # C++ Info
    /// - name: `limitPitchDegreesDown`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`112`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "limitPitchDegreesDown"))]
    #[cfg_attr(feature = "serde", serde(rename = "limitPitchDegreesDown"))]
    pub m_limitPitchDegreesDown: f32,
    /// # C++ Info
    /// - name: `onGain`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`116`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "onGain"))]
    #[cfg_attr(feature = "serde", serde(rename = "onGain"))]
    pub m_onGain: f32,
    /// # C++ Info
    /// - name: `offGain`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`120`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "offGain"))]
    #[cfg_attr(feature = "serde", serde(rename = "offGain"))]
    pub m_offGain: f32,
    /// # C++ Info
    /// - name: `onLeadIn`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`124`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "onLeadIn"))]
    #[cfg_attr(feature = "serde", serde(rename = "onLeadIn"))]
    pub m_onLeadIn: f32,
    /// # C++ Info
    /// - name: `offLeadIn`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`128`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "offLeadIn"))]
    #[cfg_attr(feature = "serde", serde(rename = "offLeadIn"))]
    pub m_offLeadIn: f32,
    /// # C++ Info
    /// - name: `initialHeadingOffset`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`132`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "initialHeadingOffset"))]
    #[cfg_attr(feature = "serde", serde(rename = "initialHeadingOffset"))]
    pub m_initialHeadingOffset: f32,
    /// # C++ Info
    /// - name: `initialPitchOffset`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`136`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "initialPitchOffset"))]
    #[cfg_attr(feature = "serde", serde(rename = "initialPitchOffset"))]
    pub m_initialPitchOffset: f32,
    /// # C++ Info
    /// - name: `initialGain`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`140`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "initialGain"))]
    #[cfg_attr(feature = "serde", serde(rename = "initialGain"))]
    pub m_initialGain: f32,
    /// # C++ Info
    /// - name: `allowSourceBonePoseCapture`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`144`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "allowSourceBonePoseCapture"))]
    #[cfg_attr(feature = "serde", serde(rename = "allowSourceBonePoseCapture"))]
    pub m_allowSourceBonePoseCapture: bool,
    /// # C++ Info
    /// - name: `sourceBonePoseCaptureBlendGain`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`148`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "sourceBonePoseCaptureBlendGain")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "sourceBonePoseCaptureBlendGain"))]
    pub m_sourceBonePoseCaptureBlendGain: f32,
    /// # C++ Info
    /// - name: `useCurrentSourceBonePoseEventId`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`152`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "useCurrentSourceBonePoseEventId")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "useCurrentSourceBonePoseEventId"))]
    pub m_useCurrentSourceBonePoseEventId: I32<'a>,
    /// # C++ Info
    /// - name: `counteractSourceBoneWorldSpaceDelta`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`156`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "counteractSourceBoneWorldSpaceDelta")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "counteractSourceBoneWorldSpaceDelta"))]
    pub m_counteractSourceBoneWorldSpaceDelta: bool,
    /// # C++ Info
    /// - name: `snapToTargetEventId`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`160`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "snapToTargetEventId"))]
    #[cfg_attr(feature = "serde", serde(rename = "snapToTargetEventId"))]
    pub m_snapToTargetEventId: I32<'a>,
    /// # C++ Info
    /// - name: `directAtCameraX`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`164`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "directAtCameraX"))]
    #[cfg_attr(feature = "serde", serde(rename = "directAtCameraX"))]
    pub m_directAtCameraX: f32,
    /// # C++ Info
    /// - name: `directAtCameraY`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`168`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "directAtCameraY"))]
    #[cfg_attr(feature = "serde", serde(rename = "directAtCameraY"))]
    pub m_directAtCameraY: f32,
    /// # C++ Info
    /// - name: `directAtCameraZ`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`172`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "directAtCameraZ"))]
    #[cfg_attr(feature = "serde", serde(rename = "directAtCameraZ"))]
    pub m_directAtCameraZ: f32,
    /// # C++ Info
    /// - name: `drawDebugInfo`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`176`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "drawDebugInfo"))]
    #[cfg_attr(feature = "serde", serde(rename = "drawDebugInfo"))]
    pub m_drawDebugInfo: bool,
    /// # C++ Info
    /// - name: `active`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`177`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "active"))]
    #[cfg_attr(feature = "serde", serde(rename = "active"))]
    pub m_active: bool,
    /// # C++ Info
    /// - name: `currentHeadingOffset`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`180`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "currentHeadingOffset"))]
    #[cfg_attr(feature = "serde", serde(rename = "currentHeadingOffset"))]
    pub m_currentHeadingOffset: f32,
    /// # C++ Info
    /// - name: `currentPitchOffset`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`184`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "currentPitchOffset"))]
    #[cfg_attr(feature = "serde", serde(rename = "currentPitchOffset"))]
    pub m_currentPitchOffset: f32,
    /// # C++ Info
    /// - name: `currentGain`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`188`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "currentGain"))]
    #[cfg_attr(feature = "serde", serde(rename = "currentGain"))]
    pub m_currentGain: f32,
    /// # C++ Info
    /// - name: `firstModify`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`192`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "firstModify"))]
    #[cfg_attr(feature = "serde", serde(rename = "firstModify"))]
    pub m_firstModify: bool,
    /// # C++ Info
    /// - name: `headingOffset`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`196`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "headingOffset"))]
    #[cfg_attr(feature = "serde", serde(rename = "headingOffset"))]
    pub m_headingOffset: f32,
    /// # C++ Info
    /// - name: `pitchOffset`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`200`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "pitchOffset"))]
    #[cfg_attr(feature = "serde", serde(rename = "pitchOffset"))]
    pub m_pitchOffset: f32,
    /// # C++ Info
    /// - name: `buseSourceBonePoseCapture`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`204`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "buseSourceBonePoseCapture"))]
    #[cfg_attr(feature = "serde", serde(rename = "buseSourceBonePoseCapture"))]
    pub m_buseSourceBonePoseCapture: bool,
    /// # C++ Info
    /// - name: `sourceBonePoseGain`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`208`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "sourceBonePoseGain"))]
    #[cfg_attr(feature = "serde", serde(rename = "sourceBonePoseGain"))]
    pub m_sourceBonePoseGain: f32,
    /// # C++ Info
    /// - name: `sourceBonePoseCapture`(ctype: `hkQsTransform`)
    /// - offset: `  0`(x86)/`224`(x86_64)
    /// - type_size: `  0`(x86)/` 48`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "sourceBonePoseCapture"))]
    #[cfg_attr(feature = "serde", serde(rename = "sourceBonePoseCapture"))]
    pub m_sourceBonePoseCapture: QsTransform,
    /// # C++ Info
    /// - name: `sourceBonePose`(ctype: `hkQsTransform`)
    /// - offset: `  0`(x86)/`272`(x86_64)
    /// - type_size: `  0`(x86)/` 48`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "sourceBonePose"))]
    #[cfg_attr(feature = "serde", serde(rename = "sourceBonePose"))]
    pub m_sourceBonePose: QsTransform,
    /// # C++ Info
    /// - name: `sourceBoneDirWS`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`320`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "sourceBoneDirWS"))]
    #[cfg_attr(feature = "serde", serde(rename = "sourceBoneDirWS"))]
    pub m_sourceBoneDirWS: Vector4,
    /// # C++ Info
    /// - name: `snapToTarget`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`336`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "snapToTarget"))]
    #[cfg_attr(feature = "serde", serde(rename = "snapToTarget"))]
    pub m_snapToTarget: bool,
    /// # C++ Info
    /// - name: `timeStep`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`340`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "timeStep"))]
    #[cfg_attr(feature = "serde", serde(rename = "timeStep"))]
    pub m_timeStep: f32,
    /// # C++ Info
    /// - name: `directAtTarget`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`344`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "directAtTarget"))]
    #[cfg_attr(feature = "serde", serde(rename = "directAtTarget"))]
    pub m_directAtTarget: bool,
    /// # C++ Info
    /// - name: `targetLocation`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`352`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "targetLocation"))]
    #[cfg_attr(feature = "serde", serde(rename = "targetLocation"))]
    pub m_targetLocation: Vector4,
    /// # C++ Info
    /// - name: `worldTransform`(ctype: `hkQsTransform`)
    /// - offset: `  0`(x86)/`368`(x86_64)
    /// - type_size: `  0`(x86)/` 48`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "worldTransform"))]
    #[cfg_attr(feature = "serde", serde(rename = "worldTransform"))]
    pub m_worldTransform: QsTransform,
    /// # C++ Info
    /// - name: `boneChainIndices`(ctype: `hkArray<void>`)
    /// - offset: `  0`(x86)/`416`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "boneChainIndices"))]
    #[cfg_attr(feature = "serde", serde(rename = "boneChainIndices"))]
    pub m_boneChainIndices: Vec<()>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSDirectAtModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSDirectAtModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xcda56038)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.parent.parent.parent.m_variableBindingSet);
            v.extend(self.parent.parent.parent.m_cachedBindables.iter());
            v.push(&self.parent.parent.m_nodeInfo);
            v
        }
    }
    impl<'a> _serde::Serialize for BSDirectAtModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xcda56038)));
            let mut serializer = __serializer
                .serialize_struct("BSDirectAtModifier", class_meta, (0u64, 432u64))?;
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
            serializer.serialize_field("directAtOn", &self.m_directAtOn)?;
            serializer.serialize_field("target", &self.m_target)?;
            serializer.serialize_field("context", &self.m_context)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("sourceBoneIndex", &self.m_sourceBoneIndex)?;
            serializer
                .serialize_field(
                    "offsetsCharacterRelative",
                    &self.m_offsetsCharacterRelative,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("startBoneIndex", &self.m_startBoneIndex)?;
            serializer.serialize_field("endBoneIndex", &self.m_endBoneIndex)?;
            serializer
                .serialize_field(
                    "limitHeadingDegreesCCW",
                    &self.m_limitHeadingDegreesCCW,
                )?;
            serializer
                .serialize_field(
                    "limitHeadingDegreesCW",
                    &self.m_limitHeadingDegreesCW,
                )?;
            serializer
                .serialize_field("limitPitchDegreesUp", &self.m_limitPitchDegreesUp)?;
            serializer
                .serialize_field(
                    "limitPitchDegreesDown",
                    &self.m_limitPitchDegreesDown,
                )?;
            serializer.serialize_field("onGain", &self.m_onGain)?;
            serializer.serialize_field("offGain", &self.m_offGain)?;
            serializer.serialize_field("onLeadIn", &self.m_onLeadIn)?;
            serializer.serialize_field("offLeadIn", &self.m_offLeadIn)?;
            serializer
                .serialize_field("initialHeadingOffset", &self.m_initialHeadingOffset)?;
            serializer
                .serialize_field("initialPitchOffset", &self.m_initialPitchOffset)?;
            serializer.serialize_field("initialGain", &self.m_initialGain)?;
            serializer
                .serialize_field(
                    "allowSourceBonePoseCapture",
                    &self.m_allowSourceBonePoseCapture,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field(
                    "sourceBonePoseCaptureBlendGain",
                    &self.m_sourceBonePoseCaptureBlendGain,
                )?;
            serializer
                .serialize_field(
                    "useCurrentSourceBonePoseEventId",
                    &self.m_useCurrentSourceBonePoseEventId,
                )?;
            serializer
                .serialize_field(
                    "counteractSourceBoneWorldSpaceDelta",
                    &self.m_counteractSourceBoneWorldSpaceDelta,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field("snapToTargetEventId", &self.m_snapToTargetEventId)?;
            serializer.serialize_field("directAtCameraX", &self.m_directAtCameraX)?;
            serializer.serialize_field("directAtCameraY", &self.m_directAtCameraY)?;
            serializer.serialize_field("directAtCameraZ", &self.m_directAtCameraZ)?;
            serializer.serialize_field("drawDebugInfo", &self.m_drawDebugInfo)?;
            serializer.serialize_field("active", &self.m_active)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer
                .serialize_field("currentHeadingOffset", &self.m_currentHeadingOffset)?;
            serializer
                .serialize_field("currentPitchOffset", &self.m_currentPitchOffset)?;
            serializer.serialize_field("currentGain", &self.m_currentGain)?;
            serializer.skip_field("firstModify", &self.m_firstModify)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.skip_field("headingOffset", &self.m_headingOffset)?;
            serializer.skip_field("pitchOffset", &self.m_pitchOffset)?;
            serializer
                .skip_field(
                    "buseSourceBonePoseCapture",
                    &self.m_buseSourceBonePoseCapture,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.skip_field("sourceBonePoseGain", &self.m_sourceBonePoseGain)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer
                .skip_field("sourceBonePoseCapture", &self.m_sourceBonePoseCapture)?;
            serializer.skip_field("sourceBonePose", &self.m_sourceBonePose)?;
            serializer.skip_field("sourceBoneDirWS", &self.m_sourceBoneDirWS)?;
            serializer.skip_field("snapToTarget", &self.m_snapToTarget)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.skip_field("timeStep", &self.m_timeStep)?;
            serializer.skip_field("directAtTarget", &self.m_directAtTarget)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.skip_field("targetLocation", &self.m_targetLocation)?;
            serializer.skip_field("worldTransform", &self.m_worldTransform)?;
            serializer
                .skip_array_field(
                    "boneChainIndices",
                    &self.m_boneChainIndices,
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
    impl<'de> _serde::Deserialize<'de> for BSDirectAtModifier<'de> {
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
                m_directAtOn,
                m_target,
                m_context,
                m_sourceBoneIndex,
                m_offsetsCharacterRelative,
                m_startBoneIndex,
                m_endBoneIndex,
                m_limitHeadingDegreesCCW,
                m_limitHeadingDegreesCW,
                m_limitPitchDegreesUp,
                m_limitPitchDegreesDown,
                m_onGain,
                m_offGain,
                m_onLeadIn,
                m_offLeadIn,
                m_initialHeadingOffset,
                m_initialPitchOffset,
                m_initialGain,
                m_allowSourceBonePoseCapture,
                m_sourceBonePoseCaptureBlendGain,
                m_useCurrentSourceBonePoseEventId,
                m_counteractSourceBoneWorldSpaceDelta,
                m_snapToTargetEventId,
                m_directAtCameraX,
                m_directAtCameraY,
                m_directAtCameraZ,
                m_drawDebugInfo,
                m_active,
                m_currentHeadingOffset,
                m_currentPitchOffset,
                m_currentGain,
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
                        "directAtOn" => Ok(__Field::m_directAtOn),
                        "target" => Ok(__Field::m_target),
                        "context" => Ok(__Field::m_context),
                        "sourceBoneIndex" => Ok(__Field::m_sourceBoneIndex),
                        "offsetsCharacterRelative" => {
                            Ok(__Field::m_offsetsCharacterRelative)
                        }
                        "startBoneIndex" => Ok(__Field::m_startBoneIndex),
                        "endBoneIndex" => Ok(__Field::m_endBoneIndex),
                        "limitHeadingDegreesCCW" => Ok(__Field::m_limitHeadingDegreesCCW),
                        "limitHeadingDegreesCW" => Ok(__Field::m_limitHeadingDegreesCW),
                        "limitPitchDegreesUp" => Ok(__Field::m_limitPitchDegreesUp),
                        "limitPitchDegreesDown" => Ok(__Field::m_limitPitchDegreesDown),
                        "onGain" => Ok(__Field::m_onGain),
                        "offGain" => Ok(__Field::m_offGain),
                        "onLeadIn" => Ok(__Field::m_onLeadIn),
                        "offLeadIn" => Ok(__Field::m_offLeadIn),
                        "initialHeadingOffset" => Ok(__Field::m_initialHeadingOffset),
                        "initialPitchOffset" => Ok(__Field::m_initialPitchOffset),
                        "initialGain" => Ok(__Field::m_initialGain),
                        "allowSourceBonePoseCapture" => {
                            Ok(__Field::m_allowSourceBonePoseCapture)
                        }
                        "sourceBonePoseCaptureBlendGain" => {
                            Ok(__Field::m_sourceBonePoseCaptureBlendGain)
                        }
                        "useCurrentSourceBonePoseEventId" => {
                            Ok(__Field::m_useCurrentSourceBonePoseEventId)
                        }
                        "counteractSourceBoneWorldSpaceDelta" => {
                            Ok(__Field::m_counteractSourceBoneWorldSpaceDelta)
                        }
                        "snapToTargetEventId" => Ok(__Field::m_snapToTargetEventId),
                        "directAtCameraX" => Ok(__Field::m_directAtCameraX),
                        "directAtCameraY" => Ok(__Field::m_directAtCameraY),
                        "directAtCameraZ" => Ok(__Field::m_directAtCameraZ),
                        "drawDebugInfo" => Ok(__Field::m_drawDebugInfo),
                        "active" => Ok(__Field::m_active),
                        "currentHeadingOffset" => Ok(__Field::m_currentHeadingOffset),
                        "currentPitchOffset" => Ok(__Field::m_currentPitchOffset),
                        "currentGain" => Ok(__Field::m_currentGain),
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
            struct __BSDirectAtModifierVisitor<'de> {
                marker: _serde::__private::PhantomData<BSDirectAtModifier<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __BSDirectAtModifierVisitor<'de> {
                type Value = BSDirectAtModifier<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct BSDirectAtModifier",
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
                    let mut m_directAtOn: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_target: _serde::__private::Option<TARGET> = _serde::__private::None;
                    let mut m_context: _serde::__private::Option<CONTEXT> = _serde::__private::None;
                    let mut m_sourceBoneIndex: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_offsetsCharacterRelative: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_startBoneIndex: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_endBoneIndex: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_limitHeadingDegreesCCW: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_limitHeadingDegreesCW: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_limitPitchDegreesUp: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_limitPitchDegreesDown: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_onGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_offGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_onLeadIn: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_offLeadIn: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_initialHeadingOffset: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_initialPitchOffset: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_initialGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_allowSourceBonePoseCapture: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_sourceBonePoseCaptureBlendGain: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_useCurrentSourceBonePoseEventId: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_counteractSourceBoneWorldSpaceDelta: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_snapToTargetEventId: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_directAtCameraX: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_directAtCameraY: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_directAtCameraZ: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_drawDebugInfo: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_active: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_currentHeadingOffset: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_currentPitchOffset: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_currentGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_firstModify: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_headingOffset: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_pitchOffset: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_buseSourceBonePoseCapture: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_sourceBonePoseGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_sourceBonePoseCapture: _serde::__private::Option<
                        QsTransform,
                    > = _serde::__private::None;
                    let mut m_sourceBonePose: _serde::__private::Option<QsTransform> = _serde::__private::None;
                    let mut m_sourceBoneDirWS: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_snapToTarget: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_timeStep: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_directAtTarget: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_targetLocation: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_worldTransform: _serde::__private::Option<QsTransform> = _serde::__private::None;
                    let mut m_boneChainIndices: _serde::__private::Option<Vec<()>> = _serde::__private::None;
                    for i in 0..45usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_directAtOn) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "directAtOn",
                                        ),
                                    );
                                }
                                m_directAtOn = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
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
                            2usize => {
                                if _serde::__private::Option::is_some(&m_context) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "context",
                                        ),
                                    );
                                }
                                m_context = _serde::__private::Some(
                                    match __A::next_value::<CONTEXT>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_sourceBoneIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sourceBoneIndex",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 1usize)?;
                                m_sourceBoneIndex = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_offsetsCharacterRelative,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "offsetsCharacterRelative",
                                        ),
                                    );
                                }
                                m_offsetsCharacterRelative = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_startBoneIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "startBoneIndex",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 1usize)?;
                                m_startBoneIndex = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_endBoneIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "endBoneIndex",
                                        ),
                                    );
                                }
                                m_endBoneIndex = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_limitHeadingDegreesCCW,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "limitHeadingDegreesCCW",
                                        ),
                                    );
                                }
                                m_limitHeadingDegreesCCW = _serde::__private::Some(
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
                                    &m_limitHeadingDegreesCW,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "limitHeadingDegreesCW",
                                        ),
                                    );
                                }
                                m_limitHeadingDegreesCW = _serde::__private::Some(
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
                                    &m_limitPitchDegreesUp,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "limitPitchDegreesUp",
                                        ),
                                    );
                                }
                                m_limitPitchDegreesUp = _serde::__private::Some(
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
                                    &m_limitPitchDegreesDown,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "limitPitchDegreesDown",
                                        ),
                                    );
                                }
                                m_limitPitchDegreesDown = _serde::__private::Some(
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
                                if _serde::__private::Option::is_some(
                                    &m_initialHeadingOffset,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "initialHeadingOffset",
                                        ),
                                    );
                                }
                                m_initialHeadingOffset = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
                                if _serde::__private::Option::is_some(
                                    &m_initialPitchOffset,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "initialPitchOffset",
                                        ),
                                    );
                                }
                                m_initialPitchOffset = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            17usize => {
                                if _serde::__private::Option::is_some(&m_initialGain) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "initialGain",
                                        ),
                                    );
                                }
                                m_initialGain = _serde::__private::Some(
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
                                    &m_allowSourceBonePoseCapture,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "allowSourceBonePoseCapture",
                                        ),
                                    );
                                }
                                m_allowSourceBonePoseCapture = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            19usize => {
                                if _serde::__private::Option::is_some(
                                    &m_sourceBonePoseCaptureBlendGain,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sourceBonePoseCaptureBlendGain",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 3usize)?;
                                m_sourceBonePoseCaptureBlendGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            20usize => {
                                if _serde::__private::Option::is_some(
                                    &m_useCurrentSourceBonePoseEventId,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useCurrentSourceBonePoseEventId",
                                        ),
                                    );
                                }
                                m_useCurrentSourceBonePoseEventId = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            21usize => {
                                if _serde::__private::Option::is_some(
                                    &m_counteractSourceBoneWorldSpaceDelta,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "counteractSourceBoneWorldSpaceDelta",
                                        ),
                                    );
                                }
                                m_counteractSourceBoneWorldSpaceDelta = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
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
                                __A::pad(&mut __map, 0usize, 3usize)?;
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
                                if _serde::__private::Option::is_some(&m_directAtCameraX) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "directAtCameraX",
                                        ),
                                    );
                                }
                                m_directAtCameraX = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            24usize => {
                                if _serde::__private::Option::is_some(&m_directAtCameraY) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "directAtCameraY",
                                        ),
                                    );
                                }
                                m_directAtCameraY = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            25usize => {
                                if _serde::__private::Option::is_some(&m_directAtCameraZ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "directAtCameraZ",
                                        ),
                                    );
                                }
                                m_directAtCameraZ = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            26usize => {
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
                            27usize => {
                                if _serde::__private::Option::is_some(&m_active) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("active"),
                                    );
                                }
                                m_active = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            28usize => {
                                if _serde::__private::Option::is_some(
                                    &m_currentHeadingOffset,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentHeadingOffset",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 2usize)?;
                                m_currentHeadingOffset = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            29usize => {
                                if _serde::__private::Option::is_some(
                                    &m_currentPitchOffset,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentPitchOffset",
                                        ),
                                    );
                                }
                                m_currentPitchOffset = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            30usize => {
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
                            31usize => {
                                if _serde::__private::Option::is_some(&m_firstModify) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "firstModify",
                                        ),
                                    );
                                }
                                m_firstModify = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            32usize => {
                                if _serde::__private::Option::is_some(&m_headingOffset) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "headingOffset",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 3usize)?;
                                m_headingOffset = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            33usize => {
                                if _serde::__private::Option::is_some(&m_pitchOffset) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pitchOffset",
                                        ),
                                    );
                                }
                                m_pitchOffset = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            34usize => {
                                if _serde::__private::Option::is_some(
                                    &m_buseSourceBonePoseCapture,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "buseSourceBonePoseCapture",
                                        ),
                                    );
                                }
                                m_buseSourceBonePoseCapture = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            35usize => {
                                if _serde::__private::Option::is_some(
                                    &m_sourceBonePoseGain,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sourceBonePoseGain",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 3usize)?;
                                m_sourceBonePoseGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            36usize => {
                                if _serde::__private::Option::is_some(
                                    &m_sourceBonePoseCapture,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sourceBonePoseCapture",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 12usize)?;
                                m_sourceBonePoseCapture = _serde::__private::Some(
                                    match __A::next_value::<QsTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            37usize => {
                                if _serde::__private::Option::is_some(&m_sourceBonePose) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sourceBonePose",
                                        ),
                                    );
                                }
                                m_sourceBonePose = _serde::__private::Some(
                                    match __A::next_value::<QsTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            38usize => {
                                if _serde::__private::Option::is_some(&m_sourceBoneDirWS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sourceBoneDirWS",
                                        ),
                                    );
                                }
                                m_sourceBoneDirWS = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            39usize => {
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
                            40usize => {
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
                            41usize => {
                                if _serde::__private::Option::is_some(&m_directAtTarget) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "directAtTarget",
                                        ),
                                    );
                                }
                                m_directAtTarget = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            42usize => {
                                if _serde::__private::Option::is_some(&m_targetLocation) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "targetLocation",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 7usize)?;
                                m_targetLocation = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            43usize => {
                                if _serde::__private::Option::is_some(&m_worldTransform) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "worldTransform",
                                        ),
                                    );
                                }
                                m_worldTransform = _serde::__private::Some(
                                    match __A::next_value::<QsTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            44usize => {
                                if _serde::__private::Option::is_some(&m_boneChainIndices) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneChainIndices",
                                        ),
                                    );
                                }
                                m_boneChainIndices = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
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
                    let m_directAtOn = match m_directAtOn {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "directAtOn",
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
                    let m_context = match m_context {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("context"),
                            );
                        }
                    };
                    let m_sourceBoneIndex = match m_sourceBoneIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sourceBoneIndex",
                                ),
                            );
                        }
                    };
                    let m_offsetsCharacterRelative = match m_offsetsCharacterRelative {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "offsetsCharacterRelative",
                                ),
                            );
                        }
                    };
                    let m_startBoneIndex = match m_startBoneIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "startBoneIndex",
                                ),
                            );
                        }
                    };
                    let m_endBoneIndex = match m_endBoneIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "endBoneIndex",
                                ),
                            );
                        }
                    };
                    let m_limitHeadingDegreesCCW = match m_limitHeadingDegreesCCW {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitHeadingDegreesCCW",
                                ),
                            );
                        }
                    };
                    let m_limitHeadingDegreesCW = match m_limitHeadingDegreesCW {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitHeadingDegreesCW",
                                ),
                            );
                        }
                    };
                    let m_limitPitchDegreesUp = match m_limitPitchDegreesUp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitPitchDegreesUp",
                                ),
                            );
                        }
                    };
                    let m_limitPitchDegreesDown = match m_limitPitchDegreesDown {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitPitchDegreesDown",
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
                    let m_initialHeadingOffset = match m_initialHeadingOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "initialHeadingOffset",
                                ),
                            );
                        }
                    };
                    let m_initialPitchOffset = match m_initialPitchOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "initialPitchOffset",
                                ),
                            );
                        }
                    };
                    let m_initialGain = match m_initialGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "initialGain",
                                ),
                            );
                        }
                    };
                    let m_allowSourceBonePoseCapture = match m_allowSourceBonePoseCapture {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "allowSourceBonePoseCapture",
                                ),
                            );
                        }
                    };
                    let m_sourceBonePoseCaptureBlendGain = match m_sourceBonePoseCaptureBlendGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sourceBonePoseCaptureBlendGain",
                                ),
                            );
                        }
                    };
                    let m_useCurrentSourceBonePoseEventId = match m_useCurrentSourceBonePoseEventId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useCurrentSourceBonePoseEventId",
                                ),
                            );
                        }
                    };
                    let m_counteractSourceBoneWorldSpaceDelta = match m_counteractSourceBoneWorldSpaceDelta {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "counteractSourceBoneWorldSpaceDelta",
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
                    let m_directAtCameraX = match m_directAtCameraX {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "directAtCameraX",
                                ),
                            );
                        }
                    };
                    let m_directAtCameraY = match m_directAtCameraY {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "directAtCameraY",
                                ),
                            );
                        }
                    };
                    let m_directAtCameraZ = match m_directAtCameraZ {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "directAtCameraZ",
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
                    let m_active = match m_active {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("active"),
                            );
                        }
                    };
                    let m_currentHeadingOffset = match m_currentHeadingOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentHeadingOffset",
                                ),
                            );
                        }
                    };
                    let m_currentPitchOffset = match m_currentPitchOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentPitchOffset",
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
                    let m_firstModify = match m_firstModify {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "firstModify",
                                ),
                            );
                        }
                    };
                    let m_headingOffset = match m_headingOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "headingOffset",
                                ),
                            );
                        }
                    };
                    let m_pitchOffset = match m_pitchOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pitchOffset",
                                ),
                            );
                        }
                    };
                    let m_buseSourceBonePoseCapture = match m_buseSourceBonePoseCapture {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "buseSourceBonePoseCapture",
                                ),
                            );
                        }
                    };
                    let m_sourceBonePoseGain = match m_sourceBonePoseGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sourceBonePoseGain",
                                ),
                            );
                        }
                    };
                    let m_sourceBonePoseCapture = match m_sourceBonePoseCapture {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sourceBonePoseCapture",
                                ),
                            );
                        }
                    };
                    let m_sourceBonePose = match m_sourceBonePose {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sourceBonePose",
                                ),
                            );
                        }
                    };
                    let m_sourceBoneDirWS = match m_sourceBoneDirWS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sourceBoneDirWS",
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
                    let m_directAtTarget = match m_directAtTarget {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "directAtTarget",
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
                    let m_worldTransform = match m_worldTransform {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "worldTransform",
                                ),
                            );
                        }
                    };
                    let m_boneChainIndices = match m_boneChainIndices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneChainIndices",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(BSDirectAtModifier {
                        __ptr,
                        parent,
                        m_directAtOn,
                        m_target,
                        m_context,
                        m_sourceBoneIndex,
                        m_offsetsCharacterRelative,
                        m_startBoneIndex,
                        m_endBoneIndex,
                        m_limitHeadingDegreesCCW,
                        m_limitHeadingDegreesCW,
                        m_limitPitchDegreesUp,
                        m_limitPitchDegreesDown,
                        m_onGain,
                        m_offGain,
                        m_onLeadIn,
                        m_offLeadIn,
                        m_initialHeadingOffset,
                        m_initialPitchOffset,
                        m_initialGain,
                        m_allowSourceBonePoseCapture,
                        m_sourceBonePoseCaptureBlendGain,
                        m_useCurrentSourceBonePoseEventId,
                        m_counteractSourceBoneWorldSpaceDelta,
                        m_snapToTargetEventId,
                        m_directAtCameraX,
                        m_directAtCameraY,
                        m_directAtCameraZ,
                        m_drawDebugInfo,
                        m_active,
                        m_currentHeadingOffset,
                        m_currentPitchOffset,
                        m_currentGain,
                        m_firstModify,
                        m_headingOffset,
                        m_pitchOffset,
                        m_buseSourceBonePoseCapture,
                        m_sourceBonePoseGain,
                        m_sourceBonePoseCapture,
                        m_sourceBonePose,
                        m_sourceBoneDirWS,
                        m_snapToTarget,
                        m_timeStep,
                        m_directAtTarget,
                        m_targetLocation,
                        m_worldTransform,
                        m_boneChainIndices,
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
                    let mut m_directAtOn: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_target: _serde::__private::Option<TARGET> = _serde::__private::None;
                    let mut m_context: _serde::__private::Option<CONTEXT> = _serde::__private::None;
                    let mut m_sourceBoneIndex: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_offsetsCharacterRelative: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_startBoneIndex: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_endBoneIndex: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_limitHeadingDegreesCCW: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_limitHeadingDegreesCW: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_limitPitchDegreesUp: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_limitPitchDegreesDown: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_onGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_offGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_onLeadIn: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_offLeadIn: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_initialHeadingOffset: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_initialPitchOffset: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_initialGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_allowSourceBonePoseCapture: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_sourceBonePoseCaptureBlendGain: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_useCurrentSourceBonePoseEventId: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_counteractSourceBoneWorldSpaceDelta: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_snapToTargetEventId: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_directAtCameraX: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_directAtCameraY: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_directAtCameraZ: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_drawDebugInfo: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_active: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_currentHeadingOffset: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_currentPitchOffset: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_currentGain: _serde::__private::Option<f32> = _serde::__private::None;
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
                            __Field::m_directAtOn => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_directAtOn) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "directAtOn",
                                        ),
                                    );
                                }
                                m_directAtOn = _serde::__private::Some(
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
                            __Field::m_context => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_context) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "context",
                                        ),
                                    );
                                }
                                m_context = _serde::__private::Some(
                                    match __A::next_value::<CONTEXT>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_sourceBoneIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_sourceBoneIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sourceBoneIndex",
                                        ),
                                    );
                                }
                                m_sourceBoneIndex = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_offsetsCharacterRelative => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_offsetsCharacterRelative,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "offsetsCharacterRelative",
                                        ),
                                    );
                                }
                                m_offsetsCharacterRelative = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_startBoneIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_startBoneIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "startBoneIndex",
                                        ),
                                    );
                                }
                                m_startBoneIndex = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_endBoneIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_endBoneIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "endBoneIndex",
                                        ),
                                    );
                                }
                                m_endBoneIndex = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_limitHeadingDegreesCCW => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_limitHeadingDegreesCCW,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "limitHeadingDegreesCCW",
                                        ),
                                    );
                                }
                                m_limitHeadingDegreesCCW = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_limitHeadingDegreesCW => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_limitHeadingDegreesCW,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "limitHeadingDegreesCW",
                                        ),
                                    );
                                }
                                m_limitHeadingDegreesCW = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_limitPitchDegreesUp => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_limitPitchDegreesUp,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "limitPitchDegreesUp",
                                        ),
                                    );
                                }
                                m_limitPitchDegreesUp = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_limitPitchDegreesDown => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_limitPitchDegreesDown,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "limitPitchDegreesDown",
                                        ),
                                    );
                                }
                                m_limitPitchDegreesDown = _serde::__private::Some(
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
                            __Field::m_initialHeadingOffset => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_initialHeadingOffset,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "initialHeadingOffset",
                                        ),
                                    );
                                }
                                m_initialHeadingOffset = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_initialPitchOffset => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_initialPitchOffset,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "initialPitchOffset",
                                        ),
                                    );
                                }
                                m_initialPitchOffset = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_initialGain => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_initialGain) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "initialGain",
                                        ),
                                    );
                                }
                                m_initialGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_allowSourceBonePoseCapture => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_allowSourceBonePoseCapture,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "allowSourceBonePoseCapture",
                                        ),
                                    );
                                }
                                m_allowSourceBonePoseCapture = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_sourceBonePoseCaptureBlendGain => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_sourceBonePoseCaptureBlendGain,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sourceBonePoseCaptureBlendGain",
                                        ),
                                    );
                                }
                                m_sourceBonePoseCaptureBlendGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_useCurrentSourceBonePoseEventId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_useCurrentSourceBonePoseEventId,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useCurrentSourceBonePoseEventId",
                                        ),
                                    );
                                }
                                m_useCurrentSourceBonePoseEventId = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_counteractSourceBoneWorldSpaceDelta => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_counteractSourceBoneWorldSpaceDelta,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "counteractSourceBoneWorldSpaceDelta",
                                        ),
                                    );
                                }
                                m_counteractSourceBoneWorldSpaceDelta = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
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
                            __Field::m_directAtCameraX => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_directAtCameraX) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "directAtCameraX",
                                        ),
                                    );
                                }
                                m_directAtCameraX = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_directAtCameraY => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_directAtCameraY) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "directAtCameraY",
                                        ),
                                    );
                                }
                                m_directAtCameraY = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_directAtCameraZ => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_directAtCameraZ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "directAtCameraZ",
                                        ),
                                    );
                                }
                                m_directAtCameraZ = _serde::__private::Some(
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
                            __Field::m_active => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_active) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("active"),
                                    );
                                }
                                m_active = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_currentHeadingOffset => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_currentHeadingOffset,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentHeadingOffset",
                                        ),
                                    );
                                }
                                m_currentHeadingOffset = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_currentPitchOffset => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_currentPitchOffset,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentPitchOffset",
                                        ),
                                    );
                                }
                                m_currentPitchOffset = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_currentGain => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_currentGain) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                    let m_directAtOn = match m_directAtOn {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "directAtOn",
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
                    let m_context = match m_context {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("context"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_sourceBoneIndex = match m_sourceBoneIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sourceBoneIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_offsetsCharacterRelative = match m_offsetsCharacterRelative {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "offsetsCharacterRelative",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_startBoneIndex = match m_startBoneIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "startBoneIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_endBoneIndex = match m_endBoneIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "endBoneIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_limitHeadingDegreesCCW = match m_limitHeadingDegreesCCW {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitHeadingDegreesCCW",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_limitHeadingDegreesCW = match m_limitHeadingDegreesCW {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitHeadingDegreesCW",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_limitPitchDegreesUp = match m_limitPitchDegreesUp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitPitchDegreesUp",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_limitPitchDegreesDown = match m_limitPitchDegreesDown {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "limitPitchDegreesDown",
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
                    let m_initialHeadingOffset = match m_initialHeadingOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "initialHeadingOffset",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_initialPitchOffset = match m_initialPitchOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "initialPitchOffset",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_initialGain = match m_initialGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "initialGain",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_allowSourceBonePoseCapture = match m_allowSourceBonePoseCapture {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "allowSourceBonePoseCapture",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_sourceBonePoseCaptureBlendGain = match m_sourceBonePoseCaptureBlendGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sourceBonePoseCaptureBlendGain",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_useCurrentSourceBonePoseEventId = match m_useCurrentSourceBonePoseEventId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useCurrentSourceBonePoseEventId",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_counteractSourceBoneWorldSpaceDelta = match m_counteractSourceBoneWorldSpaceDelta {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "counteractSourceBoneWorldSpaceDelta",
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
                    let m_directAtCameraX = match m_directAtCameraX {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "directAtCameraX",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_directAtCameraY = match m_directAtCameraY {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "directAtCameraY",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_directAtCameraZ = match m_directAtCameraZ {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "directAtCameraZ",
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
                    let m_active = match m_active {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("active"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_currentHeadingOffset = match m_currentHeadingOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentHeadingOffset",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_currentPitchOffset = match m_currentPitchOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentPitchOffset",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_currentGain = match m_currentGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentGain",
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
                    _serde::__private::Ok(BSDirectAtModifier {
                        __ptr: __ptr.clone(),
                        parent,
                        m_directAtOn,
                        m_target,
                        m_context,
                        m_sourceBoneIndex,
                        m_offsetsCharacterRelative,
                        m_startBoneIndex,
                        m_endBoneIndex,
                        m_limitHeadingDegreesCCW,
                        m_limitHeadingDegreesCW,
                        m_limitPitchDegreesUp,
                        m_limitPitchDegreesDown,
                        m_onGain,
                        m_offGain,
                        m_onLeadIn,
                        m_offLeadIn,
                        m_initialHeadingOffset,
                        m_initialPitchOffset,
                        m_initialGain,
                        m_allowSourceBonePoseCapture,
                        m_sourceBonePoseCaptureBlendGain,
                        m_useCurrentSourceBonePoseEventId,
                        m_counteractSourceBoneWorldSpaceDelta,
                        m_snapToTargetEventId,
                        m_directAtCameraX,
                        m_directAtCameraY,
                        m_directAtCameraZ,
                        m_drawDebugInfo,
                        m_active,
                        m_currentHeadingOffset,
                        m_currentPitchOffset,
                        m_currentGain,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "directAtOn",
                "target",
                "context",
                "sourceBoneIndex",
                "offsetsCharacterRelative",
                "startBoneIndex",
                "endBoneIndex",
                "limitHeadingDegreesCCW",
                "limitHeadingDegreesCW",
                "limitPitchDegreesUp",
                "limitPitchDegreesDown",
                "onGain",
                "offGain",
                "onLeadIn",
                "offLeadIn",
                "initialHeadingOffset",
                "initialPitchOffset",
                "initialGain",
                "allowSourceBonePoseCapture",
                "sourceBonePoseCaptureBlendGain",
                "useCurrentSourceBonePoseEventId",
                "counteractSourceBoneWorldSpaceDelta",
                "snapToTargetEventId",
                "directAtCameraX",
                "directAtCameraY",
                "directAtCameraZ",
                "drawDebugInfo",
                "active",
                "currentHeadingOffset",
                "currentPitchOffset",
                "currentGain",
                "firstModify",
                "headingOffset",
                "pitchOffset",
                "buseSourceBonePoseCapture",
                "sourceBonePoseGain",
                "sourceBonePoseCapture",
                "sourceBonePose",
                "sourceBoneDirWS",
                "snapToTarget",
                "timeStep",
                "directAtTarget",
                "targetLocation",
                "worldTransform",
                "boneChainIndices",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "BSDirectAtModifier",
                FIELDS,
                __BSDirectAtModifierVisitor {
                    marker: _serde::__private::PhantomData::<BSDirectAtModifier>,
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
    AppSupplied = 0isize,
    Camera = 1isize,
}
/// # C++ Info
/// - name: `CONTEXT`(ctype: `hkEnum<CONTEXT, hkInt8>`)
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
pub enum CONTEXT {
    #[default]
    None = 0isize,
    BlockAnticipate = 1isize,
    BowAttack = 2isize,
    GunAttack = 3isize,
    MagicAttack = 4isize,
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
                Self::AppSupplied => __serializer.serialize_field("AppSupplied", &0u64),
                Self::Camera => __serializer.serialize_field("Camera", &1u64),
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
    impl __serde::Serialize for CONTEXT {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::None => __serializer.serialize_field("None", &0u64),
                Self::BlockAnticipate => {
                    __serializer.serialize_field("BlockAnticipate", &1u64)
                }
                Self::BowAttack => __serializer.serialize_field("BowAttack", &2u64),
                Self::GunAttack => __serializer.serialize_field("GunAttack", &3u64),
                Self::MagicAttack => __serializer.serialize_field("MagicAttack", &4u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self.to_i8().ok_or(S::Error::custom("Failed enum CONTEXT to_i8"))?;
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
                            v if v == "0" || v.eq_ignore_ascii_case("AppSupplied") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("Camera") => {
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
                            _serde::__private::Ok(TARGET::AppSupplied)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TARGET::Camera)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &["AppSupplied", "Camera"];
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
    impl<'de> _serde::Deserialize<'de> for CONTEXT {
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
                        I8::Number(3i8) => _serde::__private::Ok(__Field::__field3),
                        I8::Number(4i8) => _serde::__private::Ok(__Field::__field4),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1, 2, 3, 4",
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
                            v if v == "0" || v.eq_ignore_ascii_case("None") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("BlockAnticipate") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2" || v.eq_ignore_ascii_case("BowAttack") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3" || v.eq_ignore_ascii_case("GunAttack") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4" || v.eq_ignore_ascii_case("MagicAttack") => {
                                _serde::__private::Ok(__Field::__field4)
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
                marker: _serde::__private::PhantomData<CONTEXT>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = CONTEXT;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "enum CONTEXT")
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
                            _serde::__private::Ok(CONTEXT::None)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(CONTEXT::BlockAnticipate)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(CONTEXT::BowAttack)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(CONTEXT::GunAttack)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(CONTEXT::MagicAttack)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "None",
                "BlockAnticipate",
                "BowAttack",
                "GunAttack",
                "MagicAttack",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "CONTEXT",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<CONTEXT>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
