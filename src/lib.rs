#![feature(lazy_cell, ptr_sub_ptr)]
use unity::{prelude::*, system::List};
use engage::gamedata::*;

// get function from engage crate, returning a mut JobData solely because this is needed to add to list
fn get_mut(name: &str) -> Option<&'static mut JobData> {
    let method = JobData::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("Get"))).unwrap();
    
    let get = unsafe {
        std::mem::transmute::<_, extern "C" fn(&Il2CppString, &MethodInfo) -> Option<&'static mut JobData>>(
            method.method_ptr,
        )
    };
    
    get(name.into(), method)
 }

#[unity::hook("App", "JobData", "OnCompleted")]
pub fn jobdata_oncompleted(this: &JobData, method_info: OptionalMethod){
    jobdata_gethighjobs(this, None);
    call_original!(this, method_info);
}

#[unity::hook("App", "JobData", "GetLowJobs")]
pub fn jobdata_getlowjobs(this: &JobData, method_info: OptionalMethod) -> &'static mut List<JobData>{
    let lowjobs = call_original!(this, method_info);
    let jobdata = JobData::get_list_mut().unwrap();
    let list = &jobdata.list.items;
    let mjid = get_lowjob(this);
    if lowjobs.len() == 0 {
        for x in 1..jobdata.len() {
            if get_rank(list[x]) == 0 {
                let lowmjid = get_name(list[x]);
                if mjid == lowmjid {
                    lowjobs.add(get_mut(list[x].jid.get_string().unwrap().as_str()).unwrap());
                }
            }
        }
    }
    lowjobs
}

#[unity::hook("App", "JobData", "GetHighJobs")]
pub fn jobdata_gethighjobs(this: &JobData, method_info: OptionalMethod) -> &'static mut List<JobData>{
    let highjobs = call_original!(this, method_info);
    let jobdata = JobData::get_list_mut().unwrap();
    let joblist = &jobdata.list.items;
    let name = getname(this);
    for x in 1..jobdata.len() {
        if get_rank(joblist[x]) == 1 {
            let lowjobs = jobdata_getlowjobs(joblist[x], None);
            let highname = getname(joblist[x]);
            if lowjobs.len() > 0 {
                for y in 0..lowjobs.len() {
                    let lowname = getname(lowjobs[y]);
                    if lowjobs[y].jid.get_string().unwrap() == this.jid.get_string().unwrap() {
                        //println!("Match Found, name: {}, lowname: {}, jobname: {}", name, lowname, getname(joblist[x]));
                        //if highjobs.len() == 1 {
                        //} else if highjobs.len() > 1 {
                        //    highjobs.add(get_mut(joblist[x].jid.get_string().unwrap().as_str()).unwrap());
                        //    //println!("Job: {}, HighJob1: {}, HighJob2: {}, HighJob3: {}, len: {}, capacity: {}", name, getname(highjobs.items[0]),
                        //    //getname(highjobs.items[1]), getname(highjobs.items[2]),
                        //    //highjobs.len(), highjobs.capacity());       
                        //} else if highjobs.len() == 0 {     
                        //}
                        let mut isnew= true;
                        if highjobs.len() > 0 {
                            for z in 0..highjobs.len() {
                                if getname(highjobs[z]) == highname { isnew = false }
                            }

                            if isnew {
                                println!("Adding {} to {}'s HighJobs", highname, lowname);
                                highjobs.add(get_mut(joblist[x].jid.get_string().unwrap().as_str()).unwrap());
                            }
                        }
                    }
                }
            } else { println!("{}'s lowjobs len is 0", highname)}
        }
    }

    highjobs
}

#[unity::from_offset("App", "JobData", "GetName")]
pub fn jobdata_getname(this: &JobData, method_info: OptionalMethod) -> &Il2CppString;

fn getname(job: &JobData) -> String {
    let name = unsafe { jobdata_getname(job, None) };
    if null(name) { return String::from("Null"); } 
    else { return name.get_string().unwrap(); }
}

//#[unity::from_offset("App", "JobData", "GetLowJobs")]
//pub fn jobdata_getlowjobs(this: &JobData, method_info: OptionalMethod) -> &List<JobData>;

//fn getlowjobs(job: &JobData) -> &List<JobData> {
//    let lowjobs =  unsafe { jobdata_getlowjobs(job, None) };
//    lowjobs
//}

#[unity::from_offset("App", "JobData", "get_Rank")]
pub fn jobdata_get_rank(this: &JobData, method_info: OptionalMethod) -> u8;

fn get_rank(job: &JobData) -> u8 {
    let rank =  unsafe { jobdata_get_rank(job, None) };
    rank
}

#[unity::from_offset("App", "JobData", "get_Name")]
pub fn jobdata_get_name(this: &JobData, method_info: OptionalMethod) -> &Il2CppString;

fn get_name(job: &JobData) -> String {
    let name = unsafe { jobdata_get_name(job, None) };
    if null(name) { return String::from("Null"); }
    else { return name.get_string().unwrap(); }
}

#[unity::from_offset("App", "JobData", "get_LowJob")]
pub fn jobdata_get_lowjob(this: &JobData, method_info: OptionalMethod) -> &Il2CppString;

fn get_lowjob(job: &JobData) -> String {
    let lowjob = unsafe { jobdata_get_lowjob(job, None) };
    if null(lowjob) { return String::from("Null"); } 
    else { return lowjob.get_string().unwrap(); }
}

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
