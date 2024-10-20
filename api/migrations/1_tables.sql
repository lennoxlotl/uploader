CREATE TABLE IF NOT EXISTS images (
  id TEXT,
  bucket_id TEXT,
  secret TEXT,
  uploaded_at BIGINT,
  size BIGINT,
  PRIMARY KEY (id),
  UNIQUE (id, bucket_id, secret)
);

CREATE UNIQUE INDEX images_secret_idx on images (secret);
