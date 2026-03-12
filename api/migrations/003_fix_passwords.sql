-- Migration: 003_fix_passwords.sql
-- Fixes password hashes for all seed users to the correct bcrypt encoding
-- of "Password@123". The previous seed used a placeholder hash.
--
-- Password: Password@123
-- Hash:     $2b$12$49a3Bj648XXrBBWJrM87TuG/zHT8JPW1Kvx1QUvQXXCz5hoQQn23u

UPDATE users
SET    password_hash = '$2b$12$49a3Bj648XXrBBWJrM87TuG/zHT8JPW1Kvx1QUvQXXCz5hoQQn23u'
WHERE  username IN ('admin', 'manager', 'staff1');
