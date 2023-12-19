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

#[unity::hook("App", "JobData", "GetHighJobs")]
pub fn jobdata_gethighjobs(this: &JobData, method_info: OptionalMethod) -> &'static mut List<JobData>{
    let highjobs = call_original!(this, method_info);
    let name = getname(this);

    if highjobs.len() == 1 {
        //println!("Job: {}, HighJob1: {}, HighJob2: {}", name, jobdata_getname(highjobs.items[0], None).get_string().unwrap_or("None".to_string()), "None".to_string());
    } else if highjobs.len() > 1 {
        if this.jid.get_string().unwrap() == "JID_ソードファイター".to_string() {
            highjobs.add(get_mut("JID_魔戦士").unwrap());
            println!("Job: {}, HighJob1: {}, HighJob2: {}, HighJob3: {}, len: {}, capacity: {}", name, getname(highjobs.items[0]),
            getname(highjobs.items[1]), getname(highjobs.items[2]),
            highjobs.len(), highjobs.capacity());       
    
        }
    } else if highjobs.len() == 0 {
        //println!("Job: {}, HighJob1: {}, HighJob2: {}", name, "None".to_string(), "None".to_string());        
    }
    highjobs
}

#[unity::from_offset("App", "JobData", "GetName")]
pub fn jobdata_getname(this: &JobData, method_info: OptionalMethod) -> &'static Il2CppString;

fn getname(job: &JobData) -> String {
    let name = unsafe { jobdata_getname(job, None) };
    return name.get_string().unwrap_or(String::from("None"));
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

    skyline::install_hooks!(jobdata_oncompleted, jobdata_gethighjobs);
}
