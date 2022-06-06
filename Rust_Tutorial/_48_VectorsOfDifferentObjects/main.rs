
trait Animal 
{   
    fn name(&self) -> &'static str; //Instance function
    fn talk(&self)                  //Instance function
    {
        println!("{} cannot talk",self.name());
    }
}
struct Human
{
    name: &'static str
}
impl Animal for Human
{   
    fn name(&self) -> &'static str
    {
        self.name
    }
    fn talk(&self)
    {
        println!("{} says hello",self.name());
    }
}

struct Cat
{
    name: &'static str
}
impl Animal for Cat
{   
    fn name(&self) -> &'static str
    {
        self.name
    }
    fn talk(&self)
    {
        println!("{} says miyav",self.name());
    }
}

enum Creature{
    Human(Human),
    Cat(Cat)
}
fn main(){
    
    // let mut creatures=Vec::new(); //let mut creatures:Vec<Human>=Vec::new()
    // creatures.push(Human{name:"John"});
    // //creatures.push(Cat{name:"Misha"}); //Gives Error

    let mut creatures=Vec::new();
    creatures.push(Creature::Human(Human{name:"John"}));
    creatures.push(Creature::Cat(Cat{name:"Misha"}));

    for c in creatures{
        //c.talk();//We can not write like that, it gives Error, because c is Creature which Enumaration type 
        match c{
            Creature::Human(h) => h.talk(),
            Creature::Cat(c) => c.talk()
        }
    }

    let mut animals:Vec<Box<Animal>> = Vec::new();
    animals.push(Box::new(Human{name:"John"}));
    animals.push(Box::new(Cat{name:"Misha"}));

    for a in animals.iter(){
        a.talk();
    }


}