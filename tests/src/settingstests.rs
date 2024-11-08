#[cfg(test)]
mod tests {
    use settings::*;

    use crate::*;

  #[test]
  fn test_sftp_settings() {
    let conf: Box<TestSettings> = get_configuration().unwrap();
    assert_eq!("localhost", conf.sftp1.host);
  }

}