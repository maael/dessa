use arcdps_bindings::{cbtevent, Ag, AgOwned};
use std::sync::mpsc::{Sender};
use serde_json::json;

use crate::emitter;
use emitter::EVENT_EMITTER;

pub fn wrapped_cbt(_tx: Sender<String>) -> fn(
    ev: Option<&cbtevent>,
    src: Option<&Ag>,
    dst: Option<&Ag>,
    skillname: Option<&'static str>,
    id: u64,
    revision: u64,
) -> () {
    move |ev, src, dst, skillname, id, revision| {
        cbt(ev, src, dst, skillname, id, revision)
    }
}

pub fn wrapped_cbt_local(_tx: Sender<String>) -> fn(
    ev: Option<&cbtevent>,
    src: Option<&Ag>,
    dst: Option<&Ag>,
    skillname: Option<&'static str>,
    id: u64,
    revision: u64,
) -> () {
    move |ev, src, dst, skillname, id, revision| {
        cbt_local(ev, src, dst, skillname, id, revision)
    }
}

pub fn cbt(
    ev: Option<&cbtevent>,
    src: Option<&Ag>,
    dst: Option<&Ag>,
    skillname: Option<&'static str>,
    id: u64,
    revision: u64
) {
    spawn_cbt(ev, src, dst, skillname, id, revision, 2);
}

pub fn cbt_local(
    ev: Option<&cbtevent>,
    src: Option<&Ag>,
    dst: Option<&Ag>,
    skillname: Option<&'static str>,
    id: u64,
    revision: u64
) -> () {
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
    add_bytes(&mut message, ev, src, dst, skillname, id, revision, indicator);
}

fn add_bytes(
    message: &mut Vec<u8>,
    ev: Option<cbtevent>,
    src: Option<AgOwned>,
    dst: Option<AgOwned>,
    skillname: Option<&str>,
    id: u64,
    revision: u64,
    indicator: u8,
) {
    let mut messages = 0;
    if let Some(ev) = ev {
        messages |= 1;
        EVENT_EMITTER.lock().unwrap().emit("arc", json!({
            "type": "arc",
            "sub_type": "ev_bytes",
            "indicator": indicator,
            "skillname": skillname,
            "id": id,
            "revision": revision,
            "src_agent": ev.src_agent.to_string(),
            "dst_agent": ev.dst_agent.to_string(),
            "value": ev.value,
            "buff_dmg": ev.buff_dmg,
            "overstack_value": ev.overstack_value,
            "skillid": ev.skillid,
            "src_instid": ev.src_instid,
            "dst_instid": ev.dst_instid,
            "src_master_instid": ev.src_master_instid,
            "dst_master_instid": ev.dst_master_instid,
            "iff": ev.iff,
            "buff": ev.buff,
            "result": ev.result,
            "is_activation": ev.is_activation,
            "is_buffremove": ev.is_buffremove,
            "is_ninety": ev.is_ninety,
            "is_fifty": ev.is_fifty,
            "is_moving": ev.is_moving,
            "is_statechange": ev.is_statechange,
            "is_flanking": ev.is_flanking,
            "is_shields": ev.is_shields,
            "is_offcycle": ev.is_offcycle,
            "pad61": ev.pad61,
            "pad62": ev.pad62,
            "pad63": ev.pad63,
            "pad64": ev.pad64,
        }).to_string());
        let mut bytes = get_ev_bytes(&ev);
        message.append(&mut bytes);
    };
    if let Some(ag) = src {
        messages |= 1 << 1;
        EVENT_EMITTER.lock().unwrap().emit("arc", json!({
            "type": "arc",
            "sub_type": "ag_bytes",
            "indicator": indicator,
            "skillname": skillname,
            "id": id,
            "revision": revision,
            "ag_id": ag.id,
            "ag_id": ag.id,
            "prof": ag.prof,
            "elite": ag.elite,
            "self_": ag.self_,
            "team": ag.team,
        }).to_string());
        let mut bytes = get_ag_bytes(&ag);
        message.append(&mut bytes);
    };
    if let Some(ag) = dst {
        messages |= 1 << 2;
        EVENT_EMITTER.lock().unwrap().emit("arc", json!({
            "type": "arc",
            "sub_type": "ag_bytes",
            "indicator": indicator,
            "skillname": skillname,
            "id": id,
            "revision": revision,
            "ag_id": ag.id.to_string(),
            "prof": ag.prof,
            "elite": ag.elite,
            "self_": ag.self_,
            "team": ag.team,
        }).to_string());
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
