use crate::consts::{BaMessage, SessionCommand};

fn parse_to_session_command(command: &[u8]) -> SessionCommand {
    let command_type = command[0];
    match command[0] {
        0 => SessionCommand::BaseTimeStep,
        1 => SessionCommand::StepSceneGraph,
        2 => SessionCommand::AddSceneGraph,
        3 => SessionCommand::RemoveSceneGraph,
        4 => SessionCommand::AddNode,
        5 => SessionCommand::NodeOnCreate,
        6 => SessionCommand::SetForegroundSceneGraph,
        7 => SessionCommand::RemoveNode,
        8 => SessionCommand::AddMaterial,
        9 => SessionCommand::RemoveMaterial,
        10 => SessionCommand::AddMaterialComponent,
        11 => SessionCommand::AddTexture,
        12 => SessionCommand::RemoveTexture,
        13 => SessionCommand::AddModel,
        14 => SessionCommand::RemoveModel,
        15 => SessionCommand::AddSound,
        16 => SessionCommand::RemoveSound,
        17 => SessionCommand::AddCollideModel,
        18 => SessionCommand::RemoveCollideModel,
        19 => SessionCommand::ConnectNodeAttribute,
        20 => SessionCommand::NodeMessage,
        21 => SessionCommand::SetNodeAttrFloat,
        22 => SessionCommand::SetNodeAttrInt32,
        23 => SessionCommand::SetNodeAttrBool,
        24 => SessionCommand::SetNodeAttrFloats,
        25 => SessionCommand::SetNodeAttrInt32s,
        26 => {
            let string = command[13..]
                .iter()
                .take_while(|&&c| c != 0)
                .map(|&c| c as char)
                .collect::<String>();
            SessionCommand::SetNodeAttrString(string)
        }
        27 => SessionCommand::SetNodeAttrNode,
        28 => SessionCommand::SetNodeAttrNodeNull,
        29 => SessionCommand::SetNodeAttrNodes,
        30 => SessionCommand::SetNodeAttrPlayer,
        31 => SessionCommand::SetNodeAttrPlayerNull,
        32 => SessionCommand::SetNodeAttrMaterials,
        33 => SessionCommand::SetNodeAttrTexture,
        34 => SessionCommand::SetNodeAttrTextureNull,
        35 => SessionCommand::SetNodeAttrTextures,
        36 => SessionCommand::SetNodeAttrSound,
        37 => SessionCommand::SetNodeAttrSoundNull,
        38 => SessionCommand::SetNodeAttrSounds,
        39 => SessionCommand::SetNodeAttrModel,
        40 => SessionCommand::SetNodeAttrModelNull,
        41 => SessionCommand::SetNodeAttrModels,
        42 => SessionCommand::SetNodeAttrCollideModel,
        43 => SessionCommand::SetNodeAttrCollideModelNull,
        44 => SessionCommand::SetNodeAttrCollideModels,
        45 => SessionCommand::PlaySoundAtPosition,
        46 => SessionCommand::PlaySound,
        47 => SessionCommand::EmitBGDynamics,
        48 => SessionCommand::EndOfFile,
        49 => SessionCommand::DynamicsCorrection,
        50 => SessionCommand::ScreenMessageBottom,
        51 => SessionCommand::ScreenMessageTop,
        52 => SessionCommand::AddData,
        53 => SessionCommand::RemoveData,
        _ => panic!("invalid command type: {}", command_type),
    }
}

fn handle_commands(buffer: &[u8]) -> Vec<SessionCommand> {
    let mut offset = 1;
    let mut commands: Vec<SessionCommand> = Vec::new();
    loop {
        let size: usize = u16::from_le_bytes({
            let buf: [u8; 2]
                = buffer[offset..][..2].try_into().unwrap();
            buf
        }) as usize;
        if offset + size > buffer.len() {
            panic!("invalid state message");
        }
        let sub_buffer = &buffer[offset + 2..][..size];

        let command = parse_to_session_command(sub_buffer);
        commands.push(command);

        offset += 2 + size;

        if offset == buffer.len() {
            break;
        }
    }
    commands
}

pub fn handle_session_message(buffer: &[u8]) -> BaMessage {
    match buffer[0] {
        0 => BaMessage::SessionReset,
        1 => {
            let commands = handle_commands(buffer);
            BaMessage::SessionCommands(commands)
        }
        2 => BaMessage::SessionDynamicsCorrection,
        3 => BaMessage::Null,
        4 => BaMessage::RequestRemotePlayer,
        5 => BaMessage::AttachRemotePlayer,
        6 => BaMessage::DetachRemotePlayer,
        7 => BaMessage::RemotePlayerInputCommands,
        8 => BaMessage::RemoveRemotePlayer,
        9 => BaMessage::PartyRoster,
        10 => BaMessage::Chat,
        11 => BaMessage::PartyMemberJoined,
        12 => BaMessage::PartyMemberLeft,
        13 => BaMessage::Multipart,
        14 => BaMessage::MultipartEnd,
        15 => BaMessage::ClientPlayerProfiles,
        16 => BaMessage::AttachRemotePlayer2,
        17 => BaMessage::HostInfo,
        18 => BaMessage::ClientInfo,
        19 => BaMessage::KickVote,
        20 => BaMessage::JMessage,
        21 => BaMessage::ClientPlayerProfilesJson,
        _ => panic!("invalid message type: {}", buffer[0]),
    }
}
