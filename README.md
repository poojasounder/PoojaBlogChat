# PoojaBlogChat

# A blog Website with user authentication

# Used Yew Framework with rocket in the backend.

# Used Postgress for database


I tried to implement the backend logic with rocket but I was not able to figure out the authenication logic yet that allows the user to login
given the time constraint. I am still working on it to be able to figure it out. Since I spend most of my time in the backend, I was not able to style my
website very well and I just wrote basic html code for the forms.

However, I do have a basic knowledge of how we can integrate backend with frontend and how we can communicate with the databases.It was a very interesting project and I will surely give it more thought and would invest more time into this.

I faced problem in solving the error when the API request is give me a 405 Status code. Even though my routes are right, I had no idea why was I getting that error. That is something which I need to look into as well. I tried different tools for backend. I tried both rocket as well as actix web actually but i needed more time to figure out actix web.

I tried to add essential authentication features such as user sign-up, login, logout, and restricting access to protected pages. I was not able to connect my backend with my frontend and as i said, my styling is not so appealing for now. But when you click on each of the link provided, it will take you to the respective forms. I created some component for functionality purpose.The first Yew component is the Spinner, which will be used to indicate when a request is in progress or being processed by the backend server.But however, I couldn't test it out fully because my backend was not working at all.

I did unit testing and testing each part of the program separately. I am able to create an user, list all users and delete an user from the database manually.

Tests
pooja@Poojas-MBP backend % docker-compose exec app cargo run --bin client users create admin2 1234 admin
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/client users create admin2 1234 admin`
User created User { id: 1, username: "admin2", password: "$argon2id$v=19$m=19456,t=2,p=1$cHidXUPAmn/oUjJxIKiEeA$OgArvkPZp20vkIpiDaZDRi6wxNp0vANk+yNn1Y5hFqg", created_at: 2023-12-04T05:35:27.215785 }
Roles assigned [Role { id: 1, code: "admin", name: "admin", created_at: 2023-12-04T05:35:27.219904 }]

pooja@Poojas-MBP backend % docker-compose exec app cargo run --bin client users list
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/client users list`
(User { id: 1, username: "admin2", password: "$argon2id$v=19$m=19456,t=2,p=1$cHidXUPAmn/oUjJxIKiEeA$OgArvkPZp20vkIpiDaZDRi6wxNp0vANk+yNn1Y5hFqg", created_at: 2023-12-04T05:35:27.215785 }, [(UserRole { id: 1, user_id: 1, role_id: 1 }, Role { id: 1, code: "admin", name: "admin", created_at: 2023-12-04T05:35:27.219904 })])

pooja@Poojas-MBP backend % docker-compose exec app cargo run --bin client users delete 1
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/client users delete 1`

To run the code:
trunk serve

References:

https://yew.rs/docs/tutorial
https://pudding-entertainment.medium.com/rust-building-web-frontend-with-yew-ca7421fe01d4



