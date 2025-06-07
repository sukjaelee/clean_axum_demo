use async_trait::async_trait;
use sqlx::QueryBuilder;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::device::domain::model::Device;
use crate::device::domain::repository::DeviceRepository;
use crate::device::dto::{CreateDeviceDto, UpdateDeviceDto, UpdateManyDevicesDto};

pub struct DeviceRepo;

const FIND_DEVICE_INFO_QUERY: &str = r#"
    select
        id,
        user_id,
        name,
        status,
        device_os,
        registered_at,
        created_by,
        created_at,
        modified_by,
        modified_at
    from
        devices
    where
        id = $1
    "#;

#[async_trait]
impl DeviceRepository for DeviceRepo {
    async fn find_all(&self, pool: PgPool) -> Result<Vec<Device>, sqlx::Error> {
        let devices = sqlx::query_as::<_, Device>(
            r#"
            select
                id,
                user_id,
                name,
                status,
                device_os,
                registered_at,
                created_by,
                created_at,
                modified_by,
                modified_at
            from
                devices
            "#,
        )
        .fetch_all(&pool)
        .await?;

        Ok(devices)
    }

    async fn find_by_id(&self, pool: PgPool, id: String) -> Result<Option<Device>, sqlx::Error> {
        let device = sqlx::query_as::<_, Device>(FIND_DEVICE_INFO_QUERY)
            .bind(id)
            .fetch_optional(&pool)
            .await?;

        Ok(device)
    }

    async fn create(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        device: CreateDeviceDto,
    ) -> Result<Device, sqlx::Error> {
        let id = Uuid::new_v4().to_string();

        sqlx::query!(
            r#"
            INSERT INTO devices 
            (id, user_id, name, status, device_os, registered_at, created_by, created_at, modified_by, modified_at) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, now(), $8, now())
            "#,
            id.clone(),
            device.user_id.clone(),
            device.name.clone(),
            device.status.to_string(),
            device.device_os.to_string(),
            device.registered_at,
            device.modified_by.clone(),
            device.modified_by
        )
        .execute(&mut **tx)
        .await?;

        let inserted_device = sqlx::query_as::<_, Device>(FIND_DEVICE_INFO_QUERY)
            .bind(id)
            .fetch_one(&mut **tx)
            .await?;

        Ok(inserted_device)
    }

    async fn update(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: String,
        device: UpdateDeviceDto,
    ) -> Result<Option<Device>, sqlx::Error> {
        let existing = sqlx::query!(r#"SELECT id FROM devices WHERE id = $1"#, id)
            .fetch_optional(&mut **tx)
            .await?;

        if existing.is_some() {
            let mut builder = QueryBuilder::<_>::new("UPDATE devices SET ");

            builder.push(" modified_at = NOW()");

            if let Some(value) = device.user_id {
                builder.push(", user_id = ").push_bind(value);
            }
            if let Some(value) = device.name {
                builder.push(", name = ").push_bind(value);
            }
            if let Some(value) = device.status {
                builder.push(", status = ").push_bind(value.to_string());
            }
            if let Some(value) = device.device_os {
                builder.push(", device_os = ").push_bind(value.to_string());
            }
            if let Some(value) = device.registered_at {
                builder.push(", registered_at = ").push_bind(value);
            }

            builder
                .push(", modified_by = ")
                .push_bind(device.modified_by.clone());

            builder.push(" WHERE id = ").push_bind(&id);
            let query = builder.build();
            query.execute(&mut **tx).await?;

            let updated_device = sqlx::query_as::<_, Device>(FIND_DEVICE_INFO_QUERY)
                .bind(&id)
                .fetch_one(&mut **tx)
                .await?;

            return Ok(Some(updated_device));
        }

        Ok(None)
    }

    async fn update_many(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_id: String,
        modified_by: String,
        update_devices: UpdateManyDevicesDto,
    ) -> Result<(), sqlx::Error> {
        let mut builder = QueryBuilder::<_>::new(
            r#"
            INSERT INTO devices 
            (id, user_id, name, status, device_os, registered_at, created_by, created_at, modified_by, modified_at)
            "#,
        );

        let now = chrono::Utc::now().naive_utc();

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
            r#"
            ON CONFLICT (id) DO UPDATE SET
            name = EXCLUDED.name,
            status = EXCLUDED.status,
            device_os = EXCLUDED.device_os,
            modified_by = EXCLUDED.modified_by,
            modified_at = EXCLUDED.modified_at
            "#,
        );

        let query = builder.build();
        query.execute(&mut **tx).await?;

        Ok(())
    }

    async fn delete(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        id: String,
    ) -> Result<bool, sqlx::Error> {
        let res = sqlx::query!(r#"DELETE FROM devices WHERE id = $1"#, id)
            .execute(&mut **tx)
            .await?;

        Ok(res.rows_affected() > 0)
    }
}
