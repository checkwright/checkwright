DROP TABLE orders;
ALTER TABLE audit
  DROP COLUMN note; -- migration-ok:
