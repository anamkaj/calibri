use std::error::Error;
use serde::Serialize;
use sqlx::{FromRow, Pool, Postgres};

//* Таблица статусов клиентов */
#[derive(Debug, FromRow, Serialize)]
pub struct ClientCalibri {
    pub id: i32,
    pub site_id: i64,
    pub sitename: String,
    pub domains: String,
    pub active: String,
    pub license_start: Option<String>,
    pub license_end: Option<String>,
    pub not_enough_money: Option<bool>,
    pub number: Option<Vec<String>>,
}
//* Получение ID клиентов */
#[derive(Debug, FromRow, Serialize)]
pub struct ClientId {
    pub site_id: i64,
}

//*Таблица звонков */
#[derive(Debug, FromRow, Serialize)]
pub struct Calls {
    pub id: i32,
    pub call_id: i64,
    pub date: String,
    pub channel_id: i64,
    pub source: String,
    pub is_lid: bool,
    pub name_type: String,
    pub traffic_type: String,
    pub landing_page: String,
    pub conversations_number: i64,
    pub call_status: String,
}
//*Таблица писем */
#[derive(Debug, FromRow, Serialize)]
pub struct Email {
    pub id: i32,
    pub email_id: i64,
    pub date: String,
    pub source: String,
    pub is_lid: bool,
    pub traffic_type: String,
    pub landing_page: String,
    pub lid_landing: String,
    pub conversations_number: i64,
}
//* Финальная Таблица статистики по звонкам и письмам */
#[derive(Debug, FromRow, Serialize)]
pub struct AllCallsClient {
    pub calls: Vec<Calls>,
    pub email: Vec<Email>,
    pub site_id: i64,
}

impl ClientCalibri {
    //* Запрос на получение всех активных клиентов из таблицы (status = получение активных или не активных клиентов ) */
    pub async fn get_all_clients_status(
        pool: Pool<Postgres>,
        status: &str,
    ) -> Result<Vec<ClientCalibri>, Box<dyn Error>> {
        if status == "true" {
            let get_status: &str =
                    "SELECT 
                    cc.id,cc.site_id,cc.sitename,cc.domains,cc.active,cc.license_start,cc.license_end,cc.not_enough_money,ph.number 
                    FROM client_calibri cc JOIN phone ph ON cc.site_id=ph.client_calibri_site_id_fk 
                    WHERE active=$1";

            let client: Vec<ClientCalibri> = sqlx::query_as(&get_status)
                .bind(status)
                .fetch_all(&pool)
                .await?;
            println!("Полученино {} записи(ей)", client.len());

            Ok(client)
        } else {
            let get_status: &str = "SELECT 
                cc.id, 
                cc.site_id, 
                cc.sitename, 
                cc.domains, 
                cc.active, 
                cc.license_start, 
                cc.license_end, 
                cc.not_enough_money, 
                ph.number 
            FROM 
                client_calibri cc 
            LEFT JOIN 
                phone ph ON cc.site_id = ph.client_calibri_site_id_fk 
            WHERE 
                cc.active = $1";

            let client: Vec<ClientCalibri> = sqlx::query_as(&get_status)
                .bind(status)
                .fetch_all(&pool)
                .await?;
            println!("Полученино {} записи(ей)", client.len());

            Ok(client)
        }
    }
}

