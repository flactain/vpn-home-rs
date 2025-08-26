CREATE TABLE request_executions (
    message_id VARCHAR(100) NOT NULL,
    vpn_id uuid NOT NULL,
    resource_id uuid NOT NULL,
    resource_type CHAR(20) NOT NULL,
    resource_handle CHAR(20) NOT NULL,
    requested_at timestamp NOT NULL,
    failed_at Timestamp,
    failed_message VARCHAR(100),
    PRIMARY KEY(message_id)
);
