pub fn stringify_seconds(seconds: u64)->String{
    let hhmmss: [u8; 3] = [
        u8::try_from(seconds/3600).expect("wtf how did you overflow the hours?"),
        u8::try_from((seconds/60)%60).unwrap(),
        u8::try_from(seconds%60).unwrap(),
    ];
    hhmmss.map(|x| format!("{:0>2}", x.to_string())).join(":")
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_stringify_seconds(){
        assert_eq!(stringify_seconds(100), "00:01:40".to_string());
        assert_eq!(stringify_seconds(0), "00:00:00".to_string());
        assert_eq!(stringify_seconds(3600), "01:00:00".to_string());
        assert_eq!(stringify_seconds(4005), "01:06:45".to_string());
    }
}