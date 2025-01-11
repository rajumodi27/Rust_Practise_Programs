struct Person {
    name:String,
    age:i32
}

impl Person{
    fn new(name:String,age:i32)->Self{
            Self{
                name,age
            }
    }


    fn set_details(&mut self,name:String,age:i32)->Self
    {
        Self{
            name,age
        }

        }


    fn prin_det(&self){
            println!("{} {}",self.name,self.age);
    }
}

fn main()
{
let mut p1=Person::new("Raju".to_string(),27);
p1.prin_det();
p1=p1.set_details("rajumodi".to_string(), 28);
p1.prin_det();

}