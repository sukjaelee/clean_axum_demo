use super::{
        dto::{CreateDeviceDto, UpdateDeviceDto, UpdateManyDevicesDto},
        model::Device,
    };
use async_trait::async_trait;
use sqlx::{mysql::MySql, Pool};

use sqlx::QueryBuilder;
use sqlx::Transaction;
use uuid::Uuid;

use super::repository::DeviceRepository;

pub struct DeviceRepo;

#[async_trait]
impl DeviceRepository for DeviceRepo {
    async fn find_all(&self, pool: Pool<MySql>) -> Result<Vec<Device>, sqlx::Error> {
        let devices = sqlx::query_as!(Device,
            r#"SELECT id, user_id, name, status, device_os, registered_at, created_by, created_at, modified_by, modified_at FROM devices"#
        )
        .fetch_all(&pool)
        .await?;

        Ok(devices)
    }

    async fn find_by_id(&self, pool: Pool<MySql>, id: String) -> Result<Option<Device>, sqlx::Error> {
        let device = sqlx::query_as!(Device, 
            r#"SELECT id, user_id, name, status, device_os, registered_at, created_by, created_at, modified_by, modified_at FROM devices WHERE id = ?"#,
            id
        )
        .fetch_optional(&pool)
        .await?;

        Ok(device)
    }

    async fn create(
        &self,
        tx: &mut Transaction<'_, MySql>,
        device: CreateDeviceDto,
    ) -> Result<Device, sqlx::Error> {
        sqlx::query!(r#"INSERT INTO devices (user_id, name, status, device_os, registered_at, created_by, created_at, modified_by, modified_at) VALUES (?, ?, ?, ?, now(), ?, now(), ?, now())"#,
                     device.user_id,
                     device.name,
                     device.status.to_string(),
                     device.device_os.to_string(),
                     device.modified_by,
                     device.modified_by,
        )
        .execute(&mut **tx)
        .await?;

        let inserted_device = sqlx::query_as!(Device,
            r#"SELECT id, user_id, name, status, device_os, registered_at, created_by, created_at, modified_by, modified_at FROM devices WHERE user_id = ? and name = ?"#,
            device.user_id,
            device.name,
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(inserted_device)
    }

    async fn update(
        &self,
        tx: &mut Transaction<'_, MySql>,
        id: String,
        device: UpdateDeviceDto,
    ) -> Result<Option<Device>, sqlx::Error> {
        let existing = sqlx::query!(r#"SELECT id FROM devices WHERE id = ?"#, &id)
            .fetch_optional(&mut **tx)
            .await?;

        if existing.is_some() {
            let mut builder = QueryBuilder::<MySql>::new("UPDATE devices SET ");
            let mut updates = Vec::new();

            if let Some(user_id) = device.user_id {
                updates.push(("user_id", user_id));
            }
            if let Some(name) = device.name {
                updates.push(("name", name));
            }
            if let Some(status) = device.status {
                updates.push(("status", status.to_string()));
            }
            if let Some(device_os) = device.device_os {
                updates.push(("device_os", device_os.to_string()));
            }
            // Always update modified_by as it is required
            updates.push(("modified_by", device.modified_by));

            for (i, (field, value)) in updates.into_iter().enumerate() {
                if i > 0 {
                    builder.push(", ");
                }
                builder.push(field).push(" = ").push_bind(value);
            }

            builder
                .push(", modified_at = NOW() WHERE id = ")
                .push_bind(&id);
            let query = builder.build();
            query.execute(&mut **tx).await?;

            let updated_device = sqlx::query_as!(Device,
                r#"SELECT id, user_id, name, status, device_os, registered_at, created_by, created_at, modified_by, modified_at FROM devices WHERE id = ?"#,
                &id
            )
            .fetch_one(&mut **tx)
            .await?;

            return Ok(Some(updated_device));
        }

        Ok(None)
    }

    async fn update_many(
        &self,
        tx: &mut Transaction<'_, MySql>,
        user_id: String,
        modified_by: String,
        update_devices: UpdateManyDevicesDto,
    ) -> Result<(), sqlx::Error> {
        let mut builder = QueryBuilder::<MySql>::new(
            r#"INSERT INTO devices (id, user_id, name, status, device_os, registered_at, created_by, created_at, modified_by, modified_at)"#
        );

        let now = time::OffsetDateTime::now_utc();

        builder.push_values(update_devices.devices.iter(), |mut b, device| {
            b.push_bind(
                device
                    .id
                    .clone()
                    .unwrap_or_else(|| Uuid::new_v4().to_string()),
            )
            .push_bind(&user_id)
            .push_bind(&device.name)
            .push_bind(device.status.to_string())
            .push_bind(device.device_os.to_string())
            .push_bind(now)
            .push_bind(&modified_by)
            .push_bind(now)
            .push_bind(&modified_by)
            .push_bind(now);
        });

        builder.push(
            r#" ON DUPLICATE KEY UPDATE
                user_id = IF(user_id = VALUES(user_id), user_id, user_id),
                name = IF(user_id = VALUES(user_id), VALUES(name), name),
                status = IF(user_id = VALUES(user_id), VALUES(status), status),
                device_os = IF(user_id = VALUES(user_id), VALUES(device_os), device_os),
                modified_by = IF(user_id = VALUES(user_id), VALUES(modified_by), modified_by),
                modified_at = IF(user_id = VALUES(user_id), VALUES(modified_at), modified_at)"#,
        );

        let query = builder.build();
        query.execute(&mut **tx).await?;

        Ok(())
    }

    async fn delete(&self, tx: &mut Transaction<'_, MySql>, id: String) -> Result<bool, sqlx::Error> {
        let res = sqlx::query!(r#"DELETE FROM devices WHERE id = ?"#, id)
            .execute(&mut **tx)
            .await?;

        Ok(res.rows_affected() > 0)
    }
}
