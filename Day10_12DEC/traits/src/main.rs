trait summary {
    fn summarize(&self) -> String ;
}


struct NewsArticle {
    headline: String ,
    location: String ,
    author: String ,
    content: String ,
}

struct Tweet {
    username : String ,
    content : String ,
    reply : bool ,
    retweet : bool ,
}

impl Summary for Tweet{
    fn summarize(&self) -> String {
        let content = format("tweet by {} : {}" , self.username ,self.content);
        content
    }
}

impl Summary for NewsArticle{
    fn summarize(&self) -> String {
        let content = format("news by {} : {}" , self.author ,self.content);
        content
    }
}

fn main() {
    println!("Hello, world!");

    let tweet = Tweet{
        username : String :: from ("adfdgfadfa") ,
        content : String :: from ("dfadfaf") , 
        reply : false  , 
        retweet : false, 
    };

    let newsarticle = NewsArticle{
        author: String:: from ("sfsdfsfs"),
        content : String :: from ("fafafafaf"),
        headline : String :: from ("dsgsgsg") ,
        location : String:: from ("sdnfsnfd"),
    };

    news_aggregator(tweet);
    news_aggregator(newsarticle);
}

fn news_aggregator(tweet : Tweet) {
    println!(" there is a news in the market ");
    println!("the news is {} and publish by {} " , tweet.content  , tweet.username);
}

fn news_aggregator_news(news : NewsArticle) {
    println!(" there is a news in the market ");
    println!("the news is {} and publish by{} " , news.content  , news.author);
}   


fn news_aggregator_tweet(source : impl Summary ) {
    println!(" there is a news in the market ");
    println!("{}" , source.summarize());
}   

