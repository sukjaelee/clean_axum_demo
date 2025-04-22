-- Seed data for users
INSERT INTO users (id, username, email, created_by, created_at, modified_by, modified_at) VALUES
  ('00000000-0000-0000-0000-000000000001', 'user01', 'user01@example.com', NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000002', 'user02', 'user02@example.com', NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000003', 'user03', 'user03@example.com', NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000004', 'user04', 'user04@example.com', NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000005', 'user05', 'user05@example.com', NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000006', 'user06', 'user06@example.com', NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000007', 'user07', 'user07@example.com', NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000008', 'user08', 'user08@example.com', NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000009', 'user09', 'user09@example.com', NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000010', 'user10', 'user10@example.com', NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000011', 'user11', 'user11@example.com', NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000012', 'user12', 'user12@example.com', NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000013', 'user13', 'user13@example.com', NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000014', 'user14', 'user14@example.com', NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000015', 'user15', 'user15@example.com', NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000016', 'user16', 'user16@example.com', NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000017', 'user17', 'user17@example.com', NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000018', 'user18', 'user18@example.com', NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000019', 'user19', 'user19@example.com', NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000020', 'user20', 'user20@example.com', NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000021', 'apitest01', 'apitest01@example.com', NULL, NOW(), NULL, NOW());

-- Seed data for devices
INSERT INTO devices (user_id, name, status, device_os, registered_at, created_by, created_at, modified_by, modified_at) VALUES
-- 4 devices per user
  -- user01
  ('00000000-0000-0000-0000-000000000001', 'device01-1', 'active', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000001', 'device01-2', 'inactive', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000001', 'device01-3', 'pending', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000001', 'device01-4', 'active', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  -- user02
  ('00000000-0000-0000-0000-000000000002', 'device02-1', 'inactive', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000002', 'device02-2', 'active', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000002', 'device02-3', 'pending', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000002', 'device02-4', 'active', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  -- user03
  ('00000000-0000-0000-0000-000000000003', 'device03-1', 'active', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000003', 'device03-2', 'inactive', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000003', 'device03-3', 'pending', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000003', 'device03-4', 'active', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  -- user04
  ('00000000-0000-0000-0000-000000000004', 'device04-1', 'inactive', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000004', 'device04-2', 'active', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000004', 'device04-3', 'pending', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000004', 'device04-4', 'active', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  -- user05
  ('00000000-0000-0000-0000-000000000005', 'device05-1', 'active', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000005', 'device05-2', 'inactive', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000005', 'device05-3', 'pending', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000005', 'device05-4', 'active', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  -- user06
  ('00000000-0000-0000-0000-000000000006', 'device06-1', 'inactive', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000006', 'device06-2', 'active', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000006', 'device06-3', 'pending', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000006', 'device06-4', 'active', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  -- user07
  ('00000000-0000-0000-0000-000000000007', 'device07-1', 'active', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000007', 'device07-2', 'inactive', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000007', 'device07-3', 'pending', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000007', 'device07-4', 'active', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  -- user08
  ('00000000-0000-0000-0000-000000000008', 'device08-1', 'inactive', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000008', 'device08-2', 'active', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000008', 'device08-3', 'pending', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000008', 'device08-4', 'active', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  -- user09
  ('00000000-0000-0000-0000-000000000009', 'device09-1', 'active', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000009', 'device09-2', 'inactive', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000009', 'device09-3', 'pending', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000009', 'device09-4', 'active', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  -- user10
  ('00000000-0000-0000-0000-000000000010', 'device10-1', 'inactive', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000010', 'device10-2', 'active', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000010', 'device10-3', 'pending', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000010', 'device10-4', 'active', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  -- user11
  ('00000000-0000-0000-0000-000000000011', 'device11-1', 'active', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000011', 'device11-2', 'inactive', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000011', 'device11-3', 'pending', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000011', 'device11-4', 'active', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  -- user12
  ('00000000-0000-0000-0000-000000000012', 'device12-1', 'inactive', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000012', 'device12-2', 'active', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000012', 'device12-3', 'pending', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000012', 'device12-4', 'active', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  -- user13
  ('00000000-0000-0000-0000-000000000013', 'device13-1', 'active', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000013', 'device13-2', 'inactive', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000013', 'device13-3', 'pending', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000013', 'device13-4', 'active', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  -- user14
  ('00000000-0000-0000-0000-000000000014', 'device14-1', 'inactive', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000014', 'device14-2', 'active', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000014', 'device14-3', 'pending', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000014', 'device14-4', 'active', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  -- user15
  ('00000000-0000-0000-0000-000000000015', 'device15-1', 'active', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000015', 'device15-2', 'inactive', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000015', 'device15-3', 'pending', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000015', 'device15-4', 'active', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  -- user16
  ('00000000-0000-0000-0000-000000000016', 'device16-1', 'inactive', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000016', 'device16-2', 'active', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000016', 'device16-3', 'pending', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000016', 'device16-4', 'active', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  -- user17
  ('00000000-0000-0000-0000-000000000017', 'device17-1', 'active', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000017', 'device17-2', 'inactive', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000017', 'device17-3', 'pending', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000017', 'device17-4', 'active', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  -- user18
  ('00000000-0000-0000-0000-000000000018', 'device18-1', 'inactive', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000018', 'device18-2', 'active', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000018', 'device18-3', 'pending', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000018', 'device18-4', 'active', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  -- user19
  ('00000000-0000-0000-0000-000000000019', 'device19-1', 'active', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000019', 'device19-2', 'inactive', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000019', 'device19-3', 'pending', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000019', 'device19-4', 'active', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  -- user20
  ('00000000-0000-0000-0000-000000000020', 'device20-1', 'active', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000020', 'device20-2', 'inactive', 'iOS', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000020', 'device20-3', 'active', 'Android', NOW(), NULL, NOW(), NULL, NOW()),
  ('00000000-0000-0000-0000-000000000020', 'device20-4', 'decommissioned', 'iOS', NOW(), NULL, NOW(), NULL, NOW());


-- Seed data for user_roles

-- for auth login
-- client_id: apitest01
-- client_secret: test_password

-- password_hash value: test_password
INSERT INTO testdb.user_auth
(user_id, password_hash, created_at, modified_at)
VALUES('00000000-0000-0000-0000-000000000021', '$argon2id$v=19$m=19456,t=2,p=1$XBFwBY52C9SpzkxON1OTLg$djDqZQvzxFKc9HOCWyZfKy+RlFTs0BJFSkcw/Tos14c', NOW(), NOW());

