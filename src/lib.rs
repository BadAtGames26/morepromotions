
#![feature(lazy_cell, ptr_sub_ptr)]

use unity::{prelude::*, system::List};
use engage::gamedata::*;


#[unity::hook("App", "JobData", "OnCompleted")]
pub fn jobdata_oncompleted(this: &JobData, method_info: OptionalMethod){
    jobdata_gethighjobs(this, None);
    call_original!(this, method_info);
}

#[unity::hook("App", "JobData", "HasHighJobs")]
pub fn jobdata_hashighjobs(this: &JobData, method_info: OptionalMethod) -> bool{
    let hashighjob = call_original!(this, method_info);
    if hashighjob == true {
        println!("class has highjobs");
    }
    hashighjob
}


#[unity::hook("App", "JobData", "GetHighJobs")]
pub fn jobdata_gethighjobs(this: &JobData, method_info: OptionalMethod) -> &'static mut List<JobData>{
    let highjobs = call_original!(this, method_info);
    let jobdata = JobData::get_list().unwrap();
    let highjob3 = "JID_魔戦士".to_string();
    let name = jobdata_getname(this, None).get_string().unwrap();

    if highjobs.len() == 1 {
        println!("Job: {}, HighJob1: {}, HighJob2: {}", name, jobdata_getname(highjobs.items[0], None).get_string().unwrap_or("None".to_string()), "None".to_string());
    } else if highjobs.len() > 1 {
        println!("Job: {}, HighJob1: {}, HighJob2: {}", name, jobdata_getname(highjobs.items[0], None).get_string().unwrap_or("None".to_string()), jobdata_getname(highjobs.items[1], None).get_string().unwrap_or("None".to_string()));       
        if this.jid.get_string().unwrap() == "JID_ソードファイター".to_string() {

        }

    } else if highjobs.len() == 0 {
        println!("Job: {}, HighJob1: {}, HighJob2: {}", name, "None".to_string(), "None".to_string());        
    }
    highjobs
}

#[unity::hook("App", "JobData", "GetName")]
pub fn jobdata_getname(this: &JobData, method_info: OptionalMethod) -> &'static Il2CppString{
    let job_name: &'static Il2CppString = call_original!(this, method_info);
    job_name
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

    skyline::install_hooks!(jobdata_oncompleted, jobdata_hashighjobs, jobdata_gethighjobs, jobdata_getname);
}
