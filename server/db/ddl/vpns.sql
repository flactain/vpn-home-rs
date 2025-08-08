--DROP TABLE vpns CASCADE;

CREATE TABLE vpns(
  vpn_id UUID NOT NULL,
  vpn_name VARCHAR(20) NOT NULL,
  owner_user_id CHAR(32) NOT NULL,
  approved_at TIMESTAMP,
  is_deleted bool DEFAULT false NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY(vpn_id)
);

COMMENT ON TABLE vpns IS 'VPN';

ALTER TABLE public.vpns ADD CONSTRAINT vpns_users_fk FOREIGN KEY (owner_user_id) REFERENCES users(user_id)ON DELETE CASCADE ON UPDATE CASCADE;

COMMENT ON COLUMN vpns.vpn_id IS 'VPN ID';
COMMENT ON COLUMN vpns.vpn_name IS 'VPN名';
COMMENT ON COLUMN vpns.owner_user_id IS 'オーナーユーザー';
COMMENT ON COLUMN vpns.approved_at IS '承認日時';