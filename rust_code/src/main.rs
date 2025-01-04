fn main() {
    /*let num:u8=5;
    let tup:(&str,u8)=("ruhi",5);
    let name=tup.0;
    let age=tup.1;
    println!("employee name={}, age={}",name,age);

    println!("num is {}",num);
    */



    let s1:String=String::from("hello");
    let (s1,len)=cal_len(s1);
    println!("len of {} is {}",s1,len);
}

fn cal_len(s:String)->(String,usize){
    let len:usize=s.len();
    return (s,len);

}
/*ownership say --> har ek ka ,aur ek hi owner hoga 
ownership has --> transfer of owneship , cloning of wonership, borrowing of ownership(ie via using reference techniques)

in reference technique ,no changes can be made in main thing that is being borrowed ,only reading can be done but 
for writting one thing can be done ie using keyword mut everywhere along with that string.So through mut changes can be made.

in reference technique,
for writing --> when we r referenceing same object to more than 1 borrower than at a time only one can work on it,also when 
next one starts to work on it ,the previous ones cant access that object then.
for reading -->  when we r referenceing same object to more than 1 borrower than all can access that object anytime when needed 
as they r only reading the objcet and not making any chnages to it.
*/

