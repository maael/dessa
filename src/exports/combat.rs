use arcdps_bindings::{cbtevent, Ag, AgOwned};

pub fn cbt(
    ev: Option<&cbtevent>,
    src: Option<&Ag>,
    dst: Option<&Ag>,
    skillname: Option<&'static str>,
    id: u64,
    revision: u64,
) {
    spawn_cbt(ev, src, dst, skillname, id, revision, 2);
}

pub fn cbt_local(
    ev: Option<&cbtevent>,
    src: Option<&Ag>,
    dst: Option<&Ag>,
    skillname: Option<&'static str>,
    id: u64,
    revision: u64,
) {
    spawn_cbt(ev, src, dst, skillname, id, revision, 3);
}

fn spawn_cbt(
    ev: Option<&cbtevent>,
    src: Option<&Ag>,
    dst: Option<&Ag>,
    skillname: Option<&'static str>,
    id: u64,
    revision: u64,
    indicator: u8,
) {
    log::info!("spawn_cbt skill: {:?}", skillname);
    cbt_with_type(
        ev.copied(),
        src.map(|x| (*x).into()),
        dst.map(|x| (*x).into()),
        skillname,
        id,
        revision,
        indicator,
    )
    // TODO: Do something with this
    // Task::spawn(cbt_with_type(
    //     ev.copied(),
    //     src.map(|x| (*x).into()),
    //     dst.map(|x| (*x).into()),
    //     skillname,
    //     id,
    //     revision,
    //     indicator,
    // ))
    // .detach();
}

fn cbt_with_type(
    ev: Option<cbtevent>,
    src: Option<AgOwned>,
    dst: Option<AgOwned>,
    skillname: Option<&'static str>,
    id: u64,
    revision: u64,
    indicator: u8,
) {
    let mut message = Vec::new();
    message.push(indicator); // indicator for local/area combat message
    add_bytes(&mut message, ev, src, dst, skillname, id, revision);
    // TODO: Do something
    // dispatch(message).await;
}

fn add_bytes(
    message: &mut Vec<u8>,
    ev: Option<cbtevent>,
    src: Option<AgOwned>,
    dst: Option<AgOwned>,
    skillname: Option<&str>,
    id: u64,
    revision: u64,
) {
    let mut messages = 0;
    if let Some(ev) = ev {
        messages |= 1;
        log::info!("get_ev_bytes skill: {:?} {:?} {:?}", ev.src_agent.to_string(), ev.dst_agent.to_string(), skillname);
        let mut bytes = get_ev_bytes(&ev);
        message.append(&mut bytes);
    };
    if let Some(ag) = src {
        messages |= 1 << 1;
        log::info!("get_ag_bytes src skill: {:?} {:?}", ag.id.to_string(), skillname);
        let mut bytes = get_ag_bytes(&ag);
        message.append(&mut bytes);
    };
    if let Some(ag) = dst {
        messages |= 1 << 2;
        log::info!("get_ag_bytes dst skill: {:?} {:?}", ag.id.to_string(), skillname);
        let mut bytes = get_ag_bytes(&ag);
        message.append(&mut bytes);
    };
    if let Some(name) = skillname {
        messages |= 1 << 3;
        let bytes = name.as_bytes();
        let mut bytes = [&bytes.len().to_le_bytes(), bytes].concat();
        message.append(&mut bytes);
    };
    message.insert(1, messages);
    message.append(&mut id.to_le_bytes().to_vec());
    message.append(&mut revision.to_le_bytes().to_vec());
}

fn get_ev_bytes(ev: &cbtevent) -> Vec<u8> {
    ev.time
        .to_le_bytes()
        .iter()
        .chain(ev.src_agent.to_le_bytes().iter())
        .chain(ev.dst_agent.to_le_bytes().iter())
        .chain(ev.value.to_le_bytes().iter())
        .chain(ev.buff_dmg.to_le_bytes().iter())
        .chain(ev.overstack_value.to_le_bytes().iter())
        .chain(ev.skillid.to_le_bytes().iter())
        .chain(ev.src_instid.to_le_bytes().iter())
        .chain(ev.dst_instid.to_le_bytes().iter())
        .chain(ev.src_master_instid.to_le_bytes().iter())
        .chain(ev.dst_master_instid.to_le_bytes().iter())
        .chain(ev.iff.to_le_bytes().iter())
        .chain(ev.buff.to_le_bytes().iter())
        .chain(ev.result.to_le_bytes().iter())
        .chain(ev.is_activation.to_le_bytes().iter())
        .chain(ev.is_buffremove.to_le_bytes().iter())
        .chain(ev.is_ninety.to_le_bytes().iter())
        .chain(ev.is_fifty.to_le_bytes().iter())
        .chain(ev.is_moving.to_le_bytes().iter())
        .chain(ev.is_statechange.to_le_bytes().iter())
        .chain(ev.is_flanking.to_le_bytes().iter())
        .chain(ev.is_shields.to_le_bytes().iter())
        .chain(ev.is_offcycle.to_le_bytes().iter())
        .chain(ev.pad61.to_le_bytes().iter())
        .chain(ev.pad62.to_le_bytes().iter())
        .chain(ev.pad63.to_le_bytes().iter())
        .chain(ev.pad64.to_le_bytes().iter())
        .cloned()
        .collect::<Vec<u8>>()
}

fn get_ag_bytes(ag: &AgOwned) -> Vec<u8> {
    let (string_length, name_bytes) = if let Some(name) = &ag.name {
        let bytes = name.as_bytes();
        (bytes.len(), Some(bytes))
    } else {
        (0, None)
    };
    log::info!("get_ag_bytes: {:?} {:?}", ag.id.to_string(), ag.name);
    if let Some(name_bytes) = name_bytes {
        string_length
            .to_le_bytes()
            .iter()
            .chain(name_bytes.iter())
            .chain(ag.id.to_le_bytes().iter())
            .chain(ag.prof.to_le_bytes().iter())
            .chain(ag.elite.to_le_bytes().iter())
            .chain(ag.self_.to_le_bytes().iter())
            .chain(ag.team.to_le_bytes().iter())
            .cloned()
            .collect()
    } else {
        string_length
            .to_le_bytes()
            .iter()
            .chain(ag.id.to_le_bytes().iter())
            .chain(ag.prof.to_le_bytes().iter())
            .chain(ag.elite.to_le_bytes().iter())
            .chain(ag.self_.to_le_bytes().iter())
            .chain(ag.team.to_le_bytes().iter())
            .cloned()
            .collect()
    }
}
