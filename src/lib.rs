#![feature(lazy_cell, ptr_sub_ptr)]
use unity::{prelude::*, system::List};
use engage::gamedata::*;

// Add the gethighjobs and getlowjobs function here for testing since it runs early
/* #[unity::hook("App", "JobData", "OnCompleted")]
pub fn jobdata_oncompleted(this: &JobData, method_info: OptionalMethod){
    jobdata_gethighjobs(this, None);
    jobdata_getlowjobs(this, None);
    call_original!(this, method_info);
} */

// Give new classes their lowjob in the lowjob list, modded classes have an empty list so we add them here
#[unity::hook("App", "JobData", "GetLowJobs")]
pub fn jobdata_getlowjobs(this: &JobData, method_info: OptionalMethod) -> &'static mut List<JobData>{
    let lowjobs = call_original!(this, method_info);
    let jobdata = JobData::get_list().unwrap();
    
    // MJID of the LowJob in Job.xml
    let lowjob = get_lowjob(this);
    // Filter through classes to find classes
    let matchingjids: Vec<String>  = jobdata.into_iter()
                                            .enumerate() // Sometimes data goes out of bounds (StructList is shorter than StructList.List which is 128 for JobData)
                                                         // so checking it here makes sure we won't panic/hard crash
                                            .filter(|(index, job)| *index <= jobdata.len() - 1 && {
                                                let jobname =  fix_mjid(job.name.get_string().unwrap());
                                                // We want to avoid JID_M000_神竜ノ子 as it is the prologue class
                                                jobname == lowjob && job.jid.get_string().unwrap() != "JID_M000_神竜ノ子".to_string()
                                            })
                                            .map(|(_index, job)| job.jid.get_string().unwrap())
                                            .collect();
    // Go through the filtered JIDs to see if they can be added to the list
    for jid in matchingjids {
        // Checking if the class already exist
        let existingjob: Option<(usize, &&mut JobData)> = lowjobs.into_iter()
                                                                 .enumerate() // Same thing can happen here, probably a better way to do it but this works
                                                                 .find(|(index, job)| *index <= lowjob.len() - 1 && job.jid.get_string().unwrap() == jid );
        if existingjob.is_none() {
            lowjobs.add(JobData::get_mut(jid.as_str()).expect("Should be able to get JobData to add new LowJob"));
        }
    }

    lowjobs
}

// Adds new classes to a LowJob's HighJob list, using the LowJob List from the HighJob
#[unity::hook("App", "JobData", "GetHighJobs")]
pub fn jobdata_gethighjobs(this: &JobData, method_info: OptionalMethod) -> &'static mut List<JobData> {
    let highjobs = call_original!(this, method_info);
    let jobdata = JobData::get_list().unwrap();

    // Change the MJID to an MID_SORTIE if needed for certain base classes
    let mjid = fix_mjid(this.name.get_string().unwrap());

    // Filter through all classes to find classes whose lowjob matchs the MJID of the current job
    let matchingjids: Vec<String>  = jobdata.into_iter()
                                            .enumerate()
                                            .filter(|(index, job)| *index <= jobdata.len() - 1 && get_lowjob(job) == mjid)
                                            .map(|(_index, job)| job.jid.get_string().unwrap())
                                            .collect();
    // Go through the filtered JIDs to see if they can be added to the list
    for jid in matchingjids {
        // Checking if the class already exist
        let existingjob: Option<(usize, &&mut JobData)> = highjobs.into_iter()
                                                                  .enumerate()
                                                                  .find(|(index, job)| *index <= highjobs.len() - 1 && job.jid.get_string().unwrap() == jid);
        if existingjob.is_none() {
            highjobs.add(JobData::get_mut(jid.as_str()).expect("Should be able to get JobData to add new HighJob"));
        }
    }

    highjobs
}

// Get MJID of LowJob
#[unity::from_offset("App", "JobData", "get_LowJob")]
pub fn jobdata_get_lowjob(this: &JobData, method_info: OptionalMethod) -> Option<&Il2CppString>;

pub fn get_lowjob(job: &JobData) -> String {
    let lowjob = unsafe {
        jobdata_get_lowjob(job, None)
    };
    if lowjob.is_some() {
        let lowjob = lowjob.unwrap().get_string().unwrap();
        lowjob
    } else {
        let lowjob = "None".to_string();
        lowjob
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

#[skyline::main(name = "promos")]
pub fn main() {
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };


        let err_msg = format!(
            "More Promotions plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        skyline::error::show_error(
            420,
            "More Promotions plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));

    skyline::install_hooks!(jobdata_gethighjobs, jobdata_getlowjobs);
    //skyline::install_hook!(jobdata_oncompleted);
}
