#![allow(non_snake_case)]

use trpl::Html;

fn main
()
{
    let args: Vec<String> = std::env::args().collect();
    trpl::run
    (
        async
        {
            let url = &args[1];
            match pageTitle(url).await
            {
                Some(title) => println!("The title for {url} was {title}"),
                None        => println!("{url} had no title"),
            }
        }
    )
}

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
//these 2 functions are equivalent
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
