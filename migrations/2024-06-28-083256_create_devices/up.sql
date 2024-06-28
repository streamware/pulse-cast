-- Your SQL goes here
CREATE TABLE "devices" (
    "id" INTEGER NOT NULL PRIMARY KEY,
    "owner" TEXT NOT NULL REFERENCES users(id),
    "device_name" TEXT NOT NULL,
    "device_type" TEXT NOT NULL,
    "device_token" TEXT NOT NULL,
    "os_version" TEXT NOT NULL,
    "enabled" BOOLEAN NOT NULL DEFAULT 1,
    "created_at" TEXT NOT NULL DEFAULT (datetime('now')),
    "updated_at" TEXT NOT NULL DEFAULT (datetime('now'))
);