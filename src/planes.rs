use xplm_sys;

#[derive(Debug)]
pub struct CountAircraft {
    pub total_aircraft: i32,
    pub active_aircraft: i32,
    pub host_id: i32,
}

impl CountAircraft {
    pub fn get() -> Self {
        let mut total_aircraft: i32 = -1;
        let mut active_aircraft: i32 = -1;
        let mut host_id: i32 = -1;

        unsafe {
            xplm_sys::XPLMCountAircraft(&mut total_aircraft, &mut active_aircraft, &mut host_id);
        }

        CountAircraft {
            total_aircraft,
            active_aircraft,
            host_id,
        }
    }
}
