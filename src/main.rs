#![allow(non_snake_case)]

use trpl::{Either,Html};

fn main
()
{
    let args: Vec<String> = std::env::args().collect();
    trpl::run
    (
        async
        {
            let titleFuture1 = pageTitle(&args[1]);
            let titleFuture2 = pageTitle(&args[2]);

            let (url,maybeTitle) =
            match trpl::race(titleFuture1,titleFuture2).await
            {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

            println!("{url} returned first");
            match maybeTitle
            {
                Some(title) => println!("Its page title is: '{title}'"),
                None        => println!("Its title could not be parsed"),
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
/*
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
*/

async fn pageTitle
(url: &str) -> (&str, Option<String>)
{
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
    .select_first("title")
    .map(|title| title.inner_html());

    (url,title)
}