impl AllCallsClient {
    //* Получение и формирование массива с письмами и звонками */
    pub async fn get_calls(
        start: String,
        end: String,
        pool: Pool<Postgres>,
    ) -> Result<Vec<AllCallsClient>, Box<dyn Error>> {
        let start_date: String = format!("{}", start);
        let end_date: String = format!("{}", end);

        //? Финальный массив со звонками */
        let mut array_call: Vec<AllCallsClient> = Vec::new();

        let get_id: &str = "SELECT site_id FROM client_calibri WHERE active=$1";

        let get_all_calls: String = format!(
            "
            SELECT *
            FROM calls
            WHERE calls.client_calibri_site_id_fk = $1
            AND 
            TO_TIMESTAMP(calls.date, 'YYYY-MM-DD\"T\"HH24:MI:SS.USZ') 
            BETWEEN 
            TO_TIMESTAMP('{}T00:00:00.000Z', 'YYYY-MM-DD\"T\"HH24:MI:SS.USZ')
            AND 
            TO_TIMESTAMP('{}T23:59:59.999Z', 'YYYY-MM-DD\"T\"HH24:MI:SS.USZ');",
            start_date, end_date
        );

        let get_all_email: String = format!(
            "SELECT * FROM email 
            WHERE email.client_calibri_site_id_fk = $1
            AND 
            TO_TIMESTAMP(email.date, 'YYYY-MM-DD\"T\"HH24:MI:SS.USZ') 
            BETWEEN 
            TO_TIMESTAMP('{}T00:00:00.000Z', 'YYYY-MM-DD\"T\"HH24:MI:SS.USZ')
                AND 
                    TO_TIMESTAMP('{}T23:59:59.999Z', 'YYYY-MM-DD\"T\"HH24:MI:SS.USZ');",
            start_date, end_date
        );

        //*Получение всех ID активных клиентов */
        let id_client: Vec<ClientId> = sqlx::query_as(&get_id)
            .bind("true")
            .fetch_all(&pool)
            .await?;

        //? Финальный массив со звонками и письмами */
        for id in id_client {
            let data_calls: Vec<Calls> = sqlx::query_as(&get_all_calls)
                .bind(id.site_id)
                .fetch_all(&pool)
                .await
                .expect("Ошибка получения звонков из ДБ");

            let data_email: Vec<Email> = sqlx::query_as(&get_all_email)
                .bind(id.site_id)
                .fetch_all(&pool)
                .await
                .expect("Ошибка получения писем из ДБ");

            array_call.push(AllCallsClient {
                calls: data_calls,
                email: data_email,
                site_id: id.site_id,
            });
        }

        Ok(array_call)
    }

    //*Получение звонков и писем по одному клиенту */
    pub async fn get_one_calls(
        start: String,
        end: String,
        id: i64,
        pool: Pool<Postgres>,
    ) -> Result<Vec<AllCallsClient>, Box<dyn Error>> {
        let start_date = format!("{}", start);
        let end_date = format!("{}", end);

        //? Финальный массив со звонками */
        let mut array_call: Vec<AllCallsClient> = Vec::new();

        let get_all_calls: String = format!(
            "SELECT *
            FROM calls
            WHERE calls.client_calibri_site_id_fk = $1
            AND 
            TO_TIMESTAMP(calls.date, 'YYYY-MM-DD\"T\"HH24:MI:SS.USZ') 
            BETWEEN 
            TO_TIMESTAMP('{}T00:00:00.000Z', 'YYYY-MM-DD\"T\"HH24:MI:SS.USZ')
            AND 
            TO_TIMESTAMP('{}T23:59:59.999Z', 'YYYY-MM-DD\"T\"HH24:MI:SS.USZ');",
            start_date, end_date
        );

        let get_all_email: String = format!(
            "SELECT * FROM email 
            WHERE email.client_calibri_site_id_fk = $1
            AND 
            TO_TIMESTAMP(email.date, 'YYYY-MM-DD\"T\"HH24:MI:SS.USZ') 
            BETWEEN 
            TO_TIMESTAMP('{}T00:00:00.000Z', 'YYYY-MM-DD\"T\"HH24:MI:SS.USZ')
            AND 
            TO_TIMESTAMP('{}T23:59:59.999Z', 'YYYY-MM-DD\"T\"HH24:MI:SS.USZ');",
            start_date, end_date
        );

        //? Финальный массив со звонками и письмами */
        let data_calls: Vec<Calls> = sqlx::query_as(&get_all_calls)
            .bind(id)
            .fetch_all(&pool)
            .await
            .expect("Ошибка получения звонков из ДБ");

        let data_email: Vec<Email> = sqlx::query_as(&get_all_email)
            .bind(id)
            .fetch_all(&pool)
            .await
            .expect("Ошибка получения писем из ДБ");

        array_call.push(AllCallsClient {
            site_id: id,
            calls: data_calls,
            email: data_email,
        });

        Ok(array_call)
    }
}
