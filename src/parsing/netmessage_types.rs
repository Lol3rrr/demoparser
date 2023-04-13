use NetmessageType::*;

pub fn netmessage_type_from_int(msg_type: i32) -> NetmessageType {
    match msg_type {
        0 => net_NOP,
        1 => net_Disconnect,
        3 => net_SplitScreenUser,
        4 => net_Tick,
        5 => net_StringCmd,
        6 => net_SetConVar,
        7 => net_SignonState,
        8 => net_SpawnGroup_Load,
        9 => net_SpawnGroup_ManifestUpdate,
        11 => net_SpawnGroup_SetCreationTick,
        12 => net_SpawnGroup_Unload,
        13 => net_SpawnGroup_LoadCompleted,
        15 => net_DebugOverlay,
        40 => svc_ServerInfo,
        41 => svc_FlattenedSerializer,
        42 => svc_ClassInfo,
        43 => svc_SetPause,
        44 => svc_CreateStringTable,
        45 => svc_UpdateStringTable,
        46 => svc_VoiceInit,
        47 => svc_VoiceData,
        48 => svc_Print,
        49 => svc_Sounds,
        50 => svc_SetView,
        51 => svc_ClearAllStringTables,
        52 => svc_CmdKeyValues,
        53 => svc_BSPDecal,
        54 => svc_SplitScreen,
        55 => svc_PacketEntities,
        56 => svc_Prefetch,
        57 => svc_Menu,
        58 => svc_GetCvarValue,
        59 => svc_StopSound,
        60 => svc_PeerList,
        61 => svc_PacketReliable,
        62 => svc_HLTVStatus,
        63 => svc_ServerSteamID,
        70 => svc_FullFrameSplit,
        71 => svc_RconServerDetails,
        72 => svc_UserMessage,
        73 => svc_HltvReplay,
        74 => svc_Broadcast_Command,
        101 => UM_AchievementEvent,
        102 => UM_CloseCaption,
        103 => UM_CloseCaptionDirect,
        104 => UM_CurrentTimescale,
        105 => UM_DesiredTimescale,
        106 => UM_Fade,
        107 => UM_GameTitle,
        110 => UM_HudMsg,
        111 => UM_HudText,
        113 => UM_ColoredText,
        114 => UM_RequestState,
        115 => UM_ResetHUD,
        116 => UM_Rumble,
        117 => UM_SayText,
        118 => UM_SayText2,
        119 => UM_SayTextChannel,
        120 => UM_Shake,
        121 => UM_ShakeDir,
        124 => UM_TextMsg,
        125 => UM_ScreenTilt,
        128 => UM_VoiceMask,
        130 => UM_SendAudio,
        131 => UM_ItemPickup,
        132 => UM_AmmoDenied,
        134 => UM_ShowMenu,
        135 => UM_CreditsMsg,
        142 => UM_CloseCaptionPlaceholder,
        143 => UM_CameraTransition,
        144 => UM_AudioParameter,
        145 => UM_ParticleManager,
        146 => UM_HudError,
        148 => UM_CustomGameEvent,
        149 => UM_AnimGraphUpdate,
        150 => UM_HapticsManagerPulse,
        151 => UM_HapticsManagerEffect,
        152 => UM_CommandQueueState,
        153 => UM_UpdateCssClasses,
        154 => UM_ServerFrameTime,
        155 => UM_LagCompensationError,
        156 => UM_RequestDllStatus,
        157 => UM_RequestUtilAction,
        158 => UM_UtilActionResponse,
        159 => UM_DllStatusResponse,
        160 => UM_RequestInventory,
        161 => UM_InventoryResponse,
        200 => GE_VDebugGameSessionIDEvent,
        200 => UM_MAX_BASE,
        201 => GE_PlaceDecalEvent,
        202 => GE_ClearWorldDecalsEvent,
        203 => GE_ClearEntityDecalsEvent,
        204 => GE_ClearDecalsForSkeletonInstanceEvent,
        205 => GE_Source1LegacyGameEventList,
        206 => GE_Source1LegacyListenEvents,
        207 => GE_Source1LegacyGameEvent,
        208 => GE_SosStartSoundEvent,
        209 => GE_SosStopSoundEvent,
        210 => GE_SosSetSoundEventParams,
        211 => GE_SosSetLibraryStackFields,
        212 => GE_SosStopSoundEventHash,
        301 => CS_UM_VGUIMenu,
        302 => CS_UM_Geiger,
        303 => CS_UM_Train,
        304 => CS_UM_HudText,
        305 => CS_UM_SayText,
        306 => CS_UM_SayText2,
        307 => CS_UM_TextMsg,
        308 => CS_UM_HudMsg,
        309 => CS_UM_ResetHud,
        310 => CS_UM_GameTitle,
        312 => CS_UM_Shake,
        313 => CS_UM_Fade,
        314 => CS_UM_Rumble,
        315 => CS_UM_CloseCaption,
        316 => CS_UM_CloseCaptionDirect,
        317 => CS_UM_SendAudio,
        318 => CS_UM_RawAudio,
        319 => CS_UM_VoiceMask,
        320 => CS_UM_RequestState,
        321 => CS_UM_Damage,
        322 => CS_UM_RadioText,
        323 => CS_UM_HintText,
        324 => CS_UM_KeyHintText,
        325 => CS_UM_ProcessSpottedEntityUpdate,
        326 => CS_UM_ReloadEffect,
        327 => CS_UM_AdjustMoney,
        328 => CS_UM_UpdateTeamMoney,
        329 => CS_UM_StopSpectatorMode,
        330 => CS_UM_KillCam,
        331 => CS_UM_DesiredTimescale,
        332 => CS_UM_CurrentTimescale,
        333 => CS_UM_AchievementEvent,
        334 => CS_UM_MatchEndConditions,
        335 => CS_UM_DisconnectToLobby,
        336 => CS_UM_PlayerStatsUpdate,
        338 => CS_UM_WarmupHasEnded,
        339 => CS_UM_ClientInfo,
        340 => CS_UM_XRankGet,
        341 => CS_UM_XRankUpd,
        345 => CS_UM_CallVoteFailed,
        346 => CS_UM_VoteStart,
        347 => CS_UM_VotePass,
        348 => CS_UM_VoteFailed,
        349 => CS_UM_VoteSetup,
        350 => CS_UM_ServerRankRevealAll,
        351 => CS_UM_SendLastKillerDamageToClient,
        352 => CS_UM_ServerRankUpdate,
        353 => CS_UM_ItemPickup,
        354 => CS_UM_ShowMenu,
        355 => CS_UM_BarTime,
        356 => CS_UM_AmmoDenied,
        357 => CS_UM_MarkAchievement,
        358 => CS_UM_MatchStatsUpdate,
        359 => CS_UM_ItemDrop,
        360 => CS_UM_GlowPropTurnOff,
        361 => CS_UM_SendPlayerItemDrops,
        362 => CS_UM_RoundBackupFilenames,
        363 => CS_UM_SendPlayerItemFound,
        364 => CS_UM_ReportHit,
        365 => CS_UM_XpUpdate,
        366 => CS_UM_QuestProgress,
        367 => CS_UM_ScoreLeaderboardData,
        368 => CS_UM_PlayerDecalDigitalSignature,
        369 => CS_UM_WeaponSound,
        370 => CS_UM_UpdateScreenHealthBar,
        371 => CS_UM_EntityOutlineHighlight,
        372 => CS_UM_SSUI,
        373 => CS_UM_SurvivalStats,
        374 => CS_UM_DisconnectToLobby2,
        375 => CS_UM_EndOfMatchAllPlayersData,
        376 => CS_UM_PostRoundDamageReport,
        379 => CS_UM_RoundEndReportData,
        380 => CS_UM_CurrentRoundOdds,
        381 => CS_UM_DeepStats,
        382 => CS_UM_UtilMsg,
        383 => CS_UM_ShootInfo,
        _ => Unknown,
    }
}
#[derive(Debug, PartialEq)]
#[warn(non_camel_case_types)]
pub enum NetmessageType {
    Unknown,
    net_NOP,
    net_Disconnect,
    net_SplitScreenUser,
    net_Tick,
    net_StringCmd,
    net_SetConVar,
    net_SignonState,
    net_SpawnGroup_Load,
    net_SpawnGroup_ManifestUpdate,
    net_SpawnGroup_SetCreationTick,
    net_SpawnGroup_Unload,
    net_SpawnGroup_LoadCompleted,
    net_DebugOverlay,
    svc_ServerInfo,
    svc_FlattenedSerializer,
    svc_ClassInfo,
    svc_SetPause,
    svc_CreateStringTable,
    svc_UpdateStringTable,
    svc_VoiceInit,
    svc_VoiceData,
    svc_Print,
    svc_Sounds,
    svc_SetView,
    svc_ClearAllStringTables,
    svc_CmdKeyValues,
    svc_BSPDecal,
    svc_SplitScreen,
    svc_PacketEntities,
    svc_Prefetch,
    svc_Menu,
    svc_GetCvarValue,
    svc_StopSound,
    svc_PeerList,
    svc_PacketReliable,
    svc_HLTVStatus,
    svc_ServerSteamID,
    svc_FullFrameSplit,
    svc_RconServerDetails,
    svc_UserMessage,
    svc_HltvReplay,
    svc_Broadcast_Command,
    GE_VDebugGameSessionIDEvent,
    GE_PlaceDecalEvent,
    GE_ClearWorldDecalsEvent,
    GE_ClearEntityDecalsEvent,
    GE_ClearDecalsForSkeletonInstanceEvent,
    GE_Source1LegacyGameEventList,
    GE_Source1LegacyListenEvents,
    GE_Source1LegacyGameEvent,
    GE_SosStartSoundEvent,
    GE_SosStopSoundEvent,
    GE_SosSetSoundEventParams,
    GE_SosSetLibraryStackFields,
    GE_SosStopSoundEventHash,
    CS_UM_VGUIMenu,
    CS_UM_Geiger,
    CS_UM_Train,
    CS_UM_HudText,
    CS_UM_SayText,
    CS_UM_SayText2,
    CS_UM_TextMsg,
    CS_UM_HudMsg,
    CS_UM_ResetHud,
    CS_UM_GameTitle,
    CS_UM_Shake,
    CS_UM_Fade,
    CS_UM_Rumble,
    CS_UM_CloseCaption,
    CS_UM_CloseCaptionDirect,
    CS_UM_SendAudio,
    CS_UM_RawAudio,
    CS_UM_VoiceMask,
    CS_UM_RequestState,
    CS_UM_Damage,
    CS_UM_RadioText,
    CS_UM_HintText,
    CS_UM_KeyHintText,
    CS_UM_ProcessSpottedEntityUpdate,
    CS_UM_ReloadEffect,
    CS_UM_AdjustMoney,
    CS_UM_UpdateTeamMoney,
    CS_UM_StopSpectatorMode,
    CS_UM_KillCam,
    CS_UM_DesiredTimescale,
    CS_UM_CurrentTimescale,
    CS_UM_AchievementEvent,
    CS_UM_MatchEndConditions,
    CS_UM_DisconnectToLobby,
    CS_UM_PlayerStatsUpdate,
    CS_UM_WarmupHasEnded,
    CS_UM_ClientInfo,
    CS_UM_XRankGet,
    CS_UM_XRankUpd,
    CS_UM_CallVoteFailed,
    CS_UM_VoteStart,
    CS_UM_VotePass,
    CS_UM_VoteFailed,
    CS_UM_VoteSetup,
    CS_UM_ServerRankRevealAll,
    CS_UM_SendLastKillerDamageToClient,
    CS_UM_ServerRankUpdate,
    CS_UM_ItemPickup,
    CS_UM_ShowMenu,
    CS_UM_BarTime,
    CS_UM_AmmoDenied,
    CS_UM_MarkAchievement,
    CS_UM_MatchStatsUpdate,
    CS_UM_ItemDrop,
    CS_UM_GlowPropTurnOff,
    CS_UM_SendPlayerItemDrops,
    CS_UM_RoundBackupFilenames,
    CS_UM_SendPlayerItemFound,
    CS_UM_ReportHit,
    CS_UM_XpUpdate,
    CS_UM_QuestProgress,
    CS_UM_ScoreLeaderboardData,
    CS_UM_PlayerDecalDigitalSignature,
    CS_UM_WeaponSound,
    CS_UM_UpdateScreenHealthBar,
    CS_UM_EntityOutlineHighlight,
    CS_UM_SSUI,
    CS_UM_SurvivalStats,
    CS_UM_DisconnectToLobby2,
    CS_UM_EndOfMatchAllPlayersData,
    CS_UM_PostRoundDamageReport,
    CS_UM_RoundEndReportData,
    CS_UM_CurrentRoundOdds,
    CS_UM_DeepStats,
    CS_UM_UtilMsg,
    CS_UM_ShootInfo,
    UM_AchievementEvent,
    UM_CloseCaption,
    UM_CloseCaptionDirect,
    UM_CurrentTimescale,
    UM_DesiredTimescale,
    UM_Fade,
    UM_GameTitle,
    UM_HudMsg,
    UM_HudText,
    UM_ColoredText,
    UM_RequestState,
    UM_ResetHUD,
    UM_Rumble,
    UM_SayText,
    UM_SayText2,
    UM_SayTextChannel,
    UM_Shake,
    UM_ShakeDir,
    UM_TextMsg,
    UM_ScreenTilt,
    UM_VoiceMask,
    UM_SendAudio,
    UM_ItemPickup,
    UM_AmmoDenied,
    UM_ShowMenu,
    UM_CreditsMsg,
    UM_CloseCaptionPlaceholder,
    UM_CameraTransition,
    UM_AudioParameter,
    UM_ParticleManager,
    UM_HudError,
    UM_CustomGameEvent,
    UM_AnimGraphUpdate,
    UM_HapticsManagerPulse,
    UM_HapticsManagerEffect,
    UM_CommandQueueState,
    UM_UpdateCssClasses,
    UM_ServerFrameTime,
    UM_LagCompensationError,
    UM_RequestDllStatus,
    UM_RequestUtilAction,
    UM_UtilActionResponse,
    UM_DllStatusResponse,
    UM_RequestInventory,
    UM_InventoryResponse,
    UM_MAX_BASE,
}
