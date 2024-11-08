#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        io::Write,
        path::{Path, PathBuf},
        str::FromStr,
    };

    use secrecy::ExposeSecret;
    use settings::get_configuration;
    use sftp::SftpConnector;
    use uuid::Uuid;

    use connectors::*;
    use models::*;

    use crate::TestSettings;

    fn get_test_settings() -> TestSettings {
        let conf: Box<TestSettings> = get_configuration().unwrap();
        *conf
    }

    fn get_me_fake_data1(settings: &TestSettings) -> models::TransferJobCommand {
        let user = models::User::new(
            Uuid::new_v4(),
            String::from_str("user test").unwrap(),
            Some(String::from_str("user@localhost.com").unwrap()),
            Some(chrono::offset::Utc::now().to_utc()),
        );
        let basic_info = models::BasicInfo::new(
            Uuid::new_v4(),
            String::from_str("name test").unwrap(),
            Some(String::from_str("description test").unwrap()),
            chrono::offset::Utc::now().to_utc(),
            chrono::offset::Utc::now().to_utc(),
            user.clone(),
            user.clone(),
        );

        let src_connection_attributes =
            models::ConnectionType::Sftp(models::SftpConnection::new_user_password_based_instance(
                settings.sftp2.host.to_string(),
                settings.sftp2.port,
                settings.sftp2.username.to_string(),
                Some(settings.sftp2.password.expose_secret().to_string()),
                10,
                Some(PathBuf::from_str(settings.sftp1.path.as_str()).unwrap()),
                Some(String::from_str("source_temp1.txt").unwrap()),
                None,
            ));

        let target_connection_attributes1 =
            models::ConnectionType::Sftp(models::SftpConnection::new_user_password_based_instance(
                settings.sftp2.host.to_string(),
                settings.sftp2.port,
                settings.sftp2.username.to_string(),
                Some(settings.sftp2.password.expose_secret().to_string()),
                10,
                Some(PathBuf::from_str(settings.sftp2.path.as_str()).unwrap()),
                Some(String::from_str("target_temp1.txt").unwrap()),
                None,
            ));

        let target_connection_attributes2 =
            models::ConnectionType::Sftp(models::SftpConnection::new_user_password_based_instance(
                settings.sftp2.host.to_string(),
                settings.sftp2.port,
                settings.sftp2.username.to_string(),
                Some(settings.sftp2.password.expose_secret().to_string()),
                10,
                Some(PathBuf::from_str(settings.sftp2.path.as_str()).unwrap()),
                Some(String::from_str("target_temp2.txt").unwrap()),
                None,
            ));

        let source_endpoint =
            models::EndpointDefinition::new(basic_info.clone(), src_connection_attributes);

        let source = models::SourceDefinition::new(source_endpoint);

        let target_endpoint1 =
            models::EndpointDefinition::new(basic_info.clone(), target_connection_attributes1);
        let target_endpoint2 =
            models::EndpointDefinition::new(basic_info.clone(), target_connection_attributes2);

        let targets = vec![
            models::TargetDefinition::new(target_endpoint1),
            models::TargetDefinition::new(target_endpoint2),
        ];

        return models::TransferJobCommand::new(basic_info.clone(), source, targets);
    }

    #[test]
    fn test_session() {
        let settings = &get_test_settings();
        let transfer_job_command = get_me_fake_data1(settings);
        let mut source_connector = SftpConnector {
            session: None,
            file: None,
        };
        &source_connector.connect(&transfer_job_command.source.endpoint);
        assert!(source_connector.session.is_some());
    }

    #[test]
    fn test_copy() {
        let settings = &get_test_settings();
        let transfer_job_command = get_me_fake_data1(settings);
        let mut source_connector = SftpConnector {
            session: None,
            file: None,
        };

        fs::create_dir_all(&settings.sftp1.path);
        let mut full_path: PathBuf = PathBuf::new();
        full_path.push(&settings.sftp1.path);
        full_path.push("source_temp1.txt");
        let mut rfile = File::create(full_path).unwrap();
        let content = "Hello, rust!";

        rfile.write_all(content.as_bytes());

        &source_connector.connect(&transfer_job_command.source.endpoint);

        &source_connector.open(&transfer_job_command.source.endpoint);

        let mut buffer: Vec<u8> = Vec::with_capacity(4096);

        &source_connector.read(&mut buffer);

        assert!(source_connector.session.is_some());
    }

    // #[test]
    // fn test_move_one_file() {
    //     let settings = get_test_settings();
    //     let transfer_job_command = get_me_fake_data1(settings);
    //     let sourceAttrib = &transfer_job_command.source.endpoint.conenction;
    //     let source_session: ssh2::Sftp;
    //     if let models::ConnectionType::Sftp(attrib) = sourceAttrib {
    //         source_session = connectors::Sftp::connect_sftp(&attrib);
    //     }

    //     let targetAttrib = &transfer_job_command
    //         .targets
    //         .get(0)
    //         .unwrap()
    //         .endpoint
    //         .conenction;
    //     let target_session: ssh2::Sftp;
    //     if let models::ConnectionType::Sftp(attrib) = targetAttrib {
    //         target_session = connectors::Sftp::connect_sftp(&attrib);
    //     }

    //     assert!(true);
    // }
}
