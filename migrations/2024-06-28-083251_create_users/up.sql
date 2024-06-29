-- Your SQL goes here
CREATE TABLE "users" (
    "id" TEXT UNIQUE NOT NULL PRIMARY KEY,
    "username" TEXT UNIQUE NOT NULL,
    "created_at" TEXT NOT NULL,
    "updated_at" TEXT NOT NULL DEFAULT (datetime('now'))
);
