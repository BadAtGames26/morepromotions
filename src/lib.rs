#![feature(lazy_cell, ptr_sub_ptr)]
use unity::{prelude::*, system::List};
use engage::gamedata::*;

// get function from engage crate, returning a mut JobData because this is needed to add to list
fn get_mut(name: &str) -> Option<&'static mut JobData> {
    let method = JobData::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("Get"))).unwrap();
    
    let get = unsafe {
        std::mem::transmute::<_, extern "C" fn(&Il2CppString, &MethodInfo) -> Option<&'static mut JobData>>(
            method.method_ptr,
        )
    };
    
    get(name.into(), method)
 }

// Add the gethighjobs function here for testing to see any issues before loading a save
#[unity::hook("App", "JobData", "OnCompleted")]
pub fn jobdata_oncompleted(this: &JobData, method_info: OptionalMethod){
    jobdata_gethighjobs(this, None);
    call_original!(this, method_info);
}

// Give new classes their lowjob in the lowjob list, modded classes have an empty list so we have to add them here
#[unity::hook("App", "JobData", "GetLowJobs")]
pub fn jobdata_getlowjobs(this: &JobData, method_info: OptionalMethod) -> &'static mut List<JobData>{
    let lowjobs = call_original!(this, method_info);
    let jobdata = JobData::get_list_mut().unwrap();
    let list = &jobdata.list.items;
    // MJID of the LowJob in Job.xml
    // MJID is used rather than localized MJID name because than Celine's and Alfred's Noble class are the treated the same and incorrectly added
    let mut mjid = get_lowjob(this);

    // Changes MJID to MID if needed
    mjid = match mjid.as_str() {
        "MJID_SwordArmor" | "MJID_LanceArmor" | "MJID_AxArmor" => "MID_SORTIE_CLASSCHANGE_BASIC_ARMOR".to_string(),
        "MJID_SwordKnight" | "MJID_LanceKnight" | "MJID_AxKnight" => "MID_SORTIE_CLASSCHANGE_BASIC_KNIGHT".to_string(),
        "MJID_SwordPegasus" | "MJID_LancePegasus" | "MJID_AxPegasus" => "MID_SORTIE_CLASSCHANGE_BASIC_PEGASUS".to_string(),
        _ => mjid,
    };
    if lowjobs.len() == 0 {
        // Go through all entries and try to find a matching MJID
        for x in 1..jobdata.len() {
            // Check if the class is a Base class, otherwise do not check it
            if get_rank(list[x]) == 0 {
                // Get MJID of the current class that is being checked
                let lowmjid = get_name(list[x]);
                if mjid == lowmjid {
                    // Adding the JobData of a class that has a matching MJID
                    lowjobs.add(get_mut(list[x].jid.get_string().unwrap().as_str()).unwrap());
                }
            }
        }
    }
    lowjobs
}

// Adds new classes to a LowJob's HighJob list, using the LowJob List from the HighJob
#[unity::hook("App", "JobData", "GetHighJobs")]
pub fn jobdata_gethighjobs(this: &JobData, method_info: OptionalMethod) -> &'static mut List<JobData>{
    let highjobs = call_original!(this, method_info);
    let jobdata = JobData::get_list_mut().unwrap();
    let joblist = &jobdata.list.items;

    for x in 1..jobdata.len() {
        // Check if the class that will be added is a Advanced class
        if get_rank(joblist[x]) == 1 {
            let lowjobs = jobdata_getlowjobs(joblist[x], None);
            // MJID of the HighJob
            let highname = get_name(joblist[x]);
            if lowjobs.len() > 0 {
                for y in 0..lowjobs.len() {
                    if lowjobs[y].jid.get_string().unwrap() == this.jid.get_string().unwrap() {
                        // Check if the class is new to the list, otherwise do not re-add to list
                        // Unsure if duplicate entries can exist, but might as well prevent it anyways
                        let mut isnew= true;
                        if highjobs.len() > 0 {
                            for z in 0..highjobs.len() {
                                // If Job already exist in HighJob list, avoid adding it
                                if get_name(highjobs[z]) == highname { isnew = false }
                            }
                            // Check to see that adding a new class will not put it beyond the array capacity, might not be needed
                            // Needs more testing, should limit it to 4 HighJobs
                            if isnew && (highjobs.len() + 1 <= highjobs.capacity()) {
                                // Add the job to HighJob list if it is a new job
                                println!("Adding {} to {}'s HighJobs", getname(joblist[x]), getname(lowjobs[y]));
                                highjobs.add(get_mut(joblist[x].jid.get_string().unwrap().as_str()).unwrap());
                            }
                        }
                    }
                }
            }
        }
    }
    highjobs
}

// Get localized name of class
#[unity::from_offset("App", "JobData", "GetName")]
pub fn jobdata_getname(this: &JobData, method_info: OptionalMethod) -> &Il2CppString;

fn getname(job: &JobData) -> String {
    let name = unsafe { jobdata_getname(job, None) };
    if null(name) { return String::from("Null"); } 
    else { return name.get_string().unwrap(); }
}

// Get rank of class
#[unity::from_offset("App", "JobData", "get_Rank")]
pub fn jobdata_get_rank(this: &JobData, method_info: OptionalMethod) -> u8;

fn get_rank(job: &JobData) -> u8 {
    let rank =  unsafe { jobdata_get_rank(job, None) };
    rank
}

// Get MJID of class
#[unity::from_offset("App", "JobData", "get_Name")]
pub fn jobdata_get_name(this: &JobData, method_info: OptionalMethod) -> &Il2CppString;

fn get_name(job: &JobData) -> String {
    let name = unsafe { jobdata_get_name(job, None) };
    if null(name) { return String::from("Null"); }
    else { return name.get_string().unwrap(); }
}

// Get MJID of LowJob
#[unity::from_offset("App", "JobData", "get_LowJob")]
pub fn jobdata_get_lowjob(this: &JobData, method_info: OptionalMethod) -> &Il2CppString;

fn get_lowjob(job: &JobData) -> String {
    let lowjob = unsafe { jobdata_get_lowjob(job, None) };
    if null(lowjob) { return String::from("Null"); } 
    else { return lowjob.get_string().unwrap(); }
}

// Checks if a string is null or empty
#[unity::from_offset("System", "String", "IsNullOrEmpty")]
pub fn string_isnullorempty(value: &Il2CppString, method_info: OptionalMethod) -> bool;

fn null(value: &Il2CppString) -> bool {
    let isnull = unsafe { string_isnullorempty(value, None) };
    return isnull;
}


#[skyline::main(name = "highjob")]
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
            "Custom plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        skyline::error::show_error(
            69,
            "Custom plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));

    skyline::install_hooks!(jobdata_oncompleted, jobdata_gethighjobs, jobdata_getlowjobs);
}
