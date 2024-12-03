use unity::prelude::*;
use engage::gamedata::*;

pub fn get_lowjob(job: &JobData) -> String {
    let lowjob = unsafe {
        jobdata_get_lowjob(job, None)
    };

    if let Some(lowname) = lowjob {
        lowname.get_string().unwrap()
    } else {
        "None".to_string()
    }
}

// Adjustment to MJID to make it a MID_SORTIE if needed
pub fn fix_mjid(mjid: String) -> String {
    let fixed_mjid = match mjid.as_str() {
        "MJID_SwordArmor" | "MJID_LanceArmor" | "MJID_AxArmor" => "MID_SORTIE_CLASSCHANGE_BASIC_ARMOR".to_string(),
        "MJID_SwordKnight" | "MJID_LanceKnight" | "MJID_AxKnight" => "MID_SORTIE_CLASSCHANGE_BASIC_KNIGHT".to_string(),
        "MJID_SwordPegasus" | "MJID_LancePegasus" | "MJID_AxPegasus" => "MID_SORTIE_CLASSCHANGE_BASIC_PEGASUS".to_string(),
        _ => mjid,
    };
    fixed_mjid
}


// Get MJID of LowJob
#[unity::from_offset("App", "JobData", "get_LowJob")]
pub fn jobdata_get_lowjob(this: &JobData, method_info: OptionalMethod) -> Option<&Il2CppString>;