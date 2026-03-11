
trait SayHi { fn hi(self); }
trait SayHello { fn hi(self); }
trait Hello { fn hi(self); }

fn _stuff<'a,T>(value: &'a mut T)
    where   &'a mut T:SayHello,
            T:SayHi
{
    value.hi(); 
}

struct MyType{}

impl MyType{
    fn hi(&self){
        println!("hi from struct");
    }
}

/* Because of orphan rule, you can't impl a primitive type that is not defined here

impl &MyType{
    fn hi(self){
        println!("hi from struct");
    }
}

impl &mut MyType{
    fn hi(self){
        println!("hi from struct");
    }
}
*/

impl SayHi for MyType{
    fn hi(self) {
        println!("hi from SayHi");
    }

    /* duplicate definitions with anme 'hi' 
    fn hi(&self) {
        println!("hi from SayHi");
    }
    */
}

impl Hello for &MyType{
    fn hi(self) {
        println!("hi from hello");
    }
}


impl SayHello for &mut MyType{
    fn hi(self) {
        println!("hi from sayhello");
    }
}


use std::any::type_name;
fn _print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}

fn main() {
    let mut var=MyType{};
    _stuff(&mut var);
    (&var).hi();
}





/*


fn do_stuff<T: Clone>(value: &T) {
    let cloned = value.clone();
}

fn do_stuff2<T>(value: &T) {
    let cloned = value.clone();
}



struct Foo {}

trait Bar {

  fn bar(&self);

}impl Foo {

  fn bar(&self) {

    println!("In struct impl!")

  }

}impl Bar for Foo {

  fn bar(&self) {

    println!("In trait impl!")

  }

}fn main() {

  let mut f = Foo{};

  f.bar(); // In trait impl

}


  */