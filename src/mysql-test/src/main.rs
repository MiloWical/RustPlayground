// This assumes the following setup:
//
// 1) A MySQL-compliant database running on localhost:3306
// 2) A root user with the password "password"
//
// If this isn't accurate to your testing config,
// change the URL on line 21 below.

use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

fn main() -> Result<()>{
  println!("Hello from mysql-test!");
  let url = "mysql://root:password@localhost:3306";

  println!("Connection string: {}", url);

  let pool = Pool::new(url)?;
  let mut conn = pool.get_conn()?;

  conn.query_drop(
    r"CREATE DATABASE IF NOT EXISTS Test")?;

  conn.query_drop(
    r"USE Test")?;

  let table_count:Option<i32> = conn.query_first(r"SELECT COUNT(*) AS table_count
  FROM information_schema.tables 
  WHERE table_schema = 'Test'
  AND table_name = 'payment';",)?;

  println!("Table count: {}", table_count.unwrap());

  let payments = vec![
    Payment { customer_id: 1, amount: 2, account_name: None },
    Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
    Payment { customer_id: 5, amount: 6, account_name: None },
    Payment { customer_id: 7, amount: 8, account_name: None },
    Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
  ];

  // If the table doesn't exist, create it.
  if table_count.unwrap() == 0 {
    create_table(&mut conn, &payments);
  }

  // Let's select payments from database. Type inference should do the trick here.
  let selected_payments = conn
      .query_map(
          "SELECT customer_id, amount, account_name from payment",
          |(customer_id, amount, account_name)| {
              Payment { customer_id, amount, account_name }
          },
      )?;

  // Let's make sure, that `payments` equals to `selected_payments`.
  // Mysql gives no guaranties on order of returned rows
  // without `ORDER BY`, so assume we are lucky.
  assert_eq!(payments, selected_payments);
  println!("Yay!");

  Ok(())
}

fn create_table(conn: &mut PooledConn, payments: &Vec<Payment>) {
  // Let's create a table for payments.
  conn.query_drop(
    r"CREATE TABLE payment (
      customer_id int not null,
      amount int not null,
      account_name text
    )").unwrap();  

  // Now let's insert payments to the database
  conn.exec_batch(
      r"INSERT INTO payment (customer_id, amount, account_name)
        VALUES (:customer_id, :amount, :account_name)",
      payments.iter().map(|p| params! {
          "customer_id" => p.customer_id,
          "amount" => p.amount,
          "account_name" => &p.account_name,
      })
  ).unwrap();
}