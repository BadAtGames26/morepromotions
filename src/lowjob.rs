use unity::{prelude::*, system::List};

use engage::gamedata::*;

use crate::util::{fix_mjid, get_lowjob};

// Give new classes their lowjob in the lowjob list, modded classes have an empty list so we add them here
#[unity::hook("App", "JobData", "GetLowJobs")]
pub fn jobdata_getlowjobs(this: &JobData, method_info: OptionalMethod) -> &'static mut List<JobData>{
    let lowjobs = call_original!(this, method_info);
    let jobdata = JobData::get_list().unwrap();
    
    // MJID of the LowJob in Job.xml
    let lowjob = get_lowjob(this);
    // Filter through classes to find classes
    let matchingjids: Vec<String>  = jobdata.iter()
                                            .enumerate() // Sometimes data goes out of bounds (StructList is shorter than StructList.List which is 128 for JobData)
                                                         // so checking it here makes sure we won't panic/hard crash
                                            .filter(|(index, job)| *index < jobdata.len() && {
                                                let jobname =  fix_mjid(job.name.get_string().unwrap());
                                                // We want to avoid JID_M000_神竜ノ子 as it is the prologue class
                                                jobname == lowjob && job.jid.get_string().unwrap() != *"JID_M000_神竜ノ子".to_string()
                                            })
                                            .map(|(_index, job)| job.jid.get_string().unwrap())
                                            .collect();
    // Go through the filtered JIDs to see if they can be added to the list
    for jid in matchingjids {
        // Checking if the class already exist
        let existingjob: Option<(usize, &&mut JobData)> = lowjobs.iter()
                                                                 .enumerate() // Same thing can happen here, probably a better way to do it but this works
                                                                 .find(|(index, job)| *index < lowjob.len()  && job.jid.get_string().unwrap() == jid );
        if existingjob.is_none() {
            lowjobs.add(JobData::get_mut(jid.as_str()).expect("Should be able to get JobData to add new LowJob"));
        }
    }

    lowjobs
}
