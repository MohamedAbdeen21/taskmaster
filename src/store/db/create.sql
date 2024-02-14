CREATE TABLE IF NOT EXISTS log (
  id iNTEGER PRIMARY KEY,
  time TIMESTAMP,
  updated_on TIMESTAMP,
  graph TEXT,
  status VARCHAR(10)
)
