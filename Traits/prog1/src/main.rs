

struct NewArticle{
    pub author:String,
    pub headline:String,
    pub content:String,


}

impl Summary for NewArticle{
    fn summary(&self)->String 
    {
        format!("{} by {}",self.headline,self.author)

    }
}


struct Tweet{
    pub username:String,
    pub content:String,
    pub _reply:bool,
    pub _retweet:bool,
}

impl Summary for Tweet{
    fn summary(&self)->String 
    {
        format!("{}:{}",self.username,self.content)
    }
}


trait Summary {
    fn summary(&self)->String;    
}


fn main() {
    let p=NewArticle{
        author:String::from("Rajj"),
        headline:String::from("headlines 1"),
        content:String::from("i went for the job"),
    
    };
    let p1=Tweet{
        username:String::from("Rajj"),
        content:String::from("headlines 1"),
        _reply:false,
        _retweet:false,
    
    };

        println!("{}",p.summary());
        println!("{}",p1.summary());
    


}
