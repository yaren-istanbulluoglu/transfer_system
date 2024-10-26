
fn main() {


    let mut database: Vec<Account> = Vec::new();



    register("Yaren".to_string(), 384838349, 394393853, &mut database);
    register("İbrahim".to_string(), 24923923, 934934993, &mut database);

    deposit(394393853, 1000.00, &mut database);

    transfer(394393853, 24923923, 10.00, &mut database);


    println!("{:?}",database)

}



#[derive(Debug,Clone)]
struct Account {
    name:String,
    account_id:u32,
    amount:f32,
    customer_no:u32
    
}


#[derive(Debug,Clone)]


enum Entry {
    normal_user,
    personal_user
}


fn register(name:String,account_id:u32,customer_no:u32,database:&mut Vec<Account>) {


    let account = Account{
        name:name,
        account_id:account_id,
        amount:0.0,
        customer_no:customer_no

    };

    database.push(account.clone());

    println!("{} Kayıdınız başarılı hoş geldiniz ",account.name)

    
}



fn show_amount(customer_no:u32,database:&mut Vec<Account>) {

    for data in database  {

        if customer_no==data.customer_no {

            println!("{} Bakiyeniz {}",data.name,data.amount)
            
        }else {
            println!("Bankamıza kayıtlı değilsiniz")
        }
        
    }    
}

fn deposit(customer_no:u32,amount:f32,database:&mut Vec<Account>) {

    for data in database  {
        if customer_no==data.customer_no {
            data.amount=data.amount+amount;

            println!("Yeni bakiyeniz {}",data.amount)
            
        }else {
            println!("Bankamıza kayıtlı değilsiniz")

            
        }
        
    }
    
}

fn withdraw(customer_no:u32,amount:f32,database:&mut Vec<Account>) {

    for data in database  {
        if customer_no==data.customer_no {
            data.amount =data.amount-amount;

            println!("Yeni bakiyeniz {}",data.amount)
            
        }
        else {
            println!("Kayıtlı değilsiniz")
        }
        
    }
    
}

fn transfer(customer_no:u32,account_id:u32,amount:f32,database:&mut Vec<Account>) {
    for data in database  {

        if customer_no==data.customer_no {

            data.amount=data.amount-amount;

            println!("Para gönderildi yeni baakiyeniz {}",data.amount)
            
        }

        if account_id==data.account_id {

            data.amount = data.amount+amount;
            println!("Hesabınıza para geldi yeni bakiyeniz {}",data.amount)
            
        }
        
    }
    
}