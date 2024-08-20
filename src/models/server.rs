use serde::Deserialize;

//* Структура ответа на POST запрос звонков и писем */
#[derive(Debug, Deserialize, Clone)]
pub struct RequestServer {
    pub date_start: String,
    pub date_end: String,
}

//* Структура ответа на POST запрос звонков и писем для одного клиента */
#[derive(Debug, Deserialize, Clone)]
pub struct RequestServerOneClient {
    pub date_start: String,
    pub date_end: String,
    pub id: i64,
}

//*Получение или активных или неактивных клиентов  */
#[derive(Debug, Deserialize, Clone)]
pub struct StatusClientList {
    pub status: String,
}
