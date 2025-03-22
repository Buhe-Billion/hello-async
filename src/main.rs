#![allow(non_snake_case)]

fn main
()
{

}

use trpl::Html;

/*
async fn pageTitle
(url: &str) -> Option<String>
{   /*
    let response = trpl::get(url).await;
    let responseText = response.text().await;
    */
    let responseText = trpl::get(url).await.text().await;
    Html::parse(&responseText)
    .select_first("title")
    .map(|titleElement| titleElement.inner_html())
}
*/

fn pageTitle
(url: &str) -> impl Future<Output = Option<String>> + '_
{
    async move
    {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html())
    }
}
