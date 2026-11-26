-- An addition numbered above the highest: a declared destructive statement, and
-- destructive text only inside a comment and a string.
DROP TABLE legacy_sessions; -- migration-ok: replaced by accounts in 0001
/* DROP TABLE accounts; would lose every account */
INSERT INTO accounts (name) VALUES ('DROP TABLE accounts');
ALTER TABLE accounts ADD COLUMN email TEXT;
