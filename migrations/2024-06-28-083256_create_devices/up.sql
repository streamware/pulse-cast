CREATE TABLE "devices" (
    "id" SERIAL NOT NULL PRIMARY KEY,
    "owner" INT NOT NULL REFERENCES users(id),
    "device_name" TEXT NOT NULL,
    "device_type" TEXT NOT NULL,
    "device_token" TEXT NOT NULL,
    "os_version" TEXT NOT NULL,
    "enabled" BOOLEAN NOT NULL DEFAULT TRUE,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);