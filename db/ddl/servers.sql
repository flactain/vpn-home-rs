CREATE TABLE servers(
  vpn_id UUID NOT NULL,
  terminal_id UUID NOT NULL,
  public_ip INET NOT NULL,
  config_name TEXT,
  public_key TEXT NOT NULL,
  keep_alive_second INTEGER,
  post_up_command TEXT,
  post_down_command TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY(vpn_id, terminal_id)
);

COMMENT ON TABLE servers IS 'サーバー';

COMMENT ON COLUMN servers.vpn_id IS 'VPN ID';
COMMENT ON COLUMN servers.terminal_id IS '端末ID';
COMMENT ON COLUMN servers.public_ip IS 'パブリックIP';
COMMENT ON COLUMN servers.config_name IS 'configファイル名';
COMMENT ON COLUMN servers.public_key IS 'パブリックキー';
COMMENT ON COLUMN servers.keep_alive_second IS 'keep-alive時間';
COMMENT ON COLUMN servers.post_up_command IS 'postUpのコマンド';
COMMENT ON COLUMN servers.post_down_command IS 'post_downのコマンド';
