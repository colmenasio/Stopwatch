use std::time;

pub struct SimpleTimer {
    start_time: time::Instant,
    pause_start_time: time::Instant,
    is_paused: bool,
    total_paused_time: u64,
}

impl SimpleTimer {
    pub fn new()->SimpleTimer{
        SimpleTimer{
            start_time: time::Instant::now(),
            pause_start_time: time::Instant::now(),
            is_paused: false,
            total_paused_time: 0,
        }
    }

    pub fn start_timer(& mut self){
        self.start_time = time::Instant::now();
        self.is_paused = false;
        self.total_paused_time = 0;
    }

    pub fn change_pause_state(& mut self){
        if self.is_paused {
            self.unpause_timer();
        } else {
            self.pause_timer();
        }
    }

    fn pause_timer(& mut self){
        self.pause_start_time = time::Instant::now();
        self.is_paused = true;
    }

    fn unpause_timer(& mut self){
        self.total_paused_time += self.pause_start_time.elapsed().as_secs();
        self.is_paused = false;
    }

    pub fn get_timer_display(&self)->String{
        let seconds_elapsed = if self.is_paused{
            (self.pause_start_time-self.start_time).as_secs()-self.total_paused_time
        } else {
            (time::Instant::now()-self.start_time).as_secs()-self.total_paused_time
        };
        SimpleTimer::stringify_seconds(seconds_elapsed)
    }

    fn stringify_seconds(seconds: u64)->String{
        let hhmmss: [u8; 3] = [
            u8::try_from(seconds/3600).expect("wtf how did you overflow the hours?"),
            u8::try_from((seconds/60)%60).unwrap(),
            u8::try_from(seconds%60).unwrap(),
        ];
        hhmmss.map(|x| format!("{:0>2}", x.to_string())).join(":")
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_stringify_seconds(){
        assert_eq!(SimpleTimer::stringify_seconds(100), "00:01:40".to_string());
        assert_eq!(SimpleTimer::stringify_seconds(0), "00:00:00".to_string());
        assert_eq!(SimpleTimer::stringify_seconds(3600), "01:00:00".to_string());
        assert_eq!(SimpleTimer::stringify_seconds(4005), "01:06:45".to_string());
    }
}