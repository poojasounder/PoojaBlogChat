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

To run the code:
trunk serve

References:

https://yew.rs/docs/tutorial
https://pudding-entertainment.medium.com/rust-building-web-frontend-with-yew-ca7421fe01d4



