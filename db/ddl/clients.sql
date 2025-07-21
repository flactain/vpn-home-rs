CREATE TABLE clients (
  vpn_id UUID NOT NULL,
  terminal_id UUID NOT NULL,
  allowed_ip INET,
  public_key TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY(vpn_id, terminal_id)
);

COMMENT ON TABLE clients IS 'クライアント';

COMMENT ON COLUMN clients.vpn_id IS 'VPN ID';
COMMENT ON COLUMN clients.server_id IS 'サーバーID';
COMMENT ON COLUMN clients.terminal_id IS '端末ID';
COMMENT ON COLUMN clients.allowed_ip IS 'クライアント使用可能IP';
COMMENT ON COLUMN clients.public_key IS 'パブリックキー';
