use super::protocol;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectCode {
    ConnectionAccepted,
    UnacceptableProtocolVersion,
    IdentifierRejected,
    ServerUnavailable,
    BadUsernamePassword,
    NotAuthorized,
    Unknown(u8),
}

impl core::fmt::Display for ConnectCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ConnectCode::ConnectionAccepted => write!(f, "Connection Accepted"),
            ConnectCode::UnacceptableProtocolVersion => write!(f, "Unacceptable Protocol Version"),
            ConnectCode::IdentifierRejected => write!(f, "Identifier Rejected"),
            ConnectCode::ServerUnavailable => write!(f, "Server Unavailable"),
            ConnectCode::BadUsernamePassword => write!(f, "Bad Username or Password"),
            ConnectCode::NotAuthorized => write!(f, "Not Authorized"),
            ConnectCode::Unknown(code) => write!(f, "Unknown({})", code),
        }
    }
}

impl From<u8> for ConnectCode {
    fn from(value: u8) -> Self {
        match value {
            protocol::CONNACK_CODE_ACCEPTED => ConnectCode::ConnectionAccepted,
            protocol::CONNACK_CODE_UNACCEPTABLE_PROTOCOL_VERSION => {
                ConnectCode::UnacceptableProtocolVersion
            }
            protocol::CONNACK_CODE_IDENTIFIER_REJECTED => ConnectCode::IdentifierRejected,
            protocol::CONNACK_CODE_SERVER_UNAVAILABLE => ConnectCode::ServerUnavailable,
            protocol::CONNACK_CODE_BAD_USERNAME_PASSWORD => ConnectCode::BadUsernamePassword,
            protocol::CONNACK_CODE_NOT_AUTHORIZED => ConnectCode::NotAuthorized,
            code => ConnectCode::Unknown(code),
        }
    }
}

impl From<ConnectCode> for u8 {
    fn from(value: ConnectCode) -> Self {
        match value {
            ConnectCode::ConnectionAccepted => protocol::CONNACK_CODE_ACCEPTED,
            ConnectCode::UnacceptableProtocolVersion => {
                protocol::CONNACK_CODE_UNACCEPTABLE_PROTOCOL_VERSION
            }
            ConnectCode::IdentifierRejected => protocol::CONNACK_CODE_IDENTIFIER_REJECTED,
            ConnectCode::ServerUnavailable => protocol::CONNACK_CODE_SERVER_UNAVAILABLE,
            ConnectCode::BadUsernamePassword => protocol::CONNACK_CODE_BAD_USERNAME_PASSWORD,
            ConnectCode::NotAuthorized => protocol::CONNACK_CODE_NOT_AUTHORIZED,
            ConnectCode::Unknown(code) => code,
        }
    }
}
