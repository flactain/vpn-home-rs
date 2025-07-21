CREATE TABLE vpns(
  vpn_id UUID NOT NULL,
  vpn_name VARCHAR(20) NOT NULL,
  owner_user_id CHAR(32),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
);

COMMENT ON TABLE vpns IS 'VPN';

COMMENT ON COLUMN vpns.vpn_id IS 'VPN ID';
COMMENT ON COLUMN vpns.vpn_name IS 'VPN名';
COMMENT ON COLUMN vpns.owner_user IS 'オーナーユーザー';
