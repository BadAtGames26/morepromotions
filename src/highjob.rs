use unity::{prelude::*, system::List};

use engage::gamedata::*;

use crate::util::{fix_mjid, get_lowjob};


// Adds new classes to a LowJob's HighJob list, using the LowJob List from the HighJob
#[unity::hook("App", "JobData", "GetHighJobs")]
pub fn jobdata_gethighjobs(this: &JobData, method_info: OptionalMethod) -> &'static mut List<JobData> {
    let highjobs = call_original!(this, method_info);
    let jobdata = JobData::get_list().unwrap();

    // Change the MJID to an MID_SORTIE if needed for certain base classes
    let mjid = fix_mjid(this.name.get_string().unwrap());

    // Filter through all classes to find classes whose lowjob matchs the MJID of the current job
    let matchingjids: Vec<String>  = jobdata.iter()
                                            .enumerate()
                                            .filter(|(index, job)| *index < jobdata.len() && get_lowjob(job) == mjid)
                                            .map(|(_index, job)| job.jid.get_string().unwrap())
                                            .collect();
    // Go through the filtered JIDs to see if they can be added to the list
    for jid in matchingjids {
        // Checking if the class already exist
        let existingjob: Option<(usize, &&mut JobData)> = highjobs.iter()
                                                                  .enumerate()
                                                                  .find(|(index, job)| *index < highjobs.len() && job.jid.get_string().unwrap() == jid);
        if existingjob.is_none() {
            highjobs.add(JobData::get_mut(jid.as_str()).expect("Should be able to get JobData to add new HighJob"));
        }
    }

    highjobs
}
