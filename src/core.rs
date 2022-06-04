pub fn parse(_: &str) -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_parse() {
        let val = Uuid::new_v4().to_string();
        let res = parse(&val);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), ());
    }
}
